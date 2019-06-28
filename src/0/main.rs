/*
 * Rustのライフタイム。
 * CreatedAt: 2019-06-28
 */
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}", r); // error[E0597]: `x` does not live long enough
}

