# Floem Android 壳工程

由 CI 自动打成 APK，屏幕上由 Floem 渲染 **test**。零 Java / Kotlin。

## 关键约束

| 项 | 值 | 原因 |
|---|---|---|
| `minSdk` | **26** | `android-activity 0.6` + winit 下限，填低装不上 |
| `abiFilters` | `arm64-v8a` | .so 是 aarch64 |
| `hasCode` | `false` | 无 Java，入口是 `ANativeActivity_onCreate` |
| `lib_name` | `floem_android_demo` | 去掉 `lib` 前缀与 `.so` 后缀，写错即闪退 |

## 闪退排查

`adb logcat | grep -E "AndroidRuntime|floem|winit"`

1. `Couldn't load libfloem_android_demo` → lib_name 或 abi 不匹配
2. `An AndroidApp as passed to android_main() is required` → .so 不是当前代码编的
3. renderer panic → vello 走 wgpu/Vulkan，设备无 Vulkan
