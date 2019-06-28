/*
 * Rustのライフタイム。
 * CreatedAt: 2019-06-28
 */
fn main() {
    let str1 = "A".to_string();
    let str2 = "AA";
    println!("{}", longest(str1, str2));
}
fn longest(x: &str, y: &str) -> &str { // error[E0106]: missing lifetime specifier
    if x.len() > y.len() { x }
    else { y }
}
