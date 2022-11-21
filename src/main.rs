fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        println!("not enought arguments");
    }
    let n: u32 = args[0].parse::<u32>().unwrap();
    let e: u32 = args[1].parse::<u32>().unwrap();

    let phi = get_phi(n);
    let d: u32 = (1 % phi) / e;

    println!("phi for number {} = {}", args[0], phi);
    println!("(n,e,d) = ({},{},{})", n, e, d);

    println!("primes of {}: {:?}", n, get_primes(n));
    println!("phi: {}", get_phi_2(n));
}
// Pierwsza metoda na obliczenie phi
fn get_phi(n: u32) -> u32 {
    let nums = get_nums(n);
    nums.len() as u32
}
fn get_nums(n: u32) -> Vec<u32> {
    let mut nums: Vec<u32> = vec![1];
    for i in 2..n {
        let mut has_common_divider = false;
        for j in 2..n {
            if i % j == 0 && n % j == 0 {
                // println!("{} ma dzielnik", i);
                has_common_divider = true;
                break;
            }
        }
        if has_common_divider == false {
            nums.push(i);
        }
    }

    nums
}
// Druga metoda na obliczenie phi
fn get_phi_2(n: u32) -> u32 {
    let primes = get_primes(n);
    if primes.len() != 2 {
        println!("n is not acceptable");
    }
    (primes[0] - 1) * (primes[1] - 1)
}
fn get_primes(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    let mut x = n;
    while x != 1 {
        for i in 2..=x {
            if x % i == 0 {
                x = x / i;
                primes.push(i);
                break;
            }
        }
    }
    primes
}
