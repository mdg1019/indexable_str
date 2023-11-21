use std::{
    fmt::Display,
    ops::{Index, Range, RangeFrom, RangeTo},
};

#[derive(Copy, Clone)]
struct CharOffset {
    chr: char,
    offset: usize,
}

pub struct IndexableStr<'a> {
    str: &'a str,
    str_length: usize,
    chars_vec: Vec<CharOffset>,
    chars_length: usize,
}

impl<'a> IndexableStr<'a> {
    pub fn new(str: &'a str) -> IndexableStr {
        let mut current_offset: usize = 0;

        let chars_vec: Vec<CharOffset> = str.chars().map(|c| {
            let char_offset = CharOffset {
                chr: c,
                offset: current_offset,
            };

            let code_point: u32 = c as u32;

            current_offset += (|| {
                if code_point <= 0x7F {
                    return 1;
                }

                if code_point <= 0x7FF {
                    return 2;
                }

                if code_point <= 0xFFFF {
                    return 3;
                }

                if code_point <= 0x10FFFF {
                    return 4;
                }

                0
            })();

            char_offset
        }).collect();

        let chars_length: usize = chars_vec.len();

        IndexableStr {
            str,
            str_length: str.len(),
            chars_vec,
            chars_length,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.str
    }

    pub fn len(&self) -> usize {
        self.chars_vec.len()
    }

    fn create_str_from_range(&self, start_index: usize, end_index: usize) -> &str {
        if end_index > self.chars_length {
            panic!("Range end: ({end_index}) must be less than or equal to the number of UTF-8 characters in the string ({})!", self.chars_length);
        }

        if end_index < start_index {
            panic!("Range end: ({end_index} must be greater than or equal to Range start: ({start_index})!")
        }

        let bytes_start: usize = self.chars_vec[start_index].offset;
        let bytes_end: usize = match end_index {
            _val if self.chars_length == end_index => self.str_length,
            _ => self.chars_vec[end_index].offset,
        };

        &self.str[bytes_start..bytes_end]  
    }
}

impl<'a> Display for IndexableStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl<'a> Index<usize> for IndexableStr<'a> {
    type Output = char;

    fn index(&self, index: usize) -> &char {
        &self.chars_vec[index].chr
    }
}

impl<'a> Index<Range<usize>> for IndexableStr<'a> {
    type Output = str;

    fn index(&self, range: Range<usize>) -> &Self::Output {
        self.create_str_from_range(range.start, range.end)
    }
}

impl<'a> Index<RangeFrom<usize>> for IndexableStr<'a> {
    type Output = str;

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        self.create_str_from_range(index.start, self.chars_length)
    }
}

impl<'a> Index<RangeTo<usize>> for IndexableStr<'a> {
    type Output = str;

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        self.create_str_from_range(0, index.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str_works() {
        let s = IndexableStr::new("0😀23456789");

        assert_eq!(s.as_str(), "0😀23456789");
    }

    #[test]
    fn test_len_works() {
        let s = IndexableStr::new("0😀23456789");

        assert_eq!(s.len(), 10);
    }

    #[test]
    fn test_to_string_works() {
        let s = IndexableStr::new("0😀23456789");

        assert_eq!(s.to_string(), "0😀23456789");
    }
    #[test]
    fn test_index_works() {
        let s = IndexableStr::new("0😀23456789");

        assert_eq!(s[1], '😀');
    }
    #[test]
    fn test_range_works() {
        let s = IndexableStr::new("0😀23456789");

        assert_eq!(&s[1..9], "😀2345678");
    }
    #[test]
    fn test_range_from_works() {
        let s = IndexableStr::new("0😀23456789");

        assert_eq!(&s[1..], "😀23456789");
    }

    #[test]
    fn test_range_to_works() {
        let s = IndexableStr::new("0😀23456789");
        println!("length: {}", s.as_str().len());

        assert_eq!(&s[..9], "0😀2345678");
    }

    #[test]
    fn test_range_when_last_character_is_multi_byte() {
        let s = IndexableStr::new("0😀2345678😀");
        println!("length: {}", s.as_str().len());

        assert_eq!(&s[..10], "0😀2345678😀");
    }

    #[test]
    fn test_range_with_ending_index_too_large() {
        let s = IndexableStr::new("0😀2345678😀");

        let result = std::panic::catch_unwind(|| s.create_str_from_range(0, 11));
        assert!(result.is_err());
    }

    #[test]
    fn test_range_with_ending_index_is_less_than_the_starting_index() {
        let s = IndexableStr::new("0😀2345678😀");

        let result = std::panic::catch_unwind(|| s.create_str_from_range(20, 10));
        assert!(result.is_err());
    }
}
