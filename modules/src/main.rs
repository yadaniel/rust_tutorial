#![allow(unused)]

// crate: start name resolution from namespace level 0 (root)
// super: start name resoultion from namespace level self-1 (previous)
// self: alias for current namespace level
// use: bring names into current namespace

///
///
///
mod m1 {
    pub fn foo() {
        println!("m1");
    }

    pub mod m2 {
        pub fn foo() {
            println!("m1::m2");
        }
    }
}

fn use_foo() {
    m1::foo(); // foo must be pub, m1 is sibling to this function (no pub on m1)
    m1::m2::foo(); // m2 and foo must be pub
}

///
///
///
mod n1 {
    fn bar() {
        println!("n1");
    }

    mod n2 {
        pub fn bar() {
            println!("n1::n2");
        }
    }

    pub fn use_bar() {
        bar(); // bar is sibling to this function (no pub on bar)
        n2::bar(); // n2 is sibling to this function (no pub on n2), bar must be pub
    }
}

///
///
///
mod q1 {
    fn foobar() {
        println!("q1");
    }

    mod q2 {
        pub fn foobar() {
            println!("q1::q2");
        }
    }

    pub fn use_foobar() {
        foobar();
        q2::foobar();
        use_foobar_local(); // note: function called before local definition

        // local
        fn use_foobar_local() {
            foobar();
            q2::foobar();
        }
    }
}

///
///
///

pub mod x1 {
    pub const C: u32 = 1u32;

    pub mod x2 {
        pub const C: u32 = 2u32;

        pub mod x3 {
            pub const C: u32 = 3u32;

            pub fn print_c() {
                println!("{}", C); // 3
                println!("{}", self::C); // 3
                println!("{}", crate::x1::C); // 1
                println!("{}", crate::x1::x2::C); // 2
                println!("{}", crate::x1::x2::x3::C); // 3

                // and both directions
                // println!("{}", crate::x1::x2::x3::super::super::C); // super can only be used in start position (compile eror)
            }
        }
    }
}

pub mod y1 {
    pub const C: u32 = 10u32;

    pub mod y2 {
        pub const C: u32 = 20u32;

        pub mod y3 {
            pub const C: u32 = 30u32;

            pub fn print_c() {
                // from inner to outer
                println!("{}", C); // 30
                println!("{}", self::C); // 30
                println!("{}", super::C); // 20
                println!("{}", super::super::C); // 10

                // and backwards
                println!("{}", super::super::super::y1::C); // 10
                println!("{}", super::super::super::y1::y2::C); // 20
                println!("{}", super::super::super::y1::y2::y3::C); // 30

                // and backwards on other module
                println!("{}", super::super::super::x1::C); // 1
                println!("{}", super::super::super::x1::x2::C); // 2
                println!("{}", super::super::super::x1::x2::x3::C); // 3
            }
        }
    }
}

///
///
///

pub fn func() {
    println!("func");

    pub fn func_local() {
        println!("func_local");
    }
}

///
///
///
mod fmod_dir; // fmod_dir/mod.rs
mod fmod_file; // fmod_file.rs

///
///
///
fn test_use1() {
    use crate::*;
    use x1::x2::x3::*;
    println!("{}", C); // 3
    {
        use x1::x2::*;
        println!("{}", C); // 2
        {
            use x1::*;
            println!("{}", C); // 1
        }
    }
}

///
///
///
fn test_use2() {
    use crate::*;
    use x1::x2::x3::*;
    // println!("{}", C); // 3
    use x1::x2::*;
    // println!("{}", C); // 2
    use x1::*;
    // println!("{}", C); // 1

    // compile error when C is used, because C is ambigious
    // C can only be used when the namespace path is given
    println!("{}", x1::C);
    println!("{}", x1::x2::C);
    println!("{}", x1::x2::x3::C);
}

///
///
///
fn main() {
    use_foo();
    n1::use_bar();
    q1::use_foobar();

    x1::x2::x3::print_c();
    y1::y2::y3::print_c();

    func();
    // func::func_local(); // func is not module (does not compile)

    fmod_dir::explain();
    fmod_file::explain();

    test_use1();
    test_use2();
}
