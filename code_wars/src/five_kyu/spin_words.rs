pub fn spin_words(words: &str) -> String {
    words
        .split_ascii_whitespace()
        .into_iter()
        .enumerate()
        .map(|(idx, item)| {
            let mut sp = " ";
            if idx == 0 {
                sp = "";
            }
            if item.len() >= 5 {
                return format!("{}{}", sp, item.chars().rev().collect::<String>());
            }
            format!("{}{}", sp, item.to_string())
        })
        .collect()
}
// best practice
// fn spin_words(words: &str) -> String {
//     words.split_ascii_whitespace().map(|word| match word.len() >= 5 {
//         true => word.chars().rev().collect(),
//         false => word.to_string()
//     }).collect::<Vec<String>>().join(" ")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spin_words() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(
            spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }
}
