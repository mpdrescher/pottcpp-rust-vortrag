# Zero Overhead
 * Kein OS Sync Mechanismus (Mutex etc.)
 * Benutzt atomic boolean values als Lock
 * Durch die Funktion std::thread::park können OS und Programm kooperativ arbeiten.
 * Futures::poll wird nicht ständig aufgerufen. (Behindert nicht den Programmfluss)

[Weiter](https://github.com/mpdrescher/pottcpp-rust-vortrag/blob/master/slides/futures/futures3.md)
