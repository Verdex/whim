
pub trait Random { 
    type T;

    fn next(&mut self) -> Self::T;
    fn range(&mut self, start : Self::T, end : Self::T) -> Self::T;
    fn ratio(&mut self) -> f64;
}

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

impl Random for Lcg60 {
    type T = u64;
    fn next(&mut self) -> Self::T {
        self.state = self.state.wrapping_mul(A);
        return self.state >> 4;
    }
    fn range(&mut self, start : Self::T, end : Self::T) -> Self::T {
        (self.ratio() * (end + 1 - start) as f64 + start as f64).floor() as Self::T
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

impl Random for Pcg32Shift {
    type T = u32;
    fn next(&mut self) -> Self::T {
        self.state = self.state.wrapping_mul(A);
        let shift = (self.state >> 60) as u8;
        return (self.state >> (shift + 13)) as u32;
    }
    fn range(&mut self, start : Self::T, end : Self::T) -> Self::T {
        (self.ratio() * (end + 1 - start) as f64 + start as f64).floor() as Self::T
    }
    fn ratio(&mut self) -> f64 {
        let a = self.next();
        let b = self.next();
        if a < b { a as f64 / b as f64 }
        else { b as f64 / a as f64 }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn lcg_60_seed_makes_state_odd() {
        let r = Lcg60::seed(0);
        assert!(r.state % 2 == 1);
    }

    #[test]
    fn lcg_60_shift_range_hits_min_and_max() {
        let mut r = Lcg60::new();
        let mut numbers : HashMap<u64, usize> = HashMap::new();
        for i in 5..=20 {
            numbers.insert(i, 0);
        }

        for _ in 0..1_000 {
            let n = r.range(5, 20);
            let count = numbers.get_mut(&n).unwrap();
            *count += 1;
        }

        assert!( numbers.into_values().all(|n| n != 0) );
    }

    #[test]
    fn pcg_32_shift_seed_makes_state_odd() {
        let r = Pcg32Shift::seed(0);
        assert!(r.state % 2 == 1);
    }

    #[test]
    fn pcg_32_shift_range_hits_min_and_max() {
        let mut r = Pcg32Shift::new();
        let mut numbers : HashMap<u32, usize> = HashMap::new();
        for i in 5..=20 {
            numbers.insert(i, 0);
        }

        for _ in 0..1_000 {
            let n = r.range(5, 20);
            let count = numbers.get_mut(&n).unwrap();
            *count += 1;
        }

        assert!( numbers.into_values().all(|n| n != 0) );
    }
}
