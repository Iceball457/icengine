#[allow(unused_variables)]
pub trait Scene {
    fn start(&mut self, engine: super::EngineCtl) {}
    fn display(&mut self, delta: std::time::Duration, engine: super::EngineCtl) {}
    fn tick(&mut self, delta: std::time::Duration, engine: super::EngineCtl) {}
    fn end(&mut self, engine: super::EngineCtl) {}
}

// impl Scene {
//     pub fn new(
//         start: fn(super::EngineCtl),
//         display: fn(std::time::Duration, super::EngineCtl),
//         tick: fn(std::time::Duration, super::EngineCtl),
//         end: fn(super::EngineCtl),
//     ) -> Self {
//         Self {
//             start,
//             display,
//             tick,
//             end,
//         }
//     }

//     pub fn start(&self, engine_control: super::EngineCtl) {
//         (self.start)(engine_control)
//     }

//     pub fn display(&self, delta: std::time::Duration, engine_control: super::EngineCtl) {
//         (self.display)(delta, engine_control)
//     }

//     pub fn tick(&self, delta: std::time::Duration, engine_control: super::EngineCtl) {
//         (self.tick)(delta, engine_control)
//     }

//     pub fn end(&self, engine_control: super::EngineCtl) {
//         (self.end)(engine_control)
//     }
// }
