# Mutability

```Rust
fn main() {
    let x = 5;
    x = 6;
    println!("{}", x);
}
```

Ausgabe:
```
error[E0384]: re-assignment of immutable variable `x`
 --> .\test.rs:3:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     x = 6;
  |     ^^^^^ re-assignment of immutable variable

error: aborting due to previous error
```

Deswegen:

```Rust
fn main() {
    let x = 5;
    x = 6;
    println!("{}", x);
}
```

Ausgabe:
```
6
```

[Weiter](https://github.com/mpdrescher/pottcpp-rust-vortrag/blob/master/slides/ownership/ownership3.md)