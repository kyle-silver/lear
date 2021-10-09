use std::cmp::min;

use rand::Rng;
use serde::Deserialize;

const SCENES: [&str; 25] = [
    include_str!("res/01.json"),
    include_str!("res/02.json"),
    include_str!("res/03.json"),
    include_str!("res/04.json"),
    include_str!("res/05.json"),
    include_str!("res/06.json"),
    include_str!("res/07.json"),
    include_str!("res/08.json"),
    include_str!("res/09.json"),
    include_str!("res/10.json"),
    include_str!("res/11.json"),
    include_str!("res/12.json"),
    include_str!("res/13.json"),
    include_str!("res/14.json"),
    include_str!("res/15.json"),
    include_str!("res/16.json"),
    include_str!("res/17.json"),
    include_str!("res/18.json"),
    include_str!("res/19.json"),
    include_str!("res/20.json"),
    include_str!("res/21.json"),
    include_str!("res/22.json"),
    include_str!("res/23.json"),
    include_str!("res/24.json"),
    include_str!("res/25.json"),
];

#[derive(Debug, Deserialize)]
struct Heading {
    act: i32,
    setting: String,
    staging: String,
}

#[derive(Debug, Deserialize)]
enum Line {
    #[serde(rename(deserialize = "text"))]
    Text(String),
    #[serde(rename(deserialize = "direction"))]
    Direction(String),
}

#[derive(Debug, Deserialize)]
struct Dialogue {
    character: String,
    lines: Vec<Line>,
    act: i32,
    scene: i32,
    start: i32,
    stop: i32,
}

#[derive(Debug, Deserialize)]
enum Block {
    Heading(Heading),
    Dialogue(Dialogue),
}

fn main() {
    let mut rng = rand::thread_rng();
    let scene = rng.gen_range(0..25) as usize;
    let blocks: Vec<Block> = serde_json::from_str(SCENES[scene]).unwrap();
    let blocks_to_show = rng.gen_range(2..=min(5, blocks.len())) as usize;
    // the fact that scene 8 is so short complicates things...
    let range = 0..(blocks.len() - blocks_to_show);
    let start = if range.len() == 0 {
        0
    } else {
        rng.gen_range(range)
    };
    for block in &blocks[start..(start + blocks_to_show)] {
        println!("{:?}", block);
    }
}
