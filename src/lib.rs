
trait Random { 
    type T;

    fn next(&mut self) -> Self::T;
    fn range(&mut self, start : Self::T, end : Self::T) -> Self::T;
    fn ratio(&mut self) -> f64;
}

struct Lcg60 {
    state : u64
}

impl Lcg60 {
    pub fn new() -> Self {
        Lcg60 { state: 0xFA_07_45_7E_55_76_FD_81 }
    }
    pub fn seed(input : u64) -> Self {
        Lcg60 { state : input | 1 }
    }
}

impl Random for Lcg60 {
    type T = u64;
    fn next(&mut self) -> Self::T {
        const A : u64 = 0x07_5A_A3_42_BB_49_C9_AD;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
