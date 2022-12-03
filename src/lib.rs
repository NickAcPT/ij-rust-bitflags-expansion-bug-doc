use bitflags::bitflags;
// Repro example taken from wgpu-types
bitflags! {
    #[repr(transparent)]
    #[derive(Default)]
    pub struct ReproFlags: u64 {
        /// `var textures: binding_array<texture_2d<f32>, 10>` (WGSL)\
        /// `uniform texture2D textures[10]` (GLSL)
        const TEXTURE_BINDING_ARRAY = 1 << 17;
    }
}
