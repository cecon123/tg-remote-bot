# nokhwa::Camera

> Reference documentation for Camera and CaptureBackendTrait

## Camera Struct

The main `Camera` struct abstracts over all backends, providing a simplified interface.

### Construction

```rust
// Default backend selection
let camera = Camera::new(
    CameraIndex::Index(0),
    RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate),
)?;

// Specific backend
let camera = Camera::with_backend(
    CameraIndex::Index(0),
    RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate),
    ApiBackend::Video4Linux,
)?;

// Custom backend
let camera = Camera::with_custom(
    idx,
    api,
    Box::new(my_custom_backend),
);
```

### Stream Control

| Method | Description | Returns |
|--------|-------------|---------|
| `open_stream()` | Start capturing | `Result<()>` |
| `stop_stream()` | Stop capturing | `Result<()>` |
| `is_stream_open()` | Check if streaming | `bool` |

### Frame Capture

| Method | Description | Returns |
|--------|-------------|---------|
| `frame()` | Get decoded frame | `Result<Buffer>` |
| `frame_raw()` | Get raw frame (no processing) | `Result<Cow<[u8]>>` |
| `write_frame_to_buffer::<F>(buf)` | Write to pre-allocated buffer | `Result<()>` |
| `frame_texture::<F>(device, queue, label)` | Get wgpu Texture | `Result<Texture>` |

### Format Control

| Method | Description |
|--------|-------------|
| `camera_format()` | Get current CameraFormat |
| `set_camera_requset(request)` | Set format via RequestedFormat |
| `resolution()` / `set_resolution(res)` | Get/set resolution |
| `frame_rate()` / `set_frame_rate(fps)` | Get/set framerate |
| `frame_format()` / `set_frame_format(fmt)` | Get/set frame format |
| `compatible_camera_formats()` | List all supported formats |
| `compatible_list_by_resolution(fourcc)` | Format by resolution |
| `compatible_fourcc()` | List supported frame formats |

### Camera Info

| Method | Description |
|--------|-------------|
| `index()` | Get CameraIndex |
| `set_index(idx)` | Set CameraIndex (re-init) |
| `backend()` | Get ApiBackend |
| `set_backend(backend)` | Set ApiBackend (re-init) |
| `info()` | Get CameraInfo |

### Camera Controls

| Method | Description |
|--------|-------------|
| `supported_camera_controls()` | List supported KnownCameraControl |
| `camera_controls()` | List CameraControl objects |
| `camera_control(control)` | Get specific control |
| `set_camera_control(id, setter)` | Set control value |

### Trait Implementations

- `Drop` - Automatically stops stream and cleans up
- `Send` - With `camera-sync-impl` feature

## CallbackCamera (threaded)

```rust
use nokhwa::threaded::CallbackCamera;

let mut cam = CallbackCamera::new(
    CameraIndex::Index(0),
    format,
    |buffer: Buffer| {
        // Called on each frame in background thread
        let img = buffer.decode_image::<RgbFormat>().unwrap();
        // ...
    },
)?;
cam.open_stream()?;
```

### CallbackCamera Methods

| Method | Description |
|--------|-------------|
| `new(index, format, callback)` | Create with callback |
| `set_callback(f)` | Change callback |
| `open_stream()` | Start streaming |
| `stop_stream()` | Stop streaming |
| `last_frame()` | Get last captured Buffer |
| `frame_raw()` | Get raw last frame |
| `frame()` | Get decoded last frame |

## CaptureBackendTrait

The trait that all backends implement. Key methods:

```rust
pub trait CaptureBackendTrait {
    fn backend(&self) -> ApiBackend;
    fn camera_format(&self) -> CameraFormat;
    fn set_camera_requset(&mut self, request: RequestedFormat) -> Result<CameraFormat>;
    fn resolution(&self) -> Resolution;
    fn set_resolution(&mut self, res: Resolution) -> Result<()>;
    fn frame_rate(&self) -> u32;
    fn set_frame_rate(&mut self, fps: u32) -> Result<()>;
    fn frame_format(&self) -> FrameFormat;
    fn set_frame_format(&mut self, fmt: FrameFormat) -> Result<()>;
    fn open_stream(&mut self) -> Result<()>;
    fn is_stream_open(&self) -> bool;
    fn frame(&mut self) -> Result<Buffer>;
    fn frame_raw(&mut self) -> Result<Cow<'_, [u8]>>;
    fn stop_stream(&mut self) -> Result<()>;
    // ... control methods
}
```
