#![allow(unused)]

extern crate lib1;       // crate option 1
use lib1::foo;           // use option 1
use crate::lib1::bar;    // use option 2

extern crate lib2 as lib; // crate option 2
// use lib::{foo, bar};   // foo, bar exist in both libs. Using already imported names is compiler error.
use crate::lib::{*};      // Using * does not result in compiler error. The already imported names remain.

/// crates have namespaces as modules
/// use option 1 omits the crate prefix
/// use option 2 allows explicitly says that names are from crate
/// crate can be aliased with as 

fn main() {
    foo();  
    bar();
    foobar();
    lib::foo();
    lib::bar();
}
