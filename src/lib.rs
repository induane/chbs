use std::fmt;

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;


/// Represents attributes that are tracked about a password string
pub struct PasswordInfo {
    length: i16,
    original_password: String,
    reduced_password: String,
    has_spaces: bool,
    has_upper: bool,
    has_lower: bool,
    has_digits: bool,
    has_special: bool,
}

impl fmt::Display for PasswordInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Length: {}, Original Password: {}, Reduced Password: {}, Has Spaces: {}, Has Uppercase: {}, Has Lowercase: {}, Has Digits: {}, Has Special Characters: {})",
            self.length, self.original_password, self.reduced_password, self.has_spaces, self.has_upper, self.has_lower, self.has_digits, self.has_special
        )
    }
}


pub fn get_entropy(password: &str) -> i16 {
    let pw_info = get_password_info(password);
    let mut score: i16 = pw_info.length;
    if pw_info.has_spaces {
        score += 1;
    }
    if pw_info.has_digits {
        score += 1;
    }
    if pw_info.has_special {
        score += 1;
    }
    if pw_info.has_upper && pw_info.has_lower {
        score += 1;
    }
    let score_f64 = score as f64;

    let entropy: f64 = log_power(score_f64, pw_info.length, 2.0_f64);
    return entropy as i16
}


pub fn get_password_info(password: &str) -> PasswordInfo {

    let mut reduced_pw: String =  remove_more_than_two_repeating_chars(&password);
    reduced_pw = remove_common_sequences(&reduced_pw);
    reduced_pw = clear_palindrome(&reduced_pw);

    let pw_info = PasswordInfo {
        length: reduced_pw.len() as i16,
        original_password: password.to_owned(),
        reduced_password: reduced_pw,
        // Has any spaces
        has_spaces: password.contains(" "),
        // Has any uppercase letters
        has_upper: password.chars().any(|x| matches!(x, 'A'..='Z')),
        // Has any lowercase letters
        has_lower: password.chars().any(|x| matches!(x, 'a'..='z')),
        has_digits: has_numerical_digits(&password),
        has_special: has_special_characters(&password),
    };
    return pw_info
}


/// Determines if any special characters are in the password string
fn has_special_characters(password: &str) -> bool {
    let stripped: String = remove_standard_characters(&password);
    let has_special = stripped.len() != 0;
    return has_special
}

/// Remove normal ascii characters from a given password
fn remove_standard_characters(password: &str) -> String {
    let boring_chars: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    let mut password_chars: String = String::new();
    for grapheme_word in password.to_owned().graphemes(true) {
        if !boring_chars.contains(grapheme_word) {
            for c in grapheme_word.chars() {
                password_chars.push(c);
            }
        }
    }
    return password_chars
}

/// Determine if a string has any numerical digits
fn has_numerical_digits(password: &str) -> bool {
    let digits: String = String::from("0123456789");
    let mut password_chars: String = String::new();
    for grapheme_word in password.to_owned().graphemes(true) {
        if !digits.contains(grapheme_word) {
            for c in grapheme_word.chars() {
                password_chars.push(c);
            }
        }
    }
    let has_digits: bool = password_chars.len() != password.len();
    return has_digits
}


fn remove_more_than_two_repeating_chars(password: &str) -> String {
    let mut prev: Option<&str> = None;
    let mut prev_prev: Option<&str> = None;
    let mut password_chars: String = String::new();
    for grapheme_word in password.to_owned().graphemes(true) {
        if Some(grapheme_word) == prev && Some(grapheme_word) == prev_prev {
            continue;
        }
        prev_prev = prev;
        prev = Some(grapheme_word);
        for c in grapheme_word.chars() {
            password_chars.push(c);
        }
    }
    return password_chars;
}

