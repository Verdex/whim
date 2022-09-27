
trait Random { 
    type T;

    fn next(&mut self) -> Self::T;
    fn range(&mut self, start : Self::T, end : Self::T) -> Self::T;
    fn ratio(&mut self) -> f64;
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
