pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut divisor = 2; // always prime
    let mut n = n;

    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }

        divisor += 1;
    }

    factors
}

// TODO: optimize?
fn next_prime(n: u64) -> u64 {
    let mut next = n;

    loop {
        next += 1;
        if is_prime(next) {
            return next;
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i <= n / i {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}
