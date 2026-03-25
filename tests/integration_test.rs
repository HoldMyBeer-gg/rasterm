use rasterm::engine::{
    raster::{Raster, Vertex},
    render::{
        shaders::{BasicFragmentShader, BasicUniform, BasicVertexShader},
        Geometry, Texture,
    },
};
use glam::{Mat4, Vec4};

#[test]
fn test_triangle_rendering() {
    let size = 64;
    let mut depth = rasterm::engine::raster::DepthBuffer::new(size, size);
    let mut target = Texture::new(size, size);

    // Use a camera looking at a small triangle that fits entirely on screen
    let uniform = BasicUniform {
        projection: Mat4::perspective_rh(60.0_f32.to_radians(), 1.0, 0.1, 100.0),
        view: Mat4::look_at_rh(
            glam::Vec3::new(0.0, 0.0, 5.0),
            glam::Vec3::ZERO,
            glam::Vec3::Y,
        ),
        matrix: Mat4::IDENTITY,
        normal_matrix: Mat4::IDENTITY,
    };

    // Small triangle centered at origin: position (3) + normal (3) + uv (2) = 8 floats
    let v0 = Vertex::from_data(vec![0.0, 0.5, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0]);
    let v1 = Vertex::from_data(vec![-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0]);
    let v2 = Vertex::from_data(vec![0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);

    let vertex_shader = BasicVertexShader;
    let fragment_shader = BasicFragmentShader;

    Raster::triangle(
        &vertex_shader,
        &fragment_shader,
        &mut depth,
        &mut target,
        &uniform,
        &v0,
        &v1,
        &v2,
    );

    // Check if any pixel was rendered — ambient light means even dark faces get ~0.15
    let mut any_rendered = false;
    let mut max_val = 0.0_f32;
    for y in 0..size {
        for x in 0..size {
            let c = target.get(x, y);
            max_val = max_val.max(c.x).max(c.y).max(c.z);
            if c.x > 0.01 || c.y > 0.01 || c.z > 0.01 {
                any_rendered = true;
            }
        }
    }

    assert!(any_rendered, "Triangle should render at least some pixels (max color: {})", max_val);
}

#[test]
fn test_depth_buffer() {
    let mut depth = rasterm::engine::raster::DepthBuffer::new(16, 16);

    // Initial state should be infinity
    assert_eq!(depth.get(8, 8), f32::INFINITY);

    // Set a depth value
    depth.set(8, 8, 0.5);
    assert_eq!(depth.get(8, 8), 0.5);

    // Clear should reset to infinity
    depth.clear();
    assert_eq!(depth.get(8, 8), f32::INFINITY);
}

#[test]
fn test_texture() {
    let mut texture = Texture::new(16, 16);

    // Set a color
    let color = Vec4::new(1.0, 0.5, 0.25, 1.0);
    texture.set(8, 8, color);

    // Read it back
    let read = texture.get(8, 8);
    assert_eq!(read, color);

    // Clear with blue
    texture.clear([0.0, 0.0, 1.0, 1.0]);
    let blue = texture.get(8, 8);
    assert_eq!(blue, Vec4::new(0.0, 0.0, 1.0, 1.0));
}

#[test]
fn test_texture_uv_sampling() {
    let mut texture = Texture::new(2, 2);
    // Set known colors at the 4 corners
    texture.set(0, 0, Vec4::new(1.0, 0.0, 0.0, 1.0)); // red
    texture.set(1, 0, Vec4::new(0.0, 1.0, 0.0, 1.0)); // green
    texture.set(0, 1, Vec4::new(0.0, 0.0, 1.0, 1.0)); // blue
    texture.set(1, 1, Vec4::new(1.0, 1.0, 1.0, 1.0)); // white

    // Sample top-left corner
    let tl = texture.sample_uv(0.0, 0.0);
    assert!((tl.x - 1.0).abs() < 0.01, "top-left should be red");

    // Sample top-right corner
    let tr = texture.sample_uv(1.0, 0.0);
    assert!((tr.y - 1.0).abs() < 0.01, "top-right should be green");

    // Sample bottom-left corner
    let bl = texture.sample_uv(0.0, 1.0);
    assert!((bl.z - 1.0).abs() < 0.01, "bottom-left should be blue");
}

#[test]
fn test_vertex_interpolation() {
    let v0 = Vertex::from_data(vec![1.0, 0.0, 0.0]);
    let v1 = Vertex::from_data(vec![0.0, 1.0, 0.0]);
    let v2 = Vertex::from_data(vec![0.0, 0.0, 1.0]);

    // Interpolate at center (equal weights)
    let result = Vertex::interpolate(&v0, &v1, &v2, 0.333, 0.333, 0.334, 1.0);

    assert!((result.data[0] - 0.333).abs() < 0.01);
    assert!((result.data[1] - 0.333).abs() < 0.01);
    assert!((result.data[2] - 0.334).abs() < 0.01);
}

#[test]
fn test_geometry_limits() {
    use rasterm::engine::raster::Vertex;

    // Valid geometry
    let verts = vec![Vertex::new(3); 10];
    let indices = vec![0, 1, 2, 3, 4, 5];
    let geo = Geometry::with_limits(verts, indices);
    assert!(geo.is_ok());

    // Index out of bounds
    let verts = vec![Vertex::new(3); 3];
    let indices = vec![0, 1, 5]; // 5 is out of bounds
    let geo = Geometry::with_limits(verts, indices);
    assert!(geo.is_err());
}

#[test]
fn test_renderer_resize() {
    use rasterm::engine::render::Renderer;

    let mut renderer = Renderer::new();
    renderer.resize(100, 50);
    renderer.clear([0.0, 0.0, 0.0, 1.0]);
    // Should not panic
    renderer.resize(200, 100);
    renderer.clear([1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_primitives() {
    use rasterm::engine::render::primitives;

    let cube = primitives::create_cube();
    assert_eq!(cube.indices.len() % 3, 0, "cube indices should be multiple of 3");
    assert!(cube.indices.len() >= 36, "cube should have at least 12 triangles");

    let pyramid = primitives::create_pyramid();
    assert_eq!(pyramid.indices.len() % 3, 0);
    assert!(pyramid.indices.len() >= 18, "pyramid should have at least 6 triangles");

    let triforce = primitives::create_triforce();
    assert_eq!(triforce.indices.len() % 3, 0);

    let goblin = primitives::create_goblin();
    assert_eq!(goblin.indices.len() % 3, 0);
    assert!(!goblin.vertices.is_empty());
}

#[test]
fn test_scene_mesh_limit() {
    use rasterm::engine::render::{Geometry, Material, Mesh, Scene};

    let mut scene = Scene::new();
    // Should accept meshes up to limit
    let ok = scene.add_mesh(Mesh::new(Geometry::new(), Material::new()));
    assert!(ok);
}
