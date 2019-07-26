fn main() {
    let du = std::time::Duration::new(1, 0);
    println!("{:?}", du);
    println!("{:?}", du.as_secs());
    println!("{:?}", du.subsec_nanos());
}
