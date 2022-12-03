#[derive(Debug)]
enum Coin {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::Bronze => 1,
            Coin::Silver => 100,
            Coin::Gold => 10_000,
            _ => 0,
        }
    }
}

fn main() {
    let diamond = Coin::Diamond;

    let value = diamond.value();

    println!("{}", value);
}
