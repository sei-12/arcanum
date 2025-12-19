use std::collections::VecDeque;

use crate::{
    LevelNum, StatusNum, TimeNum, core_actor::EffectsBuffer, effect::Effect, game_state::GameState,
    lt_common::any_point::AnyPointPercent, passive::PassiveList, potential::Potential,
    runtime_id::LtId, weapon::Weapon,
};

mod any_point;

#[derive(Debug, Clone)]
pub struct LtCommon {
    pub passive: PassiveList,
    potential: Potential,
    level: LevelNum,
    hp_per: AnyPointPercent,
    mp_per: AnyPointPercent,
    weapon: Option<Weapon>,
}

impl LtCommon {
    pub(crate) fn new_inner(potential: Potential, level: LevelNum, weapon: Option<Weapon>) -> Self {
        Self {
            potential,
            level,
            hp_per: AnyPointPercent::new_max(),
            mp_per: AnyPointPercent::new_empty(),
            passive: PassiveList::default(),
            weapon,
        }
    }

    pub(super) fn new(potential: Potential, level: LevelNum) -> Self {
        Self::new_inner(potential, level, None)
    }

    pub(crate) fn new_with_weapon(potential: Potential, level: LevelNum, weapon: Weapon) -> Self {
        Self::new_inner(potential, level, Some(weapon))
    }
    pub(crate) fn tick(
        &self,
        owner_id: LtId,
        state: &GameState,
        effects_buffer: &mut EffectsBuffer,
    ) {
        self.passive.tick(owner_id, state, effects_buffer);
        effects_buffer.push(Effect::HealMp {
            target_id: owner_id,
            num: self.mp_heal(),
        });
    }
}

impl LtCommon {
    /// 0.0以上であることが保証されている
    pub fn int(&self) -> StatusNum {
        let tmp = self.potential.int() + self.passive.status().add_int;
        if tmp < 0.0 { 0.0 } else { tmp }
    }

    /// 0.0以上であることが保証されている
    pub fn dex(&self) -> StatusNum {
        let tmp = self.potential.dex() + self.passive.status().add_dex;
        if tmp < 0.0 { 0.0 } else { tmp }
    }

    /// 0.0以上であることが保証されている
    pub fn str(&self) -> StatusNum {
        let tmp = self.potential.str() + self.passive.status().add_str;
        if tmp < 0.0 { 0.0 } else { tmp }
    }

    /// 0.0以上であることが保証されている
    pub fn vit(&self) -> StatusNum {
        let tmp = self.potential.vit() + self.passive.status().add_vit;
        if tmp < 0.0 { 0.0 } else { tmp }
    }

    /// 0.0以上であることが保証されている
    pub fn agi(&self) -> StatusNum {
        let tmp = self.potential.agi() + self.passive.status().add_agi;
        if tmp < 0.0 { 0.0 } else { tmp }
    }
}

impl LtCommon {
    fn level_scale(&self) -> StatusNum {
        (self.level + 10) as f32
    }

    pub fn level(&self) -> LevelNum {
        self.level
    }

    pub fn magic_attuck(&self) -> StatusNum {
        let base = (self.int() * 3.0 + self.dex()) / 4.0;
        (base * self.level_scale() + self.weapon.as_ref().map(|w| w.m_atk).unwrap_or(0.0))
            * self.passive.status().magic_attuck_mag_buff.get()
            * self.passive.status().magic_attuck_mag_debuff.get()
    }

    pub fn physics_attuck(&self) -> StatusNum {
        let base = (self.str() * 3.0 + self.dex()) / 4.0;
        (base * self.level_scale() + self.weapon.as_ref().map(|w| w.p_atk).unwrap_or(0.0))
            * self.passive.status().physics_attuck_mag_buff.get()
            * self.passive.status().physics_attuck_mag_debuff.get()
    }

    pub fn max_hp(&self) -> StatusNum {
        let base = (self.vit() * 6.0 + self.str() + self.dex()) / 8.0;
        let hp_scale = 3.0;

        base * hp_scale
            * self.level_scale()
            * self.passive.status().max_hp_mag_buff.get()
            * self.passive.status().max_hp_mag_debuff.get()
    }

    pub fn hp(&self) -> StatusNum {
        self.hp_per.get(self.max_hp())
    }

    pub fn recv_magic_dmg_mag(&self) -> StatusNum {
        self.passive.status().recv_magic_dmg_mag.get()
    }

    pub fn recv_physics_dmg_mag(&self) -> StatusNum {
        self.passive.status().recv_physics_dmg_mag.get()
    }

    pub fn mp(&self) -> StatusNum {
        self.mp_per.get(self.max_mp())
    }

    pub fn max_mp(&self) -> StatusNum {
        let base = (self.vit() * 2.0 + self.dex() + self.int()) / 4.0;
        let mp_scale = 50.0;
        base * mp_scale
    }

    pub fn mp_heal(&self) -> StatusNum {
        // 調整用の値
        // potentialがMの時に1sあたりMPがN回復する
        // potentialが0の時に1sあたりMPがBASE回復する
        const M: f32 = 10.0;
        const N: f32 = 7.0;
        const FPS: f32 = 100.0;
        const MP_HEAL_BASE_PER_SEC: f32 = 1.0;

        // 調整した時のために残しておきたい
        static_assertions::const_assert!(N > MP_HEAL_BASE_PER_SEC);

        let potential = (self.vit() * 2.0 + self.dex() + self.agi()) / 4.0;

        fn mp_heal_f(pot: f32) -> f32 {
            let base_frame = MP_HEAL_BASE_PER_SEC / FPS;
            let pot_frame = pot / M * (N - MP_HEAL_BASE_PER_SEC) / FPS;

            debug_assert!(
                pot_frame >= 0.0,
                "pot_frame should be non-negative: {}",
                pot_frame
            );

            base_frame + pot_frame
        }

        debug_assert!({
            let tmp = mp_heal_f(M) * FPS;
            let diff = (N - tmp).abs();
            diff < 0.00001
        });

        debug_assert!({
            let tmp = mp_heal_f(0.0) * FPS;
            let diff = (MP_HEAL_BASE_PER_SEC - tmp).abs();
            diff < 0.00001
        });

        let mp_heal = mp_heal_f(potential);

        debug_assert!(mp_heal > 0.0, "mp_heal should be positive: {}", mp_heal);

        mp_heal
    }

    pub fn speed(&self) -> TimeNum {
        self.agi() / 10.0
    }

    pub fn is_dead(&self) -> bool {
        self.hp() <= 0.0
    }
}

impl LtCommon {
    pub(crate) fn accept_damage(&mut self, dmg: StatusNum) {
        // HPは0を下回ることがある
        self.hp_per.add(self.max_hp(), -dmg);
    }

    pub(crate) fn accept_heal(&mut self, heal: StatusNum) {
        self.hp_per.add(self.max_hp(), heal);
    }

    pub(crate) fn accept_consume_mp(&mut self, num: StatusNum) {
        self.mp_per.add(self.max_mp(), -num);
    }

    pub(crate) fn accept_heal_mp(&mut self, num: StatusNum) {
        self.mp_per.add(self.max_mp(), num);
    }
}
