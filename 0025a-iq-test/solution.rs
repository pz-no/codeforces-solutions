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

    let number_count: usize = input.next::<usize>();
    let mut counters: [usize; 2] = [0, 0];
    let mut indices: [usize; 2] = [0, 0];

    let numbers: Vec<usize> = (0..number_count).map(|_| input.next::<usize>()).collect::<Vec<_>>();
    for (num_index, number) in numbers.iter().enumerate() {
        let index: usize = number % 2;
        counters[index] += 1;
        if counters[index] == 1 {
            indices[index] = num_index + 1;
        }

        if counters[index] > 1 && counters[1 - index] == 1 {
            writeln!(output, "{}", indices[1 - index]).ok();
            break;
        }

        if counters[index] == 1 && counters[1 - index] > 1 {
            writeln!(output, "{}", indices[index]).ok();
            break;
        }
    }
}


fn main() {
    solution()
}
