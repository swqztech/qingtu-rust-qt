# 轻图 Rust + Qt

Windows 桌面版「轻图」：**Qt 6 Widgets** 界面 + **Rust** 图像处理（`image` 库）。

- 多图选择、压缩（质量 / 最长边 / JPEG·PNG·保持原格式）
- 横向拼接，输出 `compressed/stitched.png`
- 与 `qingtu-cpp` / `qingtu-android` 能力对齐

## 依赖

- Qt 6.8（本机默认：`../.qt/6.8.2/msvc2022_64`，可用 `CMAKE_PREFIX_PATH` 覆盖）
- Rust toolchain（`rustup`）
- Visual Studio 2022（MSVC）
- CMake 3.21+

## 构建

```powershell
cmake -B build -DCMAKE_PREFIX_PATH=D:\codes\soft\.qt\6.8.2\msvc2022_64
cmake --build build --config Release
```

产物：`build/Release/qingtu-rust-qt.exe`（post-build 会尝试 `windeployqt`）。

## 结构

```
rust/           # qingtu_core 静态库 + C ABI
cpp/            # Qt 主窗口
```
