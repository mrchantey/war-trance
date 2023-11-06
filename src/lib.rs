#![allow(unused_attributes)]
#![feature(let_chains)]

// #[path = "./mod.rs"]
// pub mod foo;
// pub use foo::*;
pub mod app;
pub use app::*;
pub mod misc;
pub use misc::*;
pub mod agent;
pub use agent::*;
pub mod barracks;
pub use barracks::*;
pub mod ranged_attack;
pub use ranged_attack::*;
pub mod physics;
pub use physics::*;
pub mod seek;
pub use seek::*;
pub mod team;
pub use team::*;
pub mod level;
pub use level::*;
pub mod ui;
pub use ui::*;
