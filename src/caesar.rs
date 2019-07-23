use std::collections::HashSet;

pub fn cipher(text: &str, rotation: u8) -> String {
    let lower_letters: HashSet<char> = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
    .iter()
    .cloned()
    .collect();

    let upper_letters: HashSet<char> = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
    .iter()
    .cloned()
    .collect();

    let mut res: Vec<char> = vec![' '; text.len()];
    for (idx, letter) in text.chars().enumerate() {
        res[idx] = if upper_letters.contains(&letter) {
            (((((letter as u8) - ('A' as u8)) + rotation) % 26) + ('A' as u8)) as char
        } else if lower_letters.contains(&letter) {
            (((((letter as u8) - ('a' as u8)) + rotation) % 26) + ('a' as u8)) as char
        } else {
            letter
        };
        println!("idx: {}\tletter: {}\tres[idx]: {}", idx, letter, res[idx]);
    }

    res.into_iter().collect()
}

pub fn decipher(text: &str, rotation: u8) -> String {
    let lower_letters: HashSet<char> = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
    .iter()
    .cloned()
    .collect();

    let upper_letters: HashSet<char> = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
    .iter()
    .cloned()
    .collect();

    let mut res: Vec<char> = vec![' '; text.len()];
    for (idx, letter) in text.chars().enumerate() {
        res[idx] = if upper_letters.contains(&letter) {
            (((((letter as u8) - ('A' as u8)) + (26 - rotation)) % 26) + ('A' as u8)) as char
        } else if lower_letters.contains(&letter) {
            (((((letter as u8) - ('a' as u8)) + (26 - rotation)) % 26) + ('a' as u8)) as char
        } else {
            letter
        };
        println!("idx: {}\tletter: {}\tres[idx]: {}", idx, letter, res[idx]);
    }

    res.into_iter().collect()
}
