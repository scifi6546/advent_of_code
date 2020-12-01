use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: Vec<i64> = contents
        .split('\n')
        .map(|s| s.parse::<i64>().ok())
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect();
    for i in data.iter() {
        for j in data.iter() {
            for k in data.iter() {
                if i + j + k == 2020 {
                    println!("i: {} j: {} k: {}", i, j, k);
                    println!("i*j*k: {}", i * j * k);
                }
            }
        }
    }
    Ok(())
}
