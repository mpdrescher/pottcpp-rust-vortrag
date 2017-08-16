# Futures-Grundlagen:

### Was genau ist ein Future:
 * Ein Future ist alles, was das trait Future implementiert
 * Das trait Future hat die Methode `poll(&self) -> Poll<Item, Error>`
 * `Poll<Item, Error> = Result<Async<Item>, Error>`
 * Async ist ein Enum<T> mit Ready(T), NotReady
 * Funktionen mit der Signatur Fn() -> Result<T, ()> implementieren von Haus aus Future.

[Weiter](https://github.com/mpdrescher/pottcpp-rust-vortrag/blob/master/slides/futures/futures2.md)
