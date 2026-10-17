use crate::engine::raster::Vertex;
use crate::engine::render::Geometry;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Load a Wavefront .obj file and convert to Geometry
pub fn load_obj(path: &Path) -> Result<Geometry> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read OBJ file: {:?}", path))?;
    
    parse_obj(&content)
}

/// Parse OBJ file content
fn parse_obj(content: &str) -> Result<Geometry> {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();
    let mut vertex_cache: HashMap<String, usize> = HashMap::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "v" => {
                // Vertex position
                if parts.len() >= 4 {
                    let x = parts[1].parse::<f32>()?;
                    let y = parts[2].parse::<f32>()?;
                    let z = parts[3].parse::<f32>()?;
                    positions.push([x, y, z]);
                }
            }
            "vn" => {
                // Vertex normal
                if parts.len() >= 4 {
                    let nx = parts[1].parse::<f32>()?;
                    let ny = parts[2].parse::<f32>()?;
                    let nz = parts[3].parse::<f32>()?;
                    normals.push([nx, ny, nz]);
                }
            }
            "vt" => {
                // Texture coordinate
                if parts.len() >= 3 {
                    let u = parts[1].parse::<f32>()?;
                    let v = parts[2].parse::<f32>()?;
                    uvs.push([u, v]);
                }
            }
            "f" => {
                // Face (triangle or quad)
                let mut face_indices = Vec::new();
                
                for part in &parts[1..] {
                    let vertex_key = part.to_string();
                    
                    // Check if we've seen this exact vertex combo before
                    if let Some(&existing_idx) = vertex_cache.get(&vertex_key) {
                        face_indices.push(existing_idx);
                    } else {
                        // Parse vertex reference: pos/uv/normal or pos//normal or pos/uv or pos
                        let components: Vec<&str> = part.split('/').collect();

                        let pos_idx = resolve_index(components[0], positions.len(), "v")?;

                        let uv_idx = if components.len() > 1 && !components[1].is_empty() {
                            Some(resolve_index(components[1], uvs.len(), "vt")?)
                        } else {
                            None
                        };

                        let normal_idx = if components.len() > 2 && !components[2].is_empty() {
                            Some(resolve_index(components[2], normals.len(), "vn")?)
                        } else {
                            None
                        };

                        // Build vertex data: [x, y, z, nx, ny, nz, u, v]
                        let pos = positions[pos_idx];
                        let normal = normal_idx.map(|i| normals[i]).unwrap_or([0.0, 1.0, 0.0]);
                        let uv = uv_idx.map(|i| uvs[i]).unwrap_or([0.0, 0.0]);
                        
                        let vertex = Vertex::from_data(vec![
                            pos[0], pos[1], pos[2],      // position
                            normal[0], normal[1], normal[2], // normal
                            uv[0], uv[1],                 // uv
                        ]);
                        
                        let new_idx = vertices.len();
                        vertices.push(vertex);
                        vertex_cache.insert(vertex_key, new_idx);
                        face_indices.push(new_idx);
                    }
                }
                
                // Triangulate face (assumes convex polygons)
                if face_indices.len() >= 3 {
                    for i in 1..(face_indices.len() - 1) {
                        indices.push(face_indices[0]);
                        indices.push(face_indices[i]);
                        indices.push(face_indices[i + 1]);
                    }
                }
            }
            _ => {} // Ignore other commands
        }
    }
    
    Geometry::with_limits(vertices, indices)
}

/// Resolve a 1-based OBJ face index (negative = relative to end) to a 0-based
/// index, rejecting anything outside the referenced list.
fn resolve_index(text: &str, len: usize, kind: &str) -> Result<usize> {
    let raw: isize = text
        .parse()
        .with_context(|| format!("Invalid {} index: {:?}", kind, text))?;
    let resolved = if raw < 0 {
        len as isize + raw
    } else {
        raw - 1
    };
    if resolved < 0 || resolved as usize >= len {
        bail!("{} index {} out of range (have {} entries)", kind, raw, len);
    }
    Ok(resolved as usize)
}

