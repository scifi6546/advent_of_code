#![feature(str_split_once)]
use std::fs::File;
use std::io::prelude::*;
#[derive(PartialEq, Debug)]
enum Tile {
    Empty,
    Tree,
}
fn a() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tiles: Vec<Vec<Tile>> = contents
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Tree,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    //(y,x)
    let mut pos = (0usize, 0usize);
    let mut count = 0;
    loop {
        let next_pos = (pos.0 + 1, pos.1 + 3);
        if next_pos.0 == tiles.len() {
            println!("it took {}", count);
            break;
        } else {
            let temp_x = next_pos.1 % tiles[next_pos.0].len();
            if tiles[next_pos.0][temp_x] == Tile::Tree {
                count += 1;
            }
            pos = next_pos;
        }
    }

    Ok(())
}
fn calculate_slope(tiles: &Vec<Vec<Tile>>, slope_x: usize, slope_y: usize) -> usize {
    let mut pos = (0usize, 0usize);
    let mut count = 0;
    loop {
        let next_pos = (pos.0 + slope_y, pos.1 + slope_x);
        if next_pos.0 >= tiles.len() {
            println!("it took {}", count);
            return count;
        } else {
            let temp_x = next_pos.1 % tiles[next_pos.0].len();
            if tiles[next_pos.0][temp_x] == Tile::Tree {
                count += 1;
            }
            pos = next_pos;
        }
    }
}
fn b() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tiles: Vec<Vec<Tile>> = contents
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Tree,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let total = calculate_slope(&tiles, 1, 1)
        * calculate_slope(&tiles, 3, 1)
        * calculate_slope(&tiles, 5, 1)
        * calculate_slope(&tiles, 7, 1)
        * calculate_slope(&tiles, 1, 2);
    println!("total: {}", total);
    Ok(())
}

fn main() -> std::io::Result<()> {
    a()?;
    b()?;
    Ok(())
}
