use std::collections::HashSet;

fn cipher(text: &str, rotation: u8) -> String {
    let mut res: Vec<char> = vec![' '; text.len()];
    for (idx, letter) in text.chars().enumerate() {
        res[idx] = if letter >= 'A' && letter <= 'Z' {
            (((((letter as u8) - ('A' as u8)) + rotation) % 26) + ('A' as u8)) as char
        } else if letter >= 'a' && letter <= 'z' {
            (((((letter as u8) - ('a' as u8)) + rotation) % 26) + ('a' as u8)) as char
        } else {
            letter
        };
    }

    res.into_iter().collect()
}

pub fn cipher_n(text: &str, rotations: &[u8]) -> Vec<String> {
    let mut res = Vec::new();
    for &r in rotations {
        res.push(cipher(text, r))
    }
    res
}

fn decipher(text: &str, rotation: u8) -> String {
    let mut res: Vec<char> = vec![' '; text.len()];
    for (idx, letter) in text.chars().enumerate() {
        res[idx] = if letter >= 'A' && letter <= 'Z' {
            (((((letter as u8) - ('A' as u8)) + (26 - rotation)) % 26) + ('A' as u8)) as char
        } else if letter >= 'a' && letter <= 'z' {
            (((((letter as u8) - ('a' as u8)) + (26 - rotation)) % 26) + ('a' as u8)) as char
        } else {
            letter
        };
    }

    res.into_iter().collect()
}

pub fn decipher_n(text: &str, rotations: &[u8]) -> Vec<String> {
    let mut res = Vec::new();
    for &r in rotations {
        res.push(decipher(text, r))
    }
    res
}
