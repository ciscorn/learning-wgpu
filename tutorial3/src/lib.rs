use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{self, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowBuilder},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct State<'w> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'w>,
    surface_config: wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    // num_vertices: u32,
    num_indices: u32,

    size: winit::dpi::PhysicalSize<u32>,
    window: &'w Window,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct MyVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl MyVertex {
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        const ATRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<MyVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATRIBUTES,
        }
    }
}

impl<'w> State<'w> {
    async fn new(window: &'w Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window).unwrap();

        let (device, queue, surface_config) = {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        required_features: wgpu::Features::empty(),
                        // required_limits: wgpu::Limits::downlevel_defaults()
                        //     .using_resolution(adapter.limits()),
                        required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                            .using_resolution(adapter.limits()),
                    },
                    None,
                )
                .await
                .unwrap();

            // let config = surface
            //     .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            //     .unwrap();
            let surface_caps = surface.get_capabilities(&adapter);
            let format = surface_caps
                .formats
                .iter()
                .find(|&f| f.is_srgb())
                .unwrap_or(&surface_caps.formats[0]);
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: *format,
                width: size.width.max(1),
                height: size.height.max(1),
                present_mode: surface_caps.present_modes[0],
                desired_maximum_frame_latency: 2,
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: if !format.is_srgb() {
                    vec![format.add_srgb_suffix()]
                } else {
                    vec![]
                },
            };

            (device, queue, config)
        };

        surface.configure(&device, &surface_config);

        // Construct a render pipeline
        let render_pipeline = {
            let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
            });

            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[MyVertex::layout()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_config.format.add_srgb_suffix(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            })
        };

        // Create a vertex buffer
        const VERTICES: &[MyVertex] = &[
            MyVertex {
                position: [-0.0868241, 0.49240386, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // A
            MyVertex {
                position: [-0.49513406, 0.06958647, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // B
            MyVertex {
                position: [-0.21918549, -0.44939706, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // C
            MyVertex {
                position: [0.35966998, -0.3473291, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // D
            MyVertex {
                position: [0.44147372, 0.2347359, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // E
        ];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(VERTICES),
        });
        // let num_vertices = VERTICES.len() as u32;

        const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            usage: wgpu::BufferUsages::INDEX,
            contents: bytemuck::cast_slice(INDICES),
        });
        let num_indices = INDICES.len() as u32;

        State {
            surface,
            device,
            queue,
            surface_config,
            vertex_buffer,
            index_buffer,
            // num_vertices,
            num_indices,
            render_pipeline,
            size,
            window,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.size = new_size;
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.window.request_redraw();
    }

    fn update(&mut self) {
        // do nothing
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;
        let surface_texture_view =
            surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    format: Some(self.surface_config.format.add_srgb_suffix()),
                    ..Default::default()
                });

        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None, // for MSAA
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}

pub async fn run(event_loop: event_loop::EventLoop<()>, window: Window) {
    let mut state = State::new(&window).await;

    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent { event, window_id } if window_id == state.window.id() => {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        state.resize(physical_size);
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    } => {
                        target.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory"),
                            Err(err) => log::warn!("{:?}", err),
                        }
                        state.window.request_redraw();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .expect("event loop failed");
}

pub fn prepare_window() -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("winit window")
        .build(&event_loop)
        .unwrap();
    (event_loop, window)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    // Send logs to the web console.
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

    let (event_loop, window) = prepare_window();

    // Append the canvas to the document body.
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas().unwrap());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    wasm_bindgen_futures::spawn_local(run(event_loop, window));
}
