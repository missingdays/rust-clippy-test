// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

// I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option {
        res += x;
    }
    println!("{}", res);

    // https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
    let x = 3.14;
    let y = 1_f64 / x;

    // https://rust-lang.github.io/rust-clippy/master/index.html#assertions_on_constants
    assert!(false);
    assert!(true);
    const B: bool = false;
    assert!(B);
}
