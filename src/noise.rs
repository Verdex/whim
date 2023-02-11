
pub trait NoiseFuncU32 {
    fn noise(&self, input : &[u32]) -> f32;
    fn s_noise(&self, input : &[u32], seed : u32) -> f32;
}

pub trait NoiseFuncF32 {
    fn noise(&self, input : &[f32]) -> f32;
    fn s_noise(&self, input : &[f32], seed : u32) -> f32;
}