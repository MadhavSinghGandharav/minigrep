pub struct Horsepool {
    table: [usize; 256],
    ignore_case: bool,
    needle_length: usize,
    needle: Vec<u8>,
}

pub struct Match {
    pub start: usize,
    pub end: usize,
}

impl Horsepool {
    pub fn build(string: &str, ignore_case: bool) -> Self {
        let mut string = string.as_bytes().to_vec(); // convert to Vec<u8>

        if ignore_case {
            Self::convert_to_lowercase(&mut string); // inplace lowercase
        }

        let m = string.len();
        let mut table = [m; 256]; // make it mutable

        for i in 0..m - 1 {
            table[string[i] as usize] = m - i - 1;
        }
        Self {
            table,
            ignore_case,
            needle_length: m,
            needle: string,
        }
    }

    pub fn search(&self, haystack: &str) -> Vec<Match> {
        let n = haystack.len();
        let mut matches: Vec<Match> = Vec::new();

        if n < self.needle_length {
            return matches;
        }
        // convert haystack to bytes
        let mut haystack = haystack.as_bytes().to_vec();
        if self.ignore_case {
            Self::convert_to_lowercase(&mut haystack); // inplace lowercase
        }
        let mut j = self.needle_length - 1;

        // pattern matching

        while j < n {
            let mut i = 0;
            while i < self.needle_length
                && self.needle[self.needle_length - i - 1] == haystack[j - i]
            {
                i += 1;
            }

            if i == self.needle_length {
                matches.push(Match {
                    start: j - self.needle_length + 1,
                    end: j + 1,
                });
            }

            j += self.table[haystack[j] as usize];
        }

        matches
    }

    fn convert_to_lowercase(arr: &mut [u8]) {
        for byte in arr {
            if *byte >= b'A' && *byte <= b'Z' {
                *byte += 32;
            }
        }
    }
}
