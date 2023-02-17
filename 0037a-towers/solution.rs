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

    fn nexts<T: std::str::FromStr>(&mut self, count: usize) -> Vec<T> {
        (0..count).map(|_| self.next::<T>()).collect()
    }
}


fn solution() {
    let mut input: Scanner = Scanner::default();
    let mut output: BufWriter<Stdout> = BufWriter::new(stdout());

    let bar_count: usize = input.next::<usize>();
    let mut bars: Vec<usize> = input.nexts::<usize>(bar_count);
    bars.sort();

    let mut height: usize = 1;
    let mut max_length: usize = 0;

    let mut i: usize = 0;
    while i < bar_count {
        let mut j = i + 1;
        while j < bar_count {
            if bars[i] != bars[j] {
                height += 1;
                break;
            }

            j += 1;
        }

        if max_length < j - i {
            max_length = j - i;
        }

        i = j;
    }

    writeln!(output, "{0} {1}", max_length, height).ok();
}


fn main() {
    solution();
}
