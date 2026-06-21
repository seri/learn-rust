use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Bencode {
    Integer(i64),
    String(Vec<u8>),
    List(Vec<Bencode>),
    Dict(BTreeMap<Vec<u8>, Bencode>),
}

impl Bencode {
    fn parse_int(bytes: &[u8]) -> Result<(Self, &[u8]), &'static str> {
        let pos = bytes.iter().position(|&b| b == b'e').ok_or("Missing trailing 'e' for integer")?;
        let value = std::str::from_utf8(&bytes[1..pos])
            .map_err(|_| "Integer string is invalid UTF-8")
            .and_then(|s| s.parse::<i64>().map_err(|_| "String between i and e is invalid i64"))?;
        Ok((Bencode::Integer(value), &bytes[pos+1..]))
    }

    fn parse_string(bytes: &[u8]) -> Result<(Self, &[u8]), &'static str> {
        let pos = bytes.iter().position(|&b| b == b':').ok_or("Missing a colon for string")?;
        let value = std::str::from_utf8(&bytes[1..pos])
            .map_err(|_| "Integer string is invalid UTF-8")
            .and_then(|s| s.parse::<i64>().map_err(|_| "String between i and e is invalid i64"))?;
        Ok((Bencode::Integer(value), &bytes[pos+1..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_basic() {
        let (res, remainder) = Bencode::parse_int(b"i123e567").unwrap();
        assert_eq!(res, Bencode::Integer(123));
        assert_eq!(remainder, b"567");
    }
}

fn main() {
}
