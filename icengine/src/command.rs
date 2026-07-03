use crate::Scene;

pub enum Command {
    Exit,
    SetScene(Box<dyn Scene>),
}
