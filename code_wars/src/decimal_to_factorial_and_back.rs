pub fn fac(n: u64) -> u64 {
    (2..n).product()
    // let mut flag = 1;
    // let mut res = 1;

    // loop {
    //     if flag > n {
    //         break;
    //     }
    //     res *= flag;
    //     flag += 1;
    // }

    // res
}

pub fn dec2_fact_string(nb: u64) -> String {
    // your code
    let mut n = nb;
    let mut flag = 1;

    let mut tab: Vec<String> = vec![];

    while n > 0 {
        let mut num = n % flag;
        if num < 10 {
            tab.push(num.to_string());
        } else {
            tab.push(
                ((num as u8 + 55 as u8) as char).to_string()
            );
        }

        n = n / flag;
        flag += 1;
    }

    tab.reverse();
    tab
        .into_iter()
        .map(|i| {
            i.to_string()
        })
        .collect::<String>()
}

pub fn fact_string_2dec(s: String) -> u64 {
    // your code
    let mut s: Vec<char> = s.chars().collect();
    s.reverse();
    let mut res: u64 = 0;
    let mut flag: u64 = 0;
    for elem in s {
        if (elem as u64 - '0' as u64) < 10 {
            res += fac(flag) * (elem as u64 - '0' as u64);
        } else {
            res += fac(flag) * (elem as u64 - 'A' as u64 + 10);
        }

        flag += 1;
    }

    res
}
