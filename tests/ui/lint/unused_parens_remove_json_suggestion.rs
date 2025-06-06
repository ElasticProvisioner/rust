//@ compile-flags: --error-format json
//@ run-rustfix

// The output for humans should just highlight the whole span without showing
// the suggested replacement, but we also want to test that suggested
// replacement only removes one set of parentheses, rather than naïvely
// stripping away any starting or ending parenthesis characters—hence this
// test of the JSON error format.

#![deny(unused_parens)]
#![allow(unreachable_code, unused_braces)]

fn main() {

    let _b = false;

    if (_b) {
    //~^ ERROR unnecessary parentheses around `if` condition
        println!("hello");
    }

    f();

}

fn f() -> bool {
    let c = false;

    if(c) {
     //~^ ERROR unnecessary parentheses around `if` condition
        println!("next");
    }

    if (c){
     //~^ ERROR unnecessary parentheses around `if` condition
        println!("prev");
    }

    while (false && true){
    //~^ ERROR unnecessary parentheses around `while` condition
        if (c) {
        //~^ ERROR unnecessary parentheses around `if` condition
            println!("norm");
        }

    }

    while(true && false) {
    //~^ ERROR unnecessary parentheses around `while` condition
        for _ in (0 .. 3){
        //~^ ERROR unnecessary parentheses around `for` iterator expression
            println!("e~")
        }
    }

    for _ in (0 .. 3) {
    //~^ ERROR unnecessary parentheses around `for` iterator expression
        while (true && false) {
        //~^ ERROR unnecessary parentheses around `while` condition
            println!("e~")
        }
    }


    loop {
        if (break { return true }) {
        }
    }
    false
}
