fn main() {
    let s1 = String::from(";sfkgfjnnfspngpksngFJNGQ;RWGN'KDNFKL;N'knfigrngjafsngjlfsngjflsgn;klkflgnsflGJN");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
