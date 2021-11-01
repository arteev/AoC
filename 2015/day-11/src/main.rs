fn is_valid_password(s: &str) -> bool {
    if s.len() != 8 {
        return false;
    }

    let mut valid: bool = false;
    let chars = s.to_string().chars().collect::<Vec<char>>();
    if chars.iter().any(|&x| x == 'i' || x == 'o' || x == 'l') {
        return false;
    }
    for i in 0..5 {
        if chars[i] as u8 + 1 == chars[i + 1] as u8 &&
            chars[i] as u8 + 2 == chars[i + 2] as u8 {
            valid = true;
            break;
        }
    }
    if !valid {
        return false;
    }

    let mut pairs = 0;
    let mut i = 0;
    loop {
        if i >= chars.len() - 1 {
            break;
        }
        if chars[i] == chars[i + 1] {
            pairs = pairs + 1;
            i += 1;
        }
        i += 1;
    }
    pairs > 1
}

fn next_password(s: &str, pos: usize) -> (String, bool) {
    let mut result = s.to_string();
    let mut ch = result.as_bytes()[pos - 1] + 1;
    let mut switch_pos = false;
    if ch > 'z' as u8 {
        ch = 'a' as u8;
        switch_pos = true;
    }
    let ch_new = (ch as char).to_string();
    result.replace_range(pos - 1..pos, &ch_new);


    if pos > 1 && switch_pos {
        let (result_add, switch_pos_add) = next_password(&result, pos - 1);
        result = result_add.clone();
        switch_pos = switch_pos_add;
    }

    (result, switch_pos)
}

fn main() {
    let next = gen_and_print_next_password("cqjxjnds");
    gen_and_print_next_password(&next);
}

fn gen_and_print_next_password(old: &str) -> String {
    let mut password = old.to_string();
    let pos = 8;
    loop {
        let (new_password, _) = next_password(&password, pos);
        if is_valid_password(&new_password) {
            println!("new password: {}", new_password);
            return new_password;
        }
        password = new_password;
    }
}

#[cfg(test)]
mod tests {
    use crate::is_valid_password;
    use crate::next_password;

    #[test]
    fn test_is_valid_password() {
        assert_eq!(is_valid_password("abcde"), false);
        assert_eq!(is_valid_password("hijklmmn"), false);
        assert_eq!(is_valid_password("abbceffg"), false);
        assert_eq!(is_valid_password("abbcegjk"), false);
        assert_eq!(is_valid_password("abcdffaa"), true);
        assert_eq!(is_valid_password("ghjaabcc"), true);
        assert_eq!(is_valid_password("ghjaabccc"), false);
    }

    #[test]
    fn test_next_password() {
        let (p, _) = next_password("ghjaabcc", 8);
        assert_eq!(p, "ghjaabcd");

        let (p, _) = next_password("ahjaabcc", 1);
        assert_eq!(p, "bhjaabcc");
    }
}
