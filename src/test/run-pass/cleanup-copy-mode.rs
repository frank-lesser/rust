// xfail-win32
fn adder(+x: @int, +y: @int) -> int { ret *x + *y; }
fn failer() -> @int { fail; }
fn main() {
    assert(result::is_err(task::try {||
        adder(@2, failer()); ()
    }));
}

