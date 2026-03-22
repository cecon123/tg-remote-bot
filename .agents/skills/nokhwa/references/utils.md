# nokhwa::utils

> Reference documentation for utility types, formats, and controls

## Resolution

```rust
use nokhwa::utils::Resolution;

let res = Resolution::new(1920, 1080);
let width = res.width();   // u32
let height = res.height(); // u32
```

## FrameFormat (FourCC)

```rust
use nokhwa::utils::FrameFormat;

FrameFormat::MJPEG   // JPEG compressed
FrameFormat::YUYV    // YUYV 4:2:2
FrameFormat::NV12    // YUV 4:2:0 bi-planar
FrameFormat::GRAY    // Grayscale
```

## CameraFormat

```rust
use nokhwa::utils::{CameraFormat, Resolution, FrameFormat};

let fmt = CameraFormat::new(
    Resolution::new(1920, 1080),
    FrameFormat::MJPEG,
    30  // FPS
);

let res = fmt.resolution();
let fps = fmt.frame_rate();
let fourcc = fmt.format();
```

## RequestedFormat

```rust
use nokhwa::utils::{RequestedFormat, RequestedFormatType, CameraFormat};

// With specific pixel format type
RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

// Possible RequestedFormatType variants:
RequestedFormatType::AbsoluteHighestFrameRate
RequestedFormatType::AbsoluteHighestResolution
RequestedFormatType::Closest(CameraFormat { ... })
RequestedFormatType::Exact(CameraFormat { ... })
RequestedFormatType::None  // Device default
```

## CameraIndex

```rust
use nokhwa::utils::CameraIndex;

CameraIndex::Index(0)                       // Numeric index
CameraIndex::String("device-id".into())     // Device string ID (some platforms)
```

## CameraInfo

```rust
use nokhwa::utils::CameraInfo;

// Contains:
// - description: String
// - misc: String
// - human_name: String
// - index: CameraIndex
```

## ApiBackend

```rust
use nokhwa::utils::ApiBackend;

ApiBackend::Auto                // Platform default
ApiBackend::Video4Linux         // Linux V4L2
ApiBackend::AVFoundation        // macOS
ApiBackend::MediaFoundation     // Windows
ApiBackend::GStreamer           // GStreamer
ApiBackend::OpenCv              // OpenCV
ApiBackend::UniversalVideoClass // UVC
```

## KnownCameraControl

```rust
use nokhwa::utils::KnownCameraControl;

KnownCameraControl::Brightness
KnownCameraControl::Contrast
KnownCameraControl::Hue
KnownCameraControl::Saturation
KnownCameraControl::Sharpness
KnownCameraControl::Gain
KnownCameraControl::Exposure
KnownCameraControl::Focus
KnownCameraControl::Zoom
KnownCameraControl::WhiteBalance
KnownCameraControl::BacklightCompensation
KnownCameraControl::Pan
KnownCameraControl::Tilt
KnownCameraControl::Roll
```

## CameraControl

```rust
use nokhwa::utils::{CameraControl, KnownCameraControl, ControlValueDescription, ControlValueSetter};

let control = camera.camera_control(KnownCameraControl::Brightness)?;
let value = control.value();        // ControlValueDescription
let name = control.name();          // &str
let flag = control.flag();          // KnownCameraControlFlag
let min = control.min_value();
let max = control.max_value();
let step = control.step();
```

## ControlValueSetter

```rust
use nokhwa::utils::ControlValueSetter;

ControlValueSetter::Integer(i64)
ControlValueSetter::Boolean(bool)
ControlValueSetter::String(String)
```

## ControlValueDescription

```rust
use nokhwa::utils::ControlValueDescription;

ControlValueDescription::Integer { value, default, min, max, step }
ControlValueDescription::Boolean { value, default }
ControlValueDescription::String { value }
```

## Color Conversion Utilities

```rust
use nokhwa::utils::{mjpeg_to_rgb, yuyv422_to_rgb, nv12_to_rgb};

// MJPEG to RGB
let rgb = mjpeg_to_rgb(&mjpeg_data)?;

// YUYV to RGB
let rgb = yuyv422_to_rgb(&yuyv_data);

// NV12 to RGB
let rgb = nv12_to_rgb(&nv12_data, width, height);
```

## Helper Functions

```rust
use nokhwa::utils::{frame_formats, color_frame_formats};

let all = frame_formats();          // All FrameFormat values
let color = color_frame_formats();  // Color-only formats
```
