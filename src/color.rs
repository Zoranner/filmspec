//! 颜色工具模块
//!
//! 提供 RGB、HSV 等颜色空间的转换和分析功能。

use image::Rgb;

/// HSV 颜色表示
#[derive(Debug, Clone, Copy)]
pub struct Hsv {
    /// 色相 (0-360)
    pub h: f64,
    /// 饱和度 (0-1)
    pub s: f64,
    /// 明度 (0-1)
    pub v: f64,
}

impl Hsv {
    pub fn new(h: f64, s: f64, v: f64) -> Self {
        Self { h, s, v }
    }

    /// 从 RGB 转换为 HSV
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let v = max;
        let s = if max == 0.0 { 0.0 } else { delta / max };

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        Self { h, s, v }
    }

    /// 从 RGB u8 转换为 HSV
    pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgb(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }

    /// 转换为 RGB (0-1 范围)
    pub fn to_rgb(&self) -> (f64, f64, f64) {
        let c = self.v * self.s;
        let x = c * (1.0 - ((self.h / 60.0) % 2.0 - 1.0).abs());
        let m = self.v - c;

        let (r, g, b) = if self.h < 60.0 {
            (c, x, 0.0)
        } else if self.h < 120.0 {
            (x, c, 0.0)
        } else if self.h < 180.0 {
            (0.0, c, x)
        } else if self.h < 240.0 {
            (0.0, x, c)
        } else if self.h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (r + m, g + m, b + m)
    }

    /// 转换为 RGB<u8>
    pub fn to_rgb_u8(&self) -> Rgb<u8> {
        let (r, g, b) = self.to_rgb();
        Rgb([
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
        ])
    }

    /// 增强饱和度和明度
    pub fn enhance(&self, sat_factor: f64, val_factor: f64) -> Self {
        Self {
            h: self.h,
            s: (self.s * sat_factor).min(1.0),
            v: (self.v * val_factor).min(1.0),
        }
    }

    /// 是否为低饱和度（灰色）
    pub fn is_gray(&self, sat_threshold: f64, val_threshold: f64) -> bool {
        self.s < sat_threshold || self.v < val_threshold
    }
}

/// 色相直方图分析器
pub struct HueHistogram {
    bins: usize,
    histogram: Vec<u64>,
    sat_sum: Vec<f64>,
    val_sum: Vec<f64>,
    count: Vec<u64>,
    gray_sum: u64,
    gray_count: u64,
}

impl HueHistogram {
    /// 创建新的色相直方图
    /// bins: 色相区间数量（默认36，每个10度）
    pub fn new(bins: usize) -> Self {
        Self {
            bins,
            histogram: vec![0u64; bins],
            sat_sum: vec![0f64; bins],
            val_sum: vec![0f64; bins],
            count: vec![0u64; bins],
            gray_sum: 0,
            gray_count: 0,
        }
    }

    /// 添加一个像素到直方图
    pub fn add_pixel(&mut self, r: u8, g: u8, b: u8) {
        let hsv = Hsv::from_rgb_u8(r, g, b);

        // 低饱和度视为灰色，不参与色相统计
        if hsv.is_gray(0.15, 0.1) {
            let gray = ((r as u64 + g as u64 + b as u64) / 3) as u64;
            self.gray_sum += gray;
            self.gray_count += 1;
            return;
        }

        // 将色相映射到直方图区间
        let bin = ((hsv.h / 360.0) * self.bins as f64).floor() as usize % self.bins;

        // 使用饱和度作为权重，饱和度越高权重越大
        let weight = (hsv.s * hsv.v * 100.0) as u64;
        self.histogram[bin] += weight;
        self.sat_sum[bin] += hsv.s;
        self.val_sum[bin] += hsv.v;
        self.count[bin] += 1;
    }

    /// 获取主色调
    pub fn get_dominant_color(&self) -> Rgb<u8> {
        // 找到最显著的色相区间
        let dominant_bin = self
            .histogram
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        // 如果大部分像素是灰色，返回平均灰度
        let colored_pixels: u64 = self.count.iter().sum();
        if self.gray_count > colored_pixels * 2 {
            let avg_gray = if self.gray_count > 0 {
                (self.gray_sum / self.gray_count) as u8
            } else {
                128
            };
            return Rgb([avg_gray, avg_gray, avg_gray]);
        }

        // 计算该区间的平均色相、饱和度、明度
        let bin_count = self.count[dominant_bin];
        if bin_count == 0 {
            // 如果没有有效颜色，返回灰色
            let avg_gray = if self.gray_count > 0 {
                (self.gray_sum / self.gray_count) as u8
            } else {
                128
            };
            return Rgb([avg_gray, avg_gray, avg_gray]);
        }

        let avg_hue = (dominant_bin as f64 + 0.5) * (360.0 / self.bins as f64);
        let avg_sat = self.sat_sum[dominant_bin] / bin_count as f64;
        let avg_val = self.val_sum[dominant_bin] / bin_count as f64;

        // 稍微增强饱和度以获得更鲜艳的效果
        let hsv = Hsv::new(avg_hue, avg_sat, avg_val).enhance(1.2, 1.1);

        hsv.to_rgb_u8()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_hsv_red() {
        let hsv = Hsv::from_rgb(1.0, 0.0, 0.0);
        assert!((hsv.h - 0.0).abs() < 0.01);
        assert!((hsv.s - 1.0).abs() < 0.01);
        assert!((hsv.v - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_rgb_to_hsv_green() {
        let hsv = Hsv::from_rgb(0.0, 1.0, 0.0);
        assert!((hsv.h - 120.0).abs() < 0.01);
    }

    #[test]
    fn test_hsv_to_rgb_roundtrip() {
        let original = (0.5, 0.3, 0.8);
        let hsv = Hsv::from_rgb(original.0, original.1, original.2);
        let (r, g, b) = hsv.to_rgb();
        assert!((r - original.0).abs() < 0.01);
        assert!((g - original.1).abs() < 0.01);
        assert!((b - original.2).abs() < 0.01);
    }
}

