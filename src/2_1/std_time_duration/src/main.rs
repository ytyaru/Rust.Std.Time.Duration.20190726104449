fn main() {
    let du = std::time::Duration::from_nanos(12345678901);
    println!("{:?}", du);
    println!("{:?}", du.as_secs());
    println!("{:?}", du.subsec_nanos());
}
