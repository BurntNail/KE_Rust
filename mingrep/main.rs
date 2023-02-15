#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod bitvec;

use crate::bitvec::ExpandableBitVec;
use color_eyre::eyre::bail;
use owo_colors::OwoColorize;
use regex::Regex;
use std::{env, fs::read_to_string};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut args = env::args().skip(1); //skip 1 to remove name of program
    let Some(file) = args.next() else {
        bail!("You need to pass in a file as the first argument.");
    };
    let contents = read_to_string(file)?;

    let Some(pattern) = args.next() else {
        bail!("You need to pass in a pattern to look for as the second argument.");
    };
    let re = Regex::new(&pattern)?;

    let mut bv = ExpandableBitVec::new(256);
    for line in contents.lines() {
        // for m in get_matches_in_line(line, &pattern, pat_len, 0) {
        //     for index in m..m + pat_len {
        //         bv.set(index);
        //     }
        // }
        for m in re.find_iter(line) {
            for index in m.range() {
                bv.set(index);
            }
        }

        for (col, char) in line.chars().enumerate() {
            if bv.index(col) {
                print!("{}", char.green());
            } else {
                print!("{char}");
            }
        }

        bv.clear();
        println!();
    }

    Ok(())
}
