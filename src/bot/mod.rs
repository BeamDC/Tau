use std::collections::{HashMap, HashSet};
use rust_sc2::bot;

pub(crate) mod player;
pub(crate) mod speed_mining;
mod unit_creation;

#[bot]
#[derive(Default)]
pub struct Bot {
    base_indices: HashMap<u64, usize>,    // (base tag, expansion index)
    assigned: HashMap<u64, HashSet<u64>>, // (mineral, workers)
    free_workers: HashSet<u64>,           // tags of workers which aren't assigned to any work
    harvesters: HashMap<u64, (u64, u64)>, // (worker, (target mineral, nearest townhall))
}
