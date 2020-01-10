fn main() {
    fn go(x: i32) {
        if x == 0 {
            println!("LIFTOFF!!!")
        } else {
            println!("{}", x);
            go(x - 1)
        }
    }
    go(10)
}
