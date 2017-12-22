use std;
use std::io;
use std::path::Path;
use std::fs::File;
use map;
use map::Map;
use regex::Regex;

// Parse errors
#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Invalid(String),
    IntParse(std::num::ParseIntError),
    NoHeader,
}

// Read file's header, return dimensions of the pattern
fn parse_header<'a, I>(lines: &mut I) -> Result<(usize, usize), Error>
    where I: Iterator<Item=&'a str>
{
    lazy_static! {
        // regex to match "x = m, y = n", ignore "rule = ..."
        static ref HEADER: Regex = Regex::new(r"^\s*x\s*=\s*(\d+)\s*,\s*y\s*=\s*(\d+)").unwrap();
    }

    for line in lines {
        if line.starts_with("#") {
            // Other unsupported options, ignore
        } else if HEADER.is_match(line) {
            let cap = HEADER.captures_iter(line).next().unwrap();
            let x = cap[1].parse().map_err(Error::IntParse)?;
            let y = cap[2].parse().map_err(Error::IntParse)?;
            return Ok((x, y));
        } else {
            return Err(Error::Invalid(line.to_string()));
        }
    };

    Err(Error::NoHeader)
}

struct Parse {
    x: usize,
    y: usize,
    stop: bool,
    live: Vec<(usize, usize)>,
}
fn parse(line: &str, mut x: usize, mut y: usize) -> Result<Parse, Error> {
    let mut vec = Vec::new();
    let mut stop = false;
    let mut cnt = 0;

    for ch in line.chars() {
        if ch == '!' {
            stop = true;
            break;
        } else if ch == '$' {
            if cnt == 0 {
                cnt = 1;
            }
            y += cnt;
            x = 0;
            cnt = 0;
        } else if ch == 'b' || ch == 'o' {
            if cnt == 0 {
                cnt = 1;
            }
            if ch == 'o' {
                vec.extend((x..x+cnt).map(|x| (x, y)));
            }
            x += cnt;
            cnt = 0;
        } else if let Some(d) = ch.to_digit(10) {
            cnt = cnt * 10 + d as usize;
        }
        // ignore all other characters, this behavior needs to be reconsidered.
    }

    Ok(Parse {
        x: x,
        y: y,
        stop: stop,
        live: vec,
    })
}

pub fn read_file<P: AsRef<Path>>(p: P) -> Result<Map, Error> {
    let input = {
        use std::io::Read;
        let mut file = File::open(p).map_err(Error::IO)?;
        let mut s = String::new();
        file.read_to_string(&mut s).map_err(Error::IO)?;
        s
    };

    let mut lines = input.lines();
    parse_header(&mut lines)?;
    let mut map = Map::new();
    let mut x = 0;
    let mut y = 0;
    for line in lines {
        let res = parse(line, x, y)?;
        x = res.x;
        y = res.y;
        for (x, y) in res.live {
            map.set_cell_alive(&map::pos(x as i32, y as i32));
        }
        if res.stop {
            break;
        }
    }
    Ok(map)
}
