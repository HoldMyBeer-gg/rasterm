use crate::engine::raster::Vertex;
use crate::engine::render::Geometry;

/// Create a low-poly goblin character optimized for terminal rendering
/// Total: ~50 triangles - enough to be recognizable, small enough to render fast
pub fn create_goblin() -> Geometry {
    let vertices = vec![
        // HEAD (oversized for goblin proportions)
        // Front face
        Vertex::from_data(vec![-0.6,  2.0,  0.5,  0.0,  0.0,  1.0, 0.0, 0.0]), // 0
        Vertex::from_data(vec![ 0.6,  2.0,  0.5,  0.0,  0.0,  1.0, 1.0, 0.0]), // 1
        Vertex::from_data(vec![ 0.6,  3.2,  0.5,  0.0,  0.0,  1.0, 1.0, 1.0]), // 2
        Vertex::from_data(vec![-0.6,  3.2,  0.5,  0.0,  0.0,  1.0, 0.0, 1.0]), // 3
        // Back face
        Vertex::from_data(vec![-0.6,  2.0, -0.5,  0.0,  0.0, -1.0, 0.0, 0.0]), // 4
        Vertex::from_data(vec![-0.6,  3.2, -0.5,  0.0,  0.0, -1.0, 0.0, 1.0]), // 5
        Vertex::from_data(vec![ 0.6,  3.2, -0.5,  0.0,  0.0, -1.0, 1.0, 1.0]), // 6
        Vertex::from_data(vec![ 0.6,  2.0, -0.5,  0.0,  0.0, -1.0, 1.0, 0.0]), // 7
        
        // LARGE EARS (goblin feature)
        // Left ear
        Vertex::from_data(vec![-0.6,  2.8,  0.2, -1.0,  0.0,  0.0, 0.0, 0.0]), // 8
        Vertex::from_data(vec![-1.2,  3.0,  0.0, -1.0,  0.0,  0.0, 1.0, 0.0]), // 9
        Vertex::from_data(vec![-0.6,  3.2,  0.2, -1.0,  0.0,  0.0, 0.5, 1.0]), // 10
        // Right ear
        Vertex::from_data(vec![ 0.6,  2.8,  0.2,  1.0,  0.0,  0.0, 0.0, 0.0]), // 11
        Vertex::from_data(vec![ 1.2,  3.0,  0.0,  1.0,  0.0,  0.0, 1.0, 0.0]), // 12
        Vertex::from_data(vec![ 0.6,  3.2,  0.2,  1.0,  0.0,  0.0, 0.5, 1.0]), // 13
        
        // LONG NOSE (goblin feature)
        Vertex::from_data(vec![-0.2,  2.5,  0.5,  0.0,  0.0,  1.0, 0.0, 0.0]), // 14
        Vertex::from_data(vec![ 0.2,  2.5,  0.5,  0.0,  0.0,  1.0, 1.0, 0.0]), // 15
        Vertex::from_data(vec![ 0.0,  2.6,  1.2,  0.0,  0.5,  1.0, 0.5, 1.0]), // 16 - nose tip
        
        // BODY (skinny, hunched)
        Vertex::from_data(vec![-0.5,  0.5,  0.3,  0.0,  0.0,  1.0, 0.0, 0.0]), // 17
        Vertex::from_data(vec![ 0.5,  0.5,  0.3,  0.0,  0.0,  1.0, 1.0, 0.0]), // 18
        Vertex::from_data(vec![ 0.5,  2.0,  0.3,  0.0,  0.0,  1.0, 1.0, 1.0]), // 19
        Vertex::from_data(vec![-0.5,  2.0,  0.3,  0.0,  0.0,  1.0, 0.0, 1.0]), // 20
        Vertex::from_data(vec![-0.5,  0.5, -0.3,  0.0,  0.0, -1.0, 0.0, 0.0]), // 21
        Vertex::from_data(vec![-0.5,  2.0, -0.3,  0.0,  0.0, -1.0, 0.0, 1.0]), // 22
        Vertex::from_data(vec![ 0.5,  2.0, -0.3,  0.0,  0.0, -1.0, 1.0, 1.0]), // 23
        Vertex::from_data(vec![ 0.5,  0.5, -0.3,  0.0,  0.0, -1.0, 1.0, 0.0]), // 24
        
        // LEGS (short and stumpy)
        // Left leg
        Vertex::from_data(vec![-0.4,  0.0,  0.2,  0.0, -1.0,  0.0, 0.0, 0.0]), // 25
        Vertex::from_data(vec![-0.2,  0.0,  0.2,  0.0, -1.0,  0.0, 1.0, 0.0]), // 26
        Vertex::from_data(vec![-0.2,  0.5,  0.2,  0.0,  0.0,  1.0, 1.0, 1.0]), // 27
        Vertex::from_data(vec![-0.4,  0.5,  0.2,  0.0,  0.0,  1.0, 0.0, 1.0]), // 28
        // Right leg
        Vertex::from_data(vec![ 0.2,  0.0,  0.2,  0.0, -1.0,  0.0, 0.0, 0.0]), // 29
        Vertex::from_data(vec![ 0.4,  0.0,  0.2,  0.0, -1.0,  0.0, 1.0, 0.0]), // 30
        Vertex::from_data(vec![ 0.4,  0.5,  0.2,  0.0,  0.0,  1.0, 1.0, 1.0]), // 31
        Vertex::from_data(vec![ 0.2,  0.5,  0.2,  0.0,  0.0,  1.0, 0.0, 1.0]), // 32
        
        // ARMS (long and lanky)
        // Left arm
        Vertex::from_data(vec![-0.5,  1.8,  0.2, -1.0,  0.0,  0.0, 0.0, 0.0]), // 33
        Vertex::from_data(vec![-0.7,  1.8,  0.2, -1.0,  0.0,  0.0, 1.0, 0.0]), // 34
        Vertex::from_data(vec![-0.7,  0.8,  0.2, -1.0,  0.0,  0.0, 1.0, 1.0]), // 35
        Vertex::from_data(vec![-0.5,  0.8,  0.2, -1.0,  0.0,  0.0, 0.0, 1.0]), // 36
        // Right arm
        Vertex::from_data(vec![ 0.5,  1.8,  0.2,  1.0,  0.0,  0.0, 0.0, 0.0]), // 37
        Vertex::from_data(vec![ 0.7,  1.8,  0.2,  1.0,  0.0,  0.0, 1.0, 0.0]), // 38
        Vertex::from_data(vec![ 0.7,  0.8,  0.2,  1.0,  0.0,  0.0, 1.0, 1.0]), // 39
        Vertex::from_data(vec![ 0.5,  0.8,  0.2,  1.0,  0.0,  0.0, 0.0, 1.0]), // 40
    ];

    let indices = vec![
        // HEAD
        0, 1, 2,  0, 2, 3,   // Front
        4, 5, 6,  4, 6, 7,   // Back
        0, 3, 5,  0, 5, 4,   // Left
        1, 7, 6,  1, 6, 2,   // Right
        3, 2, 6,  3, 6, 5,   // Top
        0, 4, 7,  0, 7, 1,   // Bottom
        
        // EARS (make goblin recognizable)
        8, 9, 10,             // Left ear
        11, 12, 13,           // Right ear
        
        // NOSE (pointy goblin nose)
        14, 15, 16,           // Nose triangle
        
        // BODY
        17, 18, 19,  17, 19, 20,  // Front
        21, 22, 23,  21, 23, 24,  // Back
        20, 19, 23,  20, 23, 22,  // Top
        17, 21, 24,  17, 24, 18,  // Bottom
        
        // LEFT LEG
        25, 26, 27,  25, 27, 28,  // Front
        
        // RIGHT LEG
        29, 30, 31,  29, 31, 32,  // Front
        
        // LEFT ARM
        33, 34, 35,  33, 35, 36,  // Front
        
        // RIGHT ARM
        37, 38, 39,  37, 39, 40,  // Front
    ];

    Geometry { vertices, indices }
}

