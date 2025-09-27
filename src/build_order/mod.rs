use rust_sc2::ids::{UnitTypeId, UpgradeId};

pub(crate) mod reaper_expand;

#[derive(Default, Clone)]
pub enum BuildAction {
    #[default]
    None,                // do nothing
    Train(UnitTypeId),   // train a unit
    Build(UnitTypeId),   // construct a building
    Upgrade(UpgradeId),  // research an upgrade
}


#[derive(Default, Clone)]
pub enum BuildCondition {
    #[default]
    None,
    Supply(u32),
    Minerals(u32),
    Gas(u32),
    BuildComplete(UnitTypeId),
    TrainingComplete(UnitTypeId),
    ResearchComplete(UpgradeId),
}

#[derive(Default, Clone)]
pub struct BuildStep {
    pub action: BuildAction,
    pub condition: BuildCondition,
}

#[derive(Default, Clone)]
pub struct BuildOrder {
    pub steps: Vec<BuildStep>,
    pub followup: Vec<BuildStep>,
}