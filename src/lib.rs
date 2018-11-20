/// Worst leftpad implementation ever.
pub fn leftpad_slow(s: &str, to_len: usize) -> String {
    let mut s = s.to_string();
    while s.chars().count() < to_len {
        s.insert(0, ' ');
    }
    s
}
pub fn leftpad(s: &str, to_len: usize) -> String {
    format!("{:>1$}", s, to_len)
}

#[cfg(test)]
mod tests {
    use super::leftpad;

    #[test]
    fn expanded() {
        assert_eq!(leftpad("foo", 5), "   foo");
    }

    #[test]
    fn unchanged() {
        assert_eq!(leftpad("foo", 1), "foo");
    }

    #[test]
    #[ignore]
    fn large() {
        let padded = leftpad("foo", 50_000);
        assert_eq!(padded.chars().count(), 50_000);
    }
}
