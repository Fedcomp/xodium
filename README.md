# Xodium

a **work in progress** modern x window system client written entirely in rust.

Xodium is a pure rust X11 client.
At this moment only linux platform is supported.

Opening connection is as easy as:
```rust
// Connect to X server using DISPLAY environment variable
let connection = xodium::connect_default();
```
