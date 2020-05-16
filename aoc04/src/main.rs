#[derive(Debug)]
struct Pin(usize, usize, usize, usize, usize, usize);

impl Pin {
    fn new(num: usize) -> Pin {
        let d0 = num / 100000;
        let d1 = (num % 100000) / 10000;
        let d2 = (num % 10000) / 1000;
        let d3 = (num % 1000) / 100;
        let d4 = (num % 100) / 10;
        let d5 = num % 10;
        Pin(d0, d1, d2, d3, d4, d5)
    }

    fn adjacent(&self) -> bool {
        self.0 == self.1
            || self.1 == self.2
            || self.2 == self.3
            || self.3 == self.4
            || self.4 == self.5
    }

    fn adjacent_doubles(&self) -> bool {
        (self.0 == self.1 && self.1 != self.2)
            || (self.0 != self.1 && self.1 == self.2 && self.2 != self.3)
            || (self.1 != self.2 && self.2 == self.3 && self.3 != self.4)
            || (self.2 != self.3 && self.3 == self.4 && self.4 != self.5)
            || (self.3 != self.4 && self.4 == self.5)
    }

    fn growing(&self) -> bool {
        self.0 <= self.1
            && self.1 <= self.2
            && self.2 <= self.3
            && self.3 <= self.4
            && self.4 <= self.5
    }
}

fn main() {
    let lower = 145852;
    let upper = 616942;

    let pins: Vec<Pin> = (lower..=upper).into_iter().map(|p| Pin::new(p)).collect();

    let num_pins: usize = pins.iter().filter(|p| p.adjacent() && p.growing()).count();
    println!("Solution Part 1: {}", num_pins);

    let num_pins: usize = pins
        .iter()
        .filter(|p| p.adjacent_doubles() && p.growing())
        .count();
    println!("Solution Part 2: {}", num_pins);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_adjacent() {
        assert!(!Pin::new(123456).adjacent());
        assert!(Pin::new(122456).adjacent());
    }

    #[test]
    fn rising() {
        assert!(!Pin::new(123454).growing());
        assert!(Pin::new(123455).growing());
    }
}
