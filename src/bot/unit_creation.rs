use rust_sc2::prelude::UnitTypeId;
use crate::bot::Bot;

impl Bot {
    pub fn tag_unit(&mut self, tag: u64) {
        if let Some(u) = self.units.my.units.get(tag) {
            match u.type_id() {
                UnitTypeId::SCV => {
                    self.free_workers.insert(tag);
                }
                _ => {}
            }
        }
    }

    pub fn tag_building(&mut self, tag: u64) {
        if let Some(u) = self.units.my.structures.get(tag) {
            // label new bases
            if u.type_id() == self.race_values.start_townhall {
                if let Some(idx) = self
                    .expansions
                    .iter()
                    .enumerate()
                    .find(|(_, exp)| exp.base == Some(tag))
                    .map(|(idx, _)| idx)
                {
                    self.base_indices.insert(tag, idx);
                }
                return
            }

            // label all other buildings
            match u.type_id() {
                _ => {}
            }
        }
    }
}
