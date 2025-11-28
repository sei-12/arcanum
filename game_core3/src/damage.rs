use crate::{
    StatusNum,
    buttle_char::ButtleChar,
    buttle_enemy::ButtleEnemy,
    state::{GameState, LtId, chars::RuntimeCharId},
    static_char::StaticCharData,
};

#[derive(Debug, Clone, Copy)]
pub enum DamageType {
    Magic,
    Physics,
    Fixed,
}

#[derive(Debug, Clone, Copy)]
pub enum DamageCauser {
    Enemy,
    Char(RuntimeCharId),
    None,
}
impl From<LtId> for DamageCauser {
    fn from(value: LtId) -> Self {
        match value {
            LtId::Char(id) => DamageCauser::Char(id),
            LtId::Enemy => DamageCauser::Enemy,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Damage {
    causer: DamageCauser,
    target: LtId,
    ty: DamageType,
    dmg: StatusNum,
}

impl Damage {
    pub fn dmg(&self) -> StatusNum {
        assert!(self.dmg >= 0.0);
        self.dmg
    }

    pub fn causer(&self) -> DamageCauser {
        self.causer
    }

    pub fn target(&self) -> LtId {
        self.target
    }

    pub fn ty(&self) -> DamageType {
        self.ty
    }
}

impl Damage {
    pub(crate) fn new_magic_damage(
        state: &GameState,
        attucker_id: LtId,
        target_id: LtId,
        dmg_mag: StatusNum,
    ) -> Self {
        assert!(dmg_mag >= 0.0);

        let attucker = state.get_lt(attucker_id);
        let target = state.get_lt(target_id);

        let dmg = attucker.magic_attuck() * target.recv_magic_dmg_mag() * dmg_mag;

        Self {
            causer: attucker_id.into(),
            target: target_id,
            ty: DamageType::Magic,
            dmg,
        }
    }

    pub(crate) fn new_physics_damage(
        state: &GameState,
        attucker_id: LtId,
        target_id: LtId,
        dmg_mag: StatusNum,
    ) -> Self {
        assert!(dmg_mag >= 0.0);

        let attucker = state.get_lt(attucker_id);
        let target = state.get_lt(target_id);

        let dmg = attucker.physics_attuck() * target.recv_physics_dmg_mag() * dmg_mag;

        Self {
            causer: attucker_id.into(),
            target: target_id,
            ty: DamageType::Physics,
            dmg,
        }
    }
}
