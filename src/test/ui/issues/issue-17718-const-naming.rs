#![warn(unused)]
#![deny(warnings)]

const foo: isize = 3;
//~^ ERROR: should have an upper case name
//~^^ ERROR: constant is never used

fn main() {}
