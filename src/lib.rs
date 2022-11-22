use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}


impl State {
    //  Creating some wgpu types requires async code
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        //  The instance is a handle to the GPU
        //  Backends::all => Vulkan + Metal + DX12 + WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", //  Vert shader entry point
                buffers: &[
                    Vertex::desc(),
                ], //    What type of verts we want to pass to the shader
            },
            fragment: Some(wgpu::FragmentState {    //  Technically optional, so we have to wrap it in Some()
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {    //  What color outputs wgpu should set up, currently we only need one for the surface
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE), //  Replace old pixel data with new data
                    write_mask: wgpu::ColorWrites::ALL, //  Write all 4 channels (RGBA)
                })],
            }),
                primitive: wgpu::PrimitiveState {   //  Describes how to interpret our vertices when converting them into tris
                topology: wgpu::PrimitiveTopology::TriangleList,    //  Means that every 3 vertices will correspond to 1 tri
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,   //  Determines if a tri is front-facing or not. Vertices are arranged in a counter-clockwise direction.
                cull_mode: Some(wgpu::Face::Back),  //  Triangles determined to be facing backwards are culled. Typical backface culling.
                //  Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                //  Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                //  Requires Features::CONSERVATIVE_RASTERIZATION
                //  republican triangles
                conservative: false,
            },
            depth_stencil: None,    //  Not currently using a depth/stencil buffer
            multisample: wgpu::MultisampleState {
                count: 1,   //  How many samples the pipeline will use (multisampling)
                mask: !0,   //  specifices which samples should be active - we are using all of them
                alpha_to_coverage_enabled: false,   //  related to anti-aliasing, false for now
            },
            multiview: None,    //  how many array layers the render attachments can have. we arent rendering to array textures so we set it to None.
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = INDICES.len() as u32;
        
        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        //  update code to move objects
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        //  the code blow is enclosed in a {} block so we drop the mutable borrow of encoder and can be used after
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        //  clear the screen before each frame
                        //  if the screen is completely covered by objects, then it is unneccessary to clear (see Blockland)
                        //  causes bugs if you do stuff like deleting the Skybox
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);    //  Set the pipeline to the one we have created
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);   //  Draw something with 3 vertices and 1 instance.
        }
        //  submit accepts anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

const VERTICES: &[Vertex] = &[  //  Vertices arranged in a counter-clockwise order. Top, bottom left, bottom right.
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.0, 1.0, 1.0] },
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [1.0, 0.0, 1.0] },
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) { 
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    //  Reconfiure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    //  System  is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    //  All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                //  RedrawRequested will only trigger once unless we manually request it
                window.request_redraw();
            }
            _ => {}
        }
    });
}