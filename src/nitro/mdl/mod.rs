pub mod render_cmds;
mod read;
mod xform;

use cgmath::Matrix3;
use cgmath::Matrix4;
use nitro::name::Name;
use nitro::tex::TextureParameters;
use util::cur::Cur;

pub use self::read::read_mdl;
pub use self::xform::pivot_mat;

#[derive(Debug, Clone)]
pub struct Mdl<'a> {
    pub models: Vec<Model<'a>>,
}

#[derive(Debug, Clone)]
pub struct Model<'a> {
    pub name: Name,
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh<'a>>,
    pub objects: Vec<Object>,
    pub blend_matrices: Vec<BlendMatrixPair>,
    pub render_cmds_cur: Cur<'a>,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub name: Name,
    pub texture_name: Option<Name>,
    pub palette_name: Option<Name>,
    pub params: TextureParameters,
    pub width: u16,
    pub height: u16,
    pub texture_mat: Matrix4<f64>,
}

#[derive(Debug, Clone)]
pub struct Mesh<'a> {
    pub name: Name,
    pub commands: &'a [u8],
}

#[derive(Debug, Clone)]
pub struct Object {
    pub name: Name,
    pub xform: Matrix4<f64>,
}

/// A pair of matrices used for the blending render command (opcode 0x09).
///
/// The first one is used in calculating vertex positions. The second is used
/// for normals (?) which we don't currently handle.
#[derive(Debug, Clone)]
pub struct BlendMatrixPair(pub Matrix4<f64>, pub Matrix3<f64>);