
use std::ops::Range;

use crate::random::*;

const INIT : u64 = 0xFA_07_45_7E_55_76_FD_81;
const A : u64 = 0x07_5A_A3_42_BB_49_C9_AD;

pub struct Lcg60 {
    state : u64
}

impl Lcg60 {
    pub fn new() -> Self {
        Lcg60 { state: INIT }
    }
    pub fn seed(input : u64) -> Self {
        Lcg60 { state : input | 1 }
    }
}

impl RandomU64 for Lcg60 {
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(A);
        return self.state >> 4;
    }
    fn range(&mut self, range : Range<u64>) -> u64 {
        (self.ratio() * (range.end + 1 - range.start) as f64 + range.start as f64).floor() as u64 
    }
    fn ratio(&mut self) -> f64 {
        let a = self.next();
        let b = self.next();
        if a < b { a as f64 / b as f64 }
        else { b as f64 / a as f64 }
    }
} 

pub struct Pcg32Shift {
    state : u64
}

impl Pcg32Shift {
    pub fn new() -> Self {
        Pcg32Shift { state: INIT }
    }
    pub fn seed(input : u64) -> Self {
        Pcg32Shift { state : input | 1 }
    }
}

impl RandomU32 for Pcg32Shift {
    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(A);
        let shift = (self.state >> 60) as u8;
        return (self.state >> (shift + 13)) as u32;
    }
    fn range(&mut self, range : Range<u32>) -> u32 {
        (self.ratio() * (range.end + 1 - range.start) as f64 + range.start as f64).floor() as u32 
    }
    fn ratio(&mut self) -> f64 {
        let a = self.next();
        let b = self.next();
        if a < b { a as f64 / b as f64 }
        else { b as f64 / a as f64 }
    }
} 