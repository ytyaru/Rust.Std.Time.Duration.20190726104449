fn main() {
    let du = std::time::Duration::from_secs(1) + std::time::Duration::from_nanos(234567890);
    println!("{:?}", du);
    println!("{:?}", du.as_secs());
    println!("{:?}", du.subsec_nanos());

    let du = std::time::Duration::from_secs(5) - std::time::Duration::from_secs(2); // thread 'main' panicked at 'overflow when subtracting durations'
//    let du = std::time::Duration::from_secs(1) - std::time::Duration::from_secs(2); // thread 'main' panicked at 'overflow when subtracting durations'
    println!("{:?}", du);
    println!("{:?}", du.as_secs());
    println!("{:?}", du.subsec_nanos());
}