/// Create the Triforce from The Legend of Zelda (3 golden triangles)
pub fn create_triforce() -> Geometry {
    let size = 1.0;
    let height = size * 0.866; // sqrt(3)/2 for equilateral triangle
    let offset = size / 2.0;
    
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    // Helper to create a double-sided flat triangle
    let mut add_triangle = |cx: f32, cy: f32| {
        let base_idx = vertices.len();
        
        // Front face vertices - normal pointing forward (0, 0, 1)
        vertices.push(Vertex::from_data(vec![cx, cy + height, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0])); // top
        vertices.push(Vertex::from_data(vec![cx - offset, cy, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0])); // bottom-left
        vertices.push(Vertex::from_data(vec![cx + offset, cy, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0])); // bottom-right
        
        // Front face triangle
        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);
        
        // Back face vertices - same positions, normal pointing backward (0, 0, -1)
        let back_base = vertices.len();
        vertices.push(Vertex::from_data(vec![cx, cy + height, 0.0, 0.0, 0.0, -1.0, 0.5, 0.0])); // top
        vertices.push(Vertex::from_data(vec![cx - offset, cy, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0])); // bottom-left
        vertices.push(Vertex::from_data(vec![cx + offset, cy, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0])); // bottom-right
        
        // Back face triangle - reversed winding order
        indices.push(back_base);
        indices.push(back_base + 2);
        indices.push(back_base + 1);
    };
    
    // Top triangle
    add_triangle(0.0, height / 2.0);
    
    // Bottom-left triangle
    add_triangle(-offset, -height / 2.0);
    
    // Bottom-right triangle
    add_triangle(offset, -height / 2.0);
    
    Geometry { vertices, indices }
}

