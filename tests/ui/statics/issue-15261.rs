//@ build-pass
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

//@ pretty-expanded FIXME #23616

static mut n_mut: usize = 0;

static n: &'static usize = unsafe { &n_mut };
//~^ WARN shared reference of mutable static is discouraged [static_mut_ref]

fn main() {}
