use enigo::Keyboard;

#[cfg(any(target_os = "linux", target_os = "macos"))]
type KeyInstruction = (u16, bool);

#[cfg(target_os = "windows")]
type KeyInstruction = (enigo::Key, bool);

#[cfg(target_os = "linux")]
fn get_key_code(key: &str) -> KeyInstruction {
    match key {
        _ => 0,
    }
}

#[cfg(target_os = "macos")]
fn get_key_code(key: &str) -> KeyInstruction {
    match key {
        // Lowercase Alphabetic
        "a" => (0, false),
        "b" => (11, false),
        "c" => (8, false),
        "d" => (2, false),
        "e" => (14, false),
        "f" => (3, false),
        "g" => (5, false),
        "h" => (4, false),
        "i" => (34, false),
        "j" => (38, false),
        "k" => (40, false),
        "l" => (37, false),
        "m" => (46, false),
        "n" => (45, false),
        "o" => (31, false),
        "p" => (35, false),
        "q" => (12, false),
        "r" => (15, false),
        "s" => (1, false),
        "t" => (17, false),
        "u" => (32, false),
        "v" => (9, false),
        "w" => (13, false),
        "x" => (7, false),
        "y" => (16, false),
        "z" => (6, false),

        // Uppercase Alphabetic
        "A" => (0, true),
        "B" => (11, true),
        "C" => (8, true),
        "D" => (2, true),
        "E" => (14, true),
        "F" => (3, true),
        "G" => (5, true),
        "H" => (4, true),
        "I" => (34, true),
        "J" => (38, true),
        "K" => (40, true),
        "L" => (37, true),
        "M" => (46, true),
        "N" => (45, true),
        "O" => (31, true),
        "P" => (35, true),
        "Q" => (12, true),
        "R" => (15, true),
        "S" => (1, true),
        "T" => (17, true),
        "U" => (32, true),
        "V" => (9, true),
        "W" => (13, true),
        "X" => (7, true),
        "Y" => (16, true),
        "Z" => (6, true),

        // Numeric
        "0" => (29, false),
        "1" => (18, false),
        "2" => (19, false),
        "3" => (20, false),
        "4" => (21, false),
        "5" => (23, false),
        "6" => (22, false),
        "7" => (26, false),
        "8" => (28, false),
        "9" => (25, false),

        // Special Characters
        "-" => (27, false),
        "=" => (24, false),
        "!" => (18, true),
        "@" => (19, true),
        "#" => (20, true),
        "$" => (21, true),
        "%" => (23, true),
        "^" => (22, true),
        "&" => (26, true),
        "*" => (28, true),
        "(" => (25, true),
        ")" => (29, true),
        "_" => (27, true),
        "+" => (24, true),
        "[" => (33, false),
        "]" => (30, false),
        "\\" => (42, false),
        "{" => (33, true),
        "}" => (30, true),
        "|" => (42, true),
        ";" => (41, false),
        "'" => (39, false),
        ":" => (41, true),
        "\"" => (39, true),
        "," => (43, false),
        "." => (47, false),
        "/" => (44, false),
        "<" => (43, true),
        ">" => (47, true),
        "?" => (44, true),
        "`" => (50, false),
        "~" => (50, true),

        // F1 to F12 Keys
        "F1" => (122, false),
        "F2" => (120, false),
        "F3" => (99, false),
        "F4" => (118, false),
        "F5" => (96, false),
        "F6" => (97, false),
        "F7" => (98, false),
        "F8" => (100, false),
        "F9" => (101, false),
        "F10" => (109, false),
        "F11" => (103, false),
        "F12" => (111, false),

        // Command/System Codes
        "ESCAPE" => (53, false),
        "DELETE" => (51, false),
        "TAB" => (48, false),
        "RETURN" => (36, false),
        "SPACE" => (49, false),
        "CONTROL" => (59, false),
        "OPTION" => (58, false),
        "APPLE" => (55, false),
        "SHIFT" => (56, false),
        "CAPSLOCK" => (56, false),

        // Arrow Keys
        "UP" => (126, false),
        "DOWN" => (125, false),
        "LEFT" => (123, false),
        "RIGHT" => (124, false),

        // Fallback if no match
        _ => (0, false),
    }
}

