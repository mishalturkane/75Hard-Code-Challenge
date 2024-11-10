// 1. Check if a String is a Palindrome
fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    s == s.chars().rev().collect::<String>()
}

// 2. Find the Longest Common Prefix
fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() { return "".to_string(); }
    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() { return "".to_string(); }
        }
    }
    prefix
}

// 3. Check if Two Strings are Anagrams
fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut s1_chars: Vec<char> = s1.chars().collect();
    let mut s2_chars: Vec<char> = s2.chars().collect();
    s1_chars.sort_unstable();
    s2_chars.sort_unstable();
    s1_chars == s2_chars
}

// 4. Find All Substrings of Length K with No Repeating Characters
fn substrings_of_length_k(s: &str, k: usize) -> Vec<String> {
    let mut substrings = Vec::new();
    for i in 0..=s.len() - k {
        let substring: String = s[i..i + k].to_string();
        let unique_chars: std::collections::HashSet<_> = substring.chars().collect();
        if unique_chars.len() == k {
            substrings.push(substring);
        }
    }
    substrings
}

// 5. Count the Number of Words in a String
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

// 6. Reverse Words in a String
fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

// 7. Find the First Unique Character in a String
fn first_unique_char(s: &str) -> Option<char> {
    let mut char_count = std::collections::HashMap::new();
    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    s.chars().find(|&c| char_count[&c] == 1)
}

// 8. Implement String Compression (e.g., "aabcccccaaa" becomes "a2b1c5a3")
fn compress_string(s: &str) -> String {
    let mut compressed = String::new();
    let mut count = 1;
    let chars: Vec<char> = s.chars().collect();
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            compressed.push(chars[i - 1]);
            compressed.push_str(&count.to_string());
            count = 1;
        }
    }
    compressed.push(chars[chars.len() - 1]);
    compressed.push_str(&count.to_string());
    if compressed.len() < s.len() { compressed } else { s.to_string() }
}

// 9. Check if One String is a Rotation of Another
fn is_rotation(s1: &str, s2: &str) -> bool {
    s1.len() == s2.len() && (s1.repeat(2)).contains(s2)
}

// 10. Find the Longest Palindromic Substring
fn longest_palindromic_substring(s: &str) -> String {
    let mut start = 0;
    let mut end = 0;
    let s_chars: Vec<char> = s.chars().collect();

    for i in 0..s.len() {
        let len1 = expand_around_center(&s_chars, i, i);
        let len2 = expand_around_center(&s_chars, i, i + 1);
        let len = len1.max(len2);
        if len > (end - start) {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }
    s_chars[start..=end].iter().collect()
}

fn expand_around_center(s: &[char], mut left: usize, mut right: usize) -> usize {
    while right < s.len() && left > 0 && s[left] == s[right] {
        left = left.saturating_sub(1);
        right += 1;
    }
    right - left - 1
}

fn main() {
    // Testing each function with sample input

    // 1. Check if a String is a Palindrome
    println!("1. Is Palindrome: {}", is_palindrome("A man, a plan, a canal: Panama"));

    // 2. Find the Longest Common Prefix
    println!("2. Longest Common Prefix: {}", longest_common_prefix(&["flower".to_string(), "flow".to_string(), "flight".to_string()]));

    // 3. Check if Two Strings are Anagrams
    println!("3. Are Anagrams: {}", are_anagrams("listen", "silent"));

    // 4. Find All Substrings of Length K with No Repeating Characters
    println!("4. Substrings of Length K: {:?}", substrings_of_length_k("abcabc", 3));

    // 5. Count the Number of Words in a String
    println!("5. Word Count: {}", count_words("Hello World! This is Rust."));

    // 6. Reverse Words in a String
    println!("6. Reversed Words: {}", reverse_words("Hello World! This is Rust."));

    // 7. Find the First Unique Character in a String
    println!("7. First Unique Character: {:?}", first_unique_char("swiss"));

    // 8. String Compression
    println!("8. Compressed String: {}", compress_string("aabcccccaaa"));

    // 9. Check if One String is a Rotation of Another
    println!("9. Is Rotation: {}", is_rotation("waterbottle", "erbottlewat"));

    // 10. Find the Longest Palindromic Substring
    println!("10. Longest Palindromic Substring: {}", longest_palindromic_substring("babad"));
}
