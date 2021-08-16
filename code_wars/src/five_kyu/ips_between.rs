// use std::net::Ipv4Addr;

/// examples
/// ips_between("10.0.0.0", "10.0.0.50") ==  50 
/// ips_between("10.0.0.0", "10.0.1.0")  == 256 
/// ips_between("20.0.0.10", "20.0.1.0") == 246

pub fn ips_between(start: &str, end: &str) -> u32 {
    let mut res: i64 = 0;
    let start_vec: Vec<&str> = start.split('.').collect();
    let end_vec: Vec<&str> = end.split('.').collect();
    
    let start_one = start_vec[0].parse::<i64>().unwrap();
    let start_two = start_vec[1].parse::<i64>().unwrap();
    let start_three = start_vec[2].parse::<i64>().unwrap();
    let start_four = start_vec[3].parse::<i64>().unwrap();
    let end_one = end_vec[0].parse::<i64>().unwrap();
    let end_two = end_vec[1].parse::<i64>().unwrap();
    let end_three = end_vec[2].parse::<i64>().unwrap();
    let end_four = end_vec[3].parse::<i64>().unwrap();

    res += end_four - start_four;

    res += (end_three - start_three) * 256;

    res += (end_two - start_two) * 256 * 256;

    res += (end_one - start_one) * 256 * 256 * 256;

    res as u32
}

// pub fn ips_between(start: &str, end: &str) -> u32 {
//     let start: u32 = start.parse::<Ipv4Addr>().unwrap().into();
//     let end: u32 = end.parse::<Ipv4Addr>().unwrap().into();
//     end - start
    // start.split('.')
    //     .zip(end.split('.'))
    //     .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
    //     .enumerate()
    //     .fold(0, |total, (idx, (x, y))| {
    //         total + ((y * 256_i64.pow((3 - idx) as u32)) - (x * 256_i64.pow((3 - idx) as u32)))
    //     }) as u32
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ip() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
        assert_eq!(ips_between("10.0.0.0", "10.0.1.0"), 256);
    }
}