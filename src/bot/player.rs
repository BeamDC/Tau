use rust_sc2::prelude::*;
use crate::bot::Bot;

impl Player for Bot {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings {
            name: Some("Tau"),
            race: Race::Terran,
            raw_affects_selection: false,
            raw_crop_to_playable_area: false
        }
    }

    fn on_start(&mut self) -> SC2Result<()> {
        Ok(())
    }

    fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
        self.mining_roles();
        self.mining_micro();
        Ok(())
    }

    // Called once on last step
    // "result" says if your bot won or lost game
    fn on_end(&self, _result: GameResult) -> SC2Result<()> {
        /* code here */
        Ok(())
    }

    ///`https://github.com/UltraMachine/rust-sc2/blob/master/examples/events.rs`
    fn on_event(&mut self, event: Event) -> SC2Result<()> {
        match event {
            Event::UnitCreated(tag) => {
                // label new workers as free
                self.tag_unit(tag);
            }
            Event::ConstructionComplete(tag) => {
                // add track all bases that we build
                self.tag_building(tag);
            }
            Event::UnitDestroyed(tag, alliance) => {
                let remove_mineral = |bot: &mut Bot, tag| {
                    if let Some(ws) = bot.assigned.remove(&tag) {
                        for w in ws {
                            bot.harvesters.remove(&w);
                            bot.free_workers.insert(w);
                        }
                    }
                };

                match alliance {
                    Some(Alliance::Own) => {
                        // townhall destroyed
                        if let Some(idx) = self.base_indices.remove(&tag) {
                            let exp = &self.expansions[idx];
                            for m in exp.minerals.clone() {
                                remove_mineral(self, m);
                            }
                        // harvester died
                        } else if let Some((m, _)) = self.harvesters.remove(&tag) {
                            self.assigned.entry(m).and_modify(|ws| {
                                ws.remove(&tag);
                            });
                        // free worker died
                        } else {
                            self.free_workers.remove(&tag);
                        }
                    }
                    // mineral mined out
                    Some(Alliance::Neutral) => remove_mineral(self, tag),
                    Some(Alliance::Enemy) => {}
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}
