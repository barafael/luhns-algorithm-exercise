# Luhn Algorithmus

Inspiration: https://google.github.io/comprehensive-rust/exercises/day-2/luhn.html

Lösungsvorschlag: https://github.com/barafael/luhney

Mit dem Luhn Algorithmus kann man Kreditkartennummern validieren. Der Algorithmus nimmt einen String als Input und validiert in folgendermaßen:

* Leerzeichen werden ignoriert.
* Eingaben mit weniger als 2 Ziffern sind ungültig.

* Von Rechts nach Links: Verdopple jede zweite Ziffer. Beispiel: "1234" wird zu "2264".

* Summiere alle Ziffern, welche verdoppelt wurden, so wird z.B. aus "7" durch verdoppeln "14", und durch Summation wird es "5"

* Summiere alle Ziffern

* Endet das Ergebnis in einer Null, ist die Zahl gültig.

Siehe Template in `src/lib.rs` für den Anfang.

## WASM

Zum generieren der WASM-Bibliothek: `wasm-pack build --target web --release`, dann z.B. `python -m http.server`.
