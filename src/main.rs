use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone())?;
    let reader = BufReader::new(file);
    let mut fuel_total = 0;
    for line in reader.lines() {
        fuel_total += ((line?.parse::<f32>().unwrap()) / 3_f32) as i32 - 2;
    }
    println!("Total fuel required: {}", fuel_total);
    Ok(())
}
