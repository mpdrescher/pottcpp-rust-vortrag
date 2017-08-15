# Alternative zum Übergeben der Ownership: Referenzen

```Rust
fn main() {
    let x = 5;
    add_one(&x);
    println!("{}", x);
}

fn add_one(number: &usize) {
    // number zeigt auf die Variable x (owned), gehört aber selbst add_one an.
    *number += 1; // Füge 1 dem hinzu, auf das die Referenz zeigt.
    // Die Referenz number stirbt, weil das Ende des Scopes erreicht wurde.
}
```

Dieses Programm gibt einen Fehler aus,  
da auch Referenzen als mutable markiert werden müssen:

```Rust
fn main() {
    let mut x = 5; // Wichtig: x mutable machen
    add_one(&mut x);
    println!("{}", x);
}

fn add_one(number: &mut usize) {
    // number zeigt auf die Variable x (owned), gehört aber selbst add_one an.
    *number += 1; // Füge 1 dem hinzu, auf das die Referenz zeigt.
    // Die Referenz number stirbt, weil das Ende des Scopes erreicht wurde.
}
```

### Die goldene Regel:

- Es kann mehrere unveränderliche Referenzen zu einer Sache geben, __*oder*__
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