/// Create a simple cube geometry
pub fn create_cube() -> Geometry {
    // Cube vertices: position (x, y, z), normal (nx, ny, nz), uv (u, v)
    let vertices = vec![
        // Front face
        Vertex::from_data(vec![-1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 0.0]),
        Vertex::from_data(vec![ 1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 0.0]),
        Vertex::from_data(vec![ 1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 1.0]),
        Vertex::from_data(vec![-1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 1.0]),
        
        // Back face
        Vertex::from_data(vec![-1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 0.0]),
        Vertex::from_data(vec![-1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 1.0]),
        Vertex::from_data(vec![ 1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 1.0]),
        Vertex::from_data(vec![ 1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 0.0]),
        
        // Top face
        Vertex::from_data(vec![-1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 0.0, 1.0]),
        Vertex::from_data(vec![-1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 0.0, 0.0]),
        Vertex::from_data(vec![ 1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 1.0, 0.0]),
        Vertex::from_data(vec![ 1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 1.0, 1.0]),
        
        // Bottom face
        Vertex::from_data(vec![-1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0]),
        Vertex::from_data(vec![ 1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 1.0, 1.0]),
        Vertex::from_data(vec![ 1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0]),
        Vertex::from_data(vec![-1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 0.0, 0.0]),
        
        // Right face
        Vertex::from_data(vec![ 1.0, -1.0, -1.0,  1.0,  0.0,  0.0, 0.0, 0.0]),
        Vertex::from_data(vec![ 1.0,  1.0, -1.0,  1.0,  0.0,  0.0, 1.0, 0.0]),
        Vertex::from_data(vec![ 1.0,  1.0,  1.0,  1.0,  0.0,  0.0, 1.0, 1.0]),
        Vertex::from_data(vec![ 1.0, -1.0,  1.0,  1.0,  0.0,  0.0, 0.0, 1.0]),
        
        // Left face
        Vertex::from_data(vec![-1.0, -1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 0.0]),
        Vertex::from_data(vec![-1.0, -1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 0.0]),
        Vertex::from_data(vec![-1.0,  1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 1.0]),
        Vertex::from_data(vec![-1.0,  1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 1.0]),
    ];

    // Cube indices (2 triangles per face)
    let indices = vec![
        // Front
        0, 1, 2,  0, 2, 3,
        // Back
        4, 5, 6,  4, 6, 7,
        // Top
        8, 9, 10,  8, 10, 11,
        // Bottom
        12, 13, 14,  12, 14, 15,
        // Right
        16, 17, 18,  16, 18, 19,
        // Left
        20, 21, 22,  20, 22, 23,
    ];

    Geometry { vertices, indices }
}

/// Create a simple pyramid geometry
pub fn create_pyramid() -> Geometry {
    let vertices = vec![
        // Base
        Vertex::from_data(vec![-1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 0.0, 0.0]),
        Vertex::from_data(vec![ 1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0]),
        Vertex::from_data(vec![ 1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 1.0, 1.0]),
        Vertex::from_data(vec![-1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0]),
        
        // Apex
        Vertex::from_data(vec![ 0.0,  1.0,  0.0,  0.0,  1.0,  0.0, 0.5, 0.5]),
    ];

    let indices = vec![
        // Base
        0, 2, 1,  0, 3, 2,
        // Sides
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
        3, 0, 4,
    ];

    Geometry { vertices, indices }
}
