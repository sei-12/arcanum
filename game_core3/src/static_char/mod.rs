use crate::potential::Potential;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StaticCharId {
    Elena,
    Yura,
    Yuuko,
    Asya,
}

impl std::fmt::Display for StaticCharId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asya => f.write_str("Asya"),
            Self::Elena => f.write_str("Elena"),
            Self::Yura => f.write_str("Yura"),
            Self::Yuuko => f.write_str("Yuuko"),
        }
    }
}

#[derive(Debug)]
pub struct StaticCharData {
    pub id: StaticCharId,
    pub name: &'static str,
    pub potential: Potential,
}

impl StaticCharData {
    pub fn get(id: StaticCharId) -> &'static StaticCharData {
        match id {
            StaticCharId::Asya => &ASYA,
            StaticCharId::Elena => &ELENA,
            StaticCharId::Yura => &YURA,
            StaticCharId::Yuuko => &YUUKO,
        }
    }
}

const ELENA: StaticCharData = StaticCharData {
    id: StaticCharId::Elena,
    name: "エレナ",
    potential: Potential::new(17.0, 8.0, 5.0, 13.0, 7.0),
};

const YURA: StaticCharData = StaticCharData {
    id: StaticCharId::Yura,
    name: "ゆら",
    potential: Potential::new(4.0, 9.0, 18.0, 11.0, 8.0),
};

const YUUKO: StaticCharData = StaticCharData {
    id: StaticCharId::Yuuko,
    name: "幽狐",
    potential: Potential::new(15.0, 7.0, 3.0, 10.0, 15.0),
};

const ASYA: StaticCharData = StaticCharData {
    id: StaticCharId::Asya,
    name: "アーシャ",
    potential: Potential::new(6.0, 17.0, 10.0, 6.0, 11.0),
};
