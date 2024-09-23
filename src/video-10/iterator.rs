#[derive(Debug)]
struct Counter{
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self{count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter_operation: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a*b)
    .filter(|e| e%3 == 0)
    .sum();
    println!("{}", counter_operation);

}