pub fn split_once(s: &str, pat: char) -> Option<(&str, &str)> {
    let split: Vec<_> = s.splitn(2, pat).collect();
    if let [a, b] = split[..] {
        Some((a, b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_split() {
        assert_eq!(split_once("a:b", ':'), Some(("a", "b")));
    }

    #[test]
    fn incorrect_split() {
        assert_eq!(split_once("ab", ':'), None)
    }
}
