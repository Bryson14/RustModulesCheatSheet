pub mod helper1;

use crate::module_b;

pub fn mod_a_function() {
    println!("Printing from module_a/mod.rs");
    println!("Getting mod b function too");
    module_b::mod_b_function();
}
