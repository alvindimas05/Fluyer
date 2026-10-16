//! Pinch-warp mesh. Port of `ArtworkBackdropRenderer.makeMesh` (macOS).
//! Point layout: [u, v, from.x, from.y, to.x, to.y], all in 0..1 space.

type P = [f32; 6];

fn add(a: P, b: P) -> P {
    std::array::from_fn(|i| a[i] + b[i])
}

fn mul(a: P, s: f32) -> P {
    a.map(|x| x * s)
}

/// Returns interleaved vertices `[from.x, from.y, to.x, to.y, u, v]` (clip space for
/// from/to) and triangle indices.
pub fn make_mesh() -> (Vec<[f32; 6]>, Vec<u32>) {
    // Control-pair 0 at Music arm64e 0x101885858. Five divisions, six points per axis.
    #[rustfmt::skip]
    let from: [(f32, f32); 36] = [
        (0.,0.), (0.2,0.), (0.4,0.), (0.6,0.), (0.8,0.), (1.,0.),
        (0.,0.2), (-0.0933,0.4), (0.4,0.2), (0.6,0.2), (0.3653,0.1335), (1.,0.2),
        (0.,0.4), (0.4232,0.359), (0.3429,0.5349), (0.6,0.4), (0.832,0.4148), (1.,0.4),
        (0.,0.6), (0.2,0.6), (0.2293,0.7775), (0.7829,0.5595), (0.6514,0.7302), (1.,0.6),
        (0.,0.8), (0.2,0.8), (0.28,0.9195), (0.4773,0.8), (0.8,0.8), (1.,0.8),
        (0.,1.), (0.6514,1.1073), (0.4,1.), (1.,1.0317), (1.,1.1302), (1.,1.),
    ];
    let mut to = from;
    to[10] = (0.8587, 0.2234);
    to[13] = (0.4526, 0.6053);
    let mut points: Vec<P> = (0..36)
        .map(|i| {
            let (u, v) = ((i % 6) as f32 / 5., (i / 6) as f32 / 5.);
            [u, v, from[i].0, from[i].1, to[i].0, to[i].1]
        })
        .collect();
    let mut faces: Vec<[usize; 4]> = Vec::new();
    for y in 0..5 {
        for x in 0..5 {
            let a = y * 6 + x;
            faces.push([a, a + 1, a + 7, a + 6]);
        }
    }

    // ponytail: public Catmull–Clark approximation of private CAMeshTransform subdividedMesh:3.
    for _ in 0..3 {
        struct Edge {
            a: usize,
            b: usize,
            faces: Vec<usize>,
        }
        let mut edges: Vec<Edge> = Vec::new();
        let mut edge_ids = std::collections::HashMap::new();
        let mut face_edges = vec![Vec::new(); faces.len()];
        let mut vertex_faces = vec![Vec::new(); points.len()];
        let mut vertex_edges = vec![Vec::new(); points.len()];
        let centers: Vec<P> = faces
            .iter()
            .map(|f| mul(f.iter().fold([0.; 6], |s, &i| add(s, points[i])), 0.25))
            .collect();
        for (fi, face) in faces.iter().enumerate() {
            for j in 0..4 {
                let (a, b) = (face[j], face[(j + 1) % 4]);
                vertex_faces[a].push(fi);
                let ei = *edge_ids.entry((a.min(b), a.max(b))).or_insert_with(|| {
                    edges.push(Edge { a, b, faces: Vec::new() });
                    vertex_edges[a].push(edges.len() - 1);
                    vertex_edges[b].push(edges.len() - 1);
                    edges.len() - 1
                });
                edges[ei].faces.push(fi);
                face_edges[fi].push(ei);
            }
        }
        let other = |e: &Edge, i: usize| if e.a == i { e.b } else { e.a };
        let mut refined = points.clone();
        for i in 0..points.len() {
            let boundary: Vec<usize> = vertex_edges[i]
                .iter()
                .copied()
                .filter(|&e| edges[e].faces.len() == 1)
                .collect();
            if boundary.len() == 2 && vertex_faces[i].len() > 1 {
                let n0 = points[other(&edges[boundary[0]], i)];
                let n1 = points[other(&edges[boundary[1]], i)];
                refined[i] = add(mul(points[i], 0.75), mul(add(n0, n1), 0.125));
            } else if boundary.is_empty() {
                let n = vertex_edges[i].len() as f32;
                let f = mul(vertex_faces[i].iter().fold([0.; 6], |s, &f| add(s, centers[f])), 1. / n);
                let r = mul(
                    vertex_edges[i].iter().fold([0.; 6], |s, &e| {
                        add(s, mul(add(points[edges[e].a], points[edges[e].b]), 0.5))
                    }),
                    1. / n,
                );
                refined[i] = mul(add(add(f, mul(r, 2.)), mul(points[i], n - 3.)), 1. / n);
            }
            refined[i][0] = points[i][0];
            refined[i][1] = points[i][1];
        }
        let edge_base = refined.len();
        for e in &edges {
            let mid = mul(add(points[e.a], points[e.b]), 0.5);
            let mut p = mid;
            if e.faces.len() == 2 {
                p = mul(
                    add(add(points[e.a], points[e.b]), add(centers[e.faces[0]], centers[e.faces[1]])),
                    0.25,
                );
            }
            p[0] = mid[0];
            p[1] = mid[1];
            refined.push(p);
        }
        let face_base = refined.len();
        refined.extend_from_slice(&centers);
        let mut subdivided = Vec::with_capacity(faces.len() * 4);
        for (fi, face) in faces.iter().enumerate() {
            for j in 0..4 {
                subdivided.push([
                    face[j],
                    edge_base + face_edges[fi][j],
                    face_base + fi,
                    edge_base + face_edges[fi][(j + 3) % 4],
                ]);
            }
        }
        points = refined;
        faces = subdivided;
    }

    let vertices = points
        .iter()
        .map(|p| [p[2] * 2. - 1., p[3] * 2. - 1., p[4] * 2. - 1., p[5] * 2. - 1., p[0], p[1]])
        .collect();
    let indices = faces
        .iter()
        .flat_map(|f| [f[0], f[2], f[3], f[0], f[1], f[2]].map(|i| i as u32))
        .collect();
    (vertices, indices)
}

#[cfg(test)]
mod tests {
    #[test]
    fn mesh_is_three_times_subdivided_and_keeps_corners() {
        let (v, i) = super::make_mesh();
        assert_eq!(i.len(), 25 * 64 * 6);
        assert!(i.iter().all(|&x| (x as usize) < v.len()));
        // Corner (0,0) never moves: clip (-1,-1), uv (0,0).
        assert_eq!(v[0], [-1., -1., -1., -1., 0., 0.]);
        assert!(v.iter().all(|p| (0.0..=1.0).contains(&p[4]) && (0.0..=1.0).contains(&p[5])));
    }
}
