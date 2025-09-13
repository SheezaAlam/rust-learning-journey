use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let data = fs::read_to_string("hello.txt")?;
    println!("File contents: {}", data);
    Ok(())
}
//adding extra info - context


use anyhow::{Context, Ok, Result};
use std::fs;
fn main() -> Result<()>{
    let data= fs::read_to_string("hi.txt")
       .context("failed to read")?;
    println!("contents:{}",data);
    Ok(())
}
