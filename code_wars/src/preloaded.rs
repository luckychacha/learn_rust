use std::collections::HashMap;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

pub fn decode_bits(encoded: &str) -> String {
    // Trim excess zeros at the start and end
    let encoded = encoded.trim_matches('0');
    
    // Get the length of a time unit by finding the shortest sequence of zeros or ones,
    // this will represent a time unit of one which equals a dot
    let rate = {
        let rate_ones = encoded
            .split("0")
            .filter_map(|ones| (!ones.is_empty()).then(|| ones.len()))
            .min()
            .unwrap_or(usize::MAX);
        let rate_zeros = encoded
            .split("1")
            .filter_map(|zeros| (!zeros.is_empty()).then(|| zeros.len()))
            .min()
            .unwrap_or(usize::MAX);
        println!("0:{:?}", rate_zeros);
        println!("1:{:?}", rate_ones);
        println!("res:{:?}", rate_zeros.min(rate_ones));
        rate_zeros.min(rate_ones)
    };

    // Parse the encoded message
    // docode:".... . -.--   .--- ..- -.. ."
    encoded
        .chars() // Iterate through the characters
        .step_by(rate) // Only parse every n-th code
        .collect::<String>() // Collect it into a string
        // Begin converting from 1/0 to dot/dash
        .replace("111", "-") // Dash
        .replace("1", ".") // Dot
        .replace("0000000", "   ") // Word seperator
        .replace("000", " ") // Letter seperator
        .replace("0", "") // Dot/Dash seperator
}

pub fn decode_morse(encoded: &str) -> String {
    let morse_code: HashMap<String, String> = [("-...-", "="), ("...", "S"), ("--..", "Z"), (".-", "A"), ("-....-", "-"), (".-..-.", "\""), ("...--", "3"), (".----.", "\'"), ("-.--", "Y"), ("-----", "0"), (".-.-.", "+"), ("--.", "G"), ("-", "T"), (".-.-.-", "."), (".----", "1"), ("-..", "D"), ("-.-.--", "!"), ("--", "M"), ("--...", "7"), (".", "E"), ("-.-", "K"), ("---..", "8"), ("-.-.-.", ";"), ("-....", "6"), (".--.", "P"), ("..--..", "?"), ("----.", "9"), ("-.--.", "("), ("-.", "N"), (".---", "J"), ("-..-.", "/"), ("---...", ","), ("..---", "2"), ("..-", "U"), ("...-..-", "$"), ("..-.", "F"), ("...---...", "SOS"), ("-.-.", "C"), ("..", "I"), ("-..-", "X"), (".--.-.", "@"), ("..--.-", "_"), (".-.", "R"), (".....", "5"), ("....", "H"), ("--.-", "Q"), (".-..", "L"), ("...-", "V"), ("-...", "B"), ("....-", "4"), ("-.--.-", ")"), ("--..--", ","), (".-...", "&"), ("---", "O"), (".--", "W")]
                .iter().map(|&(k,v)|(k.to_string(),v.to_string())).collect();
    encoded
        .trim()
        .split("   ")
        .map(|word| {
            word.split(" ")
                .filter_map(|letter| morse_code.get(letter).map(|letter| letter.clone()))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}