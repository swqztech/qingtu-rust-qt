use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageEncoder, ImageFormat, RgbaImage};
use std::fs;
use std::path::{Path, PathBuf};

fn detect_format(path: &Path) -> ImageFormat {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        _ => ImageFormat::Png,
    }
}

fn resize_max(img: DynamicImage, max_side: i32) -> DynamicImage {
    if max_side <= 0 {
        return img;
    }
    let (w, h) = (img.width(), img.height());
    let longest = w.max(h);
    if longest <= max_side as u32 {
        return img;
    }
    let scale = max_side as f32 / longest as f32;
    let nw = ((w as f32) * scale).round().max(1.0) as u32;
    let nh = ((h as f32) * scale).round().max(1.0) as u32;
    img.resize(nw, nh, FilterType::Lanczos3)
}

fn default_out_dir(input: &Path) -> PathBuf {
    input
        .parent()
        .map(|p| p.join("compressed"))
        .unwrap_or_else(|| PathBuf::from("compressed"))
}

fn write_image(
    img: &DynamicImage,
    out: &Path,
    format: ImageFormat,
    quality: i32,
) -> Result<(), String> {
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    match format {
        ImageFormat::Jpeg => {
            let rgb = img.to_rgb8();
            let mut file = fs::File::create(out).map_err(|e| e.to_string())?;
            let q = quality.clamp(1, 100) as u8;
            let enc = JpegEncoder::new_with_quality(&mut file, q);
            enc.write_image(rgb.as_raw(), rgb.width(), rgb.height(), image::ColorType::Rgb8.into())
                .map_err(|e| e.to_string())?;
        }
        _ => {
            img.save_with_format(out, ImageFormat::Png)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub fn run_compress(
    paths: &[PathBuf],
    quality: i32,
    max_side: i32,
    format: i32,
    custom_out: bool,
    out_dir: &Path,
) -> String {
    if paths.is_empty() {
        return "请先添加图片".into();
    }
    let mut log = String::new();
    for path in paths {
        if !path.is_file() {
            log.push_str(&format!("{} → 不是有效文件\n", path.display()));
            continue;
        }
        let before = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        let img = match image::open(path) {
            Ok(i) => i,
            Err(e) => {
                log.push_str(&format!("{} → 解码失败: {}\n", path.display(), e));
                continue;
            }
        };
        let img = resize_max(img, max_side);
        let fmt = match format {
            1 => ImageFormat::Jpeg,
            2 => ImageFormat::Png,
            _ => detect_format(path),
        };
        let dir = if custom_out && !out_dir.as_os_str().is_empty() {
            out_dir.to_path_buf()
        } else {
            default_out_dir(path)
        };
        let ext = if fmt == ImageFormat::Jpeg { "jpg" } else { "png" };
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
        let out_path = dir.join(format!("{}.{}", stem, ext));
        if let Err(e) = write_image(&img, &out_path, fmt, quality) {
            log.push_str(&format!("{} → {}\n", path.display(), e));
            continue;
        }
        let after = fs::metadata(&out_path).map(|m| m.len()).unwrap_or(0);
        log.push_str(&format!(
            "{} → {} ({} → {} B)\n",
            path.display(),
            out_path.display(),
            before,
            after
        ));
    }
    log
}

fn scale_to_height(img: DynamicImage, target_h: u32) -> DynamicImage {
    if img.height() == target_h {
        return img;
    }
    let nw = ((img.width() as f64) * (target_h as f64) / (img.height() as f64))
        .round()
        .max(1.0) as u32;
    img.resize_exact(nw, target_h, FilterType::Lanczos3)
}

pub fn run_stitch(paths: &[PathBuf], custom_out: bool, out_dir: &Path) -> String {
    if paths.len() < 2 {
        return "拼接需要至少 2 张图".into();
    }
    let mut imgs: Vec<DynamicImage> = Vec::new();
    for p in paths {
        match image::open(p) {
            Ok(i) => imgs.push(i),
            Err(e) => return format!("{} 解码失败: {}", p.display(), e),
        }
    }
    let target_h = imgs.iter().map(|i| i.height()).min().unwrap_or(1);
    let scaled: Vec<DynamicImage> = imgs
        .into_iter()
        .map(|i| scale_to_height(i, target_h))
        .collect();
    let total_w: u32 = scaled.iter().map(|i| i.width()).sum();
    let mut canvas = RgbaImage::new(total_w, target_h);
    let mut x = 0u32;
    for im in &scaled {
        let rgba = im.to_rgba8();
        image::imageops::overlay(&mut canvas, &rgba, x.into(), 0);
        x += im.width();
    }
    let out = if custom_out && !out_dir.as_os_str().is_empty() {
        out_dir.join("stitched.png")
    } else if let Some(first) = paths.first() {
        default_out_dir(first).join("stitched.png")
    } else {
        PathBuf::from("stitched.png")
    };
    if let Some(parent) = out.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return format!("创建目录失败: {}", e);
        }
    }
    if let Err(e) = DynamicImage::ImageRgba8(canvas).save_with_format(&out, ImageFormat::Png) {
        return format!("写入失败: {}", e);
    }
    format!("拼接完成 → {}", out.display())
}
