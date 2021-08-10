pub fn format_duration(seconds: u64) -> String {
    // Complete this function
    let mut seconds = seconds;
    if seconds <= 0 {
        return String::from("")
    }
    let mut res: Vec<String> = Vec::new();
    let mut count: u64 = 5;

    let year: u64 = 365*24*60*60;
    let day: u64 = 24*60*60;
    let hour: u64 = 60*60;
    let minute: u64 = 60;
    let second: u64 = 1;

    while count > 0 {
        if count == 5 {
            let y = seconds / year;
            seconds = seconds % year;
            if y > 0 {
                if y > 1 {
                    res.push(format!("{} years", y));
                } else {
                    res.push(format!("{} year", y));
                }
            } 
        }
        if count == 4 {
            let d = seconds / day;
            seconds = seconds % day;
            if d > 0 {
                if d > 1 {
                    res.push(format!("{} days", d));
                } else {
                    res.push(format!("{} day", d));
                }
            } 
        }
        if count == 3 {
            let y = seconds / hour;
            seconds = seconds % hour;
            if y > 0 {
                if y > 1 {
                    res.push(format!("{} hours", y));
                } else {
                    res.push(format!("{} hour", y));
                }
            } 
        }
        if count == 2 {
            let y = seconds / minute;
            seconds = seconds % minute;
            if y > 0 {
                if y > 1 {
                    res.push(format!("{} minutes", y));
                } else {
                    res.push(format!("{} minute", y));
                }
            } 
        }
        if count == 1 {
            let y = seconds / second;
            seconds = seconds % second;
            if y > 0 {
                if y > 1 {
                    res.push(format!("{} seconds", y));
                } else {
                    res.push(format!("{} second", y));
                }
            } 
        }
        count -= 1;
    }
    if res.len() <= 2 {
        return res.join(" and ")
    }
    
    // let mut specific: Vec<String> = Vec::new();
    // for (idx, item) in res.iter().enumerate() {
    //     if idx < res.len() - 2 {
    //         specific.push(format!("{}, ", item));
    //     } else if idx == res.len() - 2 {
    //         specific.push(format!("{} and ", item));
    //     } else {
    //         specific.push(format!("{}", item));
    //     }

    // }

    // specific.concat()
    res.split_last()
        .map(|(last, others)| {
        others.join(", ") + " and " + last
    })
        .unwrap()
}