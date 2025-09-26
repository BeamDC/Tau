use std::collections::{HashMap, HashSet};
use std::time::Instant;
use rust_sc2::bot;
use crate::build_order::BuildOrder;

pub(crate) mod player;
pub(crate) mod speed_mining;
pub(crate) mod unit_creation;

#[bot]
#[derive(Default)]
pub struct Bot {
    build_order: BuildOrder,

    base_indices: HashMap<u64, usize>,    // (base tag, expansion index)
    assigned: HashMap<u64, HashSet<u64>>, // (mineral, workers)
    free_workers: HashSet<u64>,           // tags of workers which aren't assigned to any work
    harvesters: HashMap<u64, (u64, u64)>, // (worker, (target mineral, nearest townhall))
}