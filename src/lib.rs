use std::io::BufRead;
use anyhow::Result;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

pub fn read_lines(reader: impl BufRead) -> Vec<String> {
    let mut ret = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        ret.push(line);
    }

    ret
}

pub fn read_and_transform<T>(reader: impl BufRead, transformer: fn(&str) -> Result<T>) -> Result<Vec<T>> {
    let mut ret = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let value = transformer(&line)?;
        ret.push(value);
    }
    
    Ok(ret)
}

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
