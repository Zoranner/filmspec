# Filmspec

从视频文件生成电影光谱图片的命令行工具。

#### 工作原理

Filmspec 按时间均匀采样视频帧，从每帧中提取一条像素线（行或列），然后将它们堆叠在一起生成独特的视觉光谱图。

![example](images/example.png)

#### 采样与布局模式

| 切片模式 | 说明 |
|---------|------|
| row | 提取每帧中间水平行 |
| col | 提取每帧中间垂直列 |

| 布局模式 | 说明 |
|---------|------|
| h | 水平堆叠（从左到右） |
| v | 垂直堆叠（从上到下） |

#### 前置要求

- Rust 1.70+
- FFmpeg（需要在系统 PATH 中可用）

#### 安装

```bash
cargo install --path .
```

#### 使用

```bash
filmspec <INPUT> [OPTIONS]
```

#### 参数

| 参数 | 短选项 | 说明 | 默认值 |
|------|--------|------|--------|
| `<INPUT>` | - | 输入视频文件路径 | 必填 |
| `--output` | `-o` | 输出图片路径 | `<文件名>_spectrum.png` |
| `--width` | `-w` | 输出图片宽度 | 1920 |
| `--height` | `-H` | 输出图片高度 | 300 |
| `--layout` | `-l` | 布局方向 h/v | h |
| `--slice` | `-s` | 切片方向 row/col | row |

#### 示例

```bash
# 默认参数（1920×300，水平布局，行切片）
filmspec movie.mp4

# 指定输出路径和尺寸
filmspec movie.mp4 -o output.png -w 1280 -H 400

# 垂直布局（从上到下堆叠）
filmspec movie.mp4 -l v

# 列切片（提取垂直像素列）
filmspec movie.mp4 -s col
```

#### 许可证

MIT

