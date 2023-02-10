
pub struct NoiseU32(pub (crate) u32);

impl NoiseU32 {
    pub fn ratio(&self) -> f64 {
        self.0 as f64 / u32::MAX as f64
    }
    pub fn value(&self) -> u32 {
        self.0
    }
}

pub trait NoiseFuncU32 {
    fn noise(&self, input : &[u32]) -> NoiseU32;
    fn s_noise(&self, input : &[u32], seed : u32) -> NoiseU32;
}