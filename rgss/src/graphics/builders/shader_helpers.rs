#![allow(dead_code)]
use std::num::NonZeroU32;

pub struct LayoutBuilder<'res> {
    device: &'res wgpu::Device,
    label: Option<&'res str>,
    bind_groups: Vec<&'res wgpu::BindGroupLayout>,
    push_constant_ranges: Vec<wgpu::PushConstantRange>,
}

impl<'a> LayoutBuilder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        Self {
            device,
            label: None,
            bind_groups: Vec::new(),
            push_constant_ranges: Vec::new(),
        }
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_bind_group(mut self, layout: &'a wgpu::BindGroupLayout) -> Self {
        self.bind_groups.push(layout);
        self
    }

    pub fn with_push_constant<T>(mut self, stages: wgpu::ShaderStages) -> Self {
        let start = self
            .push_constant_ranges
            .last()
            .map(|r| r.range.end)
            .unwrap_or(0);
        self.push_constant_ranges.push(wgpu::PushConstantRange {
            stages,
            range: start..start + size_of::<T>() as u32,
        });
        self
    }

    pub fn with_raw_push_constant(mut self, range: wgpu::PushConstantRange) -> Self {
        self.push_constant_ranges.push(range);
        self
    }

    pub fn build(self) -> wgpu::PipelineLayout {
        let layout_desc = wgpu::PipelineLayoutDescriptor {
            label: self.label,
            bind_group_layouts: &self.bind_groups,
            push_constant_ranges: &self.push_constant_ranges,
        };
        self.device.create_pipeline_layout(&layout_desc)
    }
}

pub struct FragmentStateBuilder<'res> {
    module: &'res wgpu::ShaderModule,
    compilation_options: wgpu::PipelineCompilationOptions<'res>,
    entry_point: Option<&'res str>,
    targets: Vec<Option<wgpu::ColorTargetState>>,
}

impl<'a> FragmentStateBuilder<'a> {
    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        Self {
            module,
            compilation_options: Default::default(),
            entry_point: None,
            targets: Vec::new(),
        }
    }

    pub fn with_compilation_options(
        mut self,
        options: wgpu::PipelineCompilationOptions<'a>,
    ) -> Self {
        self.compilation_options = options;
        self
    }

    pub fn with_entry_point(mut self, entry_point: &'a str) -> Self {
        self.entry_point = Some(entry_point);
        self
    }

    pub fn with_color_target(mut self, target: impl Into<Option<wgpu::ColorTargetState>>) -> Self {
        self.targets.push(target.into());
        self
    }

    pub fn build(&self) -> wgpu::FragmentState<'_> {
        wgpu::FragmentState {
            module: self.module,
            entry_point: self.entry_point,
            compilation_options: self.compilation_options.clone(),
            targets: &self.targets,
        }
    }
}

pub struct VertexStateBuilder<'res> {
    module: &'res wgpu::ShaderModule,
    compilation_options: wgpu::PipelineCompilationOptions<'res>,
    entry_point: Option<&'res str>,
    buffers: Vec<wgpu::VertexBufferLayout<'res>>,
}

impl<'a> VertexStateBuilder<'a> {
    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        Self {
            module,
            compilation_options: Default::default(),
            entry_point: None,
            buffers: Vec::new(),
        }
    }

    pub fn with_compilation_options(
        mut self,
        options: wgpu::PipelineCompilationOptions<'a>,
    ) -> Self {
        self.compilation_options = options;
        self
    }

    pub fn with_entry_point(mut self, entry_point: &'a str) -> Self {
        self.entry_point = Some(entry_point);
        self
    }

    pub fn with_buffer(mut self, buffer: wgpu::VertexBufferLayout<'a>) -> Self {
        self.buffers.push(buffer);
        self
    }

    pub fn build(&self) -> wgpu::VertexState<'_> {
        wgpu::VertexState {
            module: self.module,
            entry_point: self.entry_point,
            compilation_options: self.compilation_options.clone(),
            buffers: &self.buffers,
        }
    }
}

pub struct RenderPipelineBuilder<'res> {
    device: &'res wgpu::Device,
    label: Option<&'res str>,
    layout: Option<&'res wgpu::PipelineLayout>,
    vertex_state: wgpu::VertexState<'res>,
    fragment_state: Option<wgpu::FragmentState<'res>>,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    multiview: Option<NonZeroU32>,
    cache: Option<&'res wgpu::PipelineCache>,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn new(
        device: &'a wgpu::Device,
        vertex_state: wgpu::VertexState<'a>,
        primitive: wgpu::PrimitiveState,
    ) -> Self {
        Self {
            device,
            label: None,
            layout: None,
            vertex_state,
            fragment_state: None,
            primitive,
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        }
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_layout(mut self, layout: &'a wgpu::PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    pub fn with_fragment_state(mut self, fragment_state: wgpu::FragmentState<'a>) -> Self {
        self.fragment_state = Some(fragment_state);
        self
    }

    pub fn with_depth_stencil(mut self, depth_stencil: wgpu::DepthStencilState) -> Self {
        self.depth_stencil = Some(depth_stencil);
        self
    }

    pub fn with_multisample(mut self, multisample: wgpu::MultisampleState) -> Self {
        self.multisample = multisample;
        self
    }

    pub fn with_multiview(mut self, multivew: NonZeroU32) -> Self {
        self.multiview = Some(multivew);
        self
    }

    pub fn with_cache(mut self, cache: &'a wgpu::PipelineCache) -> Self {
        self.cache = Some(cache);
        self
    }

    pub fn build(self) -> wgpu::RenderPipeline {
        let desc = wgpu::RenderPipelineDescriptor {
            label: self.label,
            layout: self.layout,
            vertex: self.vertex_state,
            primitive: self.primitive,
            depth_stencil: self.depth_stencil,
            multisample: self.multisample,
            fragment: self.fragment_state,
            multiview: self.multiview,
            cache: self.cache,
        };
        self.device.create_render_pipeline(&desc)
    }
}
