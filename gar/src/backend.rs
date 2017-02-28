use options;

/// A trait for configuration of window creation, implemented by each window type
pub trait WindowOptions {
    type Options;
    fn from_renderer_options(options: &options::RendererOptions) -> Self::Options;
}
