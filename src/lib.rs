pub mod card;
pub mod dice;
pub mod hex;
pub mod player;

pub trait Weight {
    fn weight(&self) -> usize;
}
