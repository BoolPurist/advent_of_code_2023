use std::{io, path::PathBuf, str::FromStr};

use clap::{Args, Parser};
#[derive(Debug, Args, Clone)]
pub struct Input {
    pub content: String,
}

impl FromStr for Input {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        if path.exists() {
            let string = std::fs::read_to_string(&path)?;
            Ok(Self { content: string })
        } else {
            Ok(Self {
                content: String::from(s),
            })
        }
    }
}

#[derive(Debug, Parser)]
pub struct TaskOverCli {
    pub input: Input,
    pub day: usize,
    pub task: usize,
}
