pub fn test() {
    let pw = String::from("tessssttttttaman");
    let new_pw = remove_more_than_two_repeating_chars(&pw);
    println!("{}", new_pw);
}


fn remove_more_than_two_repeating_chars(password: &str) -> String {
    let mut prev: Option<char> = None;
    let mut prev_prev: Option<char> = None;
    let mut password_chars: String = String::new();
    for c in password.chars() {
        if Some(c) == prev && Some(c) == prev_prev {
            continue;
        }
        prev_prev = prev;
        prev = Some(c);
        password_chars.push(c);
    // do something with `c`
    }
    return password_chars;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeate_chars_removed() {
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
}
