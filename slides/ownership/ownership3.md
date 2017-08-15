# Ownership

```Rust
fn main() {
    message();
}

fn message() {
    let ein_string = String::from("Hallo");
    // Ein_string gehört jetzt der Funktion message.
    println!("{}", ein_string);
}
```

Ausgabe:
```
Hallo
```

## Aber wieso 'gehört' ein_string der Funktion message?

```Rust
fn main() {
    message();
    println!("nicht mehr vorhanden: {}", ein_string);
}

fn message() {
    let ein_string = String::from("Hallo");
    // Ein_string gehört jetzt der Funktion message.
    println!("{}", ein_string);
    // Die Funktion, der der String gehört, ist zuende:
    // ein_string wird gelöscht...
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
