# Abgesehen vom Scope

```Rust
fn main() {
    let ein_string = String::from("Hallo");
    message(ein_string);
    println!("{}", ein_string);
}

fn message(string: String) {
    println!("{}", string);
}
```
<dl>
<br><br><br><br><br><br><br><br><br><br><br><br>
</dl>

Ausgabe:
```
error[E0382]: use of moved value: `ein_string`
 --> .\test.rs:4:20
  |
3 |     message(ein_string);
  |             ---------- value moved here
4 |     println!("{}", ein_string);
  |                    ^^^^^^^^^^ value used here after move
  |
  = note: move occurs because `ein_string` has type `std::string::String`, which does not implement the `Copy` trait

error: aborting due to previous error
```

### Rückgabe von Objekten mit Ownership

```Rust
fn main() {
    let mut ein_string = String::from("Hallo");
    ein_string = message(ein_string);
    //oder: let ein_neuer_string = message(ein_string);
    println!("{}", ein_string);
}

fn message(string: String) -> String {
    println!("{}", string);
    string
}
```

Ausgabe:
```
Hallo
Hallo
```

[Weiter](https://github.com/mpdrescher/pottcpp-rust-vortrag/slides/ownership/ownership5.md)
