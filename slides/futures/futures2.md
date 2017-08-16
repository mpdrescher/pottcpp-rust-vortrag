# Zero Overhead:
 * Kein OS Sync Mecahnismus (Mutex etc)
 * Benutzt Atomic Boolean Values als Lock
 * Durch die Funktion std::thread::park können OS und Program kooperativ Arbeiten.
 * Futures::poll wird nicht ständig aufgerufen. (Behindert nicht den Programmfluss)


[Weiter](https://github.com/mpdrescher/pottcpp-rust-vortrag/blob/master/slides/futures/futures3.md)
