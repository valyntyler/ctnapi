pub mod card;
pub mod dice;
pub mod hex;

pub trait Weight {
    fn weight(&self) -> usize;
}
