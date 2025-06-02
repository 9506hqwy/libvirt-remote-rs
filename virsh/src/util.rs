use unicode_width::UnicodeWidthStr;

pub fn char_width(value: &str) -> usize {
    UnicodeWidthStr::width_cjk(value)
}

pub fn padding(ch: char, len: u16) -> String {
    std::iter::repeat_n(ch, len as usize).collect::<String>()
}
