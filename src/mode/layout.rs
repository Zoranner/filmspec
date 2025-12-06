//! 布局渲染模块
//!
//! 提供统一的布局渲染函数，避免在不同处理模式中重复实现布局逻辑。

use image::{DynamicImage, ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};

use super::LayoutMode;

/// 根据布局模式渲染光谱图像
///
/// # 参数
/// - `frame_count`: 帧数
/// - `band_length`: 色带长度（像素条长度或圆环宽度）
/// - `inner_radius`: 内圆半径（仅环形布局使用）
/// - `layout_mode`: 布局模式
/// - `get_pixel`: 获取像素颜色的闭包，参数为 (帧索引, 色带索引)
///
/// # 返回
/// - 水平/垂直布局返回 RGB 图像
/// - 环形布局返回 RGBA 图像（环形外为透明）
pub fn render_spectrum<F>(
    frame_count: usize,
    band_length: u32,
    inner_radius: u32,
    layout_mode: LayoutMode,
    get_pixel: F,
) -> DynamicImage
where
    F: Fn(usize, usize) -> Rgb<u8>,
{
    match layout_mode {
        LayoutMode::Horizontal => {
            DynamicImage::ImageRgb8(render_horizontal(frame_count, band_length, get_pixel))
        }
        LayoutMode::Vertical => {
            DynamicImage::ImageRgb8(render_vertical(frame_count, band_length, get_pixel))
        }
        LayoutMode::Radial => DynamicImage::ImageRgba8(render_radial(
            frame_count,
            band_length,
            inner_radius,
            get_pixel,
        )),
    }
}

/// 水平布局渲染（从左到右）
fn render_horizontal<F>(frame_count: usize, band_length: u32, get_pixel: F) -> RgbImage
where
    F: Fn(usize, usize) -> Rgb<u8>,
{
    let mut image: RgbImage = ImageBuffer::new(frame_count as u32, band_length);

    for x in 0..frame_count {
        for y in 0..band_length as usize {
            let color = get_pixel(x, y);
            image.put_pixel(x as u32, y as u32, color);
        }
    }

    image
}

/// 垂直布局渲染（从上到下）
fn render_vertical<F>(frame_count: usize, band_length: u32, get_pixel: F) -> RgbImage
where
    F: Fn(usize, usize) -> Rgb<u8>,
{
    let mut image: RgbImage = ImageBuffer::new(band_length, frame_count as u32);

    for y in 0..frame_count {
        for x in 0..band_length as usize {
            let color = get_pixel(y, x);
            image.put_pixel(x as u32, y as u32, color);
        }
    }

    image
}

/// 环形布局渲染（类似CD光碟）
///
/// 从12点钟方向开始，顺时针排列帧。
/// 每帧的像素条沿径向排列（从内圈延伸到外圈）。
/// 环形外的区域为透明。
fn render_radial<F>(
    frame_count: usize,
    band_length: u32,
    inner_radius: u32,
    get_pixel: F,
) -> RgbaImage
where
    F: Fn(usize, usize) -> Rgb<u8>,
{
    let outer_radius = inner_radius + band_length;
    let image_size = outer_radius * 2;
    let center = outer_radius as f64;

    // 初始化为全透明
    let mut image: RgbaImage = ImageBuffer::new(image_size, image_size);

    // 遍历图像的每个像素
    for py in 0..image_size {
        for px in 0..image_size {
            let dx = px as f64 - center;
            let dy = py as f64 - center;
            let distance = (dx * dx + dy * dy).sqrt();

            // 检查是否在环形区域内
            if distance >= inner_radius as f64 && distance < outer_radius as f64 {
                // 计算角度（从12点钟方向顺时针）
                // atan2(dx, -dy) 使得 12 点钟方向为 0，顺时针增加
                let angle = dx.atan2(-dy);
                let normalized_angle =
                    (angle + std::f64::consts::PI) / (std::f64::consts::PI * 2.0);

                // 根据角度计算帧索引
                let frame_idx =
                    ((normalized_angle * frame_count as f64) as usize).min(frame_count - 1);

                // 根据距离计算色带索引（内圈是像素条的开始）
                let band_idx = (distance - inner_radius as f64) as usize;

                if band_idx < band_length as usize {
                    let Rgb([r, g, b]) = get_pixel(frame_idx, band_idx);
                    image.put_pixel(px, py, Rgba([r, g, b, 255]));
                }
            }
            // 环形外的像素保持透明（默认值 [0, 0, 0, 0]）
        }
    }

    image
}
