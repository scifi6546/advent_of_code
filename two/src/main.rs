#![feature(str_split_once)]
use std::fs::File;
use std::io::prelude::*;
fn a() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let count = contents
        .lines()
        .map(|x| x.split_once(':').unwrap())
        .map(|(rule, password)| {
            let (range, letter) = rule.split_once(' ').unwrap();
            let (start, end) = range.split_once('-').unwrap();
            let start_num = start.parse::<u64>().ok().unwrap();
            let end_num = end.parse::<u64>().ok().unwrap();
            let match_num = password.matches(letter).count() as u64;
            (start_num, end_num, match_num)
        })
        .filter(|(start, end, num)| num >= start && num <= end)
        .count();
    println!("a) count: {}", count);
    Ok(())
}
fn b() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let count = contents
        .lines()
        .map(|x| x.split_once(':').unwrap())
        .map(|(rule, password)| {
            let (range, letter) = rule.split_once(' ').unwrap();
            let (start, end) = range.split_once('-').unwrap();
            let start_num = start.parse::<u64>().ok().unwrap();
            let end_num = end.parse::<u64>().ok().unwrap();
            (start_num, end_num, letter.chars().nth(0).unwrap(), password)
        })
        .filter(|(start, end, letter, password)| {
            let a = if password
                .clone()
                .chars()
                .nth(start.clone() as usize)
                .unwrap()
                == letter.clone()
            {
                1
            } else {
                0
            };
            let b = if password.clone().chars().nth(end.clone() as usize).unwrap() == letter.clone()
            {
                1
            } else {
                0
            };
            if a + b == 1 {
                true
            } else {
                false
            }
        })
        .count();
    println!("b) count: {}", count);
    Ok(())
}
fn main() -> std::io::Result<()> {
    a()?;
    b()?;
    Ok(())
}
