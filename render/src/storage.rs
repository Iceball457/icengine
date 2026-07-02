pub struct Instance {
    model: Rid<Model>,
    transforms: Vec<glam::Affine3A>,
}

impl Instance {
    pub fn model(&self) -> Rid<Model> {
        self.model
    }
    pub fn transforms(&self) -> &[glam::Affine3A] {
        &self.transforms
    }
}

pub struct Model {
    mesh: Rid<Mesh>,
    pipeline: Rid<wgpu::RenderPipeline>,
}

impl Model {
    pub fn mesh(&self) -> Rid<Mesh> {
        self.mesh
    }
    pub fn pipeline(&self) -> Rid<wgpu::RenderPipeline> {
        self.pipeline
    }
}

pub struct Mesh {
    // Meshes may want to support multiple surfaces, each with their own pipeline.
    vertices: Vec<Vertex>,
    indices: Option<Vec<u16>>,
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> Option<&Vec<u16>> {
        self.indices.as_ref()
    }
}

#[repr(C)]
pub struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    color: [f32; 3],
}

pub struct Rid<T> {
    id: usize,
    generation: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Clone for Rid<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Rid<T> {}

pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_instance(&mut self) -> Rid<Instance> {}

    pub fn create_model(&mut self) -> Rid<Model> {}

    pub fn create_mesh(&mut self, mesh: Mesh) -> Rid<Mesh> {}

    pub fn create_pipeline(&mut self, pipeline: wgpu::RenderPipeline) -> Rid<wgpu::RenderPipeline> {
    }

    pub fn get_instance(&self, instance: Rid<Instance>) -> &Instance {}
    pub fn get_instance_mut(&mut self, instance: Rid<Instance>) -> &mut Instance {}
    pub fn instances_iter(&self) -> impl Iterator<Item = Instance> {}

    pub fn get_model(&self, model: Rid<Model>) -> &Model {}
    pub fn get_model_mut(&mut self, model: Rid<Model>) -> &mut Model {}

    pub fn get_mesh(&self, mesh: Rid<Mesh>) -> &Mesh {}
    pub fn get_mesh_mut(&mut self, mesh: Rid<Mesh>) -> &mut Mesh {}

    pub fn get_pipeline(&self, pipeline: Rid<wgpu::RenderPipeline>) -> &wgpu::RenderPipeline {}
    pub fn get_pipeline_mut(
        &mut self,
        pipeline: Rid<wgpu::RenderPipeline>,
    ) -> &mut wgpu::RenderPipeline {
    }

    pub fn free_model(&self, model: Rid<Model>) {}

    pub fn free_mesh(&mut self, mesh: Rid<Mesh>) {}

    pub fn free_pipeline(&mut self, pipeline: Rid<wgpu::RenderPipeline>) {}

    pub fn model_set_mesh(&mut self, model: Rid<Model>, mesh: Rid<Mesh>) {
        let model = self.get_model_mut(model);
        model.mesh = mesh;
    }

    pub fn model_set_pipeline(&mut self, model: Rid<Model>, pipeline: Rid<wgpu::RenderPipeline>) {
        let model = self.get_model_mut(model);
        model.pipeline = pipeline;
    }
}
