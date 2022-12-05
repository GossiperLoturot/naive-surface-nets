type Vector3 = (f32, f32, f32);

const IDX_TABLE: [[usize; 3]; 8] = [
    [0, 0, 0],
    [1, 0, 0],
    [1, 0, 1],
    [0, 0, 1],
    [0, 1, 0],
    [1, 1, 0],
    [1, 1, 1],
    [0, 1, 1],
];

const EDGE_TABLE: [[usize; 2]; 12] = [
    [0, 1],
    [1, 2],
    [2, 3],
    [3, 0],
    [4, 5],
    [5, 6],
    [6, 7],
    [7, 4],
    [0, 4],
    [1, 5],
    [2, 6],
    [3, 7],
];

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

// &mut [T]を*mut Tにするとなぜか速くなる
// ただし&[T]を*const Tにすると遅くなる
#[inline]
fn execute_imp(
    voxel: &[f32],
    size: usize,
    vertices: &mut [Vector3],
    triangles: &mut [u16],
) -> (usize, usize) {
    let mut vertex_count = 0;
    let mut triangle_count = 0;

    let mut idx_buf = vec![0; size * size * size];

    let mut neigh = [0; 8];
    for i in 0..8 {
        neigh[i] = IDX_TABLE[i][0] + IDX_TABLE[i][1] * size + IDX_TABLE[i][2] * size * size;
    }

    let mut make_face = |v0, v1, v2, v3, dir| {
        if dir {
            triangles[triangle_count] = v0;
            triangles[triangle_count + 1] = v3;
            triangles[triangle_count + 2] = v2;
            triangles[triangle_count + 3] = v2;
            triangles[triangle_count + 4] = v1;
            triangles[triangle_count + 5] = v0;
        } else {
            triangles[triangle_count] = v0;
            triangles[triangle_count + 1] = v1;
            triangles[triangle_count + 2] = v2;
            triangles[triangle_count + 3] = v2;
            triangles[triangle_count + 4] = v3;
            triangles[triangle_count + 5] = v0;
        }
        triangle_count += 6;
    };

    for x in 0..size - 1 {
        for y in 0..size - 1 {
            for z in 0..size - 1 {
                let idx = x + y * size + z * size * size;

                let mut kind = 0;
                if 0.0 > voxel[idx + neigh[0]] { kind |= 1; }
                if 0.0 > voxel[idx + neigh[1]] { kind |= 2; }
                if 0.0 > voxel[idx + neigh[2]] { kind |= 4; }
                if 0.0 > voxel[idx + neigh[3]] { kind |= 8; }
                if 0.0 > voxel[idx + neigh[4]] { kind |= 16; }
                if 0.0 > voxel[idx + neigh[5]] { kind |= 32; }
                if 0.0 > voxel[idx + neigh[6]] { kind |= 64; }
                if 0.0 > voxel[idx + neigh[7]] { kind |= 128; }

                if kind == 0 || kind == 255 { continue; }

                let mut vertex = (0.0, 0.0, 0.0);
                let mut cross_count = 0;

                for i in 0..12 {
                    let i0 = EDGE_TABLE[i][0];
                    let i1 = EDGE_TABLE[i][1];

                    if (kind >> i0 & 1) == (kind >> i1 & 1) { continue; }

                    let val0 = voxel[idx + neigh[i0]];
                    let val1 = voxel[idx + neigh[i1]];

                    let mix = (0.0 - val0) / (val1 - val0);

                    vertex.0 += x as f32 + lerp(IDX_TABLE[i0][0] as f32, IDX_TABLE[i1][0] as f32, mix);
                    vertex.1 += y as f32 + lerp(IDX_TABLE[i0][1] as f32, IDX_TABLE[i1][1] as f32, mix);
                    vertex.2 += z as f32 + lerp(IDX_TABLE[i0][2] as f32, IDX_TABLE[i1][2] as f32, mix);
                    cross_count += 1;
                }

                let inv = 1.0 / cross_count as f32;
                vertex.0 *= inv;
                vertex.1 *= inv;
                vertex.2 *= inv;

                vertices[vertex_count] = vertex;
                idx_buf[idx] = vertex_count as u16;
                vertex_count += 1;

                if x == 0 || y == 0 || z == 0 { continue; }

                let mut v = [0; 8];
                v[0] = idx_buf[idx - neigh[0]];
                v[1] = idx_buf[idx - neigh[1]];
                v[2] = idx_buf[idx - neigh[2]];
                v[3] = idx_buf[idx - neigh[3]];
                v[4] = idx_buf[idx - neigh[4]];
                v[5] = idx_buf[idx - neigh[5]];
                v[6] = idx_buf[idx - neigh[6]];
                v[7] = idx_buf[idx - neigh[7]];

                let dir = (kind & 1) != 0;
                if ((kind >> 1 & 1) != 0) != dir { make_face(v[0], v[3], v[7], v[4], dir); }
                if ((kind >> 3 & 1) != 0) != dir { make_face(v[0], v[4], v[5], v[1], dir); }
                if ((kind >> 4 & 1) != 0) != dir { make_face(v[0], v[1], v[2], v[3], dir); }
            }
        }
    }

    (vertex_count, triangle_count)
}

#[no_mangle]
pub unsafe extern "C" fn execute(
    voxel: *const f32,
    size: usize,
    vertices: *mut Vector3,
    vertices_size: *mut usize,
    triangles: *mut u16,
    triangles_size: *mut usize,
) {
    let voxel = std::slice::from_raw_parts(voxel, size * size * size);

    let vertices = std::slice::from_raw_parts_mut(vertices, size * size * size);

    let triangles = std::slice::from_raw_parts_mut(triangles, size * size * size * 18);

    (*vertices_size, *triangles_size) = execute_imp(voxel, size, vertices, triangles);
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, Duration};

    use rand::random;

    use super::*;

    #[test]
    fn testcase_1() {
        let mut voxel = vec![0.0; 4096];
        for i in 0..4096 {
            voxel[i] = random::<f32>() - 0.5;
        }

        let mut vertices = vec![(0.0, 0.0, 0.0); 4096];
        let mut triangles = vec![0; 4096 * 18];

        let n = 100;
        let mut mean = Duration::ZERO;
        for _ in 0..n {
            let now = SystemTime::now();
            execute_imp(&voxel, 16, &mut vertices, &mut triangles);
            mean += now.elapsed().unwrap();
        }
        mean /= n;
        println!("mean: {:?}", mean);
    }

    #[test]
    fn testcase_2() {
        let mut voxel = vec![0.0; 4096];
        for i in 0..4096 {
            voxel[i] = random::<f32>() - 0.5;
        }

        let mut vertices = vec![(0.0, 0.0, 0.0); 4096];
        let mut triangles = vec![0; 4096 * 18];
        let mut vertices_size = 0;
        let mut triangles_size = 0;

        let n = 100;
        let mut mean = Duration::ZERO;
        for _ in 0..n {
            let now = SystemTime::now();
            unsafe {
                execute(voxel.as_ptr(), 16, vertices.as_mut_ptr(), &mut vertices_size, triangles.as_mut_ptr(), &mut triangles_size);
            }
            mean += now.elapsed().unwrap();
        }
        mean /= n;
        println!("mean: {:?}", mean);
    }
}
