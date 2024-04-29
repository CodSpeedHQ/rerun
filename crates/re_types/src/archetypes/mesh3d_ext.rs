use super::Mesh3D;

#[derive(thiserror::Error, Debug)]
pub enum Mesh3DError {
    #[error("Indices array length must be divisible by 3 (triangle list), got {0}")]
    IndicesNotDivisibleBy3(usize),

    #[error("No indices were specified, so the number of positions must be divisible by 9 [(xyz xyz xyz), …], got {0}")]
    PositionsAreNotTriangles(usize),

    #[error("Index out of bounds: got index={index} with {num_vertices} vertices")]
    IndexOutOfBounds { index: u32, num_vertices: usize },

    #[error(
        "Positions & normals array must have the same length, \
        got positions={0} vs. normals={1}"
    )]
    MismatchedPositionsNormals(usize, usize),
}

impl Mesh3D {
    /// Check that this is a valid mesh, e.g. that the vertex indices are within bounds
    /// and that we have the same number of positions and normals (if any).
    pub fn sanity_check(&self) -> Result<(), Mesh3DError> {
        let num_vertices = self.num_vertices();

        if let Some(indices) = self
            .mesh_properties
            .as_ref()
            .and_then(|props| props.indices.as_ref())
        {
            if indices.len() % 3 != 0 {
                return Err(Mesh3DError::IndicesNotDivisibleBy3(indices.len()));
            }

            for &index in indices.iter() {
                if num_vertices <= index as usize {
                    return Err(Mesh3DError::IndexOutOfBounds {
                        index,
                        num_vertices,
                    });
                }
            }
        } else if self.vertex_positions.len() % 9 != 0 {
            return Err(Mesh3DError::PositionsAreNotTriangles(
                self.vertex_positions.len(),
            ));
        }

        if let Some(normals) = &self.vertex_normals {
            if normals.len() != self.vertex_positions.len() {
                return Err(Mesh3DError::MismatchedPositionsNormals(
                    self.vertex_positions.len(),
                    normals.len(),
                ));
            }
        }

        Ok(())
    }

    /// The total number of vertices.
    #[inline]
    pub fn num_vertices(&self) -> usize {
        self.vertex_positions.len()
    }

    /// The total number of triangles.
    #[inline]
    pub fn num_triangles(&self) -> usize {
        if let Some(indices) = self
            .mesh_properties
            .as_ref()
            .and_then(|props| props.indices.as_ref())
        {
            indices.len() / 3
        } else {
            self.num_vertices() / 3
        }
    }
}
