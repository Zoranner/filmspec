# Filmspec

CLI-Tool zur Generierung von Filmspektrum-Bildern aus Videodateien.

#### Funktionsweise

Filmspec sampelt gleichmäßig Frames aus einer Videodatei über die Zeit, verarbeitet jeden Frame gemäß dem ausgewählten Modus und erstellt ein einzigartiges visuelles Spektrum.

**Slice-Modus** - Extrahiert Pixellinien aus jedem Frame:

![Slice-Beispiel](images/example_slice.png)

**Hue-Modus** - Analysiert die dominante Farbe jedes Frames:

![Hue-Beispiel](images/example_hue.png)

#### Verarbeitungsmodi

| Modus | Beschreibung |
|-------|--------------|
| slice | Extrahiert eine Pixellinie (Zeile oder Spalte) aus jedem Frame und stapelt sie |
| hue | Analysiert die dominante Farbe jedes Frames und erstellt ein Farbband-Spektrum |

#### Sampling- und Layout-Modi

| Sample-Modus | Beschreibung |
|--------------|--------------|
| row | Mittlere horizontale Linie aus jedem Frame extrahieren (nur Slice-Modus) |
| col | Mittlere vertikale Linie aus jedem Frame extrahieren (nur Slice-Modus) |

| Layout-Modus | Beschreibung |
|--------------|--------------|
| h | Horizontal stapeln (links nach rechts) |
| v | Vertikal stapeln (oben nach unten) |
| r | Radiales Layout (scheibenförmig, wie eine CD) |

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
| `--output` | `-o` | Ausgabe-Bildpfad | `<Dateiname>_spectrum_slice.png` oder `<Dateiname>_spectrum_hue.png` |
| `--width` | `-w` | Frame-Anzahl (h/v: Bildbreite, r: Umfang-Frames) | 1920 |
| `--height` | `-H` | Bandlänge (h/v: Bildhöhe, r: Ringbreite) | 300 (h/v), 500 (r) |
| `--mode` | `-m` | Verarbeitungsmodus: slice/hue | slice |
| `--layout` | `-l` | Layout-Richtung: h/v/r | h |
| `--sample` | `-s` | Sample-Richtung: row/col (nur Slice-Modus) | row |
| `--inner-radius` | - | Innerer Radius für radiales Layout (Pixel) | 250 |

#### Beispiele

```bash
# Standard Slice-Modus (1920×300, horizontales Layout, Zeilen-Sample)
filmspec movie.mp4

# Farbton-Spektrum-Modus (dominante Farbanalyse)
filmspec movie.mp4 -m hue

# Benutzerdefinierter Ausgabepfad und Größe
filmspec movie.mp4 -o output.png -w 1280 -H 400

# Vertikales Layout (von oben nach unten stapeln)
filmspec movie.mp4 -l v

# Spalten-Sample im Slice-Modus (vertikale Pixellinien extrahieren)
filmspec movie.mp4 -s col

# Hue-Modus + vertikales Layout
filmspec movie.mp4 -m hue -l v -w 400 -H 1920

# Radiales Layout (scheibenförmiges Spektrum)
filmspec movie.mp4 -l r

# Radiales Layout + benutzerdefinierte Parameter
filmspec movie.mp4 -l r -w 2400 -H 600 --inner-radius 200

# Hue-Modus + radiales Layout
filmspec movie.mp4 -m hue -l r
```

#### Lizenz

MIT
