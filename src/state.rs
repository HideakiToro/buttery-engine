use bytemuck::bytes_of;
use cgmath::Vector3;
use std::sync::Arc;
use web_time::{Duration, Instant};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

use crate::{
    camera::{Camera, CameraUniform},
    camera_controller::CameraController,
    glb_parser,
    mesh::Mesh,
    offset::OffsetUniform,
    vertex::Vertex,
};

// This will store the state of the game
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    meshes: Vec<Mesh>,
    depth_format: wgpu::TextureFormat,
    last_frame_time: Instant,
    // start_time: Instant,
    pub window: Arc<Window>,
    // w_pressed: bool,
    // a_pressed: bool,
    // s_pressed: bool,
    // d_pressed: bool,
    forward_progress: f32,
    side_progress: f32,
    delta_time: f32,
    offset: OffsetUniform,
    offset_buffer: Buffer,
    offset_bind_group: BindGroup,
    show_frame_times: bool,
    camera: Camera,
    camera_controller: CameraController,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    pub egui_ctx: egui::Context,
    pub egui_state: egui_winit::State,
    pub egui_renderer: egui_wgpu::Renderer,

    open_menu: bool,
    debug_text: String,
}

impl State {
    // We don't need this to be async right now,
    // but we will in the next tutorial
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let width = if size.width < 1 { 1 } else { size.width };
        let height = if size.height < 1 { 1 } else { size.height };
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: width,
            height: height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        // Movement
        let offset = OffsetUniform {
            offset: [0.0, 0.0, 0.0, 0.0],
        };

