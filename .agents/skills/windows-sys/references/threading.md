# windows_sys::Win32::System::Threading

Process, thread, and synchronization primitives.

## Key Structs

| Struct | Description |
|--------|-------------|
| `PROCESS_INFORMATION` | Handle + ID from CreateProcess |
| `STARTUPINFOW` | Process startup configuration |
| `SRWLOCK` | Slim reader/writer lock |
| `CRITICAL_SECTION` | Legacy mutex |
| `CONDITION_VARIABLE` | Condition variable |
| `SYNCHRONIZATION_BARRIER` | Thread barrier |

## Process Creation

```rust
use windows_sys::Win32::System::Threading::{
    CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW,
    WaitForSingleObject, INFINITE,
};
use std::mem::zeroed;

unsafe {
    let mut si: STARTUPINFOW = zeroed();
    si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    let mut pi: PROCESS_INFORMATION = zeroed();

    let cmd = windows_sys::w!("notepad.exe");

    CreateProcessW(
        std::ptr::null(),
        cmd.as_ptr() as _,
        std::ptr::null(),
        std::ptr::null(),
        0,
        0,
        std::ptr::null(),
        std::ptr::null(),
        &si,
        &mut pi,
    );

    WaitForSingleObject(pi.hProcess, INFINITE);

    windows_sys::Win32::Foundation::CloseHandle(pi.hProcess);
    windows_sys::Win32::Foundation::CloseHandle(pi.hThread);
}
```

## Synchronization

| Type | Function | Description |
|------|----------|-------------|
| Mutex | `CreateMutexW` / `ReleaseMutex` | Named or unnamed mutex |
| Event | `CreateEventW` / `SetEvent` | Manual/auto reset event |
| Semaphore | `CreateSemaphoreW` / `ReleaseSemaphore` | Counting semaphore |
| SRWLock | `InitializeSRWLock` / `AcquireSRWLockExclusive` | Lightweight lock |
| CondVar | `SleepConditionVariableCS` / `WakeConditionVariable` | Condition variable |

## Wait Functions

| Function | Description |
|----------|-------------|
| `WaitForSingleObject` | Wait for one handle |
| `WaitForMultipleObjects` | Wait for multiple handles |
| `SleepEx` | Sleep with APC support |
| `SwitchToThread` | Yield to another thread |

## Priority Classes

| Constant | Description |
|----------|-------------|
| `IDLE_PRIORITY_CLASS` | Below normal |
| `NORMAL_PRIORITY_CLASS` | Default |
| `HIGH_PRIORITY_CLASS` | Time-critical |
| `REALTIME_PRIORITY_CLASS` | Highest possible |
