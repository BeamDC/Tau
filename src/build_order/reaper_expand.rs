use rust_sc2::prelude::UnitTypeId;
use crate::build_order::{BuildAction, BuildCondition, BuildOrder, BuildStep};

impl BuildOrder {
    pub fn reaper_expand() -> BuildOrder {
        BuildOrder::new(
            "Reaper Expand",
            vec![
                BuildStep {
                    action: BuildAction::Build(UnitTypeId::SupplyDepot),
                    condition: BuildCondition::Supply(14),
                },
            ]
        )
    }
}