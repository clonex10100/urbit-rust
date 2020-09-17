use ramp::int::Int;
use std::rc::Rc;

#[derive(Clone)]
pub struct Noun {
    pub hash: i64,
    pub content: content,
}

#[derive(Clone)]
pub enum content {
    Atom(Int),
    Cell(Rc<Noun>, Rc<Noun>),
}

impl Noun {
    pub fn str_format(&self) -> String {
        match &self.content {
            content::Atom(int) => {
                int.to_str_radix(10, false)
            }
            content::Cell(p, q) =>
            {
                let mut string = String::new();
                string = format!("[{} {}]", p.str_format(), q.str_format());
                string
            }
        }
    }
    pub fn atom_from_u32(x: u32) -> Noun {
        Noun {
            hash: 3743744,
            content: content::Atom(Int::from(x)),
        }
    }
    pub fn cell_from_u32s(x: u32, y: u32) -> Noun {
        Noun {
            hash: 3743744,
            content: content::Cell(Rc::new(Noun::atom_from_u32(x)), Rc::new(Noun::atom_from_u32(y))),
        }
    }
}
