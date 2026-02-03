#[cfg(windows)]
pub fn is_windows_invalid_component_name(name: &str) -> bool {
    if name.ends_with(' ') || name.ends_with('.') {
        return true;
    }

    if name
        .chars()
        .any(|c| c < '\u{20}' || WINDOWS_INVALID_CHARS.contains(&c))
    {
        return true;
    }

    is_windows_reserved_name(name)
}

#[cfg(not(windows))]
pub fn is_windows_invalid_component_name(_name: &str) -> bool {
    false
}

#[cfg(windows)]
pub fn is_windows_reserved_name(name: &str) -> bool {
    let base = name.split('.').next().unwrap_or("");
    let upper = base.to_ascii_uppercase();
    matches!(
        upper.as_str(),
        "CON" | "PRN" | "AUX" | "NUL"
    ) || is_windows_reserved_device(&upper)
}

#[cfg(not(windows))]
pub fn is_windows_reserved_name(_name: &str) -> bool {
    false
}

#[cfg(windows)]
fn is_windows_reserved_device(upper: &str) -> bool {
    if upper.len() == 4 {
        let (prefix, digit) = upper.split_at(3);
        if matches!(prefix, "COM" | "LPT") {
            return matches!(digit, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9");
        }
    }
    false
}

#[cfg(windows)]
const WINDOWS_INVALID_CHARS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
