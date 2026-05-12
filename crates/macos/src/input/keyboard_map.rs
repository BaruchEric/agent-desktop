use agent_desktop_core::{
    action::Modifier,
    error::{AdapterError, ErrorCode},
};

pub(crate) fn modifier_keycode(m: &Modifier) -> u16 {
    match m {
        Modifier::Cmd => 55,
        Modifier::Shift => 56,
        Modifier::Alt => 58,
        Modifier::Ctrl => 59,
    }
}

pub(crate) fn is_shifted_char(ch: char) -> bool {
    matches!(
        ch,
        '!' | '@'
            | '#'
            | '$'
            | '%'
            | '^'
            | '&'
            | '*'
            | '('
            | ')'
            | '_'
            | '+'
            | '{'
            | '}'
            | '|'
            | ':'
            | '"'
            | '<'
            | '>'
            | '?'
            | '~'
    )
}

pub(crate) fn char_to_keycode(ch: char) -> Option<u16> {
    let lower = ch.to_ascii_lowercase();
    Some(match lower {
        'a' => 0,
        'b' => 11,
        'c' => 8,
        'd' => 2,
        'e' => 14,
        'f' => 3,
        'g' => 5,
        'h' => 4,
        'i' => 34,
        'j' => 38,
        'k' => 40,
        'l' => 37,
        'm' => 46,
        'n' => 45,
        'o' => 31,
        'p' => 35,
        'q' => 12,
        'r' => 15,
        's' => 1,
        't' => 17,
        'u' => 32,
        'v' => 9,
        'w' => 13,
        'x' => 7,
        'y' => 16,
        'z' => 6,
        '0' | ')' => 29,
        '1' | '!' => 18,
        '2' | '@' => 19,
        '3' | '#' => 20,
        '4' | '$' => 21,
        '5' | '%' => 23,
        '6' | '^' => 22,
        '7' | '&' => 26,
        '8' | '*' => 28,
        '9' | '(' => 25,
        ' ' => 49,
        '-' | '_' => 27,
        '=' | '+' => 24,
        '[' | '{' => 33,
        ']' | '}' => 30,
        '\\' | '|' => 42,
        ';' | ':' => 41,
        '\'' | '"' => 39,
        ',' | '<' => 43,
        '.' | '>' => 47,
        '/' | '?' => 44,
        '`' | '~' => 50,
        '\t' => 48,
        _ => return None,
    })
}

pub(crate) fn key_name_to_code(key: &str) -> Result<u16, AdapterError> {
    let code = match key {
        "a" => 0,
        "b" => 11,
        "c" => 8,
        "d" => 2,
        "e" => 14,
        "f" => 3,
        "g" => 5,
        "h" => 4,
        "i" => 34,
        "j" => 38,
        "k" => 40,
        "l" => 37,
        "m" => 46,
        "n" => 45,
        "o" => 31,
        "p" => 35,
        "q" => 12,
        "r" => 15,
        "s" => 1,
        "t" => 17,
        "u" => 32,
        "v" => 9,
        "w" => 13,
        "x" => 7,
        "y" => 16,
        "z" => 6,
        "0" => 29,
        "1" => 18,
        "2" => 19,
        "3" => 20,
        "4" => 21,
        "5" => 23,
        "6" => 22,
        "7" => 26,
        "8" => 28,
        "9" => 25,
        "return" | "enter" => 36,
        "escape" | "esc" => 53,
        "tab" => 48,
        "space" => 49,
        "delete" | "backspace" => 51,
        "forwarddelete" => 117,
        "home" => 115,
        "end" => 119,
        "pageup" => 116,
        "pagedown" => 121,
        "left" => 123,
        "right" => 124,
        "down" => 125,
        "up" => 126,
        "cmd" | "command" => 55,
        "shift" => 56,
        "alt" | "option" => 58,
        "ctrl" | "control" => 59,
        "f1" => 122,
        "f2" => 120,
        "f3" => 99,
        "f4" => 118,
        "f5" => 96,
        "f6" => 97,
        "f7" => 98,
        "f8" => 100,
        "f9" => 101,
        "f10" => 109,
        "f11" => 103,
        "f12" => 111,
        other => {
            return Err(AdapterError::new(
                ErrorCode::InvalidArgs,
                format!("Unknown key: '{other}'"),
            )
            .with_suggestion("Valid keys: a-z, 0-9, return, escape, tab, space, delete, left, right, up, down, f1-f12"))
        }
    };
    Ok(code)
}
