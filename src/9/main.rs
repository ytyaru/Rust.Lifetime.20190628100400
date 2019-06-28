/*
 * Rustのライフタイム。
 * CreatedAt: 2019-06-28
 */
fn main() {
    let str1 = "AA".to_string();
    let result;
    {
        let str2 = "A";
        result = longest(str1.as_str(), str2); // error[E0597]: `str2` does not live long enough
    }
    println!("{}", result);
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    "A" // なぜか成功してしまう
//    "A".to_string();
//    if x.len() > y.len() { x }
//    else { y }
}
