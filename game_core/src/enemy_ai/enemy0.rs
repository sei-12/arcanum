use crate::{
    container::Container,
    damage::{DamageType, calc_damage},
    enemy_ai::StaticEnemyData,
    lt::Potential,
    passive::{Passive, PassivePrivate, public_passive::Burn},
    skills::TurnNum,
};

pub const ENEMY: StaticEnemyData = StaticEnemyData {
    id: 0,
    name: "ファフニール",
    potential: Potential {
        int: 10.0,
        vit: 10.0,
        str: 10.0,
        dex: 10.0,
        agi: 10.0,
    },
    assist,
    high,
    interference,
    low,
    mid,
};

fn assist(con: &mut Container) {
    con.update_enemy(|enemy| {
        enemy.passive.add(Box::new(MagicBarrier { turns: 2 }));
    });

    con.log(format!(
        "{}の補助行動。{}は自身に2ターンの魔結界を付与した",
        ENEMY.name, ENEMY.name
    ));
}

fn high(con: &mut Container) {
    let enemy = con.get_enemy();
    let target = con.get_max_hate_char();
    let target_name = target.static_data.name;
    let atk = 1.3;
    let dmg = calc_damage(enemy, target, DamageType::Physics, atk);

    con.update_char(target.static_data.id, |char| {
        char.accept_damage(dmg);
        Ok(())
    })
    .unwrap();

    con.log(format!(
        "{}の強攻撃。{}は{}に{}点の物理ダメージを与えた",
        ENEMY.name,
        ENEMY.name,
        target_name,
        dmg.round() as u32
    ));
}

fn interference(con: &mut Container) {
    let id = con.get_max_hate_char().static_data.id;

    con.update_char(id, |char| {
        char.passive.add(Box::new(Burn::new(2)));
        Ok(())
    })
    .unwrap();

    con.log(format!(
        "{}の妨害行動。{}に2ターンの火傷を与えた",
        ENEMY.name,
        con.get_char(id).unwrap().static_data.name
    ));
}

fn mid(con: &mut Container) {
    let enemy = con.get_enemy();
    let target = con.get_max_hate_char();
    let target_name = target.static_data.name;
    let atk = 1.1;
    let dmg = calc_damage(enemy, target, DamageType::Physics, atk);

    con.update_char(target.static_data.id, |char| {
        char.accept_damage(dmg);
        Ok(())
    })
    .unwrap();

    con.log(format!(
        "{}の中攻撃。{}は{}に{}点の物理ダメージを与えた",
        ENEMY.name,
        ENEMY.name,
        target_name,
        dmg.round() as u32
    ));
}

fn low(con: &mut Container) {
    let enemy = con.get_enemy();
    let target = con.get_max_hate_char();
    let target_name = target.static_data.name;
    let atk = 0.9;
    let dmg = calc_damage(enemy, target, DamageType::Magic, atk);

    con.update_char(target.static_data.id, |char| {
        char.accept_damage(dmg);
        Ok(())
    })
    .unwrap();

    con.log(format!(
        "{}の弱攻撃。{}は{}に{}点の物理ダメージを与えた",
        ENEMY.name,
        ENEMY.name,
        target_name,
        dmg.round() as u32
    ));
}

#[derive(Debug, Clone)]
struct MagicBarrier {
    turns: TurnNum,
}

impl PassivePrivate for MagicBarrier {
    fn status_effect(&self, _field: &mut crate::passive::PassiveSkillEffectField) {
        _field.magic_defence *= 0.9;
    }
}
impl Passive for MagicBarrier {
    fn id(&self) -> crate::passive::PassiveIdentifier {
        crate::passive::PassiveIdentifier::MagicBarrier
    }

    fn display(&self) -> Option<String> {
        Some(format!("魔結界 ({})", self.turns))
    }

    fn state(&self) -> Box<dyn std::any::Any> {
        Box::new(self.turns)
    }

    fn marge(&mut self, target_state: Box<dyn std::any::Any>) {
        let target = target_state
            .downcast::<TurnNum>()
            .unwrap_or_else(|_| panic!("failed to downcast"));

        self.turns += *target;
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
