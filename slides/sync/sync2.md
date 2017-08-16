# Mechanismen (Auszug):

### Arc:
 * Atomic Reference Counter
 * Teilt unveränderbare Referenzen über Threads
 * Löst Ownership intern mithilfe von Rust `unsafe` Blöcken

### Mutex:
 * MUTable EXlusive access
 * Stellt einen Container zur Verfügung, der sichere, veränderbare Referenzen zurückgibt
 * Gibt MutexGuard zurück, der sich wie ein lock_guard verhält

### mpsc-Channels:
 * Multiple Producer Single Consumer Strukturen
 * Threadsichere FIFO-Strukturen

[weiter](https://github.com/PhilippRo/pottcpp-rust-vortrag/blob/master/slides/sync/sync3.md)
