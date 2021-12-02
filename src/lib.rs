//! Remember that `fn lib_function()` can't be used outside of this file or module.
//! But `pub fn lib_function()` can!

pub fn lib_function() {
    println!("Here in lib.rs");
}

// remember the `pub` on the mod and function to be used in main.rs.
pub mod public_library {

    pub fn public_library_mod_function() {
        println!("Here in the public module `public_library` of lib.rs");
    }
}
