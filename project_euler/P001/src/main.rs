fn main() {
    let sum = range(3i, 1000i).fold(0i, |acc, i|
        if pick(i) { acc + i } else { acc }
    );
    println!("{:d}", sum);
}

fn pick(n: int) -> bool {
    multiple_of(3i, n) || multiple_of(5i, n)
}

fn multiple_of(of: int, n: int) -> bool {
    n % of == 0i
}
