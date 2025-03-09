use crate::prelude::*;
use crate::{PoolUpdate, Subscribe};
use actix::prelude::*;

use super::Control;

/// Monitor is the actor responsible for monitoring on-chain state
/// and notifying subscribers of changes.
#[derive(Clone)]
pub struct Monitor {
    price_diff_subs: Vec<Recipient<PoolUpdate>>,
    provider: RootProvider,
    factory: Address,
    token_0: Address,
    token_1: Address,
}

impl Monitor {
    pub fn new(
        provider: RootProvider,
        factory: Address,
        token_0: Address,
        token_1: Address,
    ) -> Self {
        Monitor {
            price_diff_subs: vec![],
            provider,
            factory,
            token_0,
            token_1,
        }
    }
}

impl Actor for Monitor {
    type Context = Context<Self>;
}

impl Handler<Subscribe<PoolUpdate>> for Monitor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe<PoolUpdate>, _: &mut Self::Context) {
        self.price_diff_subs.push(msg.0);
    }
}

impl Handler<Control> for Monitor {
    type Result = ();

    fn handle(&mut self, control: Control, ctx: &mut Self::Context) {
        match control {
            Control::Stop => {
                println!("Stopping Monitor");
                ctx.stop();
            }
            Control::Tick => {
                let self_clone = self.clone();
                async move {
                    for sub in self_clone.price_diff_subs {
                        sub.do_send(PoolUpdate {});
                    }
                }
                .into_actor(self)
                .wait(ctx);
            }
        }
    }
}
