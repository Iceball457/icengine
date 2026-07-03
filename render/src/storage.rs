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
    pub(crate) fn new(vertices: Vec<Vertex>, indices: Option<Vec<u16>>) -> Self {
        Self { vertices, indices }
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> Option<&Vec<u16>> {
        self.indices.as_ref()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    pub const fn new(position: [f32; 3], uv: [f32; 2], color: [f32; 3]) -> Self {
        Self {
            position,
            uv,
            color,
        }
    }
}

pub struct Rid<T> {
    index: usize,
    // generation: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Rid<T> {
    fn new(index: usize) -> Self {
        Self {
            index,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for Rid<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Rid<T> {}

pub struct Database {
    instances: Vec<Instance>,
    models: Vec<Model>,
    meshes: Vec<Mesh>,
    pipelines: Vec<wgpu::RenderPipeline>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            instances: vec![],
            models: vec![],
            meshes: vec![],
            pipelines: vec![],
        }
    }

    pub fn create_instance(&mut self, model: Rid<Model>) -> Rid<Instance> {
        let output = Rid::<Instance>::new(self.instances.len());
        self.instances.push(Instance {
            model,
            transforms: vec![],
        });
        output
    }

    pub fn create_model(
        &mut self,
        mesh: Rid<Mesh>,
        pipeline: Rid<wgpu::RenderPipeline>,
    ) -> Rid<Model> {
        let output = Rid::<Model>::new(self.models.len());
        self.models.push(Model { mesh, pipeline });
        output
    }

    pub fn create_mesh(&mut self, mesh: Mesh) -> Rid<Mesh> {
        let output = Rid::<Mesh>::new(self.meshes.len());
        self.meshes.push(mesh);
        output
    }

    pub fn create_pipeline(&mut self, pipeline: wgpu::RenderPipeline) -> Rid<wgpu::RenderPipeline> {
        let output = Rid::<wgpu::RenderPipeline>::new(self.pipelines.len());
        self.pipelines.push(pipeline);
        output
    }

    pub fn get_instance(&self, instance: Rid<Instance>) -> &Instance {
        &self.instances[instance.index]
    }
    pub fn get_instance_mut(&mut self, instance: Rid<Instance>) -> &mut Instance {
        &mut self.instances[instance.index]
    }
    pub fn instances_iter(&self) -> impl Iterator<Item = &Instance> {
        self.instances.iter()
    }

    pub fn get_model(&self, model: Rid<Model>) -> &Model {
        &self.models[model.index]
    }
    pub fn get_model_mut(&mut self, model: Rid<Model>) -> &mut Model {
        &mut self.models[model.index]
    }

    pub fn get_mesh(&self, mesh: Rid<Mesh>) -> &Mesh {
        &self.meshes[mesh.index]
    }
    pub fn get_mesh_mut(&mut self, mesh: Rid<Mesh>) -> &mut Mesh {
        &mut self.meshes[mesh.index]
    }

    pub fn get_pipeline(&self, pipeline: Rid<wgpu::RenderPipeline>) -> &wgpu::RenderPipeline {
        &self.pipelines[pipeline.index]
    }
    pub fn get_pipeline_mut(
        &mut self,
        pipeline: Rid<wgpu::RenderPipeline>,
    ) -> &mut wgpu::RenderPipeline {
        &mut self.pipelines[pipeline.index]
    }

    pub fn free_model(&self, model: Rid<Model>) {
        eprintln!("Not implemented");
    }

    pub fn free_mesh(&mut self, mesh: Rid<Mesh>) {
        eprintln!("Not implemented");
    }

    pub fn free_pipeline(&mut self, pipeline: Rid<wgpu::RenderPipeline>) {
        eprintln!("Not implemented");
    }

    pub fn model_set_mesh(&mut self, model: Rid<Model>, mesh: Rid<Mesh>) {
        let model = self.get_model_mut(model);
        model.mesh = mesh;
    }

    pub fn model_set_pipeline(&mut self, model: Rid<Model>, pipeline: Rid<wgpu::RenderPipeline>) {
        let model = self.get_model_mut(model);
        model.pipeline = pipeline;
    }
}
