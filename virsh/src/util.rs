use std::iter::repeat;
use unicode_width::UnicodeWidthStr;

pub fn char_width(value: &str) -> usize {
    UnicodeWidthStr::width_cjk(value)
}

pub fn padding(ch: char, len: u16) -> String {
    repeat(ch).take(len as usize).collect::<String>()
}
