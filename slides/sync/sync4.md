# Ownership in Sync-Strukturen:

### Lösung:
Das keyword ´move´ vor closures erzwingt, dass das Objekt der closure zugerodnet wird.

### Neues Beispiel:
```Rust
fn main() { 
    let vec = vec![1, 2, 3];
    let arc_vec = Arc::new(vec);
    for i in 0..8 {
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
error[E0382]: capture of moved value: `arc_vec`                                                                                                                                                                                               
  --> src/main.rs:14:22                                                                                                                                                                                                                       
   |                                                                                                                                                                                                                                          
13 |         thread::spawn(move ||{                                                                                                                                                                                                           
   |                       ------- value moved (into closure) here                                                                                                                                                                            
14 |             for x in arc_vec.iter(){                                                                                                                                                                                                     
   |                      ^^^^^^^ value captured here after move                                                                                                                                                                              
   |                                                                                                                                                                                                                                          
   = note: move occurs because `arc_vec` has type `std::sync::Arc<std::vec::Vec<i64>>`, which does not implement the `Copy` trait  
```
(Hatten wir schon, ist aber ein Beispiel dafür, dass primitive Typen Copy meistens implementieren.)

[weiter](https://github.com/PhilippRo/pottcpp-rust-vortrag/blob/master/slides/sync/sync5.md)
