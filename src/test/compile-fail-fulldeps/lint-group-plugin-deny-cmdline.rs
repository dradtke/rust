// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:lint_group_plugin_test.rs
// ignore-stage1
// compile-flags: -D lint-me

#![feature(phase)]

#[phase(plugin)]
extern crate lint_group_plugin_test;

fn lintme() { } //~ ERROR item is named 'lintme'

fn pleaselintme() { } //~ ERROR item is named 'pleaselintme'

pub fn main() {
    lintme();
    pleaselintme();
}
