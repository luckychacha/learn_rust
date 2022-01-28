pub fn longest_palindrome(s: String) -> String {
    let (mut start, mut end) = (0, 0);
    let s = s.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![false; s.len()]; s.len()];

    for i in (0..s.len()).rev() {
        for j in i..s.len() {
            if i == j || j - i == 1 && s[i] == s[j] {
                dp[i][j] = true;
            } else {
                dp[i][j] = s[i] == s[j] && dp[i+1][j-1];
            }
            if dp[i][j] && j - i > end - start {
                start = i;
                end = j;
            }
        }
    }

    s[start..=end].iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_palindrome_2() {
        let s = String::from("ab");
        assert_eq!(longest_palindrome(s), String::from("a"));
    }
}