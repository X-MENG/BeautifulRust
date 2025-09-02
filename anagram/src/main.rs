use std::collections::HashMap;

fn main() {
    let word = "allergy";
    let possible_anagrams = &[
        "gallery", 
        "ballerina", 
        "regally", 
        "clergy", 
        "largely", 
        "leading"
    ];
    possible_anagrams
        .iter()
        .for_each(|&s| {
            if is_anagram(word, s) {
                println!("{s} is anagram of {word}");
            } else {
                println!("{s} is not anagram of {word}");
            }
        });
}

pub fn is_anagram<'a>(word: &str, possible_anagram: &str) -> bool {
    let word_lower = word.to_lowercase();
    let mut char_map = HashMap::new();
    // 将word_lower展开为字符数组并统计每个字符出现的次数
    word_lower
        .chars()
        .for_each(|c| {
            *char_map.entry(c).or_insert(0) += 1;
        });
    // 遍历check_char，对所有出现的字符实施减法操作，
    // 如果有字符的出现次数为负数，说明不是anagram
    let check_char = |s: &str| -> bool {
        let mut char_map = char_map.clone();
        s.chars().any(|c| {
            let count = char_map.entry(c).or_insert(0);
            *count -= 1;
            *count < 0
        })
    };

    if possible_anagram.len() == word.len() && {
        let anagram_lowercase = possible_anagram.to_lowercase();
        !check_char(&anagram_lowercase) && anagram_lowercase != word_lower
    }
        {
            true
        }
        else {
            false
        }
}