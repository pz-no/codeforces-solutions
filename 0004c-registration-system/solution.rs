use std::io::{Stdout, stdout, Write, BufWriter};
use std::collections::hash_map::HashMap;


#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parsing");
            }
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).expect("Failed reading");
            self.buffer = line.split_whitespace().rev().map(String::from).collect();
        }
    }
}


fn solution() {
    let mut input: Scanner = Scanner::default();
    let mut output: BufWriter<Stdout> = BufWriter::new(stdout());

    let name_count: usize = input.next::<usize>();
    let mut name_map: HashMap<String, usize> = HashMap::new();
	for _ in 0..name_count {
        let name: String = input.next::<String>();
        match name_map.get_mut(&name) {
            None => {
                writeln!(output, "OK").ok();
                name_map.insert(name, 1);
            },
            Some(value) => {
                writeln!(output, "{0}{1}", name, value).ok();
                *value += 1;
            }
        }
    }
}


fn main() {
    solution()
}
