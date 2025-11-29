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
        if self.pos >= self.s.len() {
            return None;
        }

        let bytes = self.s.as_bytes();

        let mut start = self.pos;
        for i in start..bytes.len() {
            if !bytes[i].is_ascii_whitespace() {
                start = i;
                break;
            }
        }

        let mut end = self.s.len();
        for i in start..bytes.len() {
            if bytes[i].is_ascii_whitespace() {
                end = i;
                break;
            }
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
    }
}
