pub fn slurp_stdin() -> String {
    use std::{io, io::prelude::*};
    let mut ret = String::new();
    io::stdin().read_to_string(&mut ret).unwrap();
    ret
}
