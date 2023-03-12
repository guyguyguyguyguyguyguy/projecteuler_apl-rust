#[allow(dead_code)]
// sum of multiples of 3|5
fn q1() -> u32 {
    let nums = 0..1000;
    nums.filter(|x| (x % 3 == 0) || (x % 5 == 0)).sum()
}

#[allow(dead_code)]
// fibonnaci
fn q2() -> u32 {
    let mut n1: u32 = 1;
    let mut n2: u32 = 2;
    let mut sum: u32 = 0;
    while n2 < 4000000 {
        if n2 % 2 == 0 {
            sum += n2;
        }
        n2 += n1;
        n1 = n2 - n1;
    }
    sum
}

#[allow(dead_code)]
// largest prime factor
fn q3() -> u64 {
    let n = 600851475143;
    let mut p_fac = 2;
    let mut target = n;

    while target > 1 {
        if target % p_fac == 0 {
            target /= p_fac;
        } else {
            p_fac += 1;
        }
    }
    std::cmp::max(target, p_fac)
}

fn palindrome_test(n: &u64) -> bool {
    let my_str: String = format!("{:?}", n);
    let reversed = my_str.chars().rev().collect::<String>();

    my_str == reversed
}

#[allow(dead_code)]
// pelindrome of 3-digit numbers, very inefficient goes through all possible numbers before returning the largest one
// TODO: Find way to know to stop early
fn q4() -> u64 {
    let mut pal: u64 = 0;
    let mut p_pal: u64;
    for n in (100..1000).rev() {
        for m in (100..1000).rev() {
            p_pal = n * m;
            if palindrome_test(&p_pal) {
                pal = if p_pal > pal { p_pal } else { pal }
            }
        }
    }
    pal
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    let mut tmp;
    while y != 0 {
        tmp = y;
        y = x % y;
        x = tmp;
    }
    return x;
}

#[allow(dead_code)]
fn q5() -> u64 {
    (1..21)
        .into_iter()
        .reduce(|x, y| x * y / gcd(x, y))
        .unwrap()
}

#[allow(dead_code)]
fn q6() -> u64 {
    let nums = 1u64..101;
    nums.clone().reduce(|a, b| a + b).unwrap().pow(2) - nums.reduce(|a, b| a + b.pow(2)).unwrap()
}

#[allow(dead_code)]
fn q7() -> u64 {
    // Use Reimann approx to get range for sieve
}

fn main() {
    let ans = q6();
    println!("{}", ans);
}
