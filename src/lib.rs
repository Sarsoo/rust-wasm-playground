mod utils;

use wasm_bindgen::prelude::*;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! expr_stringer {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!("{:?} = {:?}", stringify!($expression), $expression)
    };
    ($expression:literal) => {
        format!("{:?} = {:?}", $expression, "Nothing")
    };
}

macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Wazem!");
}

#[wasm_bindgen]
pub fn find_min() {
    log!("Min (5): {}", find_min!(5u8));
    log!("Min (5/9): {}", find_min!(5u8, 9u8));
    log!("Min (10/6/9): {}", find_min!(10u8, 6u8, 9u8));
}

#[wasm_bindgen]
pub fn test_log() {
    log!("Hello browser, Love Wazem!");
    log!("{}", expr_stringer!(3 + 5));
    log!("{}", expr_stringer!("a literal"));
}