/// Takes a password string and reduces palindrome duplication
///
/// Some passwords are palindrome sequences - i.e. abcdcba which is the same
/// forwards and backwards. This fuction detects palindrome sequences and splits the
/// value in half and returns only this portion.
fn clear_palindrome(password: &str) -> String {

    let original: String = password.to_owned();
    let mut lowercase_password: String = password.to_owned();
    lowercase_password.make_ascii_lowercase();

    // Reverse the string
    let rev: String = lowercase_password.graphemes(true)
        .rev()
        .collect();

    if rev != lowercase_password {
        return original
    }

    let mut grapheme_words: Vec<&str> = original.graphemes(true).collect();
    let half: Vec<&str> = grapheme_words.drain(..grapheme_words.len()/2).collect();
    let mut updated_password: String = String::new();
    for word in half.into_iter() {
        for c in word.chars() {
            updated_password.push(c);
        }
    }
    return updated_password
}


/// Takes a password string and removes common sequences from it
///
/// Some sequences are ordinary keyboard patterns, others are sequences
/// that come from analysis of password leaks to determine the most common
/// passwords. We remove those sequences, even if the sequence is not the
/// entire passwords.
///
/// # Arguments
///
/// * `password` - A string slice holding a password value to modify
fn remove_common_sequences(password: &str) -> String {
    let mut stripped_password: String = password.to_owned();

    // Replace sequences one at a time. Order matters.
    stripped_password = stripped_password.replace("asdf", "");
    stripped_password = stripped_password.replace("jkl;", "");
    stripped_password = stripped_password.replace(";lkj", "");
    stripped_password = stripped_password.replace("fdsa", "");
    stripped_password = stripped_password.replace("asdfghjkl", "");
    stripped_password = stripped_password.replace("asdf ;lkj", "");
    stripped_password = stripped_password.replace("0123456789", "");
    stripped_password = stripped_password.replace("qwertyuiop", "");
    stripped_password = stripped_password.replace("qwerty", "");
    stripped_password = stripped_password.replace("zxcvbnm", "");
    stripped_password = stripped_password.replace("abcdefghijklmnopqrstuvwxyz", "");
    stripped_password = stripped_password.replace("password1", "");
    stripped_password = stripped_password.replace("password!", "");
    stripped_password = stripped_password.replace("password", "");
    stripped_password = stripped_password.replace("Password", "");
    stripped_password = stripped_password.replace("assword", "");
    stripped_password = stripped_password.replace("picture1", "");
    stripped_password = stripped_password.replace("Picture1", "");
    stripped_password = stripped_password.replace("picture", "");
    stripped_password = stripped_password.replace("Picture", "");
    stripped_password = stripped_password.replace("asdf", "");
    stripped_password = stripped_password.replace("rty567", "");
    stripped_password = stripped_password.replace("senha", "");
    stripped_password = stripped_password.replace("abc123", "");
    stripped_password = stripped_password.replace("Million2", "");
    stripped_password = stripped_password.replace("000000", "");
    stripped_password = stripped_password.replace("1234", "");
    stripped_password = stripped_password.replace("iloveyou", "");
    stripped_password = stripped_password.replace("aaron431", "");
    stripped_password = stripped_password.replace("qqww1122", "");
    stripped_password = stripped_password.replace("123123", "");
    return stripped_password
}


fn log_x(base: f64, n: f64) -> f64 {
    if base == 0.0_f64 {
        return 0.0_f64
    }
    return base.log2() / n.log2()
}


