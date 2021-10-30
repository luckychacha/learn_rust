pub fn next_bigger_number(n: i64) -> i64 {
    let mut chars: Vec<char> = n.to_string().chars().collect::<Vec<char>>();

    chars.sort();

    let last_index: usize = chars.len() - 1;
    let mut x: i32 = last_index as i32;
    let mut y: i32 = x - 1;

    let mut res = 0;
        
    loop {
        if y < 0 {
            x -= 1;
            if x == 0 {
                break;
            } else {
                y = x - 1;
            }
        }
        if chars[x as usize].to_digit(10) > chars[y as usize].to_digit(10) {
            let tmp = chars[x as usize];
            chars[x as usize] = chars[y as usize];
            chars[y as usize] = tmp;
            res = chars.iter().map(|v| v.to_string()).collect::<String>().parse::<i64>().unwrap();
            if res > n {
                break;
            }
        } else {
            y -= 1;
        }
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_bigger_number() {
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
    }
}
