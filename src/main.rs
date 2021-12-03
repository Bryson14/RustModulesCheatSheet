//!
//! To import `.rs` files that are in the same level as `main.rs` into `main.rs`,
//! just use `mod` and `use` keywords to bring everything into scope.
//!  Don't forget to make the imported modules and functions public!
//!

mod lib;
use lib::lib_function;
use lib::public_library;

mod module_a;
mod module_b;
use module_a::{helper1::helper1_function, mod_a_function};
use module_b::{helper2, helper3, mod_b_function};

fn main() {
    println!("Hello, world from main!");
    lib_function();
    public_library::public_library_mod_function();

    helper1_function();
    mod_a_function();

    helper2::helper2_function();
    helper3::helper3_function();
    mod_b_function();
}
