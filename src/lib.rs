use std::{
    cmp::{max, min},
    fmt::Display,
    ops::RangeInclusive,
};

use rand::{prelude::ThreadRng, Rng};
use serde::Deserialize;
use termion::style;

const SCENES: [&str; 26] = [
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
    include_str!("res/26.json"),
];

#[derive(Debug)]
struct Metadata {
    act: &'static str,
    scene: usize,
    lines: usize,
}

const METADATA: [Metadata; 26] = [
    Metadata {
        act: "1",
        scene: 1,
        lines: 332,
    },
    Metadata {
        act: "",
        scene: 2,
        lines: 191,
    },
    Metadata {
        act: "",
        scene: 3,
        lines: 27,
    },
    Metadata {
        act: "",
        scene: 4,
        lines: 352,
    },
    Metadata {
        act: "",
        scene: 5,
        lines: 48,
    },
    Metadata {
        act: "2",
        scene: 1,
        lines: 141,
    },
    Metadata {
        act: "",
        scene: 2,
        lines: 177,
    },
    Metadata {
        act: "",
        scene: 3,
        lines: 21,
    },
    Metadata {
        act: "",
        scene: 4,
        lines: 339,
    },
    Metadata {
        act: "3",
        scene: 1,
        lines: 58,
    },
    Metadata {
        act: "",
        scene: 2,
        lines: 100,
    },
    Metadata {
        act: "",
        scene: 3,
        lines: 25,
    },
    Metadata {
        act: "",
        scene: 4,
        lines: 187,
    },
    Metadata {
        act: "",
        scene: 5,
        lines: 25,
    },
    Metadata {
        act: "",
        scene: 6,
        lines: 117,
    },
    Metadata {
        act: "",
        scene: 7,
        lines: 118,
    },
    Metadata {
        act: "4",
        scene: 1,
        lines: 90,
    },
    Metadata {
        act: "",
        scene: 2,
        lines: 111,
    },
    Metadata {
        act: "",
        scene: 3,
        lines: 62,
    },
    Metadata {
        act: "",
        scene: 4,
        lines: 32,
    },
    Metadata {
        act: "",
        scene: 5,
        lines: 45,
    },
    Metadata {
        act: "",
        scene: 6,
        lines: 314,
    },
    Metadata {
        act: "",
        scene: 7,
        lines: 110,
    },
    Metadata {
        act: "5",
        scene: 1,
        lines: 78,
    },
    Metadata {
        act: "",
        scene: 2,
        lines: 13,
    },
    Metadata {
        act: "",
        scene: 3,
        lines: 386,
    },
];

pub fn display_contents() {
    println!(
        "{}{: ^19}{}",
        style::Italic,
        "The Tragedie of",
        style::Reset
    );
    println!("{: ^19}\n", "KING LEAR");
    println!("{: ^19}\n", "by");
    println!("{}{: ^19}{}", style::Italic, "William", style::Italic);
    println!("{}{: ^19}{}\n", style::Italic, "Shakespeare", style::Italic);
    println!(
        "{}{: ^5}{: ^7}{: ^7}{}",
        style::Bold,
        "Act",
        "Scene",
        "Lines",
        style::Reset
    );
    for line in &METADATA {
        if line.act != "" {
            println!("{:-^19}", "");
        }
        println!("{: ^5}{: ^7}{: ^7}", line.act, line.scene, line.lines,);
    }
}

struct TableEntry {
    start: usize,
    scenes: RangeInclusive<usize>,
}

impl TableEntry {
    fn index(&self, scene: usize) -> Option<usize> {
        if self.scenes.contains(&scene) {
            Some(self.start + scene - 1)
        } else {
            None
        }
    }
}

const TABLE_OF_CONTENTS: [TableEntry; 5] = [
    TableEntry {
        start: 0,
        scenes: (1..=5),
    },
    TableEntry {
        start: 5,
        scenes: (1..=4),
    },
    TableEntry {
        start: 9,
        scenes: (1..=7),
    },
    TableEntry {
        start: 16,
        scenes: (1..=7),
    },
    TableEntry {
        start: 23,
        scenes: (1..=3),
    },
];

#[derive(Debug, Deserialize, Clone)]
pub struct Heading {
    act: String,
    scene: String,
    setting: String,
    staging: String,
}

#[derive(Debug)]
pub enum LearError {
    IoError(std::io::Error),
    InvalidAct(usize),
    InvalidScene {
        act: usize,
        scene: usize,
    },
    InvalidLines {
        act: usize,
        scene: usize,
        lines: RangeInclusive<usize>,
    },
}

