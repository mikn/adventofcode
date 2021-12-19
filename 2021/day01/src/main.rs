use std::env;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Values {
    deq: VecDeque<i64>,
    max_len: usize,
}

impl Values {
    fn sum(&self) -> i64 {
        let mut ret = 0;
        for val in self.deq.iter() {
            ret += val;
        }
        return ret;
    }

    fn len(&self) -> usize {
        return self.deq.len();
    }

    fn add(&mut self, x: i64) {
        if self.deq.len() == self.max_len {
            self.deq.pop_front();
        }
        self.deq.push_back(x);
    }

    fn new(max_len: usize) -> Values {
        Values{ deq: VecDeque::with_capacity(max_len), max_len: max_len }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {}", filename);

    let f = File::open(filename).unwrap();

    let file = BufReader::new(&f);

    let mut vals = Values::new(3);
    let mut inc_count = 0;

    for line in file.lines() {
        let val = line.unwrap().trim().parse::<i64>().unwrap();
        if vals.len() == vals.max_len {
            let prev_sum = vals.sum();
            vals.add(val);
            if prev_sum < vals.sum() {
                inc_count += 1;
            }
        } else {
            vals.add(val);
        }
    }
    println!("The value increased {} times from the previous value!", inc_count);
}
