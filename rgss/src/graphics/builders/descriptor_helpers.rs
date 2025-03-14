#![allow(dead_code)]

#[derive(Default)]
pub struct RenderPassBuilder<'res> {
    label: wgpu::Label<'res>,
    color_attachments: Vec<Option<wgpu::RenderPassColorAttachment<'res>>>,
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'res>>,
    occlusion_query_state: Option<&'res wgpu::QuerySet>,
    timestamp_writes: Option<wgpu::RenderPassTimestampWrites<'res>>,
}

impl<'a> RenderPassBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_color_attachment(
        mut self,
        attachment: impl Into<Option<wgpu::RenderPassColorAttachment<'a>>>,
    ) -> Self {
        self.color_attachments.push(attachment.into());
        self
    }

    pub fn with_depth_stencil(
        mut self,
        attachment: wgpu::RenderPassDepthStencilAttachment<'a>,
    ) -> Self {
        self.depth_stencil_attachment = Some(attachment);
        self
    }

    pub fn with_occlusion_state(mut self, query_set: &'a wgpu::QuerySet) -> Self {
        self.occlusion_query_state = Some(query_set);
        self
    }

    pub fn with_timestamp_writes(mut self, writes: wgpu::RenderPassTimestampWrites<'a>) -> Self {
        self.timestamp_writes = Some(writes);
        self
    }

    pub fn build(&'a self) -> wgpu::RenderPassDescriptor<'a> {
        wgpu::RenderPassDescriptor {
            label: self.label,
            color_attachments: &self.color_attachments,
            // these clones are basically a copy (though the struct does not impl it)
            depth_stencil_attachment: self.depth_stencil_attachment.clone(),
            timestamp_writes: self.timestamp_writes.clone(),
            occlusion_query_set: self.occlusion_query_state,
        }
    }
}
