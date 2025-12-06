# Filmspec

A CLI tool to generate film spectrum images from video files.

[中文](docs/README_zh-CN.md) | [日本語](docs/README_ja.md) | [한국어](docs/README_ko.md) | [Español](docs/README_es.md) | [Français](docs/README_fr.md) | [Deutsch](docs/README_de.md) | [Русский](docs/README_ru.md)

#### How It Works

Filmspec uniformly samples frames from a video file over time, processes each frame according to the selected mode, and generates a unique visual spectrum.

**Slice Mode** - Extract pixel lines from each frame:

![slice example](docs/images/example_slice.png)

**Hue Mode** - Analyze dominant color of each frame:

![hue example](docs/images/example_hue.png)

#### Process Modes

| Mode | Description |
|------|-------------|
| slice | Extract a pixel line (row or column) from each frame and stack them |
| hue | Analyze dominant color of each frame and create a color band spectrum |

#### Sample & Layout Modes

| Sample Mode | Description |
|-------------|-------------|
| row | Extract middle horizontal line from each frame (slice mode only) |
| col | Extract middle vertical line from each frame (slice mode only) |

| Layout Mode | Description |
|-------------|-------------|
| h | Stack horizontally (left to right) |
| v | Stack vertically (top to bottom) |
| r | Radial layout (disc-shaped, like a CD) |

#### Prerequisites

- Rust 1.70+
- FFmpeg (must be available in system PATH)

#### Installation

```bash
cargo install --path .
```

#### Usage

```bash
filmspec <INPUT> [OPTIONS]
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `<INPUT>` | - | Input video file path | required |
| `--output` | `-o` | Output image path | `<filename>_spectrum.png` or `<filename>_hue.png` |
| `--width` | `-w` | Frame count (h/v: image width, r: circumference frames) | 1920 |
| `--height` | `-H` | Band length (h/v: image height, r: ring width) | 300 (h/v), 800 (r) |
| `--mode` | `-m` | Process mode: slice/hue | slice |
| `--layout` | `-l` | Layout direction: h/v/r | h |
| `--sample` | `-s` | Sample direction: row/col (slice mode only) | row |
| `--inner-radius` | - | Inner radius for radial layout (pixels) | 250 |

#### Examples

```bash
# Default slice mode (1920×300, horizontal layout, row sample)
filmspec movie.mp4

# Hue spectrum mode (dominant color analysis)
filmspec movie.mp4 -m hue

# Custom output path and size
filmspec movie.mp4 -o output.png -w 1280 -H 400

# Vertical layout (stack top to bottom)
filmspec movie.mp4 -l v

# Column sample in slice mode (extract vertical pixel lines)
filmspec movie.mp4 -s col

# Hue mode with vertical layout
filmspec movie.mp4 -m hue -l v -w 400 -H 1920

# Radial layout (disc-shaped spectrum)
filmspec movie.mp4 -l r

# Radial layout with custom parameters
filmspec movie.mp4 -l r -w 2400 -H 600 --inner-radius 200

# Hue mode with radial layout
filmspec movie.mp4 -m hue -l r
```

#### License

MIT
