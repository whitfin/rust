// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This test depends on a patch that was committed to upstream LLVM
// before 4.0, formerly backported to the Rust LLVM fork.

// ignore-tidy-linelength
// ignore-windows
// ignore-macos

// compile-flags: -g -C no-prepopulate-passes

// CHECK-LABEL: @main
// CHECK: {{.*}}DISubprogram{{.*}}name: "main",{{.*}}DIFlagMainSubprogram{{.*}}

pub fn main() {
}
