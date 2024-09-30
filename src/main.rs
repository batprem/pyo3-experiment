#[allow(unused_variables)]
use rand::Rng; // 0.8.5
mod packages_2;
use packages_2::plus_array;
use std::time::Instant;
use tqdm::tqdm;

fn main() {
    let mut sample: Vec<i32> = Vec::new();
    println!("Generating sample");
    for _ in tqdm(0..500_000_000) {
        sample.push(rand::thread_rng().gen_range(1..100));
    }
    println!("Finished!");
    let start = Instant::now();
    let new_list = plus_array(sample, 1);
    let elapsed = start.elapsed();
    println!("Elapsed {:?} seconds", elapsed.as_secs_f32()); 
}