pub fn length_of_longest_substring(s: String) -> i32 {
    let mut ret = 0;
    let mut l = 0;
    let mut cache = vec![0; 128];

    s.chars().enumerate().for_each(|(i, ch)| {
        // 当前字母上次最后出现的位置。
        // l = cache[ch as usize];
        l = l.max(cache[ch as usize]);
        println!("cache:{} l:{}", cache[ch as usize], l);

        ret = ret.max(i as i32 - l + 1);
        cache[ch as usize] = i as i32 + 1;
    });

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn length_of_longest_substring_1() {
        /*
            若此处 l = cache[ch as usize];
            那么遍历至 k 和 e 两个字母时，l 都是 0，其实是错误的，会导致最长子串是 5；l 应该是 2，最长子串是 3。
        */
        assert_eq!(
            length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}