impl From<serde_json::Error> for LearError {
    fn from(e: serde_json::Error) -> Self {
        LearError::IoError(e.into())
    }
}

impl Display for LearError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for LearError {}

impl Display for &Heading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}, {}{}\n{}\n\n\t{}{}{}\n\n",
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

#[derive(Debug, Deserialize, Clone)]
pub enum Line {
    #[serde(rename(deserialize = "text"))]
    Text(String),
    #[serde(rename(deserialize = "direction"))]
    Direction(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dialogue {
    character: String,
    lines: Vec<Line>,
    act: usize,
    scene: usize,
    start: usize,
    stop: usize,
}

impl Dialogue {
    fn selection(&self, range: &RangeInclusive<usize>) -> Option<Dialogue> {
        // no overlap
        if self.start > *range.end() || self.stop < *range.start() {
            return None;
        }
        // the line numbers from the selection
        let start_line = max(range.start(), &self.start);
        let stop_line = min(range.end(), &self.stop);
        // the start and stop indexes in self.lines
        let (start, _) = self
            .lines
            .iter()
            .enumerate()
            .filter(|(_, line)| matches!(line, Line::Text(_)))
            .find(|(i, _)| i + self.start >= *start_line)?;
        let (stop, _) = self
            .lines
            .iter()
            .enumerate()
            .filter(|(_, line)| matches!(line, Line::Text(_)))
            .find(|(i, _)| i + self.start >= *stop_line)?;
        // grab only the lines we want to display
        let mut lines: Vec<Line> = self.lines[start..=stop].iter().cloned().collect();
        // omit empty dialogue blocks
        if lines.is_empty() {
            return None;
        }
        // if we truncated a dialogue block, add leading and/or trailing ellipses
        if range.start() > &self.start {
            lines.insert(0, Line::Text("...".into()));
        }
        if range.end() < &self.stop {
            lines.push(Line::Text("...".into()));
        }
        Some(Dialogue {
            lines,
            start: *start_line,
            stop: *stop_line,
            character: self.character.clone(),
            ..*self
        })
    }
}

impl Display for &Dialogue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}\n",
            style::Bold,
            self.character,
            style::Reset
        ))?;
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

#[derive(Debug, Deserialize, Clone)]
pub enum Block {
    Heading(Heading),
    Dialogue(Dialogue),
}

impl Block {
    fn selection(&self, range: &RangeInclusive<usize>) -> Option<Block> {
        match self {
            Block::Heading(_) => None,
            Block::Dialogue(d) => Some(Block::Dialogue(d.selection(range)?)),
        }
    }
}

impl Display for &Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Heading(heading) => write!(f, "{}", heading),
            Block::Dialogue(dialogue) => write!(f, "{}", dialogue),
        }
    }
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

pub fn text(
    act: usize,
    scene: usize,
    lines: RangeInclusive<usize>,
) -> Result<Vec<Block>, LearError> {
    let scene_index = TABLE_OF_CONTENTS
        .get(act - 1)
        .ok_or(LearError::InvalidAct(act))?
        .index(scene)
        .ok_or(LearError::InvalidScene { act, scene })?;
    let blocks: Vec<Block> = serde_json::from_str(SCENES[scene_index])?;
    let blocks: Vec<Block> = blocks
        // .get(lines.clone())
        .iter()
        .filter_map(|b| b.selection(&lines))
        .collect();
    if blocks.is_empty() {
        return Err(LearError::InvalidLines {
            act,
            scene,
            lines: lines.clone(),
        });
    }
    Ok(blocks)
}

fn attribution(blocks: &[Block]) -> Option<String> {
    let dialogue: Vec<_> = blocks
        .iter()
        .filter_map(|b| match b {
            Block::Heading(_) => None,
            Block::Dialogue(dialogue) => Some(dialogue),
        })
        .collect();
    let first = dialogue.first()?;
    let act = first.act;
    let scene = first.scene;
    let start = first.start;
    let stop = dialogue.last()?.stop;
    Some(format!(
        "({}Lr.{} {}.{}.{}-{})",
        style::Italic,
        style::Reset,
        act,
        scene,
        start,
        stop
    ))
}

pub fn display(blocks: &[Block]) {
    let attribution = attribution(blocks);
    for block in blocks {
        print!("{}", block);
    }
    if let Some(attr) = attribution {
        println!("{: >80}", attr);
    }
}
