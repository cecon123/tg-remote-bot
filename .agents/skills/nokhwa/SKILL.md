---
name: nokhwa
description: "Documentation for nokhwa crate. Keywords: webcam, camera, capture, video, frame, stream, image, v4l, mjpeg, yuyv, nv12, rgb, pixel, wgpu, callback"
---

# nokhwa

> **Version:** 0.10.10 | **Source:** docs.rs

## Overview

A simple-to-use, cross-platform Rust webcam capture library. Supports Linux (V4L2), macOS (AVFoundation), Windows (MediaFoundation), and WASM.

```toml
# Recommended setup
nokhwa = { version = "0.10", features = ["input-native"] }

# With threaded callback camera
nokhwa = { version = "0.10", features = ["input-native", "output-threaded"] }

# With wgpu texture output
nokhwa = { version = "0.10", features = ["input-native", "output-wgpu"] }
```

**Important:** At least one `input-*` feature must be enabled.

## Quick Start

### Basic Capture

```rust
use nokhwa::Camera;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};

// Create camera with default settings
let mut camera = Camera::new(
    CameraIndex::Index(0),
    RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(
        RequestedFormatType::AbsoluteHighestFrameRate
    ),
)?;

camera.open_stream()?;
let frame = camera.frame()?; // Get a decoded RGB frame
```

### With Specific Resolution

```rust
use nokhwa::Camera;
use nokhwa::utils::{CameraIndex, CameraFormat, Resolution, FrameFormat, RequestedFormat, RequestedFormatType};

let format = RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(
    RequestedFormatType::Closest(CameraFormat::new(
        Resolution::new(1280, 720),
        FrameFormat::MJPEG,
        30
    ))
);

let mut camera = Camera::new(CameraIndex::Index(0), format)?;
camera.open_stream()?;
```

### Query Available Cameras

```rust
use nokhwa::query;

let cameras = query(nokhwa::utils::ApiBackend::Auto)?;
for cam in &cameras {
    println!("{}: {}", cam.index(), cam.human_name());
}
```

## Key Types

### Camera

The main camera struct. Abstracts over all backends.

| Method | Description |
|--------|-------------|
| `new(index, format)` | Create camera |
| `with_backend(index, format, backend)` | Create with specific backend |
| `open_stream()` | Start capturing |
| `stop_stream()` | Stop capturing |
| `frame()` | Get decoded RGB buffer |
| `frame_raw()` | Get raw (unprocessed) frame |
| `resolution()` | Current resolution |
| `frame_rate()` | Current FPS |
| `frame_format()` | Current format (MJPEG, YUYV, etc.) |
| `set_camera_requset(request)` | Change format |
| `compatible_camera_formats()` | List supported formats |
| `supported_camera_controls()` | List supported controls |
| `camera_control(control)` | Get control value |
| `set_camera_control(id, value)` | Set control value |

### Buffer

Frame buffer containing resolution, format, and pixel data.

```rust
let buffer = camera.frame()?;
let resolution = buffer.resolution();
let format = buffer.source_frame_format();
let data: &[u8] = buffer.buffer(); // Raw bytes
let image = buffer.decode_image::<nokhwa::pixel_format::RgbFormat>()?; // As image
```

### CameraFormat / Resolution

```rust
use nokhwa::utils::{CameraFormat, Resolution, FrameFormat};

let res = Resolution::new(1920, 1080);
let format = CameraFormat::new(res, FrameFormat::MJPEG, 30);
```

### CameraIndex

```rust
use nokhwa::utils::CameraIndex;

CameraIndex::Index(0)    // By numeric index
CameraIndex::String("usb-xxx".to_string())  // By device ID (some platforms)
```

## Frame Formats

| Format | Description |
|--------|-------------|
| `FrameFormat::MJPEG` | JPEG-compressed frames |
| `FrameFormat::YUYV` | YUYV 4:2:2 (YUV color space) |
| `FrameFormat::NV12` | YUV 4:2:0 bi-planar |
| `FrameFormat::GRAY` | Grayscale |

## RequestedFormat

Controls how camera format is selected:

```rust
use nokhwa::utils::{RequestedFormat, RequestedFormatType, CameraFormat, Resolution, FrameFormat};

// Highest framerate possible
RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(
    RequestedFormatType::AbsoluteHighestFrameRate
)

// Highest resolution possible
RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(
    RequestedFormatType::AbsoluteHighestResolution
)

// Closest match to desired format
RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(
    RequestedFormatType::Closest(CameraFormat::new(
        Resolution::new(1280, 720),
        FrameFormat::MJPEG,
        30
    ))
)

// None - use device default
RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(
    RequestedFormatType::None
)
```

