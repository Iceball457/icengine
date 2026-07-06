use render::camera;

fn main() -> anyhow::Result<()> {
    icengine::run(Box::new(MainScene))
}

struct MainScene;

impl icengine::Scene for MainScene {
    fn start(&mut self, mut engine: icengine::EngineCtl) {
        let rs = engine.render_server_mut();
        let mesh = rs.create_mesh(render::constants::test_quad());
        let texture = render::texture::Texture::new(
            rs,
            Some("Example Texture"),
            (2, 2),
            &[
                0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255,
            ],
        );
        let material = render::material::unlit_material(rs, &texture, [1.0, 1.0, 1.0, 1.0]);
        let model = rs.create_model(vec![render::storage::Surface::new(mesh, material)]);
        let instance = rs.create_object(model);
        rs.object_set_transforms(
            instance,
            vec![
                common::math::Matrix(glam::Mat4::IDENTITY),
                common::math::Matrix(glam::Mat4::from_mat3_translation(
                    glam::Mat3::IDENTITY,
                    glam::Vec3::new(0.0, 2.0, 0.0),
                )),
            ],
        );
        rs.camera_set_view(camera::View {
            eye: glam::Vec3::new(0.0, 0.0, 5.0),
            dir: glam::Vec3::new(0.0, 0.0, -1.0),
            up: glam::Vec3::new(0.0, 1.0, 0.0),
        });
        rs.camera_set_projection(camera::Projection::Perspective(camera::Perspective {
            fovy: 90.0,
            near: 0.01,
        }));
    }

    fn display(&mut self, _delta: std::time::Duration, _engine: icengine::EngineCtl) {
        todo!()
    }

    fn tick(&mut self, _delta: std::time::Duration, _engine: icengine::EngineCtl) {
        todo!()
    }

    fn end(&mut self, _engine: icengine::EngineCtl) {
        todo!()
    }
}
