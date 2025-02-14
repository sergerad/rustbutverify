use std::{collections::VecDeque, thread::sleep, time::Duration};

use rand::Rng;

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    // ..
    #[error("Frame id does not match channel id")]
    FrameIdMismatch,
    // ..
    #[error("Player count {0} is unexpected")]
    PlayerCountError(usize),
}

trait Ticker: std::fmt::Debug {
    fn tick(&mut self);
    fn is_done(&self) -> bool;
}

#[derive(Debug, Default)]
struct Chain<T: Ticker> {
    games: VecDeque<T>,
}

impl<T: Ticker> Chain<T> {
    #[tracing::instrument(ret)]
    fn add_game(&mut self, game: T) {
        self.games.push_back(game);
    }
}

impl<T: Ticker> Ticker for Chain<T> {
    #[tracing::instrument(ret)]
    fn tick(&mut self) {
        for game in self.games.iter_mut() {
            game.tick();
        }
    }
    #[tracing::instrument(ret)]
    fn is_done(&self) -> bool {
        self.games.iter().all(|game| game.is_done())
    }
}

#[derive(Debug, Default)]
struct Game {
    players: Vec<Player>,
    state: GameState,
}

impl Game {
    #[tracing::instrument(ret)]
    fn tick(&mut self) {
        if let GameState::InProgress(num) = self.state {
            for player in self.players.iter() {
                player.strategy.play(&mut self.state);
            }
            if num == 0 {
                self.state = GameState::Finished;
            }
        }
    }

    #[tracing::instrument(ret)]
    fn add_player(&mut self, player: Player) {
        self.players.push(player);
        if self.players.len() > 1 {
            self.state = GameState::InProgress(100);
        }
    }
}

impl Ticker for Game {
    fn tick(&mut self) {
        self.tick();
    }
    fn is_done(&self) -> bool {
        self.state == GameState::Finished
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
enum GameState {
    #[default]
    WaitingForPlayers,
    InProgress(usize),
    Finished,
}

#[derive(Debug, Default)]
struct Player {
    strategy: Box<dyn Strategy>,
}

trait Strategy {
    fn play(&self, game: &mut GameState);
}

impl std::default::Default for Box<dyn Strategy> {
    fn default() -> Self {
        Box::new(RandomStrategy::default())
    }
}

impl std::fmt::Debug for Box<dyn Strategy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box<dyn Strategy>")
    }
}

#[derive(Debug, Default)]
struct RandomStrategy {}

impl Strategy for RandomStrategy {
    #[tracing::instrument(ret)]
    fn play(&self, game: &mut GameState) {
        if let GameState::InProgress(num) = game {
            let mut rng = rand::thread_rng();
            let sub = rng.gen_range(0..20);
            *num = num.saturating_sub(sub);
        }
    }
}

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
    let mut game = Game::default();
    game.add_player(Player::default());
    game.add_player(Player::default());
    let mut chain: Chain<Game> = Chain::default();
    chain.add_game(game);
    while !chain.is_done() {
        chain.tick();
        sleep(Duration::from_secs(1));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_players() {
        let mut game = super::Game::default();
        game.add_player(super::Player::default());
        assert_eq!(game.players.len(), 1);
        assert_eq!(game.state, super::GameState::WaitingForPlayers);
        game.add_player(super::Player::default());
        assert_eq!(game.players.len(), 2);
        assert!(matches!(game.state, super::GameState::InProgress(_)));
    }
}
