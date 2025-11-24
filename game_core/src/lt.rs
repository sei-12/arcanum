use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{
    Num, SkillCooltimeNum,
    chars::StaticCharData,
    container_args::{CharData, EnemyData},
    enemy_ai::StaticEnemyData,
    error::GameError,
    passive::PassiveList,
    skills::{ActiveSkillState, StaticActiveSkill, StaticSkillId},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Potential {
    pub int: Num,
    pub vit: Num,
    pub str: Num,
    pub dex: Num,
    pub agi: Num,
}

#[derive(Debug, Clone)]
pub struct LtCommon {
    pub passive: PassiveList,
    potential: &'static Potential,
    pub level: Num,
    pub hp: Num,
    pub is_enemy: bool,
}

impl LtCommon {
    fn new(potential: &'static Potential, level: Num, is_enemy: bool) -> Self {
        let mut tmp = Self {
            passive: PassiveList::default(),
            potential,
            level,
            hp: 0.0,
            is_enemy,
        };

        tmp.hp = tmp.max_hp();

        tmp
    }
}

#[derive(Debug, Clone)]
pub struct Enemy {
    cmn: LtCommon,
    pub static_data: &'static StaticEnemyData,
}

impl Deref for Enemy {
    type Target = LtCommon;
    fn deref(&self) -> &Self::Target {
        &self.cmn
    }
}
impl DerefMut for Enemy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cmn
    }
}
impl TryFrom<&EnemyData> for Enemy {
    type Error = GameError;
    fn try_from(value: &EnemyData) -> Result<Self, Self::Error> {
        let static_data =
            StaticEnemyData::get(value.static_enemy_id).ok_or(GameError::InvalidEnemyId)?;
        let cmn = LtCommon::new(&static_data.potential, value.level, true);

        Ok(Self { cmn, static_data })
    }
}
#[derive(Debug, Clone)]
pub struct Char {
    pub static_data: &'static StaticCharData,
    cmn: LtCommon,
    pub skills: Vec<ActiveSkillState>,
    pub hate: Num,
}

impl TryFrom<&CharData> for Char {
    type Error = GameError;
    fn try_from(value: &CharData) -> Result<Self, Self::Error> {
        let static_char_data =
            StaticCharData::get(value.static_char_id).ok_or(GameError::InvalidCharId)?;

        let hate = 0.0;

        let cmn = LtCommon::new(&static_char_data.potential, value.level, false);

        let mut skills = Vec::with_capacity(value.own_skill_ids.len());
        for skill_id in value.own_skill_ids.iter() {
            let skill_state = ActiveSkillState::new(*skill_id).ok_or(GameError::InvalidSkillId)?;
            skills.push(skill_state);
        }

        Ok(Self {
            static_data: static_char_data,
            cmn,
            skills,
            hate,
        })
    }
}

impl Char {
    pub(crate) fn set_skill_cooltime(
        &mut self,
        static_skill_id: StaticSkillId,
        time: SkillCooltimeNum,
    ) -> Result<(), GameError> {
        let skill_state = self
            .skills
            .iter_mut()
            .find(|skill| skill.static_data.id == static_skill_id)
            .ok_or(GameError::InvalidSkillId)?;

        skill_state.set_skill_cooltime(time);

        Ok(())
    }

    pub(crate) fn add_hate(&mut self, hate: Num) {
        self.hate += hate;
    }

    pub(crate) fn heal_skill_cooltime(&mut self, num: SkillCooltimeNum) {
        self.skills.iter_mut().for_each(|s| {
            s.heal_skill_cooltime(num);
        });
    }

    /// スキルクールタイム回復力
    pub fn skill_cootime_heal(&self) -> Num {
        50.0 + self.agi() * 5.0
    }
}

impl Char {
    pub(crate) fn get_skill(
        &self,
        static_skill_id: StaticSkillId,
    ) -> Result<&'static StaticActiveSkill, GameError> {
        self.skills
            .iter()
            .find(|skill| skill.static_data.id == static_skill_id)
            .ok_or(GameError::InvalidSkillId)
            .map(|skill| skill.static_data)
    }
}

impl Deref for Char {
    type Target = LtCommon;
    fn deref(&self) -> &Self::Target {
        &self.cmn
    }
}
impl DerefMut for Char {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cmn
    }
}

impl LtCommon {
    fn level_scale(&self) -> Num {
        self.level + 10.0
    }

    pub fn int(&self) -> Num {
        self.potential.int
    }

    pub fn dex(&self) -> Num {
        self.potential.dex
    }

    pub fn str(&self) -> Num {
        self.potential.str
    }

    pub fn vit(&self) -> Num {
        self.potential.vit
    }

    pub fn agi(&self) -> Num {
        self.potential.agi + self.passive.effect_field().add_agi
    }

    pub fn magic_attuck(&self) -> Num {
        let s_f = self.passive.effect_field();

        let base = (self.int() * 3.0 + self.dex()) / 4.0;
        base * self.level_scale() * s_f.magic_attuck_mag
    }

    pub fn physics_attuck(&self) -> Num {
        let s_f = self.passive.effect_field();
        let base = (self.str() * 3.0 + self.dex()) / 4.0;
        base * self.level_scale() * s_f.physics_attuck_mag
    }

    pub fn max_hp(&self) -> Num {
        let s_f = self.passive.effect_field();

        let base = (self.vit() * 6.0 + self.dex()) / 7.0;
        let hp_scale = 3.0;
        let enemy_hp_scale = { if self.is_enemy { 4.0 } else { 1.0 } };

        base * hp_scale * s_f.max_hp_mag * enemy_hp_scale * self.level_scale()
    }

    pub fn magic_defence(&self) -> Num {
        self.passive.effect_field().magic_defence
    }

    pub fn physics_defence(&self) -> Num {
        self.passive.effect_field().physics_defence
    }

    pub fn add_heal_mp(&self) -> Num {
        self.passive.effect_field().add_heal_mp
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }
}

impl LtCommon {
    pub(crate) fn accept_damage(&mut self, dmg: Num) {
        self.hp -= dmg;
        if self.hp < 0.0 {
            self.hp = 0.0
        }
    }

    pub(crate) fn accept_heal(&mut self, heal: Num) {
        self.hp += heal;
        if self.hp > self.max_hp() {
            self.hp = self.max_hp();
        }
    }

    pub(crate) fn accept_turn_start_dmg(&mut self) {
        let per = self.passive.effect_field().turn_start_dmg_per;
        let dmg = self.max_hp() * per;
        self.accept_damage(dmg);
    }
}
