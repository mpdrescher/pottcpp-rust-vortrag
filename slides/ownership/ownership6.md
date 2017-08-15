# Lifetimes

Aus dem offiziellen Rust Buch:
```Rust
fn skip_prefix(line: &str, prefix: &str) -> &str {
    // ...
}

let line = "lang:en=Hello World!";
let lang = "en";

let v;
{
    let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
    v = skip_prefix(line, p.as_str());  //  |
}                                       // -+ `p` goes out of scope.
println!("{}", v);
```

Die Frage, wie lange der Rückgabewert von skip_prefix lebt ist nicht eindeutig zu beantworten. 
Deshalb:

```Rust
fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    // ...
}
```

Lifetimes __*beschreiben*__ wie lange ein Objekt (owned/referenz) lebt.

[Weiter](https://github.com/mpdrescher/pottcpp-rust-vortrag/blob/master/slides/ownership/ownership7.md)