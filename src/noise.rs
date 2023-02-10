
pub struct Noise32(pub (crate) u32);

pub trait NoiseFunc32 {
    fn noise(&self, input : &[u32]) -> Noise32;
    fn s_noise(&self, input : &[u32], seed : u32) -> Noise32;
}