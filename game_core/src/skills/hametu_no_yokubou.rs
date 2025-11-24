use crate::{
    TurnNum,
    container::Container,
    error::GameError,
    passive::{Passive, PassivePrivate},
    skills::StaticActiveSkill,
};

pub const SKILL: StaticActiveSkill = StaticActiveSkill {
    id: 5,
    name: "破滅の欲望",
    need_mp: 20.0,
    call,
    text: TEXT,
};

const TEXT: &str = "消費MP 20
クールタイム 400
ヘイト値 40
残りHPの10%分のダメージを受ける。
自身に3ターンの「破滅の欲望」を付与する。
STRが14以上なら4ターン付与する。
「破滅の欲望」：AGI+1, 追加MP回復+25, 物理攻撃力1.15倍。重複不可。重ねがけした場合はターン数が合算される。
";

fn call(static_user_id: usize, con: &mut Container) -> Result<(), GameError> {
    let user = con.get_char(static_user_id)?;
    let mut passive_turns = 3;
    if user.str() >= 14.0 {
        passive_turns += 1;
    }
    let dmg = user.hp * 0.1;

    let log = format!(
        "{}は{}ダメージを受け、自身に{}ターンの「破滅の欲望」を付与した",
        user.static_data.name,
        dmg.round(),
        passive_turns
    );

    con.consume_player_side_mp(SKILL.need_mp);
    con.update_char(static_user_id, |char| {
        char.accept_damage(dmg);
        char.add_hate(40.0);
        char.passive.add(Box::new(HametuNoYokubou {
            turns: passive_turns,
        }));
        char.set_skill_cooltime(SKILL.id, 400.0);
        Ok(())
    })?;
    con.log(log);

    Ok(())
}

#[derive(Debug, Clone)]
struct HametuNoYokubou {
    turns: TurnNum,
}
impl PassivePrivate for HametuNoYokubou {
    fn status_effect(&self, field: &mut crate::passive::PassiveSkillEffectField) {
        field.add_heal_mp += 25.0;
        field.physics_attuck_mag += 0.15;
        field.add_agi += 1.0;
    }
}
impl Passive for HametuNoYokubou {
    fn display(&self) -> Option<String> {
        Some(format!("破滅の欲望({})", self.turns))
    }
    fn id(&self) -> crate::passive::PassiveIdentifier {
        crate::passive::PassiveIdentifier::HametuNoYokubou
    }
    fn marge(&mut self, target_state: Box<dyn std::any::Any>) {
        let turn = target_state
            .downcast::<TurnNum>()
            .expect("failed to downcast");
        self.turns += *turn;
    }
    fn state(&self) -> Box<dyn std::any::Any> {
        Box::new(self.turns)
    }
    fn should_trash(&self) -> bool {
        self.turns == 0
    }
    fn trigger_turn_start(&mut self) {
        if self.turns > 0 {
            self.turns -= 1;
        }
    }
}
