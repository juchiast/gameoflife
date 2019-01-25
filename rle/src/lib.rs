use lazy_static::lazy_static;

// Parse errors
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Invalid(String),
    IntParse(std::num::ParseIntError),
    NoHeader,
}

// Read file's header, return dimensions of the pattern
fn parse_header<'a, I>(lines: &mut I) -> Result<(i32, i32), Error>
where
    I: Iterator<Item = &'a str>,
{
    lazy_static! {
        // regex to match "x = m, y = n", ignore "rule = ..."
        static ref HEADER: regex::Regex = regex::Regex::new(r"^\s*x\s*=\s*(\d+)\s*,\s*y\s*=\s*(\d+)").unwrap();
    }

    for line in lines {
        if line.starts_with('#') {
            // Other unsupported options, ignore
        } else if HEADER.is_match(line) {
            let cap = HEADER.captures_iter(line).next().unwrap();
            let x = cap[1].parse().map_err(Error::IntParse)?;
            let y = cap[2].parse().map_err(Error::IntParse)?;
            return Ok((x, y));
        } else {
            return Err(Error::Invalid(line.to_string()));
        }
    }

    Err(Error::NoHeader)
}

struct Parser {
    pub x: usize,
    pub y: usize,
    pub stop: bool,
    pub lives: Vec<(usize, usize)>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            x: 0,
            y: 0,
            stop: false,
            lives: Vec::new(),
        }
    }

    fn parse(&mut self, line: &str) -> Result<(), Error> {
        let mut cnt = 0;

        for ch in line.chars() {
            if ch == '!' {
                self.stop = true;
                break;
            } else if ch == '$' {
                if cnt == 0 {
                    cnt = 1;
                }
                self.y += cnt;
                self.x = 0;
                cnt = 0;
            } else if ch == 'b' || ch == 'o' {
                if cnt == 0 {
                    cnt = 1;
                }
                if ch == 'o' {
                    let y = self.y;
                    self.lives.extend((self.x..self.x + cnt).map(|x| (x, y)));
                }
                self.x += cnt;
                cnt = 0;
            } else if let Some(d) = ch.to_digit(10) {
                cnt = cnt * 10 + d as usize;
            }
            // ignore all other characters, this behavior needs to be reconsidered.
        }

        Ok(())
    }
}

pub fn read_file<P: AsRef<std::path::Path>>(p: P) -> Result<gol::Map, Error> {
    let input = {
        use std::io::Read;
        let mut file = std::fs::File::open(p).map_err(Error::IO)?;
        let mut s = String::new();
        file.read_to_string(&mut s).map_err(Error::IO)?;
        s
    };
    let mut lines = input.lines();

    let (m, n) = parse_header(&mut lines)?;
    let (offset_x, offset_y) = (m / 2, n / 2);

    let mut p = Parser::new();
    for line in lines {
        p.parse(line)?;
        if p.stop {
            break;
        }
    }

    let lives = p
        .lives
        .into_iter()
        .map(|(x, y)| gol::pos(x as i32 - offset_x, y as i32 - offset_y))
        .collect::<Vec<_>>();
    Ok(gol::Map::from_alives_list(lives))
}
