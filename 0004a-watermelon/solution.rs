use std::io::{Stdout, stdout, Write, BufWriter};


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

    let weight: u64 = input.next::<u64>();
    if weight > 2 && weight % 2 == 0 {
        writeln!(output, "YES").ok();
    } else {
        writeln!(output, "NO").ok();
    }
}


fn main() {
    solution();
}
