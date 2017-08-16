# Ownership in Sync-Strukturen:

### Problem:
Die Sync-Strukturen schränken Funktionsumpfang ein.

### Beispiel:
```Rust
fn main() { 
    let vec = vec![1, 2, 3];
    let arc_vec = Arc::new(vec);
    for i in 0..8 { 
        thread::spawn(||{
            for x in arc_vec.into_iter(){
                do_something_crazy(x, i);                
            }
        });
    }   
} 
```

### Compiler Error:
```
error[E0373]: closure may outlive the current function, but it borrows `arc_vec`, which is owned by the current function                                                                                                                      
  --> src/main.rs:13:23                                                                                                                                                                                                                       
   |                                                                                                                                                                                                                                          
13 |         thread::spawn(||{                                                                                                                                                                                                                
   |                       ^^ may outlive borrowed value `arc_vec`                                                                                                                                                                            
14 |             for x in arc_vec.iter(){                                                                                                                                                                                                     
   |                      ------- `arc_vec` is borrowed here                                                                                                                                                                                  
   |                                                                                                                                                                                                                                          
help: to force the closure to take ownership of `arc_vec` (and any other referenced variables), use the `move` keyword                                                                                                                        
   |                                                                                                                                                                                                                                          
13 |         thread::spawn(move ||{                                                                                                                                                                                                           
   |           
```
[weiter](https://github.com/PhilippRo/pottcpp-rust-vortrag/blob/master/slides/sync/sync4.md)
