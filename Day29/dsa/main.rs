// arraydsaquestions.rs

use std::collections::{HashMap, HashSet};

// 1. Longest Palindromic Substring
fn longest_palindromic_substring(s: &str) -> String {
    let bytes = s.as_bytes();
    let len = s.len();
    let mut start = 0;
    let mut max_length = 1;

    for i in 0..len {
        for j in i..len {
            let mut is_palindrome = true;
            for k in 0..(j - i + 1) / 2 {
                if bytes[i + k] != bytes[j - k] {
                    is_palindrome = false;
                    break;
                }
            }
            if is_palindrome && j - i + 1 > max_length {
                start = i;
                max_length = j - i + 1;
            }
        }
    }

    s[start..start + max_length].to_string()
}

// 2. Minimum Window Substring
fn minimum_window_substring(s: &str, t: &str) -> String {
    let mut need = HashMap::new();
    let mut window = HashMap::new();
    for c in t.chars() {
        *need.entry(c).or_insert(0) += 1;
    }
    let (mut left, mut right, mut valid) = (0, 0, 0);
    let (mut start, mut min_len) = (0, usize::MAX);
    let s_chars: Vec<char> = s.chars().collect();

    while right < s_chars.len() {
        let c = s_chars[right];
        right += 1;
        if let Some(count) = need.get(&c) {
            *window.entry(c).or_insert(0) += 1;
            if window[&c] == *count {
                valid += 1;
            }
        }
        while valid == need.len() {
            if right - left < min_len {
                start = left;
                min_len = right - left;
            }
            let d = s_chars[left];
            left += 1;
            if let Some(count) = need.get(&d) {
                if window[&d] == *count {
                    valid -= 1;
                }
                *window.entry(d).or_insert(0) -= 1;
            }
        }
    }
    if min_len == usize::MAX {
        "".to_string()
    } else {
        s_chars[start..start + min_len].iter().collect()
    }
}

// 3. Longest Substring Without Repeating Characters
fn longest_unique_substring(s: &str) -> usize {
    let mut char_set = HashSet::new();
    let (mut left, mut right, mut max_len) = (0, 0, 0);
    let s_chars: Vec<char> = s.chars().collect();

    while right < s_chars.len() {
        if !char_set.contains(&s_chars[right]) {
            char_set.insert(s_chars[right]);
            right += 1;
            max_len = max_len.max(right - left);
        } else {
            char_set.remove(&s_chars[left]);
            left += 1;
        }
    }
    max_len
}

// 4. Valid Anagram
fn is_valid_anagram(s: &str, t: &str) -> bool {
    let mut char_count = HashMap::new();
    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    for c in t.chars() {
        *char_count.entry(c).or_insert(0) -= 1;
        if char_count[&c] < 0 {
            return false;
        }
    }
    char_count.values().all(|&v| v == 0)
}

// 5. Word Break
fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;

    for i in 1..=s.len() {
        for word in &word_dict {
            if i >= word.len() && s[i - word.len()..i] == *word && dp[i - word.len()] {
                dp[i] = true;
                break;
            }
        }
    }
    dp[s.len()]
}

// 6. Edit Distance
fn edit_distance(word1: &str, word2: &str) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                dp[i][j] = j as i32;
            } else if j == 0 {
                dp[i][j] = i as i32;
            } else if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }
    dp[m][n]
}

// 7. Decode Ways
fn decode_ways(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let n = s.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = if &s[0..1] != "0" { 1 } else { 0 };

    for i in 2..=n {
        if &s[i - 1..i] != "0" {
            dp[i] += dp[i - 1];
        }
        if &s[i - 2..i] >= "10" && &s[i - 2..i] <= "26" {
            dp[i] += dp[i - 2];
        }
    }
    dp[n]
}

// 8. Longest Common Subsequence
fn longest_common_subsequence(text1: &str, text2: &str) -> i32 {
    let m = text1.len();
    let n = text2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if text1.as_bytes()[i - 1] == text2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}

// 9. Group Anagrams
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        anagrams.entry(key).or_default().push(s);
    }
    anagrams.into_values().collect()
}

// 10. Regular Expression Matching
fn is_match(s: &str, p: &str) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();
    let (m, n) = (s_chars.len(), p_chars.len());
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    for j in 1..=n {
        if p_chars[j - 1] == '*' {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            if p_chars[j - 1] == '.' || s_chars[i - 1] == p_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else if p_chars[j - 1] == '*' {
                dp[i][j] = dp[i][j - 2] || (dp[i - 1][j] && (p_chars[j - 2] == '.' || p_chars[j - 2] == s_chars[i - 1]));
            }
        }
    }
    dp[m][n]
}

fn main() {
    let s = "babad".to_string();
    println!("Longest Palindromic Substring: {}", longest_palindromic_substring(&s));
}
