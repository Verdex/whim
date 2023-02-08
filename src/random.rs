
use std::ops::RangeBounds;

pub trait RandomU32 { 
    fn next(&mut self) -> u32;
    fn range<R:RangeBounds<u32>>(&mut self, range : R) -> u32;
    fn ratio(&mut self) -> f64;
}

pub trait RandomU64 { 
    fn next(&mut self) -> u64;
    fn range<R:RangeBounds<u64>>(&mut self, range : R) -> u64;
    fn ratio(&mut self) -> f64;
}

pub trait RandomU128 { 
    fn next(&mut self) -> u128;
    fn range<R:RangeBounds<u128>>(&mut self, range : R) -> u128;
    fn ratio(&mut self) -> f64;
}