## Pixel Formats (FormatDecoder)

```rust
use nokhwa::pixel_format::{RgbFormat, RgbaFormat, LumaFormat};

// RgbFormat -> RGB888
// RgbaFormat -> RGBA8888
// LumaFormat -> Grayscale
```

## Threaded Camera (output-threaded)

```rust
use nokhwa::threaded::CallbackCamera;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};

let mut camera = CallbackCamera::new(
    CameraIndex::Index(0),
    RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(
        RequestedFormatType::AbsoluteHighestFrameRate
    ),
    |buffer| {
        // Called on each frame in a separate thread
        let frame = buffer.decode_image::<nokhwa::pixel_format::RgbFormat>().unwrap();
        // Process frame...
    },
)?;

camera.open_stream()?;
// Frames are delivered via callback
camera.stop_stream()?;
```

## Camera Controls

```rust
use nokhwa::utils::KnownCameraControl;

// List supported controls
let controls = camera.supported_camera_controls()?;

// Get brightness control
let brightness = camera.camera_control(KnownCameraControl::Brightness)?;

// Set brightness
camera.set_camera_control(
    KnownCameraControl::Brightness,
    nokhwa::utils::ControlValueSetter::Integer(128)
)?;
```

| Control | Description |
|---------|-------------|
| `Brightness` | Image brightness |
| `Contrast` | Image contrast |
| `Saturation` | Color saturation |
| `Sharpness` | Image sharpness |
| `Gain` | Analog gain |
| `Exposure` | Exposure time |
| `Focus` | Focus distance |
| `Zoom` | Zoom level |
| `WhiteBalance` | White balance temperature |
| `BacklightCompensation` | Backlight compensation |
| `Pan` / `Tilt` / `Roll` | Camera orientation |

## Feature Flags

| Feature | Description |
|---------|-------------|
| `input-native` | Use native platform backend (V4L2/AVFoundation/MediaFoundation) |
| `input-v4l` | Linux V4L2 backend |
| `input-avfoundation` | macOS AVFoundation backend |
| `input-mediafoundation` | Windows MediaFoundation backend |
| `input-opencv` | OpenCV-based backend |
| `input-uvc` | UVC (USB Video Class) backend |
| `input-wasm` | WASM/browser backend |
| `output-threaded` | Threaded callback camera |
| `output-wgpu` | WGPU texture output |
| `mjpeg` | MJPEG decoding |
| `camera-sync-impl` | `Send` impl for `Camera` |
| `serialize` | Serde support |

## Common Patterns

### Save Frame as Image

```rust
use nokhwa::pixel_format::RgbFormat;

let frame = camera.frame()?;
let img = frame.decode_image::<RgbFormat>()?;
img.save("frame.png")?;
```

### Continuous Capture Loop

```rust
loop {
    let frame = camera.frame()?;
    // Process frame...
    // Use break, select! or other mechanism to exit
}
```

### macOS Initialization

```rust
// macOS requires initialization before first use
#[cfg(target_os = "macos")]
nokhwa::nokhwa_initialize(|granted| {
    if !granted {
        eprintln!("Camera access denied");
    }
});
```

## Error Handling

```rust
use nokhwa::NokhwaError;

match camera.frame() {
    Ok(buffer) => { /* process */ }
    Err(NokhwaError::OpenStreamError(e)) => { eprintln!("Stream error: {}", e); }
    Err(NokhwaError::ReadFrameError(e)) => { eprintln!("Frame read error: {}", e); }
    Err(NokhwaError::UnitializedError) => { eprintln!("Not initialized"); }
    Err(e) => { eprintln!("Other error: {}", e); }
}
```

## Documentation

- `./references/camera.md` - Camera and CaptureBackendTrait
- `./references/utils.md` - Utilities, formats, and controls
- `./references/errors.md` - Error types

## Links

- [docs.rs](https://docs.rs/nokhwa)
- [crates.io](https://crates.io/crates/nokhwa)
- [GitHub](https://github.com/l1npengtul/nokhwa)
- [Examples](https://github.com/l1npengtul/nokhwa/tree/master/examples)
