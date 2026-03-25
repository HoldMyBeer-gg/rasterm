mod obj;

pub use obj::{load_obj, load_obj_auto_reduce, reduce_geometry};

use crate::engine::{animation::Animation, render::Scene};
use anyhow::Result;
use std::path::Path;

pub fn load_scene(_path: &Path) -> Result<Scene> {
    // TODO: Implement scene loading from files
    // This will parse the scene format and create meshes/materials
    Ok(Scene::new())
}

pub fn load_animation(_path: &Path) -> Result<Animation> {
    // TODO: Implement animation loading from files
    // This will parse animation keyframes
    Ok(Animation::new("default"))
}
