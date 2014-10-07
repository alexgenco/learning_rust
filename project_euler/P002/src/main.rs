fn main() {
    let mut fib = FibN { max: 4000000i, n0: 0i, n1: 1i };

    let sum = fib.fold(0i, |acc, i| if even(i) { acc + i } else { acc });
    println!("{}", sum)
}

struct FibN { max: int, n0: int, n1: int }

impl Iterator<int> for FibN {
    fn next(&mut self) -> Option<int> {
        let n2 = self.n0 + self.n1;

        if n2 >= self.max {
            None
        } else {
            self.n0 = self.n1;
            self.n1 = n2;
            Some(n2)
        }
    }
}

fn even(n: int) -> bool { n % 2i == 0i }
