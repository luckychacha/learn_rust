pub fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut i = 0;
    let mut res: Vec<Vec<i8>> = Vec::new();
    // init
    loop {
        if i == size {
            break;
        } 
        let tmp: Vec<i8> = vec![1; size];
        res.push(tmp);
        i += 1;
    }

    let mut way = 1;
    let mut round = 1;
    let mut x: usize = 1;
    let mut y: usize = 0;
    let mut step: i64 = size as i64 - 1;
    loop {
        if step == 0 {
            break;
        }

        match way {
            1 => {
                let mut tmp = 0;
                loop {
                    if tmp == step {
                        x += 1;
                        y -=1;
                        break;
                    }
                    res[x][y] = 0;
                    y += 1;
                    tmp += 1;

                }
                round += 1;
                way += 1;
            },
            2 => {
                let mut tmp = 0;
                loop {
                    if tmp == step {
                        x -= 1;
                        y -= 1;
                        break;
                    }
                    res[x][y] = 0;
                    x += 1;
                    tmp += 1;

                }
                round += 1;

                way += 1;

            },
            3 => {
                let mut tmp = 0;
                loop {
                    if tmp == step {
                        y += 1;
                        x -= 1;
                        break;
                    }
                    res[x][y] = 0;
                    y -= 1;
                    tmp += 1;

                }
                round += 1;
                way += 1;

            },
            4 => {
                let mut tmp = 0;
                loop {
                    if tmp == step {
                        x += 1;
                        y += 1;
                        break;
                    }
                    res[x][y] = 0;
                    x -= 1;
                    tmp += 1;

                }
                round += 1;
                way = 1;
            },
            _ => {

            },
        }

        if round % 2 == 0 {
            if step < 2 {
                break;
            }
            step -= 2;
        }
    }

    res
}

/// best practice
// fn spiralize(size: usize) -> Vec<Vec<i8>> {
//     let mut spiral = vec![vec![0; size]; size];
//     let mut value = 1;
    
//     for j in 0..(size + 1) / 2 {
//         for i in j..(size - j) {
//             spiral[i][j] = value;
//             spiral[j][i] = value;

//             spiral[i][size - 1 - j] = value;
//             spiral[size - 1 - j][i] = value;
//         }

//         value = (value + 1) % 2;
        
//         if j < (size - 1) / 2 || spiral[j][j - 1] == 1 {
//             spiral[j + 1][j] = value;
//         }
//     }

//     spiral
// }

/// clever
// fn spiralize(n: usize) -> Vec<Vec<i8>> {
//     (0..n)
//         .map(|i| {
//             (0..n)
//                 .map(|j| {
//                     let min = i.min(j).min(n - i - 1).min(n - j - 1);
//                     (if n % 2 == 0 && i == n / 2 && j == n / 2 - 1 {
//                         0
//                     } else if j == min && i == min + 1 {
//                         min % 2
//                     } else {
//                         1 - min % 2
//                     }) as i8
//                 })
//                 .collect()
//         })
//         .collect()
// }


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spiralize() {
        let res = spiralize(8);
        assert_eq!(
            res,
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}