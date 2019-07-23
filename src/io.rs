use std::fs;
use std::io::Result;

pub fn write_output(filename: &str, output: &str) -> Result<()>{
    fs::write(filename, output)?;
    Ok(())
}