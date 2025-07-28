use crate::Weight;

pub enum HexTerrain {
    Hill,
    Forest,
    Pasture,
    Field,
    Mountain,
    Desert,
}

impl Weight for HexTerrain {
    fn weight(&self) -> usize {
        match self {
            HexTerrain::Forest | HexTerrain::Pasture | HexTerrain::Field => 4,
            HexTerrain::Hill | HexTerrain::Mountain => 3,
            HexTerrain::Desert => 1,
        }
    }
}
