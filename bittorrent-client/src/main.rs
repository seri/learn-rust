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

    fn parse_string_raw(bytes: &[u8]) -> Result<(Vec<u8>, &[u8]), String> {
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
        Ok((Vec::from(&bytes[start + 1..end]), &bytes[end..]))
    }

    fn parse_string(bytes: &[u8]) -> Result<(Self, &[u8]), String> {
        let (raw_string, remainder) = Self::parse_string_raw(bytes)?;
        Ok((Bencode::String(raw_string), remainder))
    }

    fn parse_list(bytes: &[u8]) -> Result<(Self, &[u8]), String> {
        let mut list = Vec::<Bencode>::new();
        let mut remainder = &bytes[1..];

        while remainder.len() > 0 {
            if remainder[0] == b'e' {
                break;
            }
            let (elem, _remainder) = Self::parse(remainder)?;
            remainder = _remainder;
            list.push(elem);
        }

        Ok((Bencode::List(list), &remainder[1..]))
    }

    fn parse_dict(bytes: &[u8]) -> Result<(Self, &[u8]), String> {
        let mut dict = BTreeMap::<Vec<u8>, Bencode>::new();
        let mut remainder = &bytes[1..];

        while remainder.len() > 0 {
            if remainder[0] == b'e' {
                break;
            }
            let (key, _remainder) = Self::parse_string_raw(remainder)?;
            let (value, _remainder) = Self::parse(_remainder)?;
            remainder = _remainder;
            dict.insert(key, value);
        }

        Ok((Bencode::Dict(dict), &remainder[1..]))
    }

    fn parse(bytes: &[u8]) -> Result<(Self, &[u8]), String> {
        match bytes[0] {
            b'i' => Self::parse_int(bytes),
            b'0'..b'9' => Self::parse_string(bytes),
            b'l' => Self::parse_list(bytes),
            b'd' => Self::parse_dict(bytes),
            _ => Err(format!("Invalid first byte: {}", bytes[0])),
        }
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
        assert_eq!(
            res,
            Bencode::List(vec!(
                Bencode::String(Vec::from(b"spam")),
                Bencode::Integer(42)
            ))
        );
        assert_eq!(remainder, b"LOL");
    }

    #[test]
    fn test_dict_basic() {
        let (res, remainder) = Bencode::parse_dict(b"d3:bar4:spam3:fooi42eeLOL").unwrap();
        let expected = BTreeMap::from([
            (Vec::from(b"bar"), Bencode::String(Vec::from(b"spam"))),
            (Vec::from(b"foo"), Bencode::Integer(42)),
        ]);
        assert_eq!(res, Bencode::Dict(expected));
        assert_eq!(remainder, b"LOL");
    }
}

fn main() {}