// log_power calculates log base(x^y)
fn log_power(exp_base: f64, power: i16, log_base: f64) -> f64 {
    let mut total: f64 = 0.0;
    for _x in 1..power {
        total += log_x(log_base, exp_base);
    }
    return total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_chars_removed() {
        let test_1: String = String::from("tessssttttttaman");
        let res_1 = remove_more_than_two_repeating_chars(&test_1);
        assert!(res_1 == String::from("tessttaman"));

        let test_2: String = String::from("aaabbbccc");
        let res_2 = remove_more_than_two_repeating_chars(&test_2);
        assert!(res_2 == String::from("aabbcc"));

        let test_3: String = String::from("aaaaaaaaaaaaabc");
        let res_3 = remove_more_than_two_repeating_chars(&test_3);
        assert!(res_3 == String::from("aabc"));
    }

    #[test]
    fn sequences_stripped() {
        let test_1: String = String::from("tessssttttttaman");
        let res_1 = remove_common_sequences(&test_1);
        assert!(res_1 == String::from("tessssttttttaman"));

        let test_2: String = String::from("digitmanpassword1ok");
        let res_2 = remove_common_sequences(&test_2);
        assert!(res_2 == String::from("digitmanok"));

        let test_3: String = String::from("aaaaaapassword!aaaaaaabc");
        let res_3 = remove_common_sequences(&test_3);
        assert!(res_3 == String::from("aaaaaaaaaaaaabc"));
    }

    #[test]
    fn palendromes_reduced() {
        let test_1: String = String::from("abcdefedcba");
        let res_1 = clear_palindrome(&test_1);
        assert!(res_1 == String::from("abcde"));

        let test_2: String = String::from("abcdeffedcba");
        let res_2 = clear_palindrome(&test_2);
        assert!(res_2 == String::from("abcdef"));

        let test_3: String = String::from("AbCdeffedcba");
        let res_3 = clear_palindrome(&test_3);
        assert!(res_3 == String::from("AbCdef"));

        let test_4: String = String::from("a̐éö̲ö̲éa̐");
        let res_4 = clear_palindrome(&test_4);
        assert!(res_4 == String::from("a̐éö̲"));
    }

    #[test]
    fn can_remove_normal_chars() {
        let test_1: String = String::from("!@abcdefedcba@!");
        let res_1 = remove_standard_characters(&test_1);
        assert!(res_1 == String::from("!@@!"));

        let test_2: String = String::from("{}thispassworkdIS(*)@#");
        let res_2 = remove_standard_characters(&test_2);
        assert!(res_2 == String::from("{}(*)@#"));
    }

    #[test]
    fn can_test_for_numerical_digits() {
        let test_1: String = String::from("abcdefg2hisfsd");
        let res_1 = has_numerical_digits(&test_1);
        assert!(res_1);

        let test_2: String = String::from("{}thispassworkdIS(*)@#");
        let res_2 = has_numerical_digits(&test_2);
        assert_eq!(res_2, false);
    }

    #[test]
    fn can_test_for_special_characters() {
        let test_1: String = String::from("!!abcdefg2hisfsd");
        let res_1 = has_special_characters(&test_1);
        assert!(res_1);

        let test_2: String = String::from("boringpass");
        let res_2 = has_special_characters(&test_2);
        assert_eq!(res_2, false);
    }

    #[test]
    fn calculate_log_x() {
        let result_1: f64 = log_x(2.0_f64, 5.0_f64);
        assert_eq!(result_1, 0.43067655807339306_f64);

        let result_2: f64 = log_x(0.0_f64, 5.0_f64);
        assert_eq!(result_2, 0.0_f64);
    }

    #[test]
    fn calculate_log_power() {
        let result_1: f64 = log_power(2.0_f64, 3, 5.0_f64);
        assert_eq!(result_1, 4.643856189774724_f64);
    }

    #[test]
    fn calculate_entropy_1() {
        let test_1: String = String::from("boring");
        let result_1 = get_entropy(&test_1);
        assert_eq!(result_1, 1);
    }

    #[test]
    fn calculate_entropy_2() {
        let test_1: String = String::from("boringpass");
        let result_1 = get_entropy(&test_1);
        assert_eq!(result_1, 2);
    }

    #[test]
    fn calculate_entropy_2a() {
        let test_1: String = String::from("boringpassqwerty");
        let result_1 = get_entropy(&test_1);
        assert_eq!(result_1, 2);
    }

    #[test]
    fn calculate_entropy_3() {
        let test_1: String = String::from("Boringpass!");
        let result_1 = get_entropy(&test_1);
        assert_eq!(result_1, 2);
    }

    #[test]
    fn calculate_entropy_4() {
        let test_1: String = String::from("LongerPass!thathassomenumbers1$$");
        let result_1 = get_entropy(&test_1);
        assert_eq!(result_1, 6);
    }
}
