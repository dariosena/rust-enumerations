fn main() {
    let optional: Option<u32> = Some(1);

    if let Some(value) = optional {
        println!("{}", value);
    };
}
