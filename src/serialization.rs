use crate::nouns::{ Noun, content };
use ramp::int::Int;
use std::rc::Rc;
use std::collections::HashMap;
pub struct Jam {
    pub pos: usize,
    pub stream: Vec<u8>,
}
pub fn cue(jam: Jam) -> Noun {
    let mut map: HashMap<usize, Noun> = HashMap::new();
    let (_, noun) = cuer(jam, &mut map);
    noun
}
pub fn cuer(mut jam: Jam, map: &mut HashMap<usize, Noun>) -> (Jam, Noun) { //Add hashmap backreferencing
   let start = jam.pos.clone();
   ////println!("Og {}", start);
   if jam.bit() {
       jam.advance();
       if jam.bit() {
           jam.advance();
           //Backref
           println!("Backref");
           let (jam, atom) = rub(jam);
           let b_ref;
           if let content::Atom(int) = atom.content {
               //println!("Backref Res: {}", (int.to_f64() as usize));
               let ref_opt = map.get(&(int.to_f64() as usize));
               if let Some(bref) = ref_opt {
                   b_ref = bref.clone()
               }
               else {
                   panic!("Backref Failed (not found)")
               }

               }
           else {
               panic!("Backref failed (Wut?)")
           }
           //jam.advance();
           //jam.advance();
           (jam, b_ref)
       }
       else
       {
           jam.advance();
           //cell
           println!("Cell");
           let (jam, p) = cuer(jam, map);
           let (jam, q) = cuer(jam, map);
           let cell = Noun {
               hash: 127237123,
               content: content::Cell(Rc::new(p), Rc::new(q)),
           };
           //println!("Cell Res: {}", cell.str_format());
           map.insert(start, cell.clone());
           (jam, cell)
       }
   }
   else {
       //Atom
       println!("Atom");
       println!("Pos: {}", jam.pos /8);
       jam.advance();
       let (jam, atom) = rub(jam);
       map.insert(start, atom.clone());
       //println!("Atom Res{}", atom.str_format());
       (jam, atom)
   }
}
pub fn rub(mut jam: Jam) -> (Jam, Noun) {

   let mut leading_zeroes = 0;
   //let mut res: u128 = 0;
   let mut noun = Noun {
       hash: 12321641,
       content: content::Atom(Int::zero()),
   };
   //Atom
   ////println!("Atom");
   //Count num zeroes
   while !jam.bit() {
       leading_zeroes += 1;
       println!("{}", leading_zeroes);
       jam.advance();
   }

   jam.advance(); //skip 1 seperator
   ////println!("Zeroes {}", leading_zeroes);
   if leading_zeroes == 0 {
       return (jam, Noun::atom_from_u32(0));
   }

   //read z
   let x = (2 as usize).pow(leading_zeroes - 1);
   println!("Bex: {}", x);
   let mut z = 0;
   for i in 0..(leading_zeroes - 1) {
        if jam.bit() {
            z += 1 << i;
        }
        jam.advance();
   }
   ////println!("X: {} Z: {}", x, z);
   for i in 0..(x + z) {
        if jam.bit() {
            if let content::Atom(int) = noun.content {
                ////println!("I: {}", i);
                noun.content = content::Atom(int + (Int::one() << i));
            }
        }
        jam.advance()
   }
   (jam, noun)
}
impl Jam {
    fn bit(&self) -> bool {
        bit(&self.stream, self.pos)
    }
    fn advance(&mut self) {
        self.pos += 1;
    }
}
fn bit(stream: &Vec<u8>, pos: usize) -> bool {
    stream[pos / 8] & (1 << (pos % 8)) != 0 //Access bit pos of stream, little endian
}
