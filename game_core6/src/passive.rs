use std::{
    any::Any,
    collections::{BTreeMap, HashMap, hash_map},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use smallbox::{SmallBox, smallbox, space};

use crate::{
    StaticPassiveId,
    damage::Damage,
    effector::{Effector, TriggerPassiveEffector},
    passive::status::PassiveStatus,
    runtime_id::{LtId, RuntimeCharId},
    state::GameState,
};

#[derive(Debug)]
pub struct PassiveInstance(SmallBox<dyn StaticPassiveData, space::S1>);
impl PassiveInstance {
    pub fn new(passive: impl StaticPassiveData + 'static) -> Self {
        Self(smallbox!(passive))
    }
}

impl Deref for PassiveInstance {
    type Target = dyn StaticPassiveData;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl DerefMut for PassiveInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
impl Clone for PassiveInstance {
    fn clone(&self) -> Self {
        self.0.clone()
    }
}

pub trait StaticPassiveData: Debug {
    fn static_id(&self) -> StaticPassiveId;
    fn clone(&self) -> PassiveInstance;
    fn write_merge(&self, buffer: &mut dyn Any);
    fn should_trash(&self) -> bool;
    fn merge(&mut self, passive: &PassiveInstance);
    fn update(&mut self, msg: &PassiveUpdateMessage);
    fn status(&self, status: &mut PassiveStatus) {}
    fn trigger_turn_start(&self, owner: LtId, effector: &mut dyn Effector) {}
    fn trigger_recv_damage(&self, owner: LtId, dmg: &Damage, effector: &mut dyn Effector) {}
}

// SkillUpdateMessageと統一しても良い。本質的に同じな気がする
pub enum PassiveUpdateMessage {
    Msg(&'static str),
    Buffer([u8; 16]),
    Box(Box<dyn Any>),
}

#[derive(Debug, Clone)]
pub struct PassiveList {
    map: HashMap<StaticPassiveId, PassiveInstance>,
    added_order: AddedOrder,
    cached_status: cached_status::CachedPassiveStatus,
}

impl PassiveList {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            added_order: AddedOrder::new(),
            cached_status: cached_status::CachedPassiveStatus::new(),
        }
    }

    pub fn add(&mut self, passive: PassiveInstance) {
        assert!(!passive.should_trash());

        match self.map.entry(passive.static_id()) {
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().merge(&passive);

                if entry.get().should_trash() {
                    entry.remove();
                }
            }
            hash_map::Entry::Vacant(entry) => {
                self.added_order.add(passive.static_id());
                entry.insert(passive);
            }
        }

        self.cached_status.need_update();
    }

    pub fn update_state(&mut self, static_id: StaticPassiveId, msg: &PassiveUpdateMessage) {
        let hash_map::Entry::Occupied(mut entry) = self.map.entry(static_id) else {
            // 更新されて捨てられている可能性がある
            return;
        };

        entry.get_mut().update(msg);

        if entry.get().should_trash() {
            self.added_order.remove_expect(static_id);
            entry.remove();
        };

        self.cached_status.need_update();
    }

    pub fn status(&self) -> std::cell::Ref<'_, PassiveStatus> {
        self.cached_status.get(self.map.values())
    }

    pub fn trigger_turn_start(&self, owner: LtId, effector: &mut impl TriggerPassiveEffector) {
        self.added_order.iter().for_each(|static_id| {
            let passive = self.map.get(&static_id).unwrap();
            effector.begin();
            passive.trigger_turn_start(owner, effector);
            effector.end();
        });
    }

    pub fn trigger_recv_damage(
        &self,
        owner: LtId,
        dmg: &Damage,
        effector: &mut impl TriggerPassiveEffector,
    ) {
        assert_eq!(owner, dmg.target());
        self.added_order.iter().for_each(|static_id| {
            let passive = self.map.get(&static_id).unwrap();
            effector.begin();
            passive.trigger_recv_damage(owner, dmg, effector);
            effector.end();
        });
    }
}

