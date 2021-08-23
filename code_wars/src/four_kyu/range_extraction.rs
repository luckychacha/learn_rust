
// A format for expressing an ordered list of integers is to use a comma separated list of either
// individual integers
// or a range of integers denoted by the starting integer separated from the end integer in the range by a dash, '-'. The range includes all integers in the interval including both endpoints. It is not considered a range unless it spans at least 3 numbers. For example "12,13,15-17"
// Complete the solution so that it takes a list of integers in increasing order and returns a correctly formatted string in the range format.

pub fn range_extraction(a: &[i32]) -> String {
    // Your solution here
    let mut ranges: Vec<(i32, i32)> = Vec::new();
    let mut i = 0;
    while i < a.len() {
        let start = a[i];
        while a.len() - 1 > i && a[i+1] - a[i] == 1 {
            i += 1;
        }
        let end = a[i];
        i += 1;
        ranges.push((start, end));
    }
    println!("aaaa:{:?}", ranges);

    ranges.iter().map(|(start, end)| {
        if start == end {
            format!("{}", start)
        } else if end - start < 2 {
            format!("{},{}", start, end)
        } else {
            format!("{}-{}", start, end)
        }
    }).collect::<Vec<String>>().join(",")


}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_range_extraction() {
        assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));	

    }
}