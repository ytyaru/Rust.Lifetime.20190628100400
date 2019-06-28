/*
 * Rustのライフタイム。
 * CreatedAt: 2019-06-28
 */
fn main() {
    let str1 = "A".to_string();
    {
        let str2 = "AA";
        println!("{}", longest(str1.as_str(), str2));
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x }
    else { y }
}