#[cfg(target_os = "windows")]
fn get_key_code(key: &str) -> KeyInstruction {
    match key {
        // Lowercase Alphabetic
        "a" => (enigo::Key::A, false),
        "b" => (enigo::Key::B, false),
        "c" => (enigo::Key::C, false),
        "d" => (enigo::Key::D, false),
        "e" => (enigo::Key::E, false),
        "f" => (enigo::Key::F, false),
        "g" => (enigo::Key::G, false),
        "h" => (enigo::Key::H, false),
        "i" => (enigo::Key::I, false),
        "j" => (enigo::Key::J, false),
        "k" => (enigo::Key::K, false),
        "l" => (enigo::Key::L, false),
        "m" => (enigo::Key::M, false),
        "n" => (enigo::Key::N, false),
        "o" => (enigo::Key::O, false),
        "p" => (enigo::Key::P, false),
        "q" => (enigo::Key::Q, false),
        "r" => (enigo::Key::R, false),
        "s" => (enigo::Key::S, false),
        "t" => (enigo::Key::T, false),
        "u" => (enigo::Key::U, false),
        "v" => (enigo::Key::V, false),
        "w" => (enigo::Key::W, false),
        "x" => (enigo::Key::X, false),
        "y" => (enigo::Key::Y, false),
        "z" => (enigo::Key::Z, false),

        // Uppercase Alphabetic
        "A" => (enigo::Key::A, true),
        "B" => (enigo::Key::B, true),
        "C" => (enigo::Key::C, true),
        "D" => (enigo::Key::D, true),
        "E" => (enigo::Key::E, true),
        "F" => (enigo::Key::F, true),
        "G" => (enigo::Key::G, true),
        "H" => (enigo::Key::H, true),
        "I" => (enigo::Key::I, true),
        "J" => (enigo::Key::J, true),
        "K" => (enigo::Key::K, true),
        "L" => (enigo::Key::L, true),
        "M" => (enigo::Key::M, true),
        "N" => (enigo::Key::N, true),
        "O" => (enigo::Key::O, true),
        "P" => (enigo::Key::P, true),
        "Q" => (enigo::Key::Q, true),
        "R" => (enigo::Key::R, true),
        "S" => (enigo::Key::S, true),
        "T" => (enigo::Key::T, true),
        "U" => (enigo::Key::U, true),
        "V" => (enigo::Key::V, true),
        "W" => (enigo::Key::W, true),
        "X" => (enigo::Key::X, true),
        "Y" => (enigo::Key::Y, true),
        "Z" => (enigo::Key::Z, true),

        // Numeric
        "0" => (enigo::Key::Num0, false),
        "1" => (enigo::Key::Num1, false),
        "2" => (enigo::Key::Num2, false),
        "3" => (enigo::Key::Num3, false),
        "4" => (enigo::Key::Num4, false),
        "5" => (enigo::Key::Num5, false),
        "6" => (enigo::Key::Num6, false),
        "7" => (enigo::Key::Num7, false),
        "8" => (enigo::Key::Num8, false),
        "9" => (enigo::Key::Num9, false),

        // Special Characters
        "!" => (enigo::Key::Num1, true),
        "@" => (enigo::Key::Num2, true),
        "#" => (enigo::Key::Num3, true),
        "$" => (enigo::Key::Num4, true),
        "%" => (enigo::Key::Num5, true),
        "^" => (enigo::Key::Num6, true),
        "&" => (enigo::Key::Num7, true),
        "*" => (enigo::Key::Num8, true),
        "(" => (enigo::Key::Num9, true),
        ")" => (enigo::Key::Num0, true),
        "-" => (enigo::Key::Subtract, false),
        "_" => (enigo::Key::OEMMinus, true),
        "+" => (enigo::Key::OEMPlus, true),

        "=" => (enigo::Key::Unicode('='), false),

        "`" => (enigo::Key::Unicode('`'), false),
        "~" => (enigo::Key::Unicode('~'), true),

        "[" => (enigo::Key::Unicode('['), false),
        "]" => (enigo::Key::Unicode(']'), false),
        "\\" => (enigo::Key::Unicode('\\'), false),
        "{" => (enigo::Key::Unicode('{'), true),
        "}" => (enigo::Key::Unicode('}'), true),
        "|" => (enigo::Key::Unicode('|'), true),

        ";" => (enigo::Key::Unicode(';'), false),
        "'" => (enigo::Key::Unicode('\''), false),

        ":" => (enigo::Key::Unicode(':'), false),
        "\"" => (enigo::Key::Unicode('"'), false),

        "," => (enigo::Key::OEMComma, false),
        "." => (enigo::Key::OEMPeriod, false),
        "/" => (enigo::Key::Unicode('/'), false),

        "<" => (enigo::Key::OEMComma, true),
        ">" => (enigo::Key::OEMPeriod, true),
        "?" => (enigo::Key::Other(191), true),

        // F1 to F12 Keys
        "F1" => (enigo::Key::F1, false),
        "F2" => (enigo::Key::F2, false),
        "F3" => (enigo::Key::F3, false),
        "F4" => (enigo::Key::F4, false),
        "F5" => (enigo::Key::F5, false),
        "F6" => (enigo::Key::F6, false),
        "F7" => (enigo::Key::F7, false),
        "F8" => (enigo::Key::F8, false),
        "F9" => (enigo::Key::F9, false),
        "F10" => (enigo::Key::F10, false),
        "F11" => (enigo::Key::F11, false),
        "F12" => (enigo::Key::F12, false),

        // Sys Function Keys
        "BACKSPACE" => (enigo::Key::Backspace, false),
        "ENTER" => (enigo::Key::Return, false),
        "SPACE" => (enigo::Key::Space, false),
        "ALT" => (enigo::Key::Alt, false),
        "CONTROL" => (enigo::Key::Control, false),
        "DELETE" => (enigo::Key::Delete, false),
        "ESCAPE" => (enigo::Key::Escape, false),
        "TAB" => (enigo::Key::Tab, false),
        "WINDOWS" => (enigo::Key::Meta, false),
        "MENU" => (enigo::Key::RMenu, false),

        // Arrow Keys
        "UP" => (enigo::Key::UpArrow, false),
        "DOWN" => (enigo::Key::DownArrow, false),
        "LEFT" => (enigo::Key::LeftArrow, false),
        "RIGHT" => (enigo::Key::RightArrow, false),

        // No Match
        _ => (enigo::Key::None, false),
    }
}

pub fn send_key_action(enigo: &mut enigo::Enigo, key: &str, direction: enigo::Direction) {
    let (key, shift) = get_key_code(key);
    if shift {
        let _ = enigo.key(enigo::Key::Shift, direction);
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let _ = enigo.raw(key, direction);
        #[cfg(target_os = "windows")]
        let _ = enigo.key(key, direction);
        let _ = enigo.key(enigo::Key::Shift, direction);
    } else {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let _ = enigo.raw(key, direction);
        #[cfg(target_os = "windows")]
        let _ = enigo.key(key, direction);
    }
}
