# Ownership

```Rust
fn main() {
    let ein_string = String::from("Hallo");
    // Der Wert "Hallo" (String) gehört jetzt der Variable 'ein_string'
    println!("{}", ein_string);
}
```

Ausgabe:
```
Hallo
```

### Aber wieso 'gehört' ein_string der Variable?

```Rust
fn main() {
    let ein_string = String::from("Hallo");
    let ein_neuer_string = ein_string;
    println!("{}", ein_string);
}
```

Ausgabe:
```
error[E0382]: use of moved value: `ein_string`
 --> .\test.rs:4:20
  |
3 |     let ein_neuer_string = ein_string;
  |         ---------------- value moved here
4 |     println!("{}", ein_string);
  |                    ^^^^^^^^^^ value used here after move
  |
  = note: move occurs because `ein_string` has type `std::string::String`, which does not implement the `Copy` trait

error: aborting due to previous error
```

### Verhalten bei Funktionsaufrufen

```Rust
fn main() {
    message();
    println!("nicht mehr vorhanden: {}", ein_string);
}

fn message() {
    let ein_string = String::from("Hallo");
    println!("{}", ein_string);
}
```

Ausgabe:
```
error[E0425]: cannot find value `ein_string` in this scope
 --> .\test.rs:3:42
  |
3 |     println!("nicht mehr vorhanden: {}", ein_string);
  |                                          ^^^^^^^^^^ not found in this scope

error: aborting due to previous error
```

[Weiter](https://github.com/mpdrescher/pottcpp-rust-vortrag/blob/master/slides/ownership/plain/ownership4.md)