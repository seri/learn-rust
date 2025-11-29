pub struct SplitIterator<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> SplitIterator<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s, pos: 0 }
    }
}

impl<'a> Iterator for SplitIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.s.as_bytes();

        let mut start = bytes.len();
        for i in self.pos..bytes.len() {
            if !bytes[i].is_ascii_whitespace() {
                start = i;
                break;
            }
        }

        let mut end = self.s.len();
        for i in (start + 1)..bytes.len() {
            if bytes[i].is_ascii_whitespace() {
                end = i;
                break;
            }
        }

        if end <= start {
            return None;
        }

        let res = Some(&self.s[start..end]);
        self.pos = end + 1;
        return res;
    }
}

pub fn split(s: &str) -> impl Iterator<Item = &str> {
    SplitIterator::new(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_split() {
        let s = "hello world rust";
        let mut iter = split(s);

        assert_eq!(iter.next(), Some("hello"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), Some("rust"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_multiple_whitespaces() {
        let s = "hello  world    rust";
        let mut iter = split(s);

        assert_eq!(iter.next(), Some("hello"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), Some("rust"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_ending_white_spaces() {
        let s = "hello world rust ";
        let mut iter = split(s);

        assert_eq!(iter.next(), Some("hello"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), Some("rust"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_string() {
        let s = "";
        let mut iter = split(s);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_only_whitespaces() {
        let s = "   ";
        let mut iter = split(s);

        assert_eq!(iter.next(), None);
    }
}
