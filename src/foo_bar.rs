use crate::bar::say_hello;
use crate::foo::say_hello as say_hello_foo;

pub fn foo_bar(name: &str) {
    say_hello(name);
    say_hello_foo(name);
    println!("from foo_bar.rs")
}


pub mod foo_bar_v2_mdl {
    pub mod bar {
        pub fn hello(name: &str) {
            super::super::foo_bar(name);
        }
    }
}


pub mod foo_bar_v3_mdl {
    pub mod foo {
        pub fn hello(name: &str) {
            crate::foo_bar::foo_bar(name);
        }
    }
}