/// Reduce polygon count using simple decimation
/// Keeps approximately target_tris triangles
pub fn reduce_geometry(geometry: Geometry, target_tris: usize) -> Geometry {
    let current_tris = geometry.indices.len() / 3;
    
    if current_tris <= target_tris {
        return geometry; // Already small enough
    }
    
    // Simple decimation: keep every Nth triangle
    let keep_ratio = target_tris as f32 / current_tris as f32;
    let mut new_indices = Vec::new();
    
    for tri_idx in 0..(current_tris) {
        let should_keep = (tri_idx as f32 * keep_ratio).fract() < keep_ratio;
        
        if should_keep {
            let i = tri_idx * 3;
            new_indices.push(geometry.indices[i]);
            new_indices.push(geometry.indices[i + 1]);
            new_indices.push(geometry.indices[i + 2]);
        }
    }
    
    Geometry {
        vertices: geometry.vertices,
        indices: new_indices,
    }
}

/// Load and auto-reduce an OBJ file to be terminal-friendly
pub fn load_obj_auto_reduce(path: &Path, max_tris: usize) -> Result<Geometry> {
    let mut geometry = load_obj(path)?;
    let tri_count = geometry.indices.len() / 3;
    
    eprintln!("Loaded OBJ: {} vertices, {} triangles", geometry.vertices.len(), tri_count);
    
    // Auto-scale and center the model
    geometry = normalize_geometry(geometry);
    eprintln!("Model normalized and centered");
    
    if tri_count > max_tris {
        eprintln!("Reducing from {} to {} triangles...", tri_count, max_tris);
        let reduced = reduce_geometry(geometry, max_tris);
        eprintln!("Reduced to {} triangles", reduced.indices.len() / 3);
        Ok(reduced)
    } else {
        Ok(geometry)
    }
}

/// Normalize geometry to fit in a unit cube centered at origin
fn normalize_geometry(mut geometry: Geometry) -> Geometry {
    if geometry.vertices.is_empty() {
        return geometry;
    }
    
    // Find bounding box
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut max_z = f32::NEG_INFINITY;
    
    for vertex in &geometry.vertices {
        let x = vertex.data[0];
        let y = vertex.data[1];
        let z = vertex.data[2];
        
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        min_z = min_z.min(z);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        max_z = max_z.max(z);
    }
    
    // Calculate center and scale
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;
    let center_z = (min_z + max_z) / 2.0;
    
    let size_x = max_x - min_x;
    let size_y = max_y - min_y;
    let size_z = max_z - min_z;
    let max_size = size_x.max(size_y).max(size_z);
    
    let scale = if max_size > 0.0 { 2.0 / max_size } else { 1.0 };
    
    eprintln!("Model bounds: ({:.2}, {:.2}, {:.2}) to ({:.2}, {:.2}, {:.2})", 
              min_x, min_y, min_z, max_x, max_y, max_z);
    eprintln!("Scaling by {:.2} and centering at origin", scale);
    
    // Transform vertices
    for vertex in &mut geometry.vertices {
        // Center
        vertex.data[0] -= center_x;
        vertex.data[1] -= center_y;
        vertex.data[2] -= center_z;
        
        // Scale
        vertex.data[0] *= scale;
        vertex.data[1] *= scale;
        vertex.data[2] *= scale;
    }
    
    geometry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_obj() {
        let obj = r#"
# Simple triangle
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.5 1.0 0.0
vn 0.0 0.0 1.0
f 1//1 2//1 3//1
"#;
        
        let geometry = parse_obj(obj).unwrap();
        assert_eq!(geometry.vertices.len(), 3);
        assert_eq!(geometry.indices.len(), 3);
    }
    
    #[test]
    fn test_face_index_out_of_range_is_error() {
        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 99\n";
        assert!(parse_obj(obj).is_err());

        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 -4\n";
        assert!(parse_obj(obj).is_err());

        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 0\n";
        assert!(parse_obj(obj).is_err());
    }

    #[test]
    fn test_negative_face_index() {
        let obj = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf -3 -2 -1\n";
        let geometry = parse_obj(obj).unwrap();
        assert_eq!(geometry.indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_reduce_geometry() {
        let mut indices = Vec::new();
        for i in 0..300 {
            indices.push(i);
        }
        
        let geometry = Geometry {
            vertices: vec![Vertex::new(8); 300],
            indices,
        };
        
        let reduced = reduce_geometry(geometry, 30);
        let tri_count = reduced.indices.len() / 3;
        
        assert!(tri_count <= 30);
        assert!(tri_count >= 25); // Should be close to target
    }
}
