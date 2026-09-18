use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Bencode {
    Integer(i64),
    String(Vec<u8>),
    List(Vec<Bencode>),
    Dict(BTreeMap<Vec<u8>, Bencode>),
}

impl Bencode {
    fn parse_int(bytes: &[u8]) -> Result<(Self, &[u8]), String> {
        let pos = bytes
            .iter()
            .position(|&b| b == b'e')
            .ok_or("Missing trailing 'e' for integer")?;
        let value = std::str::from_utf8(&bytes[1..pos])
            .map_err(|_| "Integer string is invalid UTF-8")
            .and_then(|s| {
                s.parse::<i64>()
                    .map_err(|_| "String between i and e is invalid i64")
            })?;
        Ok((Bencode::Integer(value), &bytes[pos + 1..]))
    }

    fn parse_string(bytes: &[u8]) -> Result<(Self, &[u8]), String> {
        let start = bytes
            .iter()
            .position(|&b| b == b':')
            .ok_or("Missing a colon for string")?;
        let len = std::str::from_utf8(&bytes[0..start])
            .map_err(|_| "The length of string is invalid UTF-8")
            .and_then(|s| {
                s.parse::<usize>()
                    .map_err(|_| "The length of string is invalid usize")
            })?;
        let end = start + 1 + len;
        if bytes.len() < end {
            return Err(format!(
                "Expected {len} more bytes but the source has only {} bytes total",
                bytes.len()
            ));
        }
        Ok((
            Bencode::String(Vec::from(&bytes[start + 1..end])),
            &bytes[end..],
        ))
    }

    fn parse_list(bytes: &[u8]) -> Result<(Self, &[u8]), String> {
        let mut value = Vec::<Bencode>::new();
        let mut remainder = &bytes[1..];
        while remainder.len() > 0 {
            println!("remainder: {}", String::from_utf8_lossy(remainder));
            if remainder[0] == b'e' {
                break
            }
            let elem_wrapper = match remainder[0] {
                b'i' => Self::parse_int(remainder),
                b'0'..b'9' => Self::parse_string(remainder),
                b'l' => Self::parse_list(remainder),
                _ => Err(format!("Invalid first byte: {}", remainder[0]))
            };
            let (elem, _remainder) = elem_wrapper?;
            remainder = _remainder;
            value.push(elem);
        }
        Ok((Bencode::List(value), &remainder[1..]))
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

    #[test]
    fn test_integer_missing_end() {
        let res = Bencode::parse_int(b"i1234567");
        assert_eq!(res, Err(String::from("Missing trailing 'e' for integer")));
    }

    #[test]
    fn test_string_basic() {
        let (res, remainder) = Bencode::parse_string(b"5:hellofoobar").unwrap();
        assert_eq!(res, Bencode::String(Vec::from(b"hello")));
        assert_eq!(remainder, b"foobar");
    }

    #[test]
    fn test_string_invalid_len() {
        let res = Bencode::parse_string(b"100:hellofoobar");
        assert_eq!(
            res,
            Err(String::from(
                "Expected 100 more bytes but the source has only 15 bytes total"
            ))
        );
    }

    #[test]
    fn test_list_basic() {
        let (res, remainder) = Bencode::parse_list(b"l4:spami42eeLOL").unwrap();
        assert_eq!(res, Bencode::List(vec!(Bencode::String(Vec::from(b"spam")), Bencode::Integer(42))));
        assert_eq!(remainder, b"LOL");
    }
}

fn main() {}
