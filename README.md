# Filmspec

A CLI tool to generate film spectrum images from video files.

[中文](docs/README_zh-CN.md) | [日本語](docs/README_ja.md) | [한국어](docs/README_ko.md) | [Español](docs/README_es.md) | [Français](docs/README_fr.md) | [Deutsch](docs/README_de.md) | [Русский](docs/README_ru.md)

#### How It Works

Filmspec uniformly samples frames from a video file over time, extracts a single pixel line (row or column) from each frame, and stacks them together to create a unique visual spectrum.

![example](docs/images/example.png)

#### Sample & Layout Modes

| Slice Mode | Description |
|------------|-------------|
| row | Extract middle horizontal line from each frame |
| col | Extract middle vertical line from each frame |

| Layout Mode | Description |
|-------------|-------------|
| h | Stack horizontally (left to right) |
| v | Stack vertically (top to bottom) |

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
| `--output` | `-o` | Output image path | `<filename>_spectrum.png` |
| `--width` | `-w` | Output image width | 1920 |
| `--height` | `-H` | Output image height | 300 |
| `--layout` | `-l` | Layout direction h/v | h |
| `--slice` | `-s` | Slice direction row/col | row |

#### Examples

```bash
# Default (1920×300, horizontal layout, row slice)
filmspec movie.mp4

# Custom output path and size
filmspec movie.mp4 -o output.png -w 1280 -H 400

# Vertical layout (stack top to bottom)
filmspec movie.mp4 -l v

# Column slice (extract vertical pixel lines)
filmspec movie.mp4 -s col
```

#### License

MIT
