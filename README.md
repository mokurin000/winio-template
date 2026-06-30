# winio-template

This is an example project for `winio` targeting Android, Windows, macOS, Linux, and iOS.

The included CI workflow is ready to build and deploy prebuilt binaries for Windows, macOS, and Linux, as well as `aarch64`, `armv7`, and `x86_64` Android APKs.

You may also find [cargo-bundle](https://github.com/burtonageo/cargo-bundle) useful for packaging `.app`/`.ipa` bundles for macOS and iOS, or creating Windows installers.

## Getting Started

To set up your application ID, replace every occurrence of `io.github.mokurin000.example` throughout the project.

> **Note:** For the best Linux compatibility, the application ID should be a valid D-Bus well-known name.

The `APP_ID` constant in `lib.rs` does **not** affect the runtime behavior on Windows, macOS, iOS, or Android. In particular, it is unrelated to the Windows AppUserModelID (AUMID).

## Android Classes

To reduce the size of release APKs and improve security, R8 is used to shrink, optimize, and obfuscate Java classes.

However, R8 cannot automatically update JNI class references used by native code. As a result, you must either keep the required classes with ProGuard rules or disable R8 entirely.

Also, check `android/gradle/libs.version.toml` and `android/settings.gradle` for additional changes that may be required in real-world projects.

## Trouble Shoot

### Why `win10.exe`?

Although the WinUI 3 backend is technically supported on Windows 10, it requires bundling additional framework DLLs and a small patch to `winio` to use the portable `MddAddPackageDependency` API for dependency resolution.[^1]

For simplicity, this template builds Windows 10 executables using the `win32` backend instead, with the Mica backdrop disabled.

[^1]: https://github.com/compio-rs/winio/issues/116

### Rounded Corners

See [qrcode-gen@2939502](https://github.com/mokurin000/qrcode-gen/commit/2939502161d6f2b4345c9566dc6bdb761ae151dc) for a simple implementation that excludes the bottom rounded corners from the Android view size. You can also use it as a reference for implementing more sophisticated margin handling with proper DPI scaling.

### Android Signing Key

To generate your own keystore, utilize keytool from Java Development Kit:

```bash
keytool -genkey -alias testkey -keyalg RSA -keysize 2048 -validity 36500 -keystore mykey.keystore -storepass storepass
```

You should update your ***Repository secret*** `ANDROID_KEYSTORE_BASE64` and `android/key.properties` accordingly.
