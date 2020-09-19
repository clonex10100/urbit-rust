pub use serialization::{Jam, cue, rub};
pub use nouns::{Noun, Content};
pub use nock::nock;
pub use ramp::int::Int;
pub mod nouns;
pub mod serialization;
pub mod nock;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::rc::Rc;

fn main() {
    /*if let Ok(stream) = read_pill() {
        let jam = Jam {
            pos: 0,
            stream,
        };
        let result = cue(jam);
        println!("Parsed");
        let stri = result.str_format();
        println!("Stringed");
        println!("{}", stri);
    }  else {
        println!("Failed to read");
    }*/
}
fn read_pill() -> io::Result<Vec<u8>> {
    let mut file = File::open("./ivory.pill")?;
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    /*for _ in 0..10 {
        if let Some(x) = buffer.pop() {
            println!("{}", x);
        }
    }*/
    println!("{:?}", buffer);
    Ok(buffer)
}


#[cfg(test)]
mod tests {
    use crate::nouns::{Noun, Content};
    use crate::nock::{nock, NockError};
    use std::rc::Rc;
    use ramp::int::Int;
    fn test_nock(subject: &str, formula: &str, output: &str) {
        let subject = subject.parse::<Noun>().unwrap();
        let formula = formula.parse::<Noun>().unwrap();
        //let res = nock(Rc::new(subject), Rc::new(formula))?;
        let res = nock(Rc::new(subject), Rc::new(formula));
        assert_eq!(format!("{}", res.unwrap()), output);
    }
    #[test]
    fn test_noun() {
        let b = Noun {
            hash: 743927843,
            content: Content::Atom(Int::one()),
        };
        let c = Noun {
            hash: 743927843,
            content: Content::Atom(Int::one() + 574),
        };
        let d = Noun {
            hash: 7438473,
            content: Content::Cell(Rc::new(b), Rc::new(c)),
        };
        assert_eq!(format!("{}", d), "[1 575]");
    }

    #[test]
    fn test_parse() {
        let noun = "[5 10]".parse::<Noun>().unwrap();
        let noun2 = Noun::cell_from_u32s(5, 10);
        assert_eq!(format!("{}", noun), format!("{}", noun2));

        let noun = "510".parse::<Noun>().unwrap();
        let noun2 = Noun::atom_from_u32(510);
        assert_eq!(format!("{}", noun), format!("{}", noun2));
    }

    #[test]
    fn test_nock_zero() {
        test_nock("[1 10]", "[0 2]", "1");

        test_nock("[[3723737 37273] [[4743 4734] 7474]]", "[0 13]", "4734");
    }

    #[test]
    fn test_nock_one() {
        test_nock("10", "[1 10]", "10");

        test_nock("10", "[1 [10 20]]", "[10 20]");

    }

    #[test]
    fn test_nock_two() {
        test_nock("10", "[2 [[0 1] [1 [0 1]]]]", "10");
    }

    #[test]
    fn test_nock_three() {
        test_nock( "[0 1]", "[3 [0 1]]", "0");

        test_nock("1", "[3 [0 1]]", "1");
    }
    #[test]
    fn test_nock_five() {
        test_nock("[0 [1 2]]", "[5 [[1 [1 2]] [0 3]]]", "0");
    }
    #[test]
    fn test_nock_six() {
        test_nock("[1 0]", "[6 [[1 1] [[0 2] [4 [4 [4 [1 1]]]]]]]", "4");
    }
    #[test]
    fn test_nock_seven() {
        test_nock("[1 0]", "[7 [[0 1] [4 [4 [0 3]]]]]", "2");
    }
    #[test]
    fn test_nock_eight() {
        test_nock("[1 0]", "[8 [[4 [4 [4 [0 2]]]] [0 2]]]", "4");
    }

}
