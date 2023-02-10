
pub struct NoiseU32(pub (crate) u32);

impl NoiseU32 {
    pub fn ratio(&self) -> f64 {
        self.0 as f64 / u32::MAX as f64
    }
    pub fn value(&self) -> u32 {
        self.0
    }
}

pub struct NoiseF64(pub (crate) u32, pub (crate) u32);

impl NoiseF64 {
    pub fn ratio(&self) -> f64 {
        if self.0 > self.1 {
            self.1 as f64 / self.0 as f64
        }
        else {
            self.0 as f64 / self.1 as f64
        }
    }
}

pub trait NoiseFuncU32 {
    fn noise(&self, input : &[u32]) -> NoiseU32;
    fn s_noise(&self, input : &[u32], seed : u32) -> NoiseU32;
}

pub trait NoiseFuncF64 {
    fn noise(&self, input : &[f64]) -> NoiseF64;
    fn s_noise(&self, input : &[f64], seed : u32) -> NoiseF64;
}