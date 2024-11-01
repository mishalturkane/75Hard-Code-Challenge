// stringdsaquestions.rs

fn question_1_reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn question_2_is_palindrome(s: &str) -> bool {
    let filtered: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed: String = filtered.chars().rev().collect();
    filtered.eq_ignore_ascii_case(&reversed)
}

fn question_3_find_first_unique_character(s: &str) -> Option<char> {
    let mut count = std::collections::HashMap::new();
    for ch in s.chars() {
        *count.entry(ch).or_insert(0) += 1;
    }
    s.chars().find(|&ch| count[&ch] == 1)
}

fn question_4_count_vowels(s: &str) -> usize {
    s.chars().filter(|&c| "aeiouAEIOU".contains(c)).count()
}

fn question_5_is_anagram(s1: &str, s2: &str) -> bool {
    let mut chars1: Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();
    chars1.sort_unstable();
    chars2.sort_unstable();
    chars1 == chars2
}

fn question_6_remove_duplicates(s: &str) -> String {
    let mut seen = std::collections::HashSet::new();
    s.chars().filter(|&ch| seen.insert(ch)).collect()
}

fn question_7_longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() { return String::new(); }
    let mut prefix = strings[0].to_string();
    for s in strings.iter() {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() { return String::new(); }
        }
    }
    prefix
}

fn question_8_find_all_substrings(s: &str) -> Vec<String> {
    let mut substrings = vec![];
    let len = s.len();
    for i in 0..len {
        for j in i+1..=len {
            substrings.push(s[i..j].to_string());
        }
    }
    substrings
}

fn question_9_most_frequent_character(s: &str) -> Option<char> {
    let mut count = std::collections::HashMap::new();
    for ch in s.chars() {
        *count.entry(ch).or_insert(0) += 1;
    }
    count.into_iter().max_by_key(|&(_, count)| count).map(|(ch, _)| ch)
}

fn question_10_convert_to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars.as_str().chars()).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let s = "hello world";

    println!("1. Reversed String: {}", question_1_reverse_string(s));
    println!("2. Is Palindrome: {}", question_2_is_palindrome("madam"));
    println!("3. First Unique Character: {:?}", question_3_find_first_unique_character("swiss"));
    println!("4. Vowel Count: {}", question_4_count_vowels(s));
    println!("5. Is Anagram (\"listen\", \"silent\"): {}", question_5_is_anagram("listen", "silent"));
    println!("6. String without Duplicates: {}", question_6_remove_duplicates(s));
    println!("7. Longest Common Prefix: {}", question_7_longest_common_prefix(&["flower", "flow", "flight"]));
    println!("8. All Substrings: {:?}", question_8_find_all_substrings("abc"));
    println!("9. Most Frequent Character: {:?}", question_9_most_frequent_character("hello"));
    println!("10. Title Case: {}", question_10_convert_to_title_case("hello world from rust"));
}
