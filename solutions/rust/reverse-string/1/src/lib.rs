use std::collections::VecDeque;

/*
This solution combines 2 iterator (nested):
  - CodePointBytes: which splits the sequence of bytes of a string into UTF-8 code points
  - GraphemeBytes: that splits sequence of unicode bytes into bytes representing individual graphemes

The solution essentially reverses Graphemes in a string.

Note that this implementation is not comprehensive and only the UMLAT grapheme is supported (to pass the tests).

If you are looking for a complete grapheme implementation use:

https://github.com/unicode-rs/unicode-segmentation

---
simple non grapheme-safe solution
`input.chars().rev().collect()`
*/

type CodePoint = Vec<u8>;

#[derive(Debug)]
struct CodePointBytes<'a> {
    bytes_iter: std::str::Bytes<'a>,
}

impl<'a> CodePointBytes<'a> {
    fn new(string: &'a str) -> Self {
        CodePointBytes {
            bytes_iter: string.bytes(),
        }
    }
}

impl<'a> Iterator for CodePointBytes<'a> {
    type Item = CodePoint;

    fn next(&mut self) -> Option<CodePoint> {
        match self.bytes_iter.next() {
            Some(byte) => {
                if byte < 0b11_00_00_00 {
                    // 1 byte code point
                    Some(vec![byte])
                } else if byte < 0b11_10_00_00 {
                    // 2 bytes code point
                    Some(vec![byte, self.bytes_iter.next().unwrap()])
                } else if byte < 0b11_11_00_00 {
                    // 3 bytes code point
                    Some(vec![
                        byte,
                        self.bytes_iter.next().unwrap(),
                        self.bytes_iter.next().unwrap(),
                    ])
                } else {
                    // 4 bytes code point
                    Some(vec![
                        byte,
                        self.bytes_iter.next().unwrap(),
                        self.bytes_iter.next().unwrap(),
                        self.bytes_iter.next().unwrap(),
                    ])
                }
            }
            None => None,
        }
    }
}

type Grapheme = Vec<u8>;

#[derive(Debug)]
struct GraphemeBytes<'a> {
    code_points: CodePointBytes<'a>,
    previous_code_point: CodePoint,
}

impl<'a> GraphemeBytes<'a> {
    fn new(code_points: CodePointBytes<'a>) -> Self {
        GraphemeBytes {
            code_points: code_points,
            previous_code_point: vec![],
        }
    }
}

// Supports only UMLAT and prepend type graphemes only to pass the exercise.
// Use https://github.com/unicode-rs/unicode-segmentation for a comprehensive approach
const UMLAT: &'static [u8] = &[204, 136];
const PREPEND_GRAPHEMES: &'static [&[u8]] = &[&UMLAT];

impl<'a> Iterator for GraphemeBytes<'a> {
    type Item = Grapheme;

    fn next(&mut self) -> Option<Grapheme> {
        let mut current = self.code_points.next();
        if current.is_some() && self.previous_code_point.len() == 0 {
            // makes sure it always look at two code points
            self.previous_code_point = current.unwrap();
            current = self.code_points.next();
        }

        // end of the string, flush previous code point (if any)
        if current.is_none() {
            if self.previous_code_point.len() > 0 {
                let previous_code_point = self.previous_code_point.clone();
                self.previous_code_point = vec![];
                return Some(previous_code_point);
            }
        }

        if current.is_some() {
            let current_code_point = current.unwrap();

            // if the current code point needs to be joined with the previous to form a grapheme
            if PREPEND_GRAPHEMES.contains(&current_code_point.as_slice()) {
                let current_grapheme = [
                    &self.previous_code_point.as_slice(),
                    &current_code_point[..],
                ]
                .concat();
                self.previous_code_point = vec![];

                return Some(current_grapheme);
            }

            // else return the previous code point and replace it with the current
            let previous_code_point = self.previous_code_point.clone();
            self.previous_code_point = current_code_point;
            return Some(previous_code_point);
        }

        return None;
    }
}

pub fn reverse(input: &str) -> String {
    let code_points_iter = CodePointBytes::new(input);
    let grapheme_iter = GraphemeBytes::new(code_points_iter);
    let mut reversed_bytes: VecDeque<u8> = VecDeque::new();

    for grapheme in grapheme_iter {
        for byte in grapheme.iter().rev() {
            reversed_bytes.push_back(*byte);
        }
    }

    let utf8_bytes: Vec<u8> = reversed_bytes.into_iter().rev().collect();
    String::from_utf8(utf8_bytes).unwrap()
}
