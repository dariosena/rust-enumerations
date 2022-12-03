fn main() {
    let optional: Option<u32> = Some(1);

    println!("{:?}", optional);

    let number_one = optional.unwrap_or(0);

    println!("{}", number_one);
}
