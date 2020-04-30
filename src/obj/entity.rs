//! Contains the entity types that are used when working with the [`wavefront obj`] format.
//! 
//! [`wavefront obj`]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
//! 

use std::io::{Cursor, BufReader, BufWriter};
use crate::obj::read_lexer::*;
use crate::obj::format_writer::*;

pub type Format = String;

/// Contains all possible entities that can exist in an OBJ format.
#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Comment{content: String},
    Object{name: String},
    Group{name: String},
    SmoothingGroup{name: String},
    Mtllib{name: String},
    Usemtl{name: String},
    /// Vertex consists of `x`, `y`, `z` and `w` whereas `w` is optional.\
    /// Example xyzw: `v 0.1 1.2 2.3 3.4`\
    /// Example xyz: `v 0.1 1.2 2.3`
    Vertex{x: f64, y: f64, z: f64, w: Option<f64>},
    /// VertexNormal consists of `x`, `y`and `z`. The normal will usually but is not required to be a unit vector.
    /// Example: `vn 0.1 1.2 2.3`
    VertexNormal{x: f64, y: f64, z: f64},
    /// VertexTexture consists of `x`, `y` and `z` whereas `z` is optional.\
    /// Example xyz: `v 0.1 1.2 2.3`\
    /// Example xy: `vt 0.1 1.2`
    VertexTexture{x: f64, y: f64, z: Option<f64>},
    /// Face consists of an arbitrary number (whereas n >= 3) of complex vertices that describe the polygon.\
    /// Example (vertex): `f 0 3 6`\
    /// Example (vertex+normal+texture): `f 0/1/2 3/4/5 6/7/8`\
    /// Example (vertex+normal): `f 0/1 3/4 6/7`\
    /// Example (vertex+texture): `f 0//2 3//5 6//8`
    Face{vertices: Vec<FaceVertex>},
    /// Line consists of an arbitrary number (whereas n >= 2) of vertices that describe the path.
    Line{vertices: Vec<i64>},
}

impl Entity {
    pub fn token(&self) -> &str {
        match self {
            Self::Comment{..} => "#",
            Self::Object{..} => "o",
            Self::Group{..} => "g",
            Self::SmoothingGroup{..} => "s",
            Self::Mtllib{..} => "mtllib",
            Self::Usemtl{..} => "usemtl",
            Self::Vertex{..} => "v",
            Self::VertexNormal{..} => "vn",
            Self::VertexTexture{..} => "vt",
            Self::Face{..} => "f",
            Self::Line{..} => "l",
        }
    }
}

/// Describes a vertex in a face.
#[derive(Debug, Clone, PartialEq)]
pub struct FaceVertex {
    /// The vertex index itself.
    pub vertex: i64,
    /// The normal of the vertex (optional).
    pub normal: Option<i64>,
    /// The texture map info for the vertex (optional).
    pub texture: Option<i64>,
}

impl FaceVertex {
    pub fn new(vertex: i64) -> Self {
        Self {
            vertex,
            normal: None,
            texture: None,
        }
    }

    pub fn new2(vertex: i64, normal: Option<i64>, texture: Option<i64>) -> Self {
        Self {
            vertex,
            normal,
            texture,
        }
    }
}

impl ToString for Entity {
    fn to_string(&self) -> String {
        let mut result = String::new();
        FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &self);
        result
    }
}

impl From<Format> for Entity {
    fn from(input: Format) -> Self {
        ReadLexer::read_line(&mut BufReader::new(Cursor::new(input))).unwrap()
    }
}

impl Into<Format> for Entity {
    fn into(self) -> String {
        self.to_string()
    }
}