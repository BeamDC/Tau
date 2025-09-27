use std::collections::{HashMap, HashSet};
use std::time::Instant;
use rust_sc2::bot;
use rust_sc2::player::Race;
use crate::bot::r#macro::BuildManager;
use crate::build_order::BuildOrder;

pub(crate) mod player;
pub(crate) mod speed_mining;
pub(crate) mod unit_creation;
mod r#macro;

#[bot]
#[derive(Default)]
pub struct Bot {
    base_indices: HashMap<u64, usize>,    // (base tag, expansion index)
    assigned: HashMap<u64, HashSet<u64>>, // (mineral, workers)
    free_workers: HashSet<u64>,           // tags of workers which aren't assigned to any work
    harvesters: HashMap<u64, (u64, u64)>, // (worker, (target mineral, nearest townhall))
}
impl BuildManager for Bot {
    fn get_current_supply(&self) -> u32 {
        self.supply_used
    }
    fn get_max_supply(&self) -> u32 {
        self.supply_cap
    }

    fn get_free_supply(&self) -> u32 {
        self.supply_left
    }

    fn get_minerals(&self) -> u32 {
        self.minerals
    }
    fn get_gas(&self) -> u32 {
        self.vespene
    }
}