pub mod dice;
pub mod hex;

trait Weight {
    fn weight(&self) -> usize;
}
