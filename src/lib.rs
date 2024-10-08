pub struct Counter{
    count: u32,   
}

impl Counter {
    pub fn new() -> Self {
        Self {count:0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count +=1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;

    #[test]
    fn calling_one_by_one() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn combining_two_iters(){
        let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a*b)
        .filter(|x| x%3 == 0)
        .sum();
        assert_eq!(sum, 18);
    }
    
}