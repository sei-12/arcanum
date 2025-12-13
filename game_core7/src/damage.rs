use crate::{
    StatusNum,
    runtime_id::{LtId, RuntimeCharId, RuntimeEnemyId},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Magic,
    Physics,
    Fixed,
}
impl DamageType {
    pub fn type_str(&self) -> &'static str {
        match self {
            DamageType::Fixed => "固定",
            DamageType::Magic => "魔法",
            DamageType::Physics => "物理",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageCauser {
    Enemy(RuntimeEnemyId),
    Char(RuntimeCharId),
    None,
}
impl From<LtId> for DamageCauser {
    fn from(value: LtId) -> Self {
        match value {
            LtId::Char(id) => DamageCauser::Char(id),
            LtId::Enemy(id) => DamageCauser::Enemy(id),
        }
    }
}
impl DamageCauser {
    pub fn to_lt_id(self) -> Option<LtId> {
        match self {
            DamageCauser::Char(id) => Some(LtId::Char(id)),
            DamageCauser::Enemy(id) => Some(LtId::Enemy(id)),
            DamageCauser::None => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    /*     pub fn new_hp_per_dmg(state: &GameState, target_id: LtId, per: StatusNum) -> Self {
        assert!(per >= 0.0);
        let target = state.get_lt(target_id);
        let dmg = target.hp() * per;
        Self {
            causer: DamageCauser::None,
            target: target_id,
            ty: DamageType::Fixed,
            dmg,
        }
    }

    pub fn new_magic_damage(
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

    pub fn new_physics_damage(
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

    #[cfg(test)]
    pub fn test_new(causer: Option<LtId>, target: LtId, ty: DamageType, dmg: StatusNum) -> Self {
        let causer = match causer {
            Some(id) => id.into(),
            None => DamageCauser::None,
        };

        Self {
            causer,
            target,
            ty,
            dmg,
        }
    } */
}
