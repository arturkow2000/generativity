use generativity::{Id, guard};

fn assert_eq_lt<'id>(_: Id<'id>, _: Id<'id>) {}

fn main() {
    let g_a = guard!(g_a);
    let g_b = guard!(g_b);

    let a: Id = g_a.into();
    let b: Id = g_b.into();

    assert_eq_lt(a, b);

    loop {}
}
