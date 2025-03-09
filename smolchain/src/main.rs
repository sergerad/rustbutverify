//! A small, simple blockchain.
//!
//! Functional requirements
//! - Receives transactions
//! - Transaction pool
//! - Blocks are mined from transactions
//!
//! Non-functional requirements
//! - Not important...
//!
//! Key entities
//! - Transactions
//! - Blocks
//! - Signatures
//! - Hashes
//! - Sequencer
//! - Mempool
//! - RPC
//!
//! Runtime components
//! - Sequencer
//! - RPC (server)
//!
//! State
//! - Transaction pool (rpc writes, sequencer reads)
//! - Blocks (sequencer writes, rpc reads)
//!
//! Practical considerations
//! - RPC server in practice will use a threadpool
//! - Ideally RPC requests don't block on mempool directly (backpressure)
//! - ...

use std::{
    collections::VecDeque,
    hash::Hash,
    sync::mpsc::{Receiver, Sender},
};
use tracing::{info, warn};

#[derive(Debug, serde::Serialize, Hash)]
struct Transaction {
    nonce: u32,
}

impl Transaction {
    pub fn random() -> Self {
        Self {
            nonce: rand::random(),
        }
    }
}

pub struct Blake3(blake3::Hasher);

impl std::hash::Hasher for Blake3 {
    fn write(&mut self, input: &[u8]) {
        self.0.update(input);
    }
    fn finish(&self) -> u64 {
        self.0.finalize();
        0
    }
}

impl Blake3 {
    pub fn output(&self) -> blake3::Hash {
        self.0.finalize()
    }
}

impl From<&Transaction> for Vec<u8> {
    fn from(tx: &Transaction) -> Self {
        bincode::serialize(tx).unwrap()
    }
}

#[derive(Debug)]
struct Block {
    hash: blake3::Hash,
    txs: Vec<Transaction>,
}

impl Block {
    pub fn new(txs: impl Iterator<Item = Transaction>) -> Self {
        let (txs, hasher) = txs.fold(
            (Vec::new(), Blake3(blake3::Hasher::new())),
            |(mut txs, mut hasher), tx| {
                tx.hash(&mut hasher);
                //hasher.update(tx.into);
                txs.push(tx);
                (txs, hasher)
            },
        );
        Self {
            txs,
            hash: hasher.output(),
        }
    }
}

#[derive(Debug)]
struct Rpc {
    tx: Sender<Message>,
}

impl Rpc {
    pub fn new(tx: Sender<Message>) -> Self {
        Self { tx }
    }

    pub fn run(mut self) {
        let delay = std::time::Duration::from_secs(1);
        loop {
            let msg = Message::NewTx(Transaction::random());
            self.send(msg);
            std::thread::sleep(delay);
        }
    }

    #[tracing::instrument()]
    fn send(&mut self, msg: Message) {
        if let Err(e) = self.tx.send(msg) {
            warn!("Failed to send transaction: ({e})");
        }
    }
}

#[derive(Debug)]
struct Sequencer {
    rx: Receiver<Message>,
    pool: VecDeque<Transaction>,
    chain: Vec<Block>,
}

impl Sequencer {
    pub fn new(rx: Receiver<Message>) -> Self {
        Self {
            rx,
            pool: Default::default(),
            chain: Default::default(),
        }
    }

    pub fn run(mut self) {
        loop {
            let next_block_time = &std::time::Instant::now()
                .checked_add(std::time::Duration::from_secs(4))
                .unwrap();
            while std::time::Instant::now().le(next_block_time) {
                self.recv();
            }
            self.mine();
        }
    }

    #[tracing::instrument(ret)]
    fn recv(&mut self) {
        match self.rx.recv_timeout(std::time::Duration::from_secs(1)) {
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => info!("No message"),
            Err(e) => warn!("Failed to receive message {e}"),
            Ok(msg) => match msg {
                Message::NewTx(tx) => {
                    self.pool.push_back(tx);
                }
                _ => todo!(),
            },
        }
    }

    #[tracing::instrument(ret)]
    fn mine(&mut self) {
        let block = Block::new(self.pool.drain(..));
        self.chain.push(block);
    }
}

#[derive(Debug)]
enum Message {
    NewTx(Transaction),
    GetLatestBlock,
    LatestBlock(Block),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let (tx, rx) = std::sync::mpsc::channel::<Message>();

    let handles = vec![
        std::thread::spawn(|| Rpc::new(tx).run()),
        std::thread::spawn(|| Sequencer::new(rx).run()),
    ];

    for h in handles {
        h.join().map_err(|_| eyre::eyre!("Failed on join"))?;
    }

    Ok(())
}
