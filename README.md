Code in this project has so far only been based on:
1. https://sotrh.github.io/learn-wgpu/intermediate/tutorial10-lighting/#ray-path-tracing
2. Copilot (primarily the depth buffer)

The code has been split between multiple files for easier extendability.

To build this project, this guide was used: https://github.com/gfx-rs/wgpu/wiki/Running-on-the-Web-with-WebGPU-and-WebGL

Instancing has been skipped as it was not necessary so far.

# Refactor plan:
 
1. Split renderer from core behaviour
- Core Struct
```rust
#[derive(Default)]
pub struct ButteryEngine {
    world_model: ButterWorldModel,
    renderer: Option<Box<dyn ButteryRenderer>>
}

impl ButterEngine {
    pub fn new() -> Self {
        let renderer = Box::new(StumblyRenderer::new()) as Box<dyn ButteryRenderer>;
        let mut self = Self {
            world_model: ButteryWorldModel::default(),
            renderer,
        };

        self.renderer.on_update = self.on_update;
        self.renderer.on_key_press = self.on_key_press;

        self
    }

    fn on_update(&mut self) {
        // Execute game code...

        self.renderer.set_world_model(self.world_model);
    }

    fn on_key_press(&mut self, key: ButterKey) {
        // Execute Keypress game code...
    }
}
```
- Renderer trait
```rust
pub trait ButteryRenderer {
    /// Sets what UI/HUD elements should be shown
    fn set_ui_model(&mut self, ui_model: ButteryUIModel);

    /// Sets stuff like objects, 3D Models, Lights, Cameras, etc.
    fn update_world_model(&mut self, world_model: ButterWorldModel);
}
```
- Asset loading as part of core but separate module

2. Move all non-engine code into example/test project