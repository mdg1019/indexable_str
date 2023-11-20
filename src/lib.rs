use std::{
    fmt::Display,
    ops::{Index, Range, RangeFrom, RangeTo},
};

struct CharOffset {
    chr: char,
    offset: usize,
}

pub struct IndexableStr<'a> {
    str: &'a str,
    char_offset_vec: Vec<CharOffset>,
}

impl<'a> IndexableStr<'a> {
    pub fn new(str: &'a str) -> IndexableStr {
        let mut current_offset: usize = 0;

        let char_offset_vec = str.chars().map(|c| {
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

        IndexableStr {
            str,
            char_offset_vec,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.str
    }

    pub fn len(&self) -> usize {
        self.char_offset_vec.len()
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
        &self.char_offset_vec[index].chr
    }
}

impl<'a> Index<Range<usize>> for IndexableStr<'a> {
    type Output = str;

    fn index(&self, range: Range<usize>) -> &str {
        let start_index: usize = self.char_offset_vec[range.start].offset;
        let end_index: usize = self.char_offset_vec[range.end - 1].offset;

        &self.str[start_index..=end_index]
    }
}

impl<'a> Index<RangeFrom<usize>> for IndexableStr<'a> {
    type Output = str;

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        let start_index: usize = self.char_offset_vec[index.start].offset;
        let end_index: usize = self.char_offset_vec[self.char_offset_vec.len() - 1].offset;

        &self.str[start_index..=end_index]
    }
}

impl<'a> Index<RangeTo<usize>> for IndexableStr<'a> {
    type Output = str;

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        let end_index: usize = self.char_offset_vec[index.end - 1].offset;

        &self.str[0..=end_index]
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
}
