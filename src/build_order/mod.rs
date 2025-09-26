use rust_sc2::ids::{UnitTypeId, UpgradeId};

pub(crate) mod reaper_expand;

#[derive(Default, Clone)]
pub enum BuildAction {
    #[default]
    None,                // do nothing
    Train(UnitTypeId),   // train a unit
    Build(UnitTypeId),   // construct a building
    Research(UpgradeId), // research an upgrade
    // Upgrade(_),          // upgrade a building
}

#[derive(Default, Clone)]
pub enum BuildCondition {
    #[default]
    None,
    Supply(u32),
    Minerals(u32),
    Gas(u32),
}

#[derive(Default, Clone)]
pub struct BuildStep {
    pub action: BuildAction,
    pub condition: BuildCondition,
}

#[derive(Default, Clone)]
pub struct BuildOrder {
    pub name: String,
    pub steps: Vec<BuildStep>,
}

impl BuildOrder {
    pub fn new(name: &str, steps: Vec<BuildStep>) -> Self {
        // treat steps as a stack
        let steps = steps
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<BuildStep>>();

        Self {
            name: name.to_string(),
            steps,
        }
    }
}