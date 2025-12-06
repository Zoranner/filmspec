# Filmspec

从视频文件生成电影光谱图片的命令行工具。

#### 工作原理

Filmspec 按时间均匀采样视频帧，根据所选模式处理每一帧，生成独特的视觉光谱图。

**Slice 模式** - 从每帧提取像素线：

![slice 示例](images/example_slice.png)

**Hue 模式** - 分析每帧主色调：

![hue 示例](images/example_hue.png)

#### 处理模式

| 模式 | 说明 |
|------|------|
| slice | 从每帧提取一条像素线（行或列）并堆叠成光谱 |
| hue | 分析每帧的主色调，生成色彩带光谱 |

#### 采样与布局模式

| 采样模式 | 说明 |
|---------|------|
| row | 提取每帧中间水平行（仅 slice 模式） |
| col | 提取每帧中间垂直列（仅 slice 模式） |

| 布局模式 | 说明 |
|---------|------|
| h | 水平堆叠（从左到右） |
| v | 垂直堆叠（从上到下） |
| r | 环形布局（圆盘状，类似 CD 光碟） |

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
| `--output` | `-o` | 输出图片路径 | `<文件名>_spectrum_slice.png` 或 `<文件名>_spectrum_hue.png` |
| `--width` | `-w` | 帧数量（h/v: 图片宽度，r: 圆周帧数） | 1920 |
| `--height` | `-H` | 色带长度（h/v: 图片高度，r: 圆环宽度） | 300 (h/v)，500 (r) |
| `--mode` | `-m` | 处理模式：slice/hue | slice |
| `--layout` | `-l` | 布局方向：h/v/r | h |
| `--sample` | `-s` | 采样方向：row/col（仅 slice 模式） | row |
| `--inner-radius` | - | 环形布局内圆半径（像素） | 250 |

#### 示例

```bash
# 默认 slice 模式（1920×300，水平布局，行采样）
filmspec movie.mp4

# 色调光谱模式（主色调分析）
filmspec movie.mp4 -m hue

# 指定输出路径和尺寸
filmspec movie.mp4 -o output.png -w 1280 -H 400

# 垂直布局（从上到下堆叠）
filmspec movie.mp4 -l v

# slice 模式下使用列采样（提取垂直像素列）
filmspec movie.mp4 -s col

# 色调模式 + 垂直布局
filmspec movie.mp4 -m hue -l v -w 400 -H 1920

# 环形布局（圆盘状光谱）
filmspec movie.mp4 -l r

# 环形布局 + 自定义参数
filmspec movie.mp4 -l r -w 2400 -H 600 --inner-radius 200

# 色调模式 + 环形布局
filmspec movie.mp4 -m hue -l r
```

#### 许可证

MIT
