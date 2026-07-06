use crate::{data::Vertex, material::Material, texture::Texture};
use common::math::Matrix;

#[derive(Clone, Debug)]
pub struct Object {
    model: Rid<Model>,
    transforms: Vec<Matrix>,
}

impl Object {
    #[must_use]
    pub const fn model(&self) -> Rid<Model> {
        self.model
    }
    #[must_use]
    pub fn transforms(&self) -> &[Matrix] {
        &self.transforms
    }
}

pub struct Model {
    surfaces: Vec<Surface>,
}

impl Model {
    #[must_use]
    pub fn get_surface<I>(&self, index: I) -> Option<&I::Output>
    where
        I: std::slice::SliceIndex<[Surface]>,
    {
        self.surfaces.get(index)
    }
    pub fn surfaces_iter(&self) -> impl Iterator<Item = &Surface> {
        self.surfaces.iter()
    }
}

pub struct Surface {
    mesh: Rid<Mesh>,
    material: Rid<Material>,
}

impl Surface {
    #[must_use]
    pub const fn new(mesh: Rid<Mesh>, material: Rid<Material>) -> Self {
        Self { mesh, material }
    }
    #[must_use]
    pub const fn mesh(&self) -> Rid<Mesh> {
        self.mesh
    }
    #[must_use]
    pub const fn material(&self) -> Rid<Material> {
        self.material
    }
}

pub struct Mesh {
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
    materials: Vec<Material>,
    pipelines: Vec<wgpu::RenderPipeline>,
    textures: Vec<super::texture::Texture>,
}

impl Database {
    // # Instantiation

    #[must_use]
    pub const fn new() -> Self {
        Self {
            instances: vec![],
            models: vec![],
            meshes: vec![],
            materials: vec![],
            pipelines: vec![],
            textures: vec![],
        }
    }

    // # Field Access

    // # Public API

    // ## Create resources

    pub fn create_object(&mut self, model: Rid<Model>) -> Rid<Object> {
        let output = Rid::<Object>::new(self.instances.len());
        self.instances.push(Object {
            model,
            transforms: vec![],
        });
        output
    }

    pub fn create_model(&mut self, surfaces: Vec<Surface>) -> Rid<Model> {
        let output = Rid::<Model>::new(self.models.len());
        self.models.push(Model { surfaces });
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

    pub fn create_material(&mut self, material: Material) -> Rid<Material> {
        let output = Rid::<Material>::new(self.materials.len());
        self.materials.push(material);
        output
    }

    pub fn create_texture(&mut self, texture: Texture) -> Rid<Texture> {
        let output = Rid::<Texture>::new(self.textures.len());
        self.textures.push(texture);
        output
    }

    // # Resource Access

    #[must_use]
    pub fn get_object(&self, instance: Rid<Object>) -> &Object {
        &self.instances[instance.index]
    }
    pub fn get_object_mut(&mut self, instance: Rid<Object>) -> &mut Object {
        &mut self.instances[instance.index]
    }
    pub fn objects_iter(&self) -> impl Iterator<Item = &Object> {
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
    pub fn get_material(&self, material: Rid<Material>) -> &Material {
        &self.materials[material.index]
    }
    pub fn get_material_mut(&mut self, material: Rid<Material>) -> &mut Material {
        &mut self.materials[material.index]
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

    // # Resource Disposal

    pub fn free_model(&self, _model: Rid<Model>) {
        eprintln!("Not implemented");
    }

    pub fn free_mesh(&mut self, _mesh: Rid<Mesh>) {
        eprintln!("Not implemented");
    }

    pub fn free_pipeline(&mut self, _pipeline: Rid<wgpu::RenderPipeline>) {
        eprintln!("Not implemented");
    }

    // # Object APIs

    pub fn object_set_transforms(&mut self, instance: Rid<Object>, transforms: Vec<Matrix>) {
        let instance = self.get_object_mut(instance);
        instance.transforms = transforms;
    }

    // # Model APIs

    pub fn model_add_surface(
        &mut self,
        model: Rid<Model>,
        mesh: Rid<Mesh>,
        material: Rid<Material>,
    ) {
        let model = self.get_model_mut(model);
        model.surfaces.push(Surface { mesh, material });
    }

    pub fn model_set_surface(
        &mut self,
        model: Rid<Model>,
        surface_index: usize,
        mesh: Rid<Mesh>,
        material: Rid<Material>,
    ) {
        let model = self.get_model_mut(model);
        if let Some(surface) = model.surfaces.get_mut(surface_index) {
            surface.mesh = mesh;
            surface.material = material;
        }
    }

    pub fn model_remove_surface(&mut self, model: Rid<Model>, surface_index: usize) {
        let model = self.get_model_mut(model);
        model.surfaces.remove(surface_index);
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
