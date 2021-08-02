
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