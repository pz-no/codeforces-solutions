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

    let (height, width, size) = (input.next::<u64>(), input.next::<u64>(), input.next::<u64>());
    let height_ratio: u64 = height / size + (height % size != 0) as u64;
    let width_ratio: u64 = width / size + (width % size != 0) as u64;
    writeln!(output, "{}", height_ratio * width_ratio).ok();
}


fn main() {
    solution();
}
