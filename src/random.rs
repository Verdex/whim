
use std::ops::Range;

pub trait RandomU32 { 
    fn next(&mut self) -> u32;
    fn range(&mut self, range : Range<u32>) -> u32;
    fn ratio(&mut self) -> f64;
}

pub trait RandomU64 { 
    fn next(&mut self) -> u64;
    fn range(&mut self, range : Range<u64>) -> u64;
    fn ratio(&mut self) -> f64;
}

pub trait RandomU128 { 
    fn next(&mut self) -> u128;
    fn range(&mut self, range : Range<u128>) -> u128;
    fn ratio(&mut self) -> f64;
}