use std::{ops::{Index, Range, RangeFrom, RangeTo}, fmt::Display};

pub struct IndexableStr<'a> {
    str: &'a str,
    char_vec: Vec<char>, 
}

impl<'a> IndexableStr<'a> {
    pub fn new(str: &'a str) -> IndexableStr {
        IndexableStr { 
            str, 
            char_vec: str.chars().collect(),
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.str
    }

    pub fn len(&self) -> usize {
        self.char_vec.len()
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
        &self.char_vec[index]
    }
}

impl<'a> Index<Range<usize>> for IndexableStr<'a> {
    type Output = str;

    fn index(&self, range: Range<usize>) -> &str {
        &self.str[range]
    }
}

impl<'a> Index<RangeFrom<usize>> for IndexableStr<'a> {
    type Output = str;

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.str[index.start..self.str.len()]
    } 
}

impl<'a> Index<RangeTo<usize>> for IndexableStr<'a> {
    type Output = str;

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.str[0..index.end]
    } 
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_as_str_works() {
        let s = IndexableStr::new("0123456789");

        assert_eq!(s.as_str(), "0123456789");
    }

    #[test]
    fn test_len_works() {
        let s = IndexableStr::new("0123456789");

        assert_eq!(s.len(), 10);
    }

    #[test]
    fn test_to_string_works() {
        let s = IndexableStr::new("0123456789");

        assert_eq!(s.to_string(), "0123456789");
    }
    #[test]
    fn test_index_works() {
        let s = IndexableStr::new("0123456789");

        assert_eq!(s[2], '2');
    }
    #[test]
    fn test_range_works() {
        let s = IndexableStr::new("0123456789");

        assert_eq!(&s[1..9], "12345678");
    }
    #[test]
    fn test_range_from_works() {
        let s = IndexableStr::new("0123456789");

        assert_eq!(&s[1..], "123456789");
    }
    #[test]
    fn test_range_to_works() {
        let s = IndexableStr::new("0123456789");

        assert_eq!(&s[..9], "012345678");
    }
}    