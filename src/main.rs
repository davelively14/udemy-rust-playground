mod fib;

fn main() {
    for i in 1..41 {
        println!("{}: {}", i, fib::basic(i));
    }

    for i in 1..41 {
        println!("{}: {}", i, fib::dynamic(i));
    }
}
