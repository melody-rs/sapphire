#![allow(dead_code)]

pub struct BindGroupLayoutBuilder<'res> {
    device: &'res wgpu::Device,
    label: wgpu::Label<'res>,
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl<'res> BindGroupLayoutBuilder<'res> {
    pub fn new(device: &'res wgpu::Device) -> Self {
        Self {
            device,
            label: None,
            entries: Vec::new(),
        }
    }

    pub fn label(mut self, label: &'res str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn append(
        mut self,
        visibility: wgpu::ShaderStages,
        ty: wgpu::BindingType,
        count: Option<std::num::NonZeroU32>,
    ) -> Self {
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.entries.len() as u32,
            visibility,
            ty,
            count,
        });
        self
    }

    #[must_use]
    pub fn build(self) -> wgpu::BindGroupLayout {
        let descriptor = wgpu::BindGroupLayoutDescriptor {
            label: self.label,
            entries: &self.entries,
        };
        self.device.create_bind_group_layout(&descriptor)
    }
}

pub struct BindGroupBuilder<'res> {
    device: &'res wgpu::Device,
    label: wgpu::Label<'res>,
    layout: &'res wgpu::BindGroupLayout,
    entries: Vec<wgpu::BindGroupEntry<'res>>,
}

impl<'res> BindGroupBuilder<'res> {
    pub fn new(device: &'res wgpu::Device, layout: &'res wgpu::BindGroupLayout) -> Self {
        Self {
            label: None,
            device,
            layout,
            entries: Vec::new(),
        }
    }

    pub fn label(mut self, label: &'res str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn append(mut self, resource: wgpu::BindingResource<'res>) -> Self {
        self.entries.push(wgpu::BindGroupEntry {
            binding: self.entries.len() as u32,
            resource,
        });
        self
    }

    pub fn append_buffer(self, buffer: &'res wgpu::Buffer) -> Self {
        self.append(buffer.as_entire_binding())
    }

    pub fn append_buffer_with_size(self, buffer: &'res wgpu::Buffer, size: u64) -> Self {
        self.append(wgpu::BindingResource::Buffer(wgpu::BufferBinding {
            buffer,
            offset: 0,
            size: std::num::NonZeroU64::new(size),
        }))
    }

    pub fn append_sampler(self, sampler: &'res wgpu::Sampler) -> Self {
        self.append(wgpu::BindingResource::Sampler(sampler))
    }

    pub fn append_sampler_array(self, sampler_array: &'res [&'res wgpu::Sampler]) -> Self {
        self.append(wgpu::BindingResource::SamplerArray(sampler_array))
    }

    pub fn append_texture_view(self, texture: &'res wgpu::TextureView) -> Self {
        self.append(wgpu::BindingResource::TextureView(texture))
    }

    pub fn append_texture_view_array(
        self,
        texture_view_array: &'res [&'res wgpu::TextureView],
    ) -> Self {
        self.append(wgpu::BindingResource::TextureViewArray(texture_view_array))
    }

    #[must_use]
    pub fn build(self) -> wgpu::BindGroup {
        let descriptor = wgpu::BindGroupDescriptor {
            label: self.label,
            layout: self.layout,
            entries: &self.entries,
        };
        self.device.create_bind_group(&descriptor)
    }
}
