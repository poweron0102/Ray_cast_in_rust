// Hello, this is the Powers Script file
// the operation will be executed in this way:
//
// Operation_code , var1_or_number , ver2_or_number , result_var;
//
//
//
//
//

use std::collections::HashMap;
use std::ops::Deref;
use crate::in_game::In_game;
use crate::pwss::Var_type::*;

enum Var_type<'a> {
    S0vi(&'a i32),   // 0vi
    S0gi(&'a i32),   // 0gi

    S0vf(&'a f32),   // 0vf
    S0gf(&'a f32),   // 0gf

    S0vb(&'a bool),  // 0vb
    S0gb(&'a bool),  // 0gb

    S0vs(&'a str),   // 0vs
    S0gs(&'a str),   // 0gs

    M0vi(&'a mut i32),   // 0vi
    M0gi(&'a mut i32),   // 0gi

    M0vf(&'a mut f32),   // 0vf
    M0gf(&'a mut f32),   // 0gf

    M0vb(&'a mut bool),  // 0vb
    M0gb(&'a mut bool),  // 0gb

    M0vs(&'a mut str),   // 0vs
    M0gs(&'a mut str),   // 0gs
}
impl<'a> Var_type<'static> {
    fn get_ref_0V(vars: &str, in_game:&'a mut In_game) -> Var_type<'a> {
        match &vars[0..0] {
            "i" => S0vi(in_game.pwss.var_i32.get(vars).unwrap()),
            "f" => S0vf(in_game.pwss.var_f32.get(vars).unwrap()),
            "b" => S0vb(in_game.pwss.var_bool.get(vars).unwrap()),
            "s" => S0vs(in_game.pwss.var_str.get(vars).unwrap()),
            _   => panic!("There is no {} type in Powers Script", &vars[0..0])
        }
    }
    fn get_ref_0Vmut(vars: &str, in_game:&'a mut In_game) -> Var_type<'a> {
        match &vars[0..0] {
            "i" => M0vi(in_game.pwss.var_i32.get_mut(vars).unwrap()),
            "f" => M0vf(in_game.pwss.var_f32.get_mut(vars).unwrap()),
            "b" => M0vb(in_game.pwss.var_bool.get_mut(vars).unwrap()),
            "s" => M0vs(in_game.pwss.var_str.get_mut(vars).unwrap()),
            _   => panic!("There is no {} type in Powers Script", &vars[0..0])
        }
    }

    fn type_id(&self) -> usize {
        match self {
            S0vi(_) => {1}
            S0gi(_) => {1}
            S0vf(_) => {2}
            S0gf(_) => {2}
            S0vb(_) => {3}
            S0gb(_) => {3}
            S0vs(_) => {4}
            S0gs(_) => {4}
            M0vi(_) => {1}
            M0gi(_) => {1}
            M0vf(_) => {2}
            M0gf(_) => {2}
            M0vb(_) => {3}
            M0gb(_) => {3}
            M0vs(_) => {4}
            M0gs(_) => {4}
        }
    }
}

fn get_vars(vars_needed: usize, vars: Vec<&str>, in_game:&mut In_game, pos: (usize, usize)) -> (Var_type<'static>, Var_type<'static>, Var_type<'static>) {
    if vars.len() != vars_needed { panic!("Tile in position: {:?} as bean gived the wrong number of agrs", pos) }
    let  var1t = vars[0];
    let  var2t = vars[1];
    let  var3t = vars[2];
    let v1; let v2; let v3;

    //match &var1t[..1] {
    //    "0v" => { v1 = Var_type::get_ref_0V(&var1t[1..], in_game) }
    //    _ => todo!()
    //}

    if &var1t[..1] == "0v" { v1 = Var_type::get_ref_0V(&var1t[1..], in_game) }
    if &var2t[..1] == "0v" { v2 = Var_type::get_ref_0V(&var2t[1..], in_game) }
    if &var3t[..1] == "0v" { v3 = Var_type::get_ref_0Vmut(&var3t[1..], in_game) }

    if v1.type_id() == v2.type_id() && v2.type_id() == v3.type_id() {
        (v1, v2, v3)
    }else { panic!() }
}

fn add(vars: Vec<&str>, in_game:&mut In_game, pos: (usize, usize)) {

}
fn set(vars: Vec<&str>, in_game:&mut In_game, pos: (usize, usize)) {
    todo!()
}

#[derive(Clone)]
pub struct PwsS<'a> {
    operation_code: HashMap<&'a str, fn(Vec<&str>, &mut In_game, pos: (usize, usize))>,
    var_i32: HashMap<String, i32>,
    var_f32: HashMap<String, f32>,
    var_bool: HashMap<String, bool>,
    var_str: HashMap<String, String>,
}
impl PwsS<'_> {
    pub fn new() -> PwsS<'static> {
        let operation_code = HashMap::from([
            ("add", add as fn(Vec<&str>, &mut In_game, (usize, usize))),
            ("set", set as fn(Vec<&str>, &mut In_game, (usize, usize))),
        ]);
        PwsS {
            operation_code,
            var_i32:  HashMap::new(),
            var_f32:  HashMap::new(),
            var_bool: HashMap::new(),
            var_str:  HashMap::new(),
        }
    }


}

