extern crate nom;


use crust::parse::test;

fn main() {
    test("- -1 + + 1", - -1 + 1);  // rust does not allow + as a unary op (I do ;)
    test("(-1-1)+(-1+3)", (-1 - 1) + (-1) + 3);
    // just to check that right associative works (you don't need to implement pow)
    test("2+3**2**3*5+1", 2 + 3i32.pow(2u32.pow(3)) * 5 + 1);
    test("(12*2)/3-4", (12 * 2) / 3 - 4);
    test("1*2+3", 1 * 2 + 3);
    // just to check that we get a parse error
    test("1*2+3+3*21-a12+2", 1 * 2 + 3 + 3 * 21 - 12 + 2);
}


