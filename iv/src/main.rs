pub mod inner;
pub mod iters;
pub mod traits;
use rand::Rng;
use std::collections::HashMap;

pub fn random(n: u32) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut flag = HashMap::new();

    while ret.len() < n as usize {
        let v = rng.gen_range(0..n);

        if !flag.contains_key(&v) {
            ret.push(v);
            flag.insert(v, true);
        }
    }
    ret
}

fn main() {
    let random = random(2000);
    println!("{:?}", &random);
}
