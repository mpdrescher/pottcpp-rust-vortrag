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
    // Der Wert "Hallo" (String) gehört jetzt der Variable 'ein_string'
    let ein_neuer_string = ein_string;
    // Der Wert "Hallo" gehört jetzt nicht mehr ein_string, sondern ein_neuer_string
    println!("{}", ein_string);
}
```

Ausgabe:
```
error[E0382]: use of moved value: `ein_string`
 --> .\test.rs:5:20
  |
4 |     let ein_neuer_string = ein_string;
  |         ---------------- value moved here
5 |     println!("{}", ein_string);
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
    // Der String "Hallo" gehört jetzt der Variablen ein_string, welche sich in message befindet
    println!("{}", ein_string);
    // ein_string geht out-of-scope, also wird auch dessen Wert gelöscht. Der Speicher wird aufgeräumt.
    // (Strings sind Heap-allocated)
}
```

Ausgabe:
```
error[E0425]: cannot find value `ein_string` in this scope
 --> .\test.rs:3:20
  |
3 |     println!("{}", ein_string);
  |                    ^^^^^^^^^^ not found in this scope

error: aborting due to previous error
```

[Weiter](https://github.com/mpdrescher/pottcpp-rust-vortrag/blob/master/slides/ownership/ownership4.md)