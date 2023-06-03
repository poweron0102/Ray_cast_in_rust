// Hello, this is the Powers Lang file
// the operation will be executed in this way:
//
// Operation_code , var1_or_number , ver2_or_number , result_var;
//
//
//
//
//

use std::collections::HashMap;
use crate::in_game::In_game;

enum Var_type {
    t0i: i32,
    t0g:
}

fn add(vars: Vec<&str>, in_game:&mut In_game, pos: (usize, usize)) {
    if vars.len() != 3 { panic!("Tile in position: {:?} as bean gived the wrong number of agrs", pos) }
    let mut  var1 = vars[0];
    let mut var2 = vars[1];
    let mut result = vars[2];
    if &var1[..1] == "0i" {  }

}
fn set(vars: Vec<&str>, in_game:&mut In_game, pos: (usize, usize)) {
    todo!()
}

#[derive(Clone)]
pub struct PwsL<'a> {
    operation_code: HashMap<&'a str, fn(Vec<&str>, &mut In_game, pos: (usize, usize))>,
    var_i32: HashMap<String, i32>,
}
impl PwsL<'_> {
    pub fn new() -> PwsL<'static> {
        let operation_code = HashMap::from([
            ("add", add as fn(Vec<&str>, &mut In_game, (usize, usize))),
            ("set", set as fn(Vec<&str>, &mut In_game, (usize, usize))),
        ]);
        PwsL{
            operation_code,
            var_i32: HashMap::new(),
        }
    }


}

