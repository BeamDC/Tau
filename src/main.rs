mod bot;
mod constants;

use rust_sc2::prelude::*;
use bot::Bot;

fn main() -> SC2Result<()> {
    let mut bot = Bot::default();
    //VeryEasy, Easy, Medium, Hard, VeryHard
    let computer = Computer::new(Race::Random, Difficulty::VeryEasy, Some(AIBuild::RandomBuild));
    let options = LaunchOptions {
        realtime: false,
        ..Default::default()
    };
    run_vs_computer(&mut bot, computer, "Simple64", options)
}
