use std::{cmp::min, fmt::Display};

use rand::{prelude::ThreadRng, Rng};
use serde::Deserialize;
use termion::style;

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
pub struct Heading {
    act: String,
    scene: String,
    setting: String,
    staging: String,
}

impl Display for &Heading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}, {}{}\n{}\n\n\t{}{}{}\n",
            style::Bold,
            self.act,
            self.scene,
            style::Reset,
            self.setting,
            style::Italic,
            self.staging,
            style::Reset
        )
    }
}

#[derive(Debug, Deserialize)]
pub enum Line {
    #[serde(rename(deserialize = "text"))]
    Text(String),
    #[serde(rename(deserialize = "direction"))]
    Direction(String),
}

#[derive(Debug, Deserialize)]
pub struct Dialogue {
    character: String,
    lines: Vec<Line>,
    act: i32,
    scene: i32,
    start: i32,
    stop: i32,
}

impl Display for &Dialogue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}\n",
            style::Bold,
            self.character,
            style::Reset
        ))?;
        // for line in &self.lines {
        //     f.write_fmt(format_args!("{}", line))?;
        // }
        let mut iter = self.lines.iter().peekable();
        let mut prev = None;
        while let Some(line) = iter.next() {
            if let Some(&Line::Direction(_)) = prev {
                match line {
                    Line::Text(text) => f.write_fmt(format_args!("\n\t{}\n", text))?,
                    Line::Direction(direction) => f.write_fmt(format_args!(
                        "\t{}{}{}\n",
                        style::Italic,
                        direction,
                        style::Reset
                    ))?,
                };
            } else {
                match line {
                    Line::Text(text) => f.write_fmt(format_args!("\t{}\n", text))?,
                    Line::Direction(direction) => f.write_fmt(format_args!(
                        "\n\t{}{}{}\n",
                        style::Italic,
                        direction,
                        style::Reset
                    ))?,
                };
            }
            prev = Some(line);
        }
        f.write_fmt(format_args!("\n"))?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub enum Block {
    Heading(Heading),
    Dialogue(Dialogue),
}

pub fn blocks_to_show(rng: &mut ThreadRng) -> Result<Vec<Block>, std::io::Error> {
    let scene = rng.gen_range(0..25) as usize;
    let blocks: Vec<Block> = serde_json::from_str(SCENES[scene])?;
    let blocks_to_show = rng.gen_range(2..=min(5, blocks.len())) as usize;
    // the fact that scene 8 is so short complicates things...
    let range = 0..(blocks.len() - blocks_to_show);
    let start = if range.len() == 0 {
        0
    } else {
        rng.gen_range(range)
    };
    let blocks = blocks
        .into_iter()
        .skip(start)
        .take(blocks_to_show)
        .collect();
    Ok(blocks)
}

pub fn display(blocks: &[Block]) {
    for block in blocks {
        match block {
            Block::Heading(heading) => {
                // println!("{}{}{}", style::Bold, heading.act, style::Reset);
                // println!("{}{}{}", style::Bold, heading.setting, style::Reset);
                // println!("{}{}{}", style::Italic, heading.staging, style::Reset);
                println!("{}", heading);
            }
            Block::Dialogue(dialogue) => {
                print!("{}", dialogue);
            }
        }
    }
}
