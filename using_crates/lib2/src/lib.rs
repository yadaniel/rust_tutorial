pub fn foo() {
    println!("foo from lib2");
}

pub fn bar() {
    println!("bar from lib2");
}

pub fn foobar() {
    println!("foobar from lib2");
}

pub struct Foobar {
    pub name: &'static str,
    pub cnt: u8,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
