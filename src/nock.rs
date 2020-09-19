/*Nock 4K

A noun is an atom or a cell.  An atom is a natural number.  A cell is an ordered pair of nouns.

Reduce by the first matching pattern; variables match any noun.

nock(a)             *a
[a b c]             [a [b c]]

?[a b]              0
?a                  1
+[a b]              +[a b]
+a                  1 + a
=[a a]              0
=[a b]              1

/[1 a]              a
/[2 a b]            a
/[3 a b]            b
/[(a + a) b]        /[2 /[a b]]
/[(a + a + 1) b]    /[3 /[a b]]
/a                  /a

#[1 a b]            a
#[(a + a) b c]      #[a [b /[(a + a + 1) c]] c]
#[(a + a + 1) b c]  #[a [/[(a + a) c] b] c]
#a                  #a

*[a [b c] d]        [*[a b c] *[a d]]

*[a 0 b]            /[b a]
*[a 1 b]            b
*[a 2 b c]          *[*[a b] *[a c]]
*[a 3 b]            ?*[a b]
*[a 4 b]            +*[a b]
*[a 5 b c]          =[*[a b] *[a c]]

*[a 6 b c d]        *[a *[[c d] 0 *[[2 3] 0 *[a 4 4 b]]]]
*[a 7 b c]          *[*[a b] c]
*[a 8 b c]          *[[*[a b] a] c]
*[a 9 b c]          *[*[a c] 2 [0 1] 0 b]
*[a 10 [b c] d]     #[b *[a c] *[a d]]

*[a 11 [b c] d]     *[[*[a c] *[a d]] 0 3]
*[a 11 b c]         *[a c]

*a                  *a*/
use crate::nouns::{Noun, Content};
use std::rc::Rc;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NockError {
    msg: String,
}

impl fmt::Display for NockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for NockError {
}

pub fn nock(subject: Rc<Noun>, formula: Rc<Noun>) -> Result<Rc<Noun>, NockError> {
    if let Content::Cell(op, var) = &formula.content {
        if let Content::Atom(op) = &op.content {
            match op.to_f64() as usize {
                0 => Ok(subject.at(Rc::clone(var))),
                1 => Ok(Rc::clone(var)),
                2 => {
                    if let Content::Cell(b, c) = &var.content {
                        let formula = nock(Rc::clone(&subject), Rc::clone(c))?;
                        let subject = nock(subject, Rc::clone(b))?;
                        nock(subject, formula)
                    }
                    else {
                        Err(NockError{
                            msg: "2".to_string(),
                        })
                    }
                }
                3 => {
                    match nock(subject, Rc::clone(var))?.content {
                        Content::Cell(_, _) => Ok(Rc::new(Noun::atom_from_u32(0))),
                        _ =>  Ok(Rc::new(Noun::atom_from_u32(1))),

                    }
                }
                4 => {
                    let result = nock(subject, Rc::clone(var))? ;
                    if let Content::Atom(int) = &result.content {
                        Ok(Noun::from_ramp(int + 1))
                        
                    } else {
                        Err(NockError{
                            msg: "4".to_string(),
                        })
                    }
                }
                5 => {
                    if let Content::Cell(a, b) = &var.content {
                        let a = nock(Rc::clone(&subject), Rc::clone(a))?;
                        let b = nock(subject, Rc::clone(b))?;
                        if a == b {
                            Ok(Rc::new(Noun::atom_from_u32(0)))
                        } else {
                            Ok(Rc::new(Noun::atom_from_u32(1)))
                        }
                    }
                    else {
                        Err(NockError{
                            msg: "5".to_string(),
                        })
                    }
                }
                6 => {
                    if let Content::Cell(a, b) = &var.content {
                        let a = nock(Rc::clone(&subject), Rc::clone(a))?;
                        if let Content::Atom(a) = &a.content {
                            if let Content::Cell(c, d) = &b.content{
                                return match a.to_f64() as usize {
                                    0 => nock(subject, Rc::clone(c)),
                                    1 => nock(subject, Rc::clone(d)),
                                    _ =>  Err(NockError{msg: "5".to_string(),}),
                                };
                            } 
                        }
                    }
                    Err(NockError{
                        msg: "6".to_string(),
                    })
                }
                7 => {
                    if let Content::Cell(a, b) = &var.content {
                        nock(nock(subject, Rc::clone(a))?, Rc::clone(b))
                    }
                    else {
                        Err(NockError{
                            msg: "7".to_string(),
                        })
                    }
                }
//*[a 8 b c]          *[[*[a b] a] c]
//*[a 9 b c]          *[*[a c] 2 [0 1] 0 b]
//*[a 10 [b c] d]     #[b *[a c] *[a d]]

//*[a 11 [b c] d]     *[[*[a c] *[a d]] 0 3]
//*[a 11 b c]         *[a c]
                8 => {
                    if let Content::Cell(a, b) = &var.content {
                        let noun = Noun {
                            hash: 472842,
                            content: Content::Cell(nock(Rc::clone(&subject), Rc::clone(a))?, subject),
                        };
                        nock(Rc::new(noun), Rc::clone(b))
                    }
                    else {
                        Err(NockError{
                            msg: "8".to_string(),
                        })
                    }
                }
                _ => Err(NockError {
                        msg: "Invalid Operator".to_string()
                    })
            }
        }
        else {
            Err(NockError{
                msg: "Opcode must be an atom".to_string(),
            })
        }
    }
    else {
        Err(NockError{
            msg: "Not enough Arguments".to_string(),
        })
    }
}
