/*
 * Rustのライフタイム。
 * CreatedAt: 2019-06-28
 */
/*
fn main() {
    let str1 = "A".to_string();
    let result;
    {
        let str2 = "AA";
        result = longest(str1.as_str(), str2)
    }
    println!("{}", result); // error[E0425]: cannot find value `str2` in this scope
}
*/
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x }
    else { y }
}
