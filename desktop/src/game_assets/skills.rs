use game_core6::{
    StaticSkillId,
    damage::Damage,
    effect::Effect,
    skill::{SkillCost, SkillInstance, SkillTrait},
};

pub enum StaticSkillIdEnum {
    Fireball = 1,
}
impl From<StaticSkillIdEnum> for game_core6::StaticSkillId {
    fn from(val: StaticSkillIdEnum) -> Self {
        val as game_core6::StaticSkillId
    }
}

#[derive(Debug)]
pub struct Fireball;
impl SkillTrait for Fireball {
    fn call(
        &self,
        user_id: game_core6::runtime_id::RuntimeCharId,
        _skill_id: game_core6::runtime_id::RuntimeSkillId,
        target_id: Option<game_core6::runtime_id::RuntimeEnemyId>,
        effector: &mut dyn game_core6::effector::EffectorTrait,
    ) -> Result<game_core6::skill::SkillCost, game_core6::WinOrLoseOrNextwave> {
        let target = effector
            .state()
            .get_enemys_highest_target_priority(target_id)
            .next()
            .unwrap();

        effector.accept_effect(Effect::Damage(Damage::new_magic_damage(
            effector.state(),
            user_id.into(),
            target.lt_id(),
            1.0,
        )))?;

        Ok(SkillCost::from_defalut(self.doc()))
    }

    fn clone(&self) -> SkillInstance {
        SkillInstance::new(Self)
    }
    fn doc(&self) -> &game_core6::skill::SkillDocument {
        &game_core6::skill::SkillDocument {
            name: "ファイヤーボール",
            description: "todo",
            id: StaticSkillIdEnum::Fireball as StaticSkillId,
            default_need_mp: 50,
            defalut_hate: 50,
            defalut_cooldown: 50,
        }
    }

    fn need_mp(&self, state: &game_core6::state::GameState) -> game_core6::MpNum {
        self.doc().default_need_mp
    }

    fn update(&mut self, msg: &game_core6::skill::SkillUpdateMessage) {
        unimplemented!()
    }
}
