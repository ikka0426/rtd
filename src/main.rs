#![allow(unused)]

use std::fs;
use anyhow::Result;
use crate::cli::Cli;

pub mod cli;
pub mod todos;

fn main() -> Result<()> {
  let cli = Cli::new();
  cli.run()
}