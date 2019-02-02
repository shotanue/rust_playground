fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}


fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        return x.to_string();
    } else {
        return y.to_string();
    }
}