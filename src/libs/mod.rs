pub fn pp<'a>(object: &'a dyn std::fmt::Debug) {
    println!("{:#?}", object);
}
