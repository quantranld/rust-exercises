pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => {
                let mana = match self.level {
                    1..=9 => None,
                    10.. => Some(100),
                    _ => self.mana
                };

                return Some(Self { health: 100, mana: mana, level: self.level });
            },
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana >= mana_cost {
                    self.mana = Some(mana - mana_cost);
                    return mana_cost * 2;
                }

                return 0;
            },
            None => {
                if self.health >= mana_cost {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }

                return 0;
            }
        }
    }
}
