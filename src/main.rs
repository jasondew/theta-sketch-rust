use std::collections::hash_map::DefaultHasher;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::hash::Hasher;
use std::io;

#[derive(Debug)]
pub struct Sketch {
    data: BinaryHeap<u64>,
    k: usize,
    count: u64,
}

impl Sketch {
    pub fn new(k: usize) -> Sketch {
        Sketch {
            data: BinaryHeap::with_capacity(k + 1),
            k: k,
            count: 0,
        }
    }

    pub fn add(&mut self, value: &str) {
        let hash: u64 = Self::hash(value);

        if self.data.iter().find(|&&x| x == hash).is_none() {
            self.data.push(hash);
            if self.data.len() > self.k {
                self.data.pop();
            }
        }
        self.count += 1
    }

    pub fn estimate(&self) -> Option<u64> {
        if let Some(max_k) = self.data.peek() {
            let normalized_max_k: f64 = *max_k as f64 / u64::MAX as f64;
            let estimate: f64 = (self.k - 1) as f64 / normalized_max_k;

            Some(estimate.round() as u64)
        } else {
            None
        }
    }

    fn hash(value: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

fn main() {
    let mut sketch = Sketch::new(32);
    let mut line = String::new();
    let mut set = HashSet::new();

    while io::stdin().read_line(&mut line).is_ok() {
        let line = std::mem::take(&mut line);
        if line == "" {
            break;
        }

        set.insert(line.clone());
        sketch.add(&line);
    }

    println!("sketch {:?}", sketch);
    println!("estimate = {}", sketch.estimate().unwrap());
    println!("  actual = {}", set.len());
}
