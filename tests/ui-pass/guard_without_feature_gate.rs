// `guard!` must work without `#![feature(super_let)]`.
use generativity::{Id, guard};

struct Branded<'id>(#[allow(dead_code)] Id<'id>);

fn main() {
    let a = guard!(a);
    let b = guard!();
    let _ = (Branded(a.into()), Branded(b.into()));
}
