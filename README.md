# indexable_str

This Rust library features the `IndexableStr` struct. To better understand how strings work in Rust, I decided to create a personal project that parses JSON files. When creating the lexer, I tried a number of ways to correctly index a string to retrieve string slices and `char`s. Once I finally started to wrap my head around Rust strings and lifetimes, I finally created a simple version of `IndexableStr`. I found it very convenient to use and it had a bonus of making my code far more readable than the hacky approaches I'd previously tried. Because others may find it useful too, I decided to split `IndexableStr` into a separate library project.

See the <a href="https://mdg1019.github.io/indexable_str/doc/indexable_str/index.html" target="_blank">rustdoc file</a> for the api details.

# Examples
```rust
// Gets a char from a specified index.
use indexable_str::IndexableStr;

let s = IndexableStr::new("0😀2345678😀");

assert_eq!(s[1], '😀');
```

```rust
// Gets a string slice from a specified range.
use indexable_str::IndexableStr;

let s = IndexableStr::new("0😀2345678😀");

assert_eq!(&s[1..9], "😀2345678")
```

```rust
// Parses a string of signed integers, which are separated by whitespace
use regex::Regex;
use indexable_str::IndexableStr;
  
let text = IndexableStr::new("0 1 2\n  -11  -12  -13\n");
let signed_integer_pattern: Regex = Regex::new(r#"\b(0)|(-?[1-9]\d*)\b"#).unwrap();
let mut signed_integer_vec: Vec<i64> = Vec::new();
let mut cursor: usize = 0;
 
while cursor < text.len() {
   let c = text[cursor];

    match c {
        ' ' | '\t' | '\r' | '\n' => {
            cursor += 1;
            continue;
        },
        _=> (), 
    }

    if let Some(captures) = signed_integer_pattern.captures(&text[cursor..]) {
        let num_string = captures[0].to_string();
        let num = num_string.parse::<i64>();
        signed_integer_vec.push(num.unwrap());

        cursor += num_string.len();

        continue;
    }

    panic!("Unexpected character '{}' at position ({})!", c, cursor);
}
 
assert_eq!(signed_integer_vec.len(), 6);
assert_eq!(signed_integer_vec[0], 0);
assert_eq!(signed_integer_vec[1], 1);
assert_eq!(signed_integer_vec[2], 2);
assert_eq!(signed_integer_vec[3], -11);
assert_eq!(signed_integer_vec[4], -12);
assert_eq!(signed_integer_vec[5], -13);
```
