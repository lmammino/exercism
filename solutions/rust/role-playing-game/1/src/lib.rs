// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        let mana = if self.level < 10 { None } else { Some(100) };

        Some(Player {
            health: 100,
            mana,
            ..*(self.clone())
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana >= mana_cost {
                    self.mana = Some(mana - mana_cost);
                    return mana_cost * 2;
                }
            }
            None => {
                self.health = self.health.checked_sub(mana_cost).unwrap_or_default();
            }
        };

        0
    }
}
