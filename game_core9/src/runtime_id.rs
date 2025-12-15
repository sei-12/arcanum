#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RuntimeCharId {
    pub(crate) idx: u8,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RuntimeSkillId {
    pub(crate) char_id: RuntimeCharId,
    pub(crate) idx: u8,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LtId {
    Char(RuntimeCharId),
    Enemy,
}

impl From<RuntimeCharId> for LtId {
    fn from(value: RuntimeCharId) -> Self {
        LtId::Char(value)
    }
}
impl From<&RuntimeCharId> for LtId {
    fn from(value: &RuntimeCharId) -> Self {
        LtId::Char(*value)
    }
}
