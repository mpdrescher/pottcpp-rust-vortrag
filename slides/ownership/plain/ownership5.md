# Alternative zum Übergeben der Ownership: Referenzen

```Rust
fn main() {
    let x = 5;
    add_one(&x);
    println!("{}", x);
}

fn add_one(number: &usize) {
    *number += 1;
}
```

Dieses Programm gibt einen Fehler aus,  
da auch Referenzen als mutable markiert werden müssen:

```Rust
fn main() {
    let mut x = 5;
    add_one(&mut x);
    println!("{}", x);
}

fn add_one(number: &mut usize) {
    *number += 1;
}
```

### Die goldene Regel:

- Es kann mehrere unveränderliche Referenzen zu einem Objekt geben, __*oder*__
- Genau eine veränderliche.

### Einfrieren:

Wird eine Referenz von einem Objekt erstellt, kann dieses nicht mehr verändert werden:

```Rust
fn main() {
    let mut x = 5;
    {
        let reference = &x;
        // x += 1; !Fehler!
    }
    x += 1;
}
```

Das heisst auch, dass es nicht möglich ist ein Objekt out-of-scope gehen zu lassen,
auf welches noch Referenzen zeigen.

[Weiter](https://github.com/mpdrescher/pottcpp-rust-vortrag/blob/master/slides/ownership/ownership6.md)