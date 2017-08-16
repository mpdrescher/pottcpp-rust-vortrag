
# Ownership in Sync-Strukturen:

### Lösung:
Lokale Copy des Arcs.

### Neues Beispiel:
```Rust
fn main() { 
    let vec = vec![1, 2, 3];
    let arc_vec = Arc::new(vec);
    for i in 0..8 {
        let arc_vec = arc_vec.clone();
        thread::spawn(move ||{
            for x in arc_vec.into_iter(){
                do_something_crazy(&x, i);
            }
        });
    }
}
```

### Error:
```
error[E0507]: cannot move out of borrowed content                                                                                                                                                                                             
  --> src/main.rs:15:22                                                                                                                                                                                                                       
   |                                                                                                                                                                                                                                          
15 |             for x in arc_vec.into_iter(){                                                                                                                                                                                                
   |                      ^^^^^^^ cannot move out of borrowed content                                                                                                                                                                         
                                                                                                                                                                                                                                              
error: aborting due to previous error 
```
[weiter](https://github.com/PhilippRo/pottcpp-rust-vortrag/blob/master/slides/sync/sync6.md)
