use std::io;

fn main() {
    
    println!("Enter the number for the fibonacci sequence.");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to readline.");

    let num: u64 = num.trim().parse().expect("Please Type a number!");
    for i in 0..num {
        fib(i);
    }
}

fn fib(n: u64) {
    let mut _start: u64 = 0;
    let mut next: u64 = 1;
    let mut third_wheel: u64 = 0;
    let mut n_iter: u64 = 0;

    if n == 0 {
        println!("fib({n}) is {_start}");
    } else if n == 1 {
        println!("fib({n}) is {next}");
    } else {
        loop {
            next = _start + next;
            _start = third_wheel;
            third_wheel = next;
            n_iter += 1;

            if n_iter == n {
                break println!("fib({n}) is {next}");
            }
        };
    }
}
