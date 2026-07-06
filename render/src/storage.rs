use crate::data::Vertex;
use common::math::Matrix;

#[derive(Clone, Debug)]
pub struct Object {
    model: Rid<Model>,
    instances: Vec<Matrix>,
}

impl Object {
    #[must_use]
    pub const fn model(&self) -> Rid<Model> {
        self.model
    }
    #[must_use]
    pub fn transforms(&self) -> &[Matrix] {
        &self.instances
    }
}

pub struct Model {
    mesh: Rid<Mesh>,
    pipeline: Rid<wgpu::RenderPipeline>,
}

impl Model {
    #[must_use]
    pub const fn mesh(&self) -> Rid<Mesh> {
        self.mesh
    }
    #[must_use]
    pub const fn pipeline(&self) -> Rid<wgpu::RenderPipeline> {
        self.pipeline
    }
}

pub struct Mesh {
    // Meshes may want to support multiple surfaces, each with their own pipeline.
    vertices: Vec<Vertex>,
    indices: Option<Vec<u16>>,
}

impl Mesh {
    pub(crate) const fn new(vertices: Vec<Vertex>, indices: Option<Vec<u16>>) -> Self {
        Self { vertices, indices }
    }

    #[must_use]
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    #[must_use]
    pub fn indices(&self) -> Option<&[u16]> {
        self.indices.as_deref()
    }
}

pub struct Rid<T> {
    index: usize,
    // generation: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> std::fmt::Debug for Rid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rid").field("index", &self.index).finish()
    }
}

impl<T> Rid<T> {
    const fn new(index: usize) -> Self {
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
    instances: Vec<Object>,
    models: Vec<Model>,
    meshes: Vec<Mesh>,
    pipelines: Vec<wgpu::RenderPipeline>,
}

impl Database {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            instances: vec![],
            models: vec![],
            meshes: vec![],
            pipelines: vec![],
        }
    }

    pub fn create_instance(&mut self, model: Rid<Model>) -> Rid<Object> {
        let output = Rid::<Object>::new(self.instances.len());
        self.instances.push(Object {
            model,
            instances: vec![],
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

    #[must_use]
    pub fn get_instance(&self, instance: Rid<Object>) -> &Object {
        &self.instances[instance.index]
    }
    pub fn get_instance_mut(&mut self, instance: Rid<Object>) -> &mut Object {
        &mut self.instances[instance.index]
    }
    pub fn instances_iter(&self) -> impl Iterator<Item = &Object> {
        self.instances.iter()
    }

    #[must_use]
    pub fn get_model(&self, model: Rid<Model>) -> &Model {
        &self.models[model.index]
    }
    pub fn get_model_mut(&mut self, model: Rid<Model>) -> &mut Model {
        &mut self.models[model.index]
    }

    #[must_use]
    pub fn get_mesh(&self, mesh: Rid<Mesh>) -> &Mesh {
        &self.meshes[mesh.index]
    }
    pub fn get_mesh_mut(&mut self, mesh: Rid<Mesh>) -> &mut Mesh {
        &mut self.meshes[mesh.index]
    }

    #[must_use]
    pub fn get_pipeline(&self, pipeline: Rid<wgpu::RenderPipeline>) -> &wgpu::RenderPipeline {
        &self.pipelines[pipeline.index]
    }
    pub fn get_pipeline_mut(
        &mut self,
        pipeline: Rid<wgpu::RenderPipeline>,
    ) -> &mut wgpu::RenderPipeline {
        &mut self.pipelines[pipeline.index]
    }

    pub fn free_model(&self, _model: Rid<Model>) {
        eprintln!("Not implemented");
    }

    pub fn free_mesh(&mut self, _mesh: Rid<Mesh>) {
        eprintln!("Not implemented");
    }

    pub fn free_pipeline(&mut self, _pipeline: Rid<wgpu::RenderPipeline>) {
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

    pub fn object_set_transforms(&mut self, instance: Rid<Object>, transforms: Vec<Matrix>) {
        let instance = self.get_instance_mut(instance);
        instance.instances = transforms;
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
