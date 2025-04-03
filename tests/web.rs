//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use tsg_js::{add, greet};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_add_function() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(100, 200), 300);
}

#[wasm_bindgen_test]
fn test_greet_function_exists() {
    // This just tests that the function can be called without errors
    // We can't easily test the alert functionality in this environment
    greet();
    assert!(true);
}
