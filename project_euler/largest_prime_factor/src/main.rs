fn main() {
    let mut num = 600851475143i;
    let mut cur = 2i;

    while cur <= num {
        if num % cur == 0 {
            num /= cur;
        } else {
            cur += 1;
        }
    }

    println!("{}", cur);
}
