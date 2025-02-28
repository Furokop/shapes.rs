#![allow(dead_code)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::needless_return)]

fn main() {}

// Internal Types
mod basetype;
mod component;
mod math;
mod out;
mod scene;
mod shape;

// Exports
pub mod generators {
    pub use crate::shape::shape_gen::CubeGenerator;
    pub use crate::shape::shape_gen::TorusGenerator;
    pub mod selfmade {
        pub use crate::shape::shape_gen::ShapeGen;
    }
}

pub mod renderer {
    pub use crate::math::projection::pers_proj;
}

pub mod components {
    pub use crate::component::Camera;
    pub use crate::component::Light3D;
}

pub mod base {
    pub use crate::basetype::Angle;
    pub use crate::basetype::Angle3D;
    pub use crate::basetype::Coord;
    pub use crate::basetype::Vector3D;
    pub use crate::shape::rotator::Rotator;
}

pub mod buffer {
    pub use crate::out::terminal::SimpleTerminalBuffer;
    pub use crate::out::Buffer;
}

pub use crate::component::Object;
pub use crate::scene::Scene;
