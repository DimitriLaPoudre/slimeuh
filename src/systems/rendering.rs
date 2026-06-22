use crate::systems::{
    input::Input,
    renderer::{Renderer, RendererConfig},
};

pub fn create_rendering(config: RendererConfig) -> (Input, Renderer) {
    let renderer = Renderer::new(config);
    let input = Input::new(renderer.get_window());

    return (input, renderer);
}
