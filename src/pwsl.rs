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

fn add(vars: &str, in_game:&mut In_game) {
    //let (var1, var2) = vars.split(",");
}
fn set(vars: &str, in_game:&mut In_game) {
    todo!()
}

pub struct PwsL<'a> {
    operation_code: HashMap<&'a str, fn(&str, &mut In_game)>,
    var_i32: HashMap<String, i32>,
}
impl PwsL<'_> {
    pub fn new() -> PwsL<'static> {
        let operation_code = HashMap::from([
            ("add", add as fn(&str, &mut In_game)),
            ("set", set as fn(&str, &mut In_game)),
        ]);
        PwsL{
            operation_code,
            var_i32: HashMap::new(),
        }
    }


}

