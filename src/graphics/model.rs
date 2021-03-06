use prelude::*;

use gfx;
use super::renderer::Vertex;

use world::chunk::Chunk;
use world::block::*;
use world::registry::Registry;

pub struct Model<R: gfx::Resources> {
    pub vbo: gfx::handle::Buffer<R, Vertex>,
    pub slice: gfx::Slice<R>,
    pub model: TransformMatrix,
}

// fn darken(color: &mut Color, amount: u8) {
//     for i in 0..3 {
//         color[i] = U8Norm(if amount <= color[i].0 {
//             color[i].0 - amount
//         } else {
//             0
//         })
//     }
// }

const TEXTURE_SIZE: u16 = 64;
const TEXTURE_NORMALIZER: u16 = (0x10000 / TEXTURE_SIZE as u32) as u16;
const TEXEL_SIZE: u16 = 16;
const TEXEL_NORMALIZER: u16 = TEXTURE_NORMALIZER * TEXEL_SIZE;

impl<R: gfx::Resources> Model<R> {
    pub fn new<F: gfx::traits::FactoryExt<R>>(factory: &mut F,
                                              chunk: &Chunk,
                                              registry: &Registry)
                                              -> Option<Model<R>> {
        let mut verts = Vec::new();

        for loc in chunk.iter() {
            let block = chunk.get_block_local(loc);
            if !block.is_empty() {
                let texture = registry.lookup_texture(block.id)
                    .expect("Could not find texture for block id");

                for face in Face::iter() {
                    if block.is_visible(*face) {
                        let face_texture = texture.get_face(*face);
                        let true_texture = point2(face_texture.x as u16 * TEXEL_NORMALIZER,
                                                  face_texture.y as u16 * TEXEL_NORMALIZER);

                        let light = block.face_light(*face);
                        match *face {
                            Face::Bottom => {
                                make_bottom(loc, true_texture, light, &mut verts);
                            }
                            Face::Top => {
                                make_top(loc, true_texture, light, &mut verts);
                            }
                            Face::Left => {
                                make_left(loc, true_texture, light, &mut verts);
                            }
                            Face::Right => {
                                make_right(loc, true_texture, light, &mut verts);
                            }
                            Face::Front => {
                                make_front(loc, true_texture, light, &mut verts);
                            }
                            Face::Back => {
                                make_back(loc, true_texture, light, &mut verts);
                            }
                        }
                    }
                }
            }
        }

        if verts.len() > 0 {
            let (vbo, slice) = factory.create_vertex_buffer_with_slice(verts.as_slice(), ());
            Some(Model {
                vbo: vbo,
                slice: slice,
                model: Matrix4::from_translation(vec3(chunk.origin.x as f32,
                                                      chunk.origin.y as f32,
                                                      chunk.origin.z as f32))
                    .into(),
            })
        } else {
            None
        }
    }
}


fn make_bottom(origin: Point3<u8>,
               texture: Point2<u16>,
               light: TotalLightLevel,
               vert_out: &mut Vec<Vertex>) {
    let v = [vnew(point3(origin.x, origin.y, origin.z + 1),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x, origin.y, origin.z), texture, light),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, TEXEL_NORMALIZER - 1),
                  light),

             vnew(point3(origin.x, origin.y, origin.z), texture, light),
             vnew(point3(origin.x + 1, origin.y, origin.z),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, TEXEL_NORMALIZER - 1),
                  light)];

    vert_out.extend_from_slice(&v);
}

fn make_top(origin: Point3<u8>,
            texture: Point2<u16>,
            light: TotalLightLevel,
            vert_out: &mut Vec<Vertex>) {
    let v = [vnew(point3(origin.x, origin.y + 1, origin.z + 1),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x, origin.y + 1, origin.z), texture, light),

             vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light),
             vnew(point3(origin.x, origin.y + 1, origin.z), texture, light)];

    vert_out.extend_from_slice(&v);
}

fn make_back(origin: Point3<u8>,
             texture: Point2<u16>,
             light: TotalLightLevel,
             vert_out: &mut Vec<Vertex>) {
    let v = [vnew(point3(origin.x, origin.y, origin.z), texture, light),
             vnew(point3(origin.x, origin.y + 1, origin.z),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x + 1, origin.y, origin.z),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light),

             vnew(point3(origin.x, origin.y + 1, origin.z),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z),
                  texture + vec2(TEXEL_NORMALIZER - 1, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x + 1, origin.y, origin.z),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light)];

    vert_out.extend_from_slice(&v);
}

fn make_front(origin: Point3<u8>,
              texture: Point2<u16>,
              light: TotalLightLevel,
              vert_out: &mut Vec<Vertex>) {
    let v = [vnew(point3(origin.x, origin.y, origin.z + 1), texture, light),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light),
             vnew(point3(origin.x, origin.y + 1, origin.z + 1),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light),

             vnew(point3(origin.x + 1, origin.y, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x, origin.y + 1, origin.z + 1),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light)];

    vert_out.extend_from_slice(&v);
}

fn make_left(origin: Point3<u8>,
             texture: Point2<u16>,
             light: TotalLightLevel,
             vert_out: &mut Vec<Vertex>) {
    let v = [vnew(point3(origin.x, origin.y, origin.z), texture, light),
             vnew(point3(origin.x, origin.y, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light),
             vnew(point3(origin.x, origin.y + 1, origin.z),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light),

             vnew(point3(origin.x, origin.y, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light),
             vnew(point3(origin.x, origin.y + 1, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x, origin.y + 1, origin.z),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light)];

    vert_out.extend_from_slice(&v);
}

fn make_right(origin: Point3<u8>,
              texture: Point2<u16>,
              light: TotalLightLevel,
              vert_out: &mut Vec<Vertex>) {
    let v = [vnew(point3(origin.x + 1, origin.y, origin.z), texture, light),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light),

             vnew(point3(origin.x + 1, origin.y + 1, origin.z),
                  texture + vec2(0, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x + 1, origin.y + 1, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, TEXEL_NORMALIZER - 1),
                  light),
             vnew(point3(origin.x + 1, origin.y, origin.z + 1),
                  texture + vec2(TEXEL_NORMALIZER - 1, 0),
                  light)];

    vert_out.extend_from_slice(&v);
}

fn vnew(position: Point3<u8>, texture: Point2<u16>, light: TotalLightLevel) -> Vertex {
    let uv = [U16Norm(texture.x), U16Norm(texture.y)];
    let light_val = (((light.0).0 & 0b1111) << 4) | ((light.1).0 & 0b1111);

    Vertex {
        position: [position.x, position.y, position.z, light_val],
        uv: uv,
    }
}
