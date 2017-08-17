# Konzept: Traits

### Lösung:
`into_iter` wird mit `iter` ausgetauscht:

```Rust
fn main() { 
    let vec = vec![1, 2, 3];
    let arc_vec = Arc::new(vec);
    for i in 0..8 { 
        let arc_vec = arc_vec.clone();
        thread::spawn(move ||{
            for x in arc_vec.iter(){
                do_something_crazy(x, i);
            }   
        });
    }
}   
```

### Unterschied zwischen iter und into_iter:
 * iter iteriert über Referenzen
 * into_iter konsumiert den Vektor, iteriert also über die Werte
 * Arc gibt nur Referenzen zurück (ergibt auch Sinn, da Ressourcen über Threads geteilt werden)

### In diesem Kontext wäre into_iter ein Fehler:
 * Würde die Ressource für alle anderen Threads verbrauchen oder den gesamten Vec kopieren
 * Der Fehler wurde vom Compiler erkannt

### Kann man alle Ressourcen über Threads im Arc teilen:
 * Nur die, die Send implementieren.
 * Send ist ein Marker (Trait).
 * Trait Bounds:
    - Beim Generic wird Trait Bound definiert: `struct Arc<T: Send + ?Sized etc.>`
    - Die Implementierung der Container kann auch Trait Bound sein: 
    ```Rust
        impl<T> Send for Vec<T>
                    where T: Send {}
    ```
        Implementiert `Send` für den Vec nur wenn auch die Elemente von Vec `Send` implementieren
 * Send ist ein Marker (Keine Methoden, muss aber explizit implemetiert werden)
 * Das Tolle: Das Typesystem kann festellen, ob man Sachen in einen Thread steckt, die nicht reingehören.

### 2 wichtige Traits:
 * Sync (Struktur kann Referenzen über mehrere Threads teilen)
 * Send (Struktur kann zwischen Threads gemoved werdern) 
