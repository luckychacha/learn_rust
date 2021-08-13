use code_wars::{anagrams, array_diff, bouncing_ball, persistence, print, product_fib, rot13, sum_pairs};
mod preloaded;
mod format_duration;
mod five_kyu;

fn main() {
    println!("{:?}", five_kyu::primes::count_kprimes(5, 1000, 1100));
    println!("{:?}", five_kyu::primes::puzzle(143));
    println!("{:?}", five_kyu::primes::puzzle(144));

    println!("{:?}", array_diff(vec![1,2,2,2,3], vec![2]));

    // 4
    println!("res:{}", persistence(999));
    // 3
    // println!("res:{}", persistence(39));
    // 0
    // println!("res:{}", persistence(4));

    println!("{}", rot13(&String::from("Nibvq fhpprff ng nyy pbfgf!")));

    println!("{:?}",print(5));
    println!("{:?}", product_fib(4895));
    // println!("{:?}", product_fib(5895));

    println!("bouncing_ball:{:?}", bouncing_ball(3.0, 0.66, 1.5));
    // println!("bouncing_ball:{:?}", bouncing_ball(2.0, 0.5, 1.0));
    // println!("bouncing_ball:{:?}", bouncing_ball(30.0, 0.66, 1.5));
    // println!("bouncing_ball:{:?}", bouncing_ball(40.0, 0.4, 10.0));
    // println!("bouncing_ball:{:?}", bouncing_ball(10.0, 0.6, 10.0));

    let words: Vec<String> = ["aabb", "abcd", "bbaa", "dada"].iter().map(|w| w.to_string()).collect();
    println!(
        "anagrams:{:?}",
        anagrams("abba", &words)
    );


    let res = preloaded::decode_morse(
        &preloaded::decode_bits(
            "1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"
        )
    );
    println!("{:?}", res);

    println!("{:?}", format_duration::format_duration(3662));

}

#[cfg(test)]
mod test {
    use super::*;

    fn testing_count_kprimes(k: i32, start: i32, nd: i32, exp: Vec<i32>) -> () {
        assert_eq!(five_kyu::primes::count_kprimes(k, start, nd), exp)
    }
    #[test]
    fn basics_count_kprimes() {
        testing_count_kprimes(5, 1000, 1100, vec![1020, 1026, 1032, 1044, 1050, 1053, 1064, 1072, 1092, 1100]);
        testing_count_kprimes(12, 100000, 100100, vec![]);
    }

    #[test]
    fn returns_expected() {
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
    }

    #[test]
    fn test_print() {
        println!("{:?}",print(5));
    }
}