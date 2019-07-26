fn main() {
    let du = std::time::Duration::from_secs(1);
    println!("{:?}", du);
    println!("{:?}", du.as_secs());
    println!("{:?}", du.subsec_nanos());
}
