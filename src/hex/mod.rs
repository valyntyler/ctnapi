use crate::hex::number::HexNumber;
use crate::hex::terrain::HexTerrain;

pub mod number;
pub mod terrain;

pub struct Hex {
    pub number: HexNumber,
    pub terrain: HexTerrain,
}
