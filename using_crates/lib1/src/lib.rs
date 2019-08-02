pub fn foo() {
    println!("foo from lib1");
}

pub fn bar() {
    println!("bar from lib1");
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
