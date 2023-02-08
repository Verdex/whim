
use std::ops::{Bound, RangeBounds};

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
    fn range<R:RangeBounds<u64>>(&mut self, range : R) -> u64 {
        let start = to_start_u64(range.start_bound()) as f64;
        let end = to_end_u64(range.end_bound()) as f64;
        (self.ratio() * (end + 1.0 - start) + start).floor() as u64 
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
    fn range<R:RangeBounds<u32>>(&mut self, range : R) -> u32 {
        let start = to_start_u32(range.start_bound()) as f64;
        let end = to_end_u32(range.end_bound()) as f64;
        (self.ratio() * (end + 1.0 - start) + start).floor() as u32 
    }
    fn ratio(&mut self) -> f64 {
        let a = self.next();
        let b = self.next();
        if a < b { a as f64 / b as f64 }
        else { b as f64 / a as f64 }
    }
} 

pub struct XorShift32 {
    state : u32
}

impl XorShift32 {
    pub fn new() -> Self {
        XorShift32 { state: INIT as u32 }
    }
    pub fn seed(input : u32) -> Self {
        XorShift32 { state : input | 1 }
    }
}

impl RandomU32 for XorShift32 {
    fn next(&mut self) -> u32 {
        let mut t = self.state;
        t ^= t << 11;
        t ^= t >> 19;
        t ^= t << 7;
        t ^= t >> 17;
        t ^= t << 3;
        self.state = t;
        return t;
    }
    fn range<R:RangeBounds<u32>>(&mut self, range : R) -> u32 {
        let start = to_start_u32(range.start_bound()) as f64;
        let end = to_end_u32(range.end_bound()) as f64;
        (self.ratio() * (end + 1.0 - start) + start).floor() as u32 
    }
    fn ratio(&mut self) -> f64 {
        let a = self.next();
        let b = self.next();
        if a < b { a as f64 / b as f64 }
        else { b as f64 / a as f64 }
    }
} 

fn to_start_u32(x : Bound<&u32>) -> u32 {
    match x {
        Bound::Included(x) => *x,
        Bound::Unbounded => 0,
        Bound::Excluded(x) => *x + 1,
    }
}

fn to_end_u32(x : Bound<&u32>) -> u32 {
    match x {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x - 1,
        Bound::Unbounded => u32::MAX,
    }
}

fn to_start_u64(x : Bound<&u64>) -> u64 {
    match x {
        Bound::Included(x) => *x,
        Bound::Unbounded => 0,
        Bound::Excluded(x) => *x + 1,
    }
}

fn to_end_u64(x : Bound<&u64>) -> u64 {
    match x {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x - 1,
        Bound::Unbounded => u64::MAX,
    }
}