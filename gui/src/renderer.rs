use pixels::{
    wgpu::{self, util::DeviceExt},
    PixelsContext,
};
use ultraviolet::Mat4;
use wgpu::{Extent3d, Texture, TextureDescriptor, TextureDimension, TextureUsages, TextureView};

// this is mostly copied from the NoiseRenderer example

pub(crate) struct TransformRenderer {
    textures: Textures,
    sampler: wgpu::Sampler,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    transform_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
}

impl TransformRenderer {
    pub(crate) fn new(pixels: &pixels::Pixels, width: u32, height: u32) -> Self {
        let device = pixels.device();
        let shader = wgpu::include_wgsl!("../shaders/transform.wgsl");
        let module = device.create_shader_module(&shader);

        let textures = Textures::create(pixels, width, height);

        // Create a texture sampler with nearest neighbor
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("TransformRenderer sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 1.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        });

        // Create vertex buffer; array-of-array of position and texture coordinates
        let vertex_data: [[f32; 2]; 3] = [
            // One full-screen triangle
            // See: https://github.com/parasyte/pixels/issues/180
            [-1.0, -1.0],
            [3.0, -1.0],
            [-1.0, 3.0],
        ];
        let vertex_data_slice = bytemuck::cast_slice(&vertex_data);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TransformRenderer vertex buffer"),
            contents: vertex_data_slice,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: (vertex_data_slice.len() / vertex_data.len()) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        };

        let matrix = Mat4::identity();
        let transform_bytes = matrix.as_byte_slice();
        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TransformRenderer_matrix_transform_buffer"),
            contents: transform_bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });
        let bind_group = create_bind_group(
            device,
            &bind_group_layout,
            &textures,
            &sampler,
            &transform_buffer,
        );

        // Create pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("TransformRenderer pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("TransformRenderer pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: "fs_main",
                targets: &[
                    wgpu::ColorTargetState {
                        format: pixels.render_texture_format(),
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    },
                    wgpu::ColorTargetState {
                        format: pixels.render_texture_format(),
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    },
                ],
            }),
            multiview: None,
        });

        Self {
            textures,
            sampler,
            bind_group_layout,
            bind_group,
            render_pipeline,
            transform_buffer,
            vertex_buffer,
        }
    }

    pub(crate) fn get_texture_view(&self) -> &TextureView {
        &self.textures.input_view
    }

    pub(crate) fn resize(&mut self, pixels: &pixels::Pixels, width: u32, height: u32) {
        self.textures = Textures::create(pixels, width, height);
        self.bind_group = create_bind_group(
            pixels.device(),
            &self.bind_group_layout,
            &self.textures,
            &self.sampler,
            &self.transform_buffer,
        );
    }

    pub(crate) fn update(&self, queue: &wgpu::Queue, transform_matrix: Mat4) {
        queue.write_buffer(&self.transform_buffer, 0, transform_matrix.as_byte_slice());
    }

    pub(crate) fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &TextureView,
        context: &PixelsContext,
    ) {
        let clip_rect = context.scaling_renderer.clip_rect();
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("TransformRenderer render pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &self.textures.intermediate_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: true,
                        },
                    },
                    wgpu::RenderPassColorAttachment {
                        view: render_target,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    },
                ],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.set_scissor_rect(clip_rect.0, clip_rect.1, clip_rect.2, clip_rect.3);
            rpass.draw(0..3, 0..1);
        }
    }

    pub fn copy_texture_back(&self, encoder: &mut wgpu::CommandEncoder) {
        println!("copy_texture_back");
        encoder.copy_texture_to_texture(
            self.textures.intermediate.as_image_copy(),
            self.textures.background.as_image_copy(),
            self.textures.size,
        );
    }
}

pub struct Textures {
    pub size: Extent3d,
    pub input: Texture,
    pub input_view: TextureView,
    pub intermediate: Texture,
    pub intermediate_view: TextureView,
    pub background: Texture,
    pub background_view: TextureView,
}

impl Textures {
    pub fn create(pixels: &pixels::Pixels, width: u32, height: u32) -> Self {
        dbg!(pixels.render_texture_format());
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let (input, input_view) = Self::create_texture(
            pixels,
            size,
            TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
        );
        let (intermediate, intermediate_view) = Self::create_texture(
            pixels,
            size,
            TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_SRC,
        );
        let (background, background_view) = Self::create_texture(
            pixels,
            size,
            TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST,
        );
        Self {
            size,
            input,
            input_view,
            intermediate,
            intermediate_view,
            background,
            background_view,
        }
    }

    fn create_texture(
        pixels: &pixels::Pixels,
        size: Extent3d,
        texture_usages: TextureUsages,
    ) -> (Texture, TextureView) {
        let texture = pixels.device().create_texture(&TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: pixels.render_texture_format(),
            usage: texture_usages,
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        (texture, view)
    }
}

fn _create_texture_view(
    pixels: &pixels::Pixels,
    width: u32,
    height: u32,
) -> (Texture, TextureView) {
    let device = pixels.device();
    let texture_descriptor = TextureDescriptor {
        label: None,
        size: pixels::wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: pixels.render_texture_format(),
        usage: TextureUsages::TEXTURE_BINDING
            | TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::COPY_SRC,
    };

    let texture = device.create_texture(&texture_descriptor);
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

fn create_bind_group(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    textures: &Textures,
    sampler: &wgpu::Sampler,
    transform_buffer: &wgpu::Buffer,
) -> pixels::wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&textures.input_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: transform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&textures.background_view),
            },
        ],
    })
}
