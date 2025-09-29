
pub fn mut_string() {
    let mut x = "abc".to_owned();
    let y = &mut x;

    y.push_str("123");
    // println!("{y}");
    // println!("{x}");

    x.push_str("----");
    // println!("{y}");
}