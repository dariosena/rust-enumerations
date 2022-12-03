fn main() {
    let optional: Option<u32> = Some(1);

    let Some(value) = optional else {
        return;
    };

    println!("{}", value);
}