impl Default for PassiveList {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct AddedOrder {
    count: u64,
    sorted: BTreeMap<u64, StaticPassiveId>,
    runtime_id_to_count: HashMap<StaticPassiveId, u64>,
}

impl AddedOrder {
    fn new() -> Self {
        Self {
            count: 0,
            sorted: BTreeMap::new(),
            runtime_id_to_count: HashMap::new(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = StaticPassiveId> {
        self.sorted.values().copied()
    }

    fn add(&mut self, id: StaticPassiveId) {
        self.count += 1;
        let old_item = self.sorted.insert(self.count, id);
        if old_item.is_some() {
            panic!("呼び出し元は重複しないIDであることを保証しないといけない")
        }
        self.runtime_id_to_count.insert(id, self.count);
    }

    fn remove_expect(&mut self, id: StaticPassiveId) {
        let count = self
            .runtime_id_to_count
            .remove(&id)
            .expect("呼び出し元は存在するIDであることを保証していないといけない");
        self.sorted.remove(&count);
    }
}

//--------------------------------------------------//
//                                                  //
//                      CACHED                      //
//                                                  //
//--------------------------------------------------//
mod cached_status {
    use std::cell::{Ref, RefCell};

    use crate::passive::{PassiveInstance, PassiveStatus};

    #[derive(Debug, Clone)]
    pub(super) struct CachedPassiveStatus {
        need_update: RefCell<bool>,
        cache: RefCell<PassiveStatus>,
    }

    impl CachedPassiveStatus {
        pub fn new() -> Self {
            Self {
                need_update: RefCell::new(false),
                cache: RefCell::new(PassiveStatus::default()),
            }
        }

        pub fn need_update(&mut self) {
            *self.need_update.borrow_mut() = true;
        }

        pub fn get<'a>(
            &self,
            passives: impl Iterator<Item = &'a PassiveInstance>,
        ) -> Ref<'_, PassiveStatus> {
            if *self.need_update.borrow() {
                let mut cache = self.cache.borrow_mut();
                cache.reset();
                passives.for_each(|item| {
                    item.status(&mut cache);
                });
                *self.need_update.borrow_mut() = false
            };
            self.cache.borrow()
        }
    }
}
//--------------------------------------------------//
//                                                  //
//                      STATUS                      //
//                                                  //
//--------------------------------------------------//
pub mod status {
    use crate::StatusNum;

    // メリット効果は加算 デメリット効果は乗算
    //
    // メリット効果を乗算にすると倍々ですぐにとんでもない値になりそう。
    // デメリット効果を減算にすると負の値になったりして扱いにくい。
    //
    // 例外としてPotentialに関するデメリット効果は減算で良い
    // その代わりLtCommon側で0以下の処理などを加える必要はある

    #[derive(Debug, Clone)]
    pub struct PassiveStatus {
        /// 魔法攻撃力に乗算させる値 default = 1.0
        pub magic_attuck_mag_buff: BuffMagnificationNum,
        /// 魔法攻撃力に乗算させる値 default = 1.0
        pub magic_attuck_mag_debuff: DebuffMagnificationNum,

        /// 物理攻撃力に乗算させる値
        pub physics_attuck_mag_buff: BuffMagnificationNum,
        /// 物理攻撃力に乗算させる値
        pub physics_attuck_mag_debuff: DebuffMagnificationNum,

        /// 最大HPに乗算させる値
        pub max_hp_mag_buff: BuffMagnificationNum,
        /// 最大HPに乗算させる値
        pub max_hp_mag_debuff: DebuffMagnificationNum,

        /// 被魔法ダメージ倍率
        pub recv_magic_dmg_mag: RecvDamageMagnificationNum,

        /// 被物理ダメージ倍率
        pub recv_physics_dmg_mag: RecvDamageMagnificationNum,

        pub add_agi: StatusNum,
        pub add_str: StatusNum,
    }

    impl Default for PassiveStatus {
        fn default() -> Self {
            Self {
                max_hp_mag_buff: BuffMagnificationNum::default(),
                max_hp_mag_debuff: DebuffMagnificationNum::default(),
                recv_magic_dmg_mag: RecvDamageMagnificationNum::default(),
                recv_physics_dmg_mag: RecvDamageMagnificationNum::default(),
                add_agi: 0.0,
                add_str: 0.0,
                magic_attuck_mag_buff: BuffMagnificationNum::default(),
                magic_attuck_mag_debuff: DebuffMagnificationNum::default(),
                physics_attuck_mag_buff: BuffMagnificationNum::default(),
                physics_attuck_mag_debuff: DebuffMagnificationNum::default(),
            }
        }
    }

    impl PassiveStatus {
        pub(crate) fn reset(&mut self) {
            *self = Self::default();
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct RecvDamageMagnificationNum {
        value: StatusNum,
    }
    impl Default for RecvDamageMagnificationNum {
        fn default() -> Self {
            Self { value: 1.0 }
        }
    }
    impl RecvDamageMagnificationNum {
        pub fn get(&self) -> StatusNum {
            self.value
        }
        pub fn mul(&mut self, val: StatusNum) {
            assert!(val >= 0.0);
            self.value *= val;
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct BuffMagnificationNum {
        value: StatusNum,
    }
    impl Default for BuffMagnificationNum {
        fn default() -> Self {
            Self { value: 1.0 }
        }
    }
    impl BuffMagnificationNum {
        /// 0以上が保証されている
        pub fn get(&self) -> StatusNum {
            assert!(self.value >= 0.0);
            self.value
        }
        pub fn add(&mut self, buff: StatusNum) {
            assert!(buff >= 0.0);
            self.value += buff;
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct DebuffMagnificationNum {
        value: StatusNum,
    }
    impl Default for DebuffMagnificationNum {
        fn default() -> Self {
            Self { value: 1.0 }
        }
    }
    impl DebuffMagnificationNum {
        /// 0以上が保証されている
        pub fn get(&self) -> StatusNum {
            assert!(self.value >= 0.0);
            self.value
        }
        pub fn mul(&mut self, buff: StatusNum) {
            assert!(buff <= 1.0);
            self.value *= buff;
        }
    }
}
