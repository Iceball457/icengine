fn main() -> anyhow::Result<()> {
    icengine::run(Box::new(MainScene))
}

struct MainScene;

impl icengine::Scene for MainScene {
    fn start(&mut self, engine: icengine::EngineCtl) {
        let rs = engine.render_server_mut();
        let mesh = *render::constants::TEST_QUAD;
        let mesh = rs.create_mesh(mesh);
        let model = rs.create_model(mesh, pipeline);
        let instance = rs.create_instance(model);
        // rs.instance_set_transforms(instance, &[])
    }

    fn display(&mut self, delta: std::time::Duration, engine: icengine::EngineCtl) {
        todo!()
    }

    fn tick(&mut self, delta: std::time::Duration, engine: icengine::EngineCtl) {
        todo!()
    }

    fn end(&mut self, engine: icengine::EngineCtl) {
        todo!()
    }
}
