use std::iter::Map;
use std::ops::Add;

// It should remove all values from list a, which are present in list b keeping their order.
pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    // a.into_iter()
    //     .filter(|item| {
    //     !b.contains(item)
    // })
    //     .collect()

    let mut a = a;
    a.retain(|x| !b.contains(x));

    a
}

/*
    For example:
    persistence(999) // returns 4, because 9*9*9=729, 7*2*9=126,
                     // 1*2*6=12, and finally 1*2=2
    persistence(39) // returns 3, because 3*9=27, 2*7=14, 1*4=4
                    // and 4 has only one digit
    persistence(4) // returns 0, because 4 is already a one-digit number

    best practise
    pub fn persistence(num: u64) -> u64 {
        let mut n = num;
        let mut count = 0;
        while n > 9 {
            n = prod(n);
            count +=1;
        }
        count
    }

    fn prod(n: u64) -> u64 {
        let mut n = n;
        let mut prod = 1;
        while n > 0 {
            prod *= n%10;
            n /= 10;
        }
        prod
    }
 */
pub fn persistence(num: u64) -> u64 {
    let mut num = num;
    let mut count: u64 = 0;
    loop {
        if 1.ge(&num.to_string().len()) {
            break;
        }
        let mut sum = 1;

        for item in num.to_string().chars() {
            match item.to_digit(10) {
                None => {
                    break;
                }
                Some(digit) => {
                    sum *= digit as u64
                }
            }
        }
        num = sum;
        count += 1;
    }
    count
}

/*
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));

    fn to_index(idx : i8) -> usize {
        (idx as i16 + 128) as usize
    }


    fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
        let mut mem = Vec::with_capacity(256);
        mem.resize(256, 0);

        for &i in ints {
            if mem[to_index(s - i)] == 1 { return Some((s - i, i));}
            mem[to_index(i)] = 1;
        }
        return None;
    }
use std::collections::HashSet;

    pub fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
        let mut occurs = HashSet::new();

        for &item in ints {
            if let Some(&a) = occurs.get(&(s - item)) {
                return Some((a, b));
            } else {
                occurs.insert(item);
            }
        }

        None
    }
 */

fn to_index(idx: i8) -> usize {
    (idx as i16 + 128) as usize
}

pub fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut occur = Vec::with_capacity(256);
    occur.resize(256, 0);
    for &i in ints {
        if occur[to_index(s - i)] == 1 {
            return Some((s-i, i));
        }
        occur[to_index(i)] = 1;
    }

    None
}


pub fn rot13(message: &str) -> String {
    // ROT13 is a simple letter substitution cipher
    // that replaces a letter with the letter 13 letters
    // after it in the alphabet. ROT13 is an example of
    // the Caesar cipher.
    message.chars().map(|c| {
        match c.is_ascii_alphabetic() {
            true => {
                let first = match c.is_ascii_uppercase() {
                    true => {
                         'A' as u8
                    }
                    false => {
                        'a' as u8
                    }
                };
                (first + (c as u8 + 13 - first) % 26) as char
            }
            false => {
                c
            }
        }

    }).collect()
}

pub fn print(n: i32) -> Option<String> {
    /*
      assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
      assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()) );
      assert_eq!(print(-3),None);
      assert_eq!(print(2),None);
      assert_eq!(print(0),None);
      assert_eq!(print(1), Some("*\n".to_string()) );
     */
    if n % 2 == 0 || n <= 0 {
        return None
    }

    let n = n as usize;
    let diamond: String = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| {
            format!("{}{}\n", " ".repeat((n-i)/2),"*".repeat(i))
        })
        .collect();

    Some(diamond)

}

pub fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut a = 0;
    let mut b = 1;

    while a * b < prod {
        let c = a + b;
        a = b;
        b = c;
    }
    (a, b, a * b == prod)
}

pub fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    let mut res = 0;

    if bounce.ge(&1_f64) || bounce.le(&0_f64) {
        return -1;
    }
    if h.le(&window) {
        return -1
    }

    let mut h = h;

    while h.gt(&window) {
        println!("h:{}", h);
        res += 1;
        if res % 2 == 1 {
            h *= bounce;
        }

    } 

    res
}

pub fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    // your code here

    let example: Vec<char> = atoz(word);

    words.iter()
        .filter(|word| {
            atoz(&word).eq(&example)
        })
        .cloned()
        .collect::<Vec<_>>()
}

fn atoz(word: &str) -> Vec<char> {
    let mut atoz = word.chars().collect::<Vec<_>>();
    atoz.sort();
    atoz
}
