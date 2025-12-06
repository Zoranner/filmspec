# Filmspec

CLI-Tool zur Generierung von Filmspektrum-Bildern aus Videodateien.

#### Funktionsweise

Filmspec sampelt gleichmäßig Frames aus einer Videodatei über die Zeit, extrahiert eine Pixellinie (Zeile oder Spalte) aus jedem Frame und stapelt sie zusammen, um ein einzigartiges visuelles Spektrum zu erstellen.

![example](images/example.png)

#### Sampling- und Layout-Modi

| Slice-Modus | Beschreibung |
|-------------|--------------|
| row | Mittlere horizontale Linie aus jedem Frame extrahieren |
| col | Mittlere vertikale Linie aus jedem Frame extrahieren |

| Layout-Modus | Beschreibung |
|--------------|--------------|
| h | Horizontal stapeln (links nach rechts) |
| v | Vertikal stapeln (oben nach unten) |

#### Voraussetzungen

- Rust 1.70+
- FFmpeg (muss im System-PATH verfügbar sein)

#### Installation

```bash
cargo install --path .
```

#### Verwendung

```bash
filmspec <INPUT> [OPTIONS]
```

#### Optionen

| Option | Kurz | Beschreibung | Standard |
|--------|------|--------------|----------|
| `<INPUT>` | - | Eingabe-Videodateipfad | erforderlich |
| `--output` | `-o` | Ausgabe-Bildpfad | `<Dateiname>_spectrum.png` |
| `--width` | `-w` | Ausgabe-Bildbreite | 1920 |
| `--height` | `-H` | Ausgabe-Bildhöhe | 300 |
| `--layout` | `-l` | Layout-Richtung h/v | h |
| `--slice` | `-s` | Slice-Richtung row/col | row |

#### Beispiele

```bash
# Standard (1920×300, horizontales Layout, Zeilen-Slice)
filmspec movie.mp4

# Benutzerdefinierter Ausgabepfad und Größe
filmspec movie.mp4 -o output.png -w 1280 -H 400

# Vertikales Layout (von oben nach unten stapeln)
filmspec movie.mp4 -l v

# Spalten-Slice (vertikale Pixellinien extrahieren)
filmspec movie.mp4 -s col
```

#### Lizenz

MIT

