#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Face {
    pub vertex_indexes: Vec<usize>,
}

#[derive(Debug)]
pub struct ObjModel {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

pub fn parse(contents: &str) -> ObjModel {
    let mut vertices: Vec<Vertex> = vec![];
    let mut faces: Vec<Face> = vec![];

    for line in contents.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.starts_with("v ") {
            let vertex_line: Vec<&str> = line.split(" ").collect();

            vertices.push(Vertex {
                x: vertex_line[1].parse().unwrap_or(0.0),
                y: vertex_line[2].parse().unwrap_or(0.0),
                z: vertex_line[3].parse().unwrap_or(0.0),
            });
        }

        if trimmed_line.starts_with("f ") {
            let face_line: Vec<&str> = trimmed_line.split(" ").skip(1).collect();

            let mut vertex_indexes: Vec<usize> = vec![];

            for face_vertex in face_line {
                let vertex_line: Vec<&str> = face_vertex.split("/").collect();

                let index = vertex_line[0].parse().unwrap_or(0);
                vertex_indexes.push(index - 1);
            }

            faces.push(Face { vertex_indexes });
        }
    }

    ObjModel { vertices, faces }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertices() {
        let contents = r#"
        v 1.0 2.0 3.0
        v 4.0 5.0 6.0
        v 7.0 8.0 9.0
        "#;

        let model = parse(&contents);

        assert_eq!(model.vertices.len(), 3);
    }

    #[test]
    fn test_faces() {
        let contents = r#"
        f 3572/4012/3570 3567/4013/3571 3582/4014/3572
        f 3552/4023/3581 3556/4024/3582 3573/4020/3578 3569/4019/3577
        "#;

        let model = parse(&contents);

        assert_eq!(model.faces.len(), 2);

        assert_eq!(model.faces[0].vertex_indexes.len(), 3);
        assert_eq!(model.faces[0].vertex_indexes[0], 3571);
        assert_eq!(model.faces[0].vertex_indexes[1], 3566);
        assert_eq!(model.faces[0].vertex_indexes[2], 3581);

        assert_eq!(model.faces[1].vertex_indexes.len(), 4);
        assert_eq!(model.faces[1].vertex_indexes[0], 3551);
        assert_eq!(model.faces[1].vertex_indexes[1], 3555);
        assert_eq!(model.faces[1].vertex_indexes[2], 3572);
        assert_eq!(model.faces[1].vertex_indexes[3], 3568);
    }

    #[test]
    fn test_nothing_parsed() {
        let contents = r##"
        # Blender 5.3.0
        # www.blender.org
        mtllib sap_avali.mtl
        o Body
        vn 0.0230 0.9988 0.0433
        vt 0.545880 0.879236
        "##;

        let model = parse(&contents);

        assert_eq!(model.faces.len(), 0);
        assert_eq!(model.vertices.len(), 0);
    }
}
