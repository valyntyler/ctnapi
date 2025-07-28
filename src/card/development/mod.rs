use crate::Weight;

pub enum DevelopmentCard {
    Monopoly,
    RoadBuilding,
    Invention,
    Knight,
    VictoryPoint,
}

impl Weight for DevelopmentCard {
    fn weight(&self) -> usize {
        match self {
            DevelopmentCard::Knight => 14,
            DevelopmentCard::VictoryPoint => 5,
            _ => 2,
        }
    }
}
