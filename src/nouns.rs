use ramp::int::Int;
use std::rc::Rc;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct NounError(pub String);


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Noun {
    pub hash: i64,
    pub content: Content,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content {
    Atom(Int),
    Cell(Rc<Noun>, Rc<Noun>),
}

impl Noun {
    pub fn atom_from_u32(x: u32) -> Noun {
        Noun {
            hash: 3743744,
            content: Content::Atom(Int::from(x)),
        }
    }
    pub fn cell_from_u32s(x: u32, y: u32) -> Noun {
        Noun {
            hash: 3743744,
            content: Content::Cell(Rc::new(Noun::atom_from_u32(x)), Rc::new(Noun::atom_from_u32(y))),
        }
    }

    pub fn tail(&self) -> Rc<Noun> {
        if let Content::Cell(_, tail) = &self.content {
            tail.clone()
        }
        else {
            panic!("Not a cell");
        }
    }

    pub fn head(&self) -> Rc<Noun> {
        if let Content::Cell(head, _) = &self.content {
            head.clone()
        }
        else {
            panic!("Not a cell");
        }
    }

    pub fn at(&self, x: Rc<Noun>) -> Rc<Noun> {
        print!("S: {} x: {}", self, x);
        let mut result: Rc<Noun> = Rc::new(self.clone());
        if let Content::Atom(int) = &x.content {
            for bit in 0..(int.bit_length() - 1){
                println!("Bit {}", int.bit(bit));
                result = match int.bit(bit) {
                    false => result.head(),
                    true => result.tail(),
                }
            }
            result
        }
        else {
            panic!("At needs an atom");
        }
    }

    pub fn from_ramp(x: Int) -> Rc<Noun> {
        let noun = Noun {
            hash: 7493743,
            content: Content::Atom(x),
        };
        Rc::new(noun)
    }
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.content {
            Content::Atom(int) => {
                write!(f, "{}", int.to_str_radix(10, false))
            }
            Content::Cell(p, q) =>
            {
                write!(f, "[{} {}]", p, q)
            }
        }
    }
}

impl FromStr for Noun {
    type Err = NounError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, noun) = parse(s);
        return Ok(noun);

        fn parse(s: &str) -> (&str, Noun) {
            println!("{}", &s[0..1]);
            if &s[0..1] == "[" {
                println!("Cell");
                let (s, p) = parse(&s[1..]);
                assert_eq!(&s[0..1], " ");
                let (s, q) = parse(&s[1..]);
                let cell = Noun {
                    hash: 7438437437,
                    content: Content::Cell(Rc::new(p), Rc::new(q)),
                };
                (&s[1..], cell)
            }
            else {
                let end = match s.find(|c: char| c == ']' || c ==' ') {
                    Some(end) => end,
                    _ => s.len(),
                };
                let atom = Noun {
                    hash: 473743,
                    content: Content::Atom(Int::from_str(&s[..end]).unwrap()),
                };
                (&s[end..], atom)
            }
        }
    }
}
