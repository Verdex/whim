
use crate::noise::*;

pub struct CCIndexU32 { }

impl CCIndexU32 {
    pub fn new() -> Self {
        CCIndexU32 { }
    }
}

pub struct CCIndexF32 { }

impl CCIndexF32 {
    pub fn new() -> Self {
        CCIndexF32 { }
    }
}

const Y_PRIME : u32 = 0x6AC2D;
const Z_PRIME : u32 = 0x41ED2;
const T_PRIME : u32 = 0xCD67;

const DIM_PRIMES : [u32;4] = [1, Y_PRIME, Z_PRIME, T_PRIME];

const PRIME_1 : u32 = 0xF2AEFA17;
const PRIME_2 : u32 = 0xF08D8857;
const PRIME_3 : u32 = 0x35A3B0C5;
const PRIME_4 : u32 = 0x1E11CC53;
const PRIME_5 : u32 = 0x6EBA8DF;
const _PRIME_6 : u32 = 0x1586BB73;
const _PRIME_7 : u32 = 0xB5D8B5A5;
const _PRIME_8 : u32 = 0xAC5B5253;
const _PRIME_9 : u32 = 0xBE648801;
const _PRIME_A : u32 = 0x9920A971;
const _PRIME_B : u32 = 0x1999C52F;

fn cc_index_noise(mut input : u32, seed : u32) -> u32 {
    input = input.wrapping_mul(PRIME_1);
    input = input.wrapping_add(seed);
    input = input.wrapping_add(PRIME_2);

    let shift = (input & 0xF) + 1;
    input ^= input >> shift;

    let shift = ((input >> 4) & 0xF) + 1;
    input ^= input << shift;

    let shift = ((input >> 28) & 0xF) + 1;
    input ^= input >> shift;

    let shift = ((input >> 24) & 0xF) + 1;
    input ^= input << shift;

    input = input.wrapping_mul(input);
    input = input.wrapping_mul(PRIME_3);
    input = input.wrapping_add(PRIME_4 * ((input >> 13) & 1));
    input ^= input >> 17;
    input = input.wrapping_add(PRIME_5 * ((input >> 19) & 1));
    input ^= input << 7;
    input
}

impl NoiseFuncU32 for CCIndexU32 {
    fn noise(&self, input : &[u32]) -> f32 {
        if input.len() > 4 {
            panic!("CCIndexU32 noise only supports 1-4 D noise");
        }

        let input = input.into_iter()
                         .zip(DIM_PRIMES.into_iter())
                         .map(|(a, b)| a.wrapping_mul(b))
                         .fold(0u32, |tot, x| tot.wrapping_add(x));

        let output = cc_index_noise(input, 0);
        output as f32 / u32::MAX as f32
    }

    fn s_noise(&self, input : &[u32], seed : u32) -> f32 {
        if input.len() > 4 {
            panic!("CCIndexU32 noise only supports 1-4 D noise");
        }

        let input = input.into_iter()
                         .zip(DIM_PRIMES.into_iter())
                         .map(|(a, b)| a.wrapping_mul(b))
                         .fold(0u32, |tot, x| tot.wrapping_add(x));

        let output = cc_index_noise(input, seed);
        output as f32 / u32::MAX as f32
    }
}

impl NoiseFuncF32 for CCIndexF32 {
    fn noise(&self, input : &[f32]) -> f32 {
        if input.len() > 4 {
            panic!("CCIndexF32 noise only supports 1-4 D noise");
        }

        let input = input.into_iter()
                         .map(|x| u32::from_be_bytes(x.to_be_bytes()))
                         .zip(DIM_PRIMES.into_iter())
                         .map(|(a, b)| a.wrapping_mul(b))
                         .fold(0u32, |tot, x| tot.wrapping_add(x));

        let output = cc_index_noise(input, 0);
        output as f32 / u32::MAX as f32
    }

    fn s_noise(&self, input : &[f32], seed : u32) -> f32 {
        if input.len() > 4 {
            panic!("CCIndexF32 noise only supports 1-4 D noise");
        }

        let input = input.into_iter()
                         .map(|x| u32::from_be_bytes(x.to_be_bytes()))
                         .zip(DIM_PRIMES.into_iter())
                         .map(|(a, b)| a.wrapping_mul(b))
                         .fold(0u32, |tot, x| tot.wrapping_add(x));

        let output = cc_index_noise(input, seed);
        output as f32 / u32::MAX as f32
    }
}

/*fn cc_noise(mut input : u32, seed : u32) -> u32 {

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
}*/

/*impl NoiseFuncU32 for CCNoiseFuncU32 {
    fn noise(&self, input : &[u32]) -> f32 {
        if input.len() > 4 {
            panic!("noise only supports 1-4 D noise");
        }

        let input = input.into_iter()
                         .zip(DIM_PRIMES.into_iter())
                         .map(|(a, b)| a.wrapping_mul(b))
                         .fold(0u32, |tot, x| tot.wrapping_add(x));

        let output = cc_noise(input, 0);
        output as f32 / u32::MAX as f32
    }

    fn s_noise(&self, input : &[u32], seed : u32) -> f32 {
        if input.len() > 4 {
            panic!("noise only supports 1-4 D noise");
        }

        let input = input.into_iter()
                         .zip(DIM_PRIMES.into_iter())
                         .map(|(a, b)| a.wrapping_mul(b))
                         .fold(0u32, |tot, x| tot.wrapping_add(x));

        let output = cc_noise(input, seed);
        output as f32 / u32::MAX as f32
    }
}


                let x = x.to_be_bytes(); let y = y.to_be_bytes();
                let z = z.to_be_bytes();
                let t = t.to_be_bytes();

                let x = u64::from_be_bytes(x);
                let y = u64::from_be_bytes(y);
                let z = u64::from_be_bytes(z);
                let t = u64::from_be_bytes(t);

impl NoiseFuncF64 for CCNoiseFuncF64 {
    fn noise(&self, input : &[f64]) -> NoiseF64 {
    }

    fn s_noise(&self, input : &[f64], seed : u32) -> NoiseF64 {
    }
}*/

