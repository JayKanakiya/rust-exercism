pub fn reverse(input: &str) -> String {
    let mut res = String::new();
    for i in input.chars().rev() {
        res.push(i);
    }
    // println!("{}",res);
    res;
}
