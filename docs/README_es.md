# Filmspec

Herramienta CLI para generar imágenes de espectro de películas a partir de archivos de video.

#### Cómo Funciona

Filmspec muestrea uniformemente fotogramas de un archivo de video a lo largo del tiempo, procesa cada fotograma según el modo seleccionado y genera un espectro visual único.

**Modo Slice** - Extrae líneas de píxeles de cada fotograma:

![ejemplo slice](images/example_slice.png)

**Modo Hue** - Analiza el color dominante de cada fotograma:

![ejemplo hue](images/example_hue.png)

#### Modos de Procesamiento

| Modo | Descripción |
|------|-------------|
| slice | Extrae una línea de píxeles (fila o columna) de cada fotograma y las apila |
| hue | Analiza el color dominante de cada fotograma y crea un espectro de bandas de color |

#### Modos de Muestreo y Diseño

| Modo de Muestreo | Descripción |
|------------------|-------------|
| row | Extrae la línea horizontal central de cada fotograma (solo modo slice) |
| col | Extrae la línea vertical central de cada fotograma (solo modo slice) |

| Modo de Diseño | Descripción |
|----------------|-------------|
| h | Apilar horizontalmente (izquierda a derecha) |
| v | Apilar verticalmente (arriba a abajo) |
| r | Diseño radial (forma de disco, como un CD) |

#### Requisitos Previos

- Rust 1.70+
- FFmpeg (debe estar disponible en el PATH del sistema)

#### Instalación

```bash
cargo install --path .
```

#### Uso

```bash
filmspec <INPUT> [OPTIONS]
```

#### Opciones

| Opción | Corto | Descripción | Predeterminado |
|--------|-------|-------------|----------------|
| `<INPUT>` | - | Ruta del archivo de video | requerido |
| `--output` | `-o` | Ruta de la imagen de salida | `<nombre>_spectrum.png` o `<nombre>_hue.png` |
| `--width` | `-w` | Cantidad de fotogramas (h/v: ancho de imagen, r: fotogramas de circunferencia) | 1920 |
| `--height` | `-H` | Longitud de banda (h/v: alto de imagen, r: ancho del anillo) | 300 (h/v), 800 (r) |
| `--mode` | `-m` | Modo de procesamiento: slice/hue | slice |
| `--layout` | `-l` | Dirección del diseño: h/v/r | h |
| `--sample` | `-s` | Dirección de muestreo: row/col (solo modo slice) | row |
| `--inner-radius` | - | Radio interno para diseño radial (píxeles) | 250 |

#### Ejemplos

```bash
# Modo slice predeterminado (1920×300, diseño horizontal, muestreo por fila)
filmspec movie.mp4

# Modo espectro de tonalidad (análisis de color dominante)
filmspec movie.mp4 -m hue

# Ruta y tamaño personalizados
filmspec movie.mp4 -o output.png -w 1280 -H 400

# Diseño vertical (apilar de arriba a abajo)
filmspec movie.mp4 -l v

# Muestreo por columna en modo slice (extraer líneas de píxeles verticales)
filmspec movie.mp4 -s col

# Modo hue + diseño vertical
filmspec movie.mp4 -m hue -l v -w 400 -H 1920

# Diseño radial (espectro en forma de disco)
filmspec movie.mp4 -l r

# Diseño radial + parámetros personalizados
filmspec movie.mp4 -l r -w 2400 -H 600 --inner-radius 200

# Modo hue + diseño radial
filmspec movie.mp4 -m hue -l r
```

#### Licencia

MIT
