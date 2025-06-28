// Copyright © 2022-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! rlog {
    ( $( $t:tt )* ) => {
        println!($( $t )*)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// Wrap web-sys console log function in a println! style macro
#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! rlog {
    ( $( $t:tt )* ) => {
        $crate::log::log(&format!( $( $t )* ))
    }
}

#[macro_export]
macro_rules! print_success {
    ( $s:expr, $( $t:tt )* ) => {
        $crate::rlog!("{:>12} {}",
            owo_colors::OwoColorize::bold(
                &owo_colors::OwoColorize::green(&$s)), format!($( $t )*))
    }
}

#[macro_export]
macro_rules! print_status {
    ( $s:expr, $( $t:tt )* ) => {
        $crate::rlog!("{:>12} {}",
            owo_colors::OwoColorize::bold(
                &owo_colors::OwoColorize::cyan(&$s)), format!($( $t )*))
    }
}

#[macro_export]
macro_rules! print_info {
    ( $s:expr, $( $t:tt )* ) => {
        $crate::rlog!("{:>12} {}",
            owo_colors::OwoColorize::bold(
                &owo_colors::OwoColorize::blue(&$s)), format!($( $t )*))
    }
}

#[macro_export]
macro_rules! print_warn {
    ( $s:expr, $( $t:tt )* ) => {
        $crate::rlog!("{:>12} {}",
            owo_colors::OwoColorize::bold(
                &owo_colors::OwoColorize::yellow(&$s)), format!($( $t )*))
    }
}

#[macro_export]
macro_rules! print_fail {
    ( $s:expr, $( $t:tt )* ) => {
        $crate::rlog!("{:>12} {}",
            owo_colors::OwoColorize::bold(
                &owo_colors::OwoColorize::red(&$s)), format!($( $t )*))
    }
}

#[macro_export]
macro_rules! panic_panic {
    ( $s:expr, $( $t:tt )* ) => {
        panic!("{:>12} {}", owo_colors::OwoColorize::bold(
                &owo_colors::OwoColorize::red(&$s)), format!($( $t )*))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_print_success_macro_expands() {
        // This test checks that the macro expands and does not panic.
        print_success!("SUCCESS", "Test message: {}", 42);
    }

    #[test]
    fn test_print_status_macro_expands() {
        print_status!("STATUS", "Status: {}", "ok");
    }

    #[test]
    fn test_print_info_macro_expands() {
        print_info!("INFO", "Info: {}", "details");
    }

    #[test]
    fn test_print_warn_macro_expands() {
        print_warn!("WARN", "Warning: {}", "be careful");
    }

    #[test]
    fn test_print_fail_macro_expands() {
        print_fail!("FAIL", "Failure: {}", "something went wrong");
    }

    #[test]
    #[should_panic]
    fn test_panic_panic_macro_panics() {
        panic_panic!("PANIC", "This should panic: {}", 123);
    }
}
