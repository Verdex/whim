
use crate::noise::*;

pub struct CCNoiseFunc32(); 

const Y_PRIME : u32 = 0x6AC2D;
const Z_PRIME : u32 = 0x41ED2;
const T_PRIME : u32 = 0xCD67;

// 11110010101011101111101000010111:4071553559:F2AEFA17
// 11110000100011011000100001010111:4035807319:F08D8857
// 00110101101000111011000011000101:899920069:35A3B0C5

// 00011110000100011100110001010011:504482899:1E11CC53
// 00000110111010111010100011011111:116107487:6EBA8DF
// 00010101100001101011101101110011:361151347:1586BB73

// 10110101110110001011010110100101:3050878373:B5D8B5A5
// 10101100010110110101001001010011:2891666003:AC5B5253
// 10111110011001001000100000000001:3194259457:BE648801
// 10011001001000001010100101110001:2569054577:9920A971


// 00011001100110011100010100101111:429507887:1999C52F


fn cc_noise(mut x : u32, seed : u32) -> u32 {
    x = x.wrapping_add(11);
    x = x.wrapping_mul(17);
    x = x.wrapping_mul(x);
    x ^= x >> 3;
    x
}

impl NoiseFunc32 for CCNoiseFunc32 {
    fn noise(&self, input : &[u32]) -> Noise32 {
        match input {
            [x, y, z, t] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME))
                             .wrapping_add(z.wrapping_mul(Z_PRIME))
                             .wrapping_add(t.wrapping_mul(T_PRIME));
                let n = cc_noise(input, 0);
                Noise32(n)
            },
            [x, y, z] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME))
                             .wrapping_add(z.wrapping_mul(Z_PRIME));
                let n = cc_noise(input, 0);
                Noise32(n)
            },
            [x, y] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME));
                let n = cc_noise(input, 0);
                Noise32(n)
            },
            [x] => {
                let n = cc_noise(*x, 0);
                Noise32(n)
            },
            _ => panic!("CCNoise32 only supports 1-4 D noise"),
        }
    }

    fn s_noise(&self, input : &[u32], seed : u32) -> Noise32 {
        match input {
            [x, y, z, t] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME))
                             .wrapping_add(z.wrapping_mul(Z_PRIME))
                             .wrapping_add(t.wrapping_mul(T_PRIME));
                let n = cc_noise(input, seed);
                Noise32(n)
            },
            [x, y, z] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME))
                             .wrapping_add(z.wrapping_mul(Z_PRIME));
                let n = cc_noise(input, seed);
                Noise32(n)
            },
            [x, y] => {
                let input = x.wrapping_add(y.wrapping_mul(Y_PRIME));
                let n = cc_noise(input, seed);
                Noise32(n)
            },
            [x] => {
                let n = cc_noise(*x, seed);
                Noise32(n)
            },
            _ => panic!("CCNoise32 only supports 1-4 D noise"),
        }
    }
}

// seed is probably just add in towards the beginning but not first thing
// multi param is something like  x + (prime1 * y) + (prime2 * z) ...

// xor (shift dependent amount)
// special bit indicates mul against prime 

