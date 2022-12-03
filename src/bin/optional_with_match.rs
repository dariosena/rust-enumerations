fn main() {
    let optional: Option<u32> = Some(1);

    let value = match optional {
        Some(value) => value,
        None => 0,
    };

    println!("{}", value);
}
