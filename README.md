<!-- inline html -->

<div align="center">

# 🕯️ Candle
A cross-platform toy real-time renderer made with Rust 🦀 and wgpu.
</div>

## Goals 
**This project doesn't aim to be a production renderer!** I'm making it to better understand various computer graphics concepts and hone my skills in Rust.

That said, here's a rough plan of what I'll try to implement:

### Phase 1: Basic Forward Rendering
#### Core
- [x] Basic window initialization, egui, wgpu integration
- [ ] A simple perspective camera, movement logic
- [ ] GLTF meshes
- [ ] Basic textures (albedo, normal)

#### Simple Lightning (Forward Rendering)
- [ ] Cook-Torrance PBR shading
- [ ] Directional & point lights
- [ ] Shadow mapping
- [ ] Support for HDR skybox

#### Optimizations
- [ ] Frustum culling
- [ ] Depth pre-pass
- [ ] Mipmaps

### Phase 2: Deferred Rendering
- [ ] Will add stuff here later

### Phase 3: Ray Tracing
- [ ] Ray-traced shadows
- [ ] Ray-traced reflections
