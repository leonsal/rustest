pub fn test1() {
    // Static string allocated in the program binary
    let s1 = "static";
    println!("Static: {}", s1);

    // Dynamic string from static string
    let mut s2 = "dynamic".to_string();
    println!("Dynamic1:{} len:{}", s2, s2.len());

    // Appends to allocated string
    s2.push_str(" string");
    println!("Dynamic1:{} len:{}", s2, s2.len());

    // Insert character to allocated string
    s2.insert(0usize, 'A');
    println!("Dynamic1:{} len:{}", s2, s2.len());

    // Create empty string and appends data
    let mut s3 = String::new();
    s3.push_str("another string áéíóúãẽĩõũ");
    println!("Dynamic2:{} len:{}", s3, s3.len());

    // Appends some characters
    s3.push('1');
    s3.push('2');

    // Iterates over the string characters
    for c in s3.chars() {
        print!("{c} ");
    }
    println!();

    // Iterates over the string bytes
    for b in s3.bytes() {
        print!("{b} ");
    }
    println!();
}
