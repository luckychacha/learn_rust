pub fn count_kprimes(k: i32, start: i32, nd: i32) -> Vec<i32> {
    // your code
    let mut flag = start;
    let mut res: Vec<i32> = Vec::new();

    loop {
        if flag > nd {
            break;
        }
        if find_k(flag) == k {
            res.push(flag);
        }

        flag += 1;
    }

    res
}

pub fn puzzle(s: i32) -> i32 {
    // your code
    let a = count_kprimes(1, 0, s);
    let b = count_kprimes(3, 0, s);
    let c = count_kprimes(7, 0, s);

    let mut count = 0;
    for i in a {
        for j in &b {
            for k in &c {
                if i + k + j == s {
                    count += 1;
                }
            }
        }
    }
    count
}


fn find_k(n: i32) -> i32 {
    let mut n = n;
    let mut res: i32 = 0;
    let mut i: i32 = 2;

    loop {
        if i * i > n {
            break;
        }
        while n % i == 0 {
            n = n / i;
            res += 1;
        }
        i += 1;
    }
    if n > 1 {
        res += 1;
    }
    res
}