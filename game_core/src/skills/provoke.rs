use crate::{
    container::Container,
    error::GameError,
    passive::{Passive, PassiveIdentifier, PassivePrivate, public_passive::Hardening},
    skills::{StaticActiveSkill, TurnNum},
};

pub const SKILL: StaticActiveSkill = StaticActiveSkill {
    id: 3,
    name: "プロボーク",
    need_mp: 60.0,
    call,
    text: TEXT,
};

const TEXT: &str = "消費MP 60
クールタイム 5ターン
ヘイト値 200
スキル使用者のINTとDEXの合計値が25以上ならヘイト値が50加算される。
スキル使用者のVITが15以上ならスキル使用者に3ターンの「硬化」を付与する。
敵のINTが7以下なら敵に3ターンの「怒り狂う」を付与する。
";

fn call(static_user_id: usize, con: &mut Container) -> Result<(), GameError> {
    let cooltime = 5;
    let mut hate = 200.0;
    let user = con.get_char(static_user_id)?;
    let enemy = con.get_enemy();

    if user.int() + user.dex() >= 25.0 {
        hate += 50.0;
    }

    let hardeing = if user.vit() >= 15.0 {
        Some(Box::new(Hardening::new(2)))
    } else {
        None
    };

    let ikarikuruu = if enemy.int() <= 7.0 {
        Some(Box::new(Ikarikuruu { turns: 3 }))
    } else {
        None
    };

    let log = format!(
        "{}が{}を挑発",
        user.static_data.name, enemy.static_data.name
    );

    con.update_char(static_user_id, |char| {
        char.set_skill_cooltime(SKILL.id, cooltime)?;
        char.add_hate(hate);
        if let Some(kouka) = hardeing {
            char.passive.add(kouka);
        }
        Ok(())
    })?;

    con.update_enemy(|enemy| {
        if let Some(ikarikuruu) = ikarikuruu {
            enemy.passive.add(ikarikuruu);
        }
    });

    con.log(log);
    Ok(())
}

#[derive(Debug, Clone)]
struct Ikarikuruu {
    turns: TurnNum,
}
impl PassivePrivate for Ikarikuruu {}
impl Passive for Ikarikuruu {
    fn display(&self) -> Option<String> {
        Some(format!("怒り狂う({})", self.turns))
    }
    fn id(&self) -> PassiveIdentifier {
        PassiveIdentifier::Ikarikuruu
    }
    fn marge(&mut self, target_state: Box<dyn std::any::Any>) {
        let turn = target_state
            .downcast::<TurnNum>()
            .expect("failed to downcast");
        self.turns += *turn;
    }
    fn should_trash(&self) -> bool {
        self.turns == 0
    }
    fn state(&self) -> Box<dyn std::any::Any> {
        Box::new(self.turns)
    }
    fn trigger_turn_start(&mut self) {
        if self.turns > 0 {
            self.turns -= 1;
        }
    }
}
