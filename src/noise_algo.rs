
use crate::noise::*;

pub struct CCNoiseFuncU32(); 

impl CCNoiseFuncU32 {
    pub fn new() -> Self {
        CCNoiseFuncU32()
    }
}

const Y_PRIME : u32 = 0x6AC2D;
const Z_PRIME : u32 = 0x41ED2;
const T_PRIME : u32 = 0xCD67;


const PRIME_1 : u32 = 0xF2AEFA17;
const PRIME_2 : u32 = 0xF08D8857;
const PRIME_3 : u32 = 0x35A3B0C5;
const PRIME_4 : u32 = 0x1E11CC53;
const PRIME_5 : u32 = 0x6EBA8DF;
const PRIME_6 : u32 = 0x1586BB73;
const PRIME_7 : u32 = 0xB5D8B5A5;
const PRIME_8 : u32 = 0xAC5B5253;
const PRIME_9 : u32 = 0xBE648801;
const _PRIME_A : u32 = 0x9920A971;
const _PRIME_B : u32 = 0x1999C52F;

fn cc_noise(mut input : u32, seed : u32) -> u32 {

    input = input.wrapping_mul(PRIME_1);
    input = input.wrapping_add(seed);
    input = input.wrapping_add(PRIME_2);
    input = input.wrapping_mul(PRIME_3);
    input = input.wrapping_add(PRIME_4 * ((input >> 3) & 1));
    input = input.wrapping_mul(PRIME_5);

    input = input.wrapping_add(PRIME_6 * ((input >> 11) & 1));
    input = input.wrapping_mul(PRIME_7);
    
    input = input.wrapping_mul(input);

    let shift = (input & 0xF) + 3;

    input ^= input >> shift;

    input = input.wrapping_add(PRIME_8 * ((input >> 23) & 1));
    input = input.wrapping_mul(PRIME_9);

    input ^= input >> 17;

    input 
}

impl NoiseFuncU32 for CCNoiseFuncU32 {
    fn noise(&self, input : &[u32]) -> NoiseU32 {
        match input {
            [x, y, z, t] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME))
                             .wrapping_add(z.wrapping_mul(Z_PRIME))
                             .wrapping_add(t.wrapping_mul(T_PRIME));
                let n = cc_noise(input, 0);
                NoiseU32(n)
            },
            [x, y, z] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME))
                             .wrapping_add(z.wrapping_mul(Z_PRIME));
                let n = cc_noise(input, 0);
                NoiseU32(n)
            },
            [x, y] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME));
                let n = cc_noise(input, 0);
                NoiseU32(n)
            },
            [x] => {
                let n = cc_noise(*x, 0);
                NoiseU32(n)
            },
            _ => panic!("CCNoiseU32 only supports 1-4 D noise"),
        }
    }

    fn s_noise(&self, input : &[u32], seed : u32) -> NoiseU32 {
        match input {
            [x, y, z, t] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME))
                             .wrapping_add(z.wrapping_mul(Z_PRIME))
                             .wrapping_add(t.wrapping_mul(T_PRIME));
                let n = cc_noise(input, seed);
                NoiseU32(n)
            },
            [x, y, z] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME))
                             .wrapping_add(z.wrapping_mul(Z_PRIME));
                let n = cc_noise(input, seed);
                NoiseU32(n)
            },
            [x, y] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME));
                let n = cc_noise(input, seed);
                NoiseU32(n)
            },
            [x] => {
                let n = cc_noise(*x, seed);
                NoiseU32(n)
            },
            _ => panic!("CCNoise32 only supports 1-4 D noise"),
        }
    }
}


// xor (shift dependent amount)
// special bit indicates mul against prime 

