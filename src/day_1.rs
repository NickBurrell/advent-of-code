use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone())?;
    let reader = BufReader::new(file);
    let mut fuel_total = 0;
    let mut extra_cost = 0;
    for line in reader.lines() {
        let line_value = line?.parse::<f32>().unwrap();
        fuel_total += ((line_value) / 3_f32) as i32 - 2;
        let mut last_fuel = (((line_value / 3_f32) as i32 - 2) as f32 / 3_f32) as i32 - 2;
        loop {
            if last_fuel >= 0 {
                extra_cost += last_fuel;
                last_fuel = ((last_fuel as f32) / 3_f32) as i32 - 2;
            } else {
                break;
            }
        }
    }
    println!("Total fuel required: {}", fuel_total);

    println!("New total fuel required: {}", fuel_total + extra_cost);
    Ok(())
}
