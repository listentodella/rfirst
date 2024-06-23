fn main() {
    let s1 = "Hello".to_string();
    let s2 = "123".to_string();
    let ret = max(&s1, &s2);

    println!("bigger one = {}", ret);

    let ret = get_max(&s1);
    println!("bigger one = {}", ret);
}

fn max(s1: &str, s2: &str) -> &str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

fn get_max(s1: &str) -> &str {
    max(s1, "123456")
}
