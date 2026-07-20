// Local fork note (enzyme AUD-661):
//
// Upstream gates the two renderers on their own features alone, so enabling both at once
// defines `GraphicsConfig`/`Renderer` twice and fails to build. Cargo unions features across
// the dependency graph, so a workspace where one crate asks for `opengl` and another asks for
// `wgpu` hits exactly that. Give `wgpu` precedence instead of erroring, which lets a consumer
// opt into wgpu without having to strip `opengl` from every other crate in the graph.
#[cfg(all(feature = "opengl", not(feature = "wgpu")))]
mod opengl;
#[cfg(all(feature = "opengl", not(feature = "wgpu")))]
pub use opengl::renderer::{GraphicsConfig, Renderer};

#[cfg(feature = "wgpu")]
mod wgpu;
#[cfg(feature = "wgpu")]
pub use wgpu::renderer::{GraphicsConfig, Renderer};
