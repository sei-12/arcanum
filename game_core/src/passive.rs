use std::{
    any::Any,
    cell::{Ref, RefCell},
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
};

use dyn_clone::DynClone;

use crate::Num;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PassiveIdentifier {
    Burn,
    MagicBarrier,
    // MagicDefence,
}

#[derive(Debug, Clone)]
pub(crate) struct PassiveSkillEffectField {
    /// 自身のターン開始時に受けるダメージ
    /// (最大HPの割合)
    pub turn_start_dmg_per: Num,

    /// 魔法攻撃力に乗算させる値 default = 1.0
    pub magic_attuck_mag: Num,

    /// 物理攻撃力に乗算させる値 default = 1.0
    pub physics_attuck_mag: Num,

    /// 最大HPに乗算させる値
    /// default = 1.0
    pub max_hp_mag: Num,

    /// 魔法ダメージに乗算させる値.
    /// つまり大きいほど防御力が低いことになる.
    /// defalut = 1.0
    /// ### Rule
    /// 加算、減算はするな
    /// todo: ドキュメントで禁止するだけじゃなくて何らかのRustの機能を使って禁止にしたい
    pub magic_defence: Num,

    /// 物理ダメージに乗算させる値.
    /// つまり大きいほど防御力が低いことになる.
    /// defalut = 1.0
    /// ### Rule
    /// 加算、減算はするな
    /// todo: ドキュメントで禁止するだけじゃなくて何らかのRustの機能を使って禁止にしたい
    pub physics_defence: Num,
}

impl PassiveSkillEffectField {
    fn reset(&mut self) {
        *self = Self::default()
    }
}

impl Default for PassiveSkillEffectField {
    fn default() -> Self {
        Self {
            turn_start_dmg_per: 0.0,
            magic_attuck_mag: 1.0,
            max_hp_mag: 1.0,
            magic_defence: 1.0,
            physics_attuck_mag: 1.0,
            physics_defence: 1.0,
        }
    }
}

pub(crate) trait PassivePrivate {
    fn status_effect(&self, _field: &mut PassiveSkillEffectField) {}
}

/// パッシブを作成するときのメモ
/// パッシブのはどのファイルに追加しても構わない。
/// ただし/src/passive.rsのPassiveIdentifierにIdを追加すること
#[allow(private_bounds)]
pub trait Passive: Debug + DynClone + PassivePrivate {
    fn should_trash(&self) -> bool;
    // fn margeble(&self) -> bool;
    fn state(&self) -> Box<dyn Any>;
    fn marge(&mut self, target_state: Box<dyn Any>);
    fn id(&self) -> PassiveIdentifier;
    fn display(&self) -> Option<String>;

    fn trigger_turn_start(&mut self) {}
}

dyn_clone::clone_trait_object!(Passive);

#[derive(Debug, Clone)]
pub struct PassiveList {
    passives: HashMap<PassiveIdentifier, Box<dyn Passive>>,
    cache: CachedStatusEffectField,
}

#[allow(clippy::derivable_impls)]
impl Default for PassiveList {
    fn default() -> Self {
        Self {
            passives: HashMap::new(),
            cache: CachedStatusEffectField::new(),
        }
    }
}

impl PassiveList {
    fn trash(&mut self) {
        self.passives.retain(|_, item| !item.should_trash());
    }

    pub fn displayble_passives(&self) -> impl Iterator<Item = String> {
        self.passives.iter().filter_map(|(_, p)| p.display())
    }

    pub fn have(&self, passive_id: PassiveIdentifier) -> bool {
        self.passives.contains_key(&passive_id)
    }

    pub(crate) fn add(&mut self, passive: Box<dyn Passive>) {
        assert!(
            !passive.should_trash(),
            "PassiveListに追加されるアイテムが不正"
        );

        let id = passive.id();

        self.passives
            .entry(id)
            .and_modify(|e| e.marge(passive.state()))
            .or_insert(passive);

        self.cache.update();
    }

    pub(crate) fn trigger_turn_start(&mut self) {
        self.passives.iter_mut().for_each(|(_, item)| {
            item.trigger_turn_start();
        });
        self.trash();
        self.cache.update();
    }

    pub(crate) fn effect_field(&self) -> Ref<'_, PassiveSkillEffectField> {
        self.cache.get(self.passives.values())
    }
}

#[derive(Debug, Clone)]
struct CachedStatusEffectField {
    need_update: RefCell<bool>,
    cache: RefCell<PassiveSkillEffectField>,
}

impl CachedStatusEffectField {
    fn new() -> Self {
        Self {
            need_update: RefCell::new(false),
            cache: RefCell::new(PassiveSkillEffectField::default()),
        }
    }

    fn update(&mut self) {
        *self.need_update.borrow_mut() = true;
    }

    fn get<'a>(
        &self,
        passives: impl Iterator<Item = &'a Box<dyn Passive>>,
    ) -> Ref<'_, PassiveSkillEffectField> {
        if *self.need_update.borrow() {
            let mut cache = self.cache.borrow_mut();
            cache.reset();
            passives.for_each(|item| {
                item.status_effect(&mut cache);
            });
            *self.need_update.borrow_mut() = false
        };
        self.cache.borrow()
    }
}

pub(crate) mod public_passive {
    use crate::passive::{Passive, PassivePrivate};

    #[derive(Debug, Clone)]
    pub struct Burn {
        turns: u32,
    }
    impl Burn {
        pub fn new(turns: u32) -> Self {
            Burn { turns }
        }
    }

    impl PassivePrivate for Burn {
        fn status_effect(&self, field: &mut super::PassiveSkillEffectField) {
            field.magic_defence *= 0.9;
        }
    }

    impl Passive for Burn {
        fn id(&self) -> super::PassiveIdentifier {
            super::PassiveIdentifier::Burn
        }

        fn display(&self) -> Option<String> {
            Some(format!("火傷({})", self.turns))
        }

        fn state(&self) -> Box<dyn std::any::Any> {
            Box::new(self.turns)
        }

        fn marge(&mut self, target_state: Box<dyn std::any::Any>) {
            let t = target_state
                .downcast_ref::<u32>()
                .expect("failed to downcast");
            self.turns += t;
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
}

#[cfg(test)]
mod tests {
    use crate::passive::{Passive, PassiveList, public_passive::Burn};

    #[test]
    fn test1() {
        let mut list = PassiveList::default();
        list.add(Box::new(Burn::new(1)));
        let burn = Burn::new(1);
        assert!(list.passives.contains_key(&burn.id()))
    }
}