        let offset_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Offset Buffer"),
            contents: bytes_of(&offset),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let offset_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Offset BindGroup Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    count: None,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });

        let offset_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Offset Bind Group"),
            layout: &offset_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: offset_buffer.as_entire_binding(),
            }],
        });

        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let camera_controller = CameraController::new(5.0);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // More Bindgroup stuff here...

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let depth_format = wgpu::TextureFormat::Depth24Plus;

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &offset_bind_group_layout,
                    &texture_bind_group_layout,
                    &camera_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // 1.
                buffers: &[Vertex::desc()],   // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }), // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None,     // 6.
        });

        let mut meshes = Vec::new();

        let mut glb_meshes = glb_parser::parse_glb(
            "./models/gitf-gitb/cubes.glb",
            &device,
            &queue,
            &texture_bind_group_layout,
        )
        .await?;
        meshes.append(&mut glb_meshes);

        let egui_ctx = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            None,
            None,
            None,
        );
        let egui_renderer = egui_wgpu::Renderer::new(
            &device,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            egui_wgpu::RendererOptions::PREDICTABLE,
        );

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
            window,
            meshes,
            depth_format,
            last_frame_time: web_time::Instant::now(),
            // start_time: web_time::Instant::now(),
            // w_pressed: false,
            // a_pressed: false,
            // s_pressed: false,
            // d_pressed: false,
            forward_progress: 0.5,
            side_progress: 0.5,
            delta_time: 1.0 / 60.0,
            offset,
            offset_buffer,
            offset_bind_group,
            show_frame_times: false,
            camera,
            camera_controller,
            camera_bind_group,
            camera_buffer,
            camera_uniform,

            egui_ctx,
            egui_state,
            egui_renderer,

            open_menu: false,
            debug_text: "I am debug text".to_string(),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            #[cfg(target_arch = "wasm32")]
            let (width, height) = {
                let width = if width > 2048 { 2048 } else { width };
                let height = if height > 2048 { 2048 } else { height };
                (width, height)
            };
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // I don't see a logical reason for this to panic, but it might, so it could cause instability...
        // let progress = now
        //     .checked_duration_since(self.start_time)
        //     .unwrap()
        //     .as_secs_f32()
        //     % 2.0;
        // println!("{progress}");

        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.depth_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // Offset
            self.offset.offset[2] = self.forward_progress * 4.0 - 2.0;
            self.offset.offset[0] = self.side_progress * 4.0 - 2.0;
            self.queue
                .write_buffer(&self.offset_buffer, 0, bytes_of(&self.offset));
            render_pass.set_bind_group(0, &self.offset_bind_group, &[]);

            //Camera
            render_pass.set_bind_group(2, &self.camera_bind_group, &[]);

            // Cloning Meshes here would literally clone every model on each frame...
            for mesh in &self.meshes {
                render_pass.set_bind_group(1, &mesh.texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), mesh.index_format);
                render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
            }
        }

        // Start UI-Setup
        let raw_input = self.egui_state.take_egui_input(&self.window);

        let mut style = (*self.egui_ctx.style()).clone();
        // Example: rounder corners and more spacing
        style.visuals.window_corner_radius = 20.0.into();
        style.visuals.window_fill = egui::Color32::from_rgb(0, 0, 255);
        style.visuals.collapsing_header_frame = false;
        self.egui_ctx.set_style(style);

        self.egui_ctx.begin_pass(raw_input);

        // egui::TopBottomPanel::top("app")
        //     .frame(egui::Frame::NONE.fill(egui::Color32::from_rgb(30, 30, 255)))
        //     .show(&self.egui_ctx, |ui| {
        //         ui.label("Hello from egui!");
        //         ui.label(format!("Meshes: {}", self.meshes.len()));
        //         ui.separator();
        //         ui.text_edit_singleline(&mut self.debug_text);
        //         if ui.button("Print text").clicked() {
        //             println!("Text: {}", self.debug_text);
        //         }
        //     });

        if self.open_menu {
            egui::Area::new("central_panel".into())
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(&self.egui_ctx, |ui| {
                    // Constrain the whole area
                    ui.set_max_width(600.0);
                    ui.set_max_height(400.0);
                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(30, 30, 40))
                        .corner_radius(10.0)
                        .inner_margin(egui::Margin::same(16))
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label("Hello from egui!");
                                ui.label(format!("Meshes: {}", self.meshes.len()));
                                ui.separator();
                                ui.text_edit_singleline(&mut self.debug_text);
                                if ui.button("Print text").clicked() {
                                    println!("Text: {}", self.debug_text);
                                }
                            });
                        });
                });
        }

        // egui::Window::new("Debug UI").show(&self.egui_ctx, |ui| {
        //     ui.label("Hello from egui!");
        //     ui.label(format!("Meshes: {}", self.meshes.len()));
        //     ui.separator();
        //     ui.text_edit_singleline(&mut self.debug_text);
        //     if ui.button("Print text").clicked() {
        //         println!("Text: {}", self.debug_text);
        //     }
        // });
        // End egui frame
        let full_output = self.egui_ctx.end_pass();
        let paint_jobs = self
            .egui_ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        // Apply window commands (cursor, IME, etc.)
        let platform_output = full_output.platform_output;
        self.egui_state
            .handle_platform_output(&self.window, platform_output);

        // Upload textures
        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer
                .update_texture(&self.device, &self.queue, *id, image_delta);
        }
        // End UI-Setup

        // Render UI over 3D-Meshes
        let screen_desc = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window.scale_factor() as f32,
        };
        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &paint_jobs,
            &screen_desc,
        );

        let ui_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui UI pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        let mut ui_pass = ui_pass.forget_lifetime();
        self.egui_renderer
            .render(&mut ui_pass, &paint_jobs, &screen_desc);
        drop(ui_pass);

        // Free textures that egui wants to remove
        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            // (KeyCode::KeyW, pressed) => {
            //     self.w_pressed = pressed;
            // }
            // (KeyCode::KeyA, pressed) => {
            //     self.a_pressed = pressed;
            // }
            // (KeyCode::KeyS, pressed) => {
            //     self.s_pressed = pressed;
            // }
            // (KeyCode::KeyD, pressed) => {
            //     self.d_pressed = pressed;
            // }
            (KeyCode::KeyE, true) => {
                self.open_menu = !self.open_menu;
            }
            (code, is_pressed) if !self.open_menu => {
                self.camera_controller.handle_key(code, is_pressed);
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // let mut speed = 1.0_f32;
        // if (self.d_pressed || self.a_pressed) && (self.w_pressed || self.s_pressed) {
        //     speed = (speed * 2.0).sqrt() / 2.0;
        // }

        // if self.d_pressed {
        //     self.side_progress += self.delta_time * speed;
        //     // self.side_progress %= 2.0;
        // }
        // if self.a_pressed {
        //     self.side_progress -= self.delta_time * speed;
        //     // if self.side_progress < 0.0 {
        //     //     self.side_progress += 2.0;
        //     // }
        // }
        // if self.w_pressed {
        //     self.forward_progress += self.delta_time * speed;
        //     // self.forward_progress %= 2.0;
        // }
        // if self.s_pressed {
        //     self.forward_progress -= self.delta_time * speed;
        //     // if self.forward_progress < 0.0 {
        //     //     self.forward_progress += 2.0;
        //     // }
        // }

        self.camera_controller
            .update_camera(&mut self.camera, self.delta_time);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn calc_time(&mut self) {
        let now = web_time::Instant::now();
        let delta = now
            .checked_duration_since(self.last_frame_time)
            .unwrap_or(Duration::from_millis(16))
            .as_secs_f32();
        // println!("Rendered in {:#?}ms", delta);

        if self.show_frame_times {
            let delta_micro = now
                .checked_duration_since(self.last_frame_time)
                .unwrap_or(Duration::from_millis(16))
                .as_micros();

            println!("{delta_micro}");
        }

        self.last_frame_time = now;
        self.delta_time = delta;
    }
}
