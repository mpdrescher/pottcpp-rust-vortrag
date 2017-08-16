# Ownership bei Methoden

Beispiele:

```
Vec:
...
fn new() -> Vec<_>
fn iter(&self) -> Iter<T>
fn iter_mut(&mut self) -> IterMut<T>
fn into_iter(self) -> Iter<'a, T>
...
```

```Rust
let mut vec: Vec<usize> = vec!(1, 2, 3);
vec.iter().map(|x| ...); //x ist vom typ &usize
vec.iter_mut().map(|x| ...); //x ist vom typ &mut usize
vec.into_iter().map(|x| ...); //x ist vom typ usize
//vec hat ownership abgegeben
```