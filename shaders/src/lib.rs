#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr, lang_items),
    register_attr(spirv)
)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
//#![deny(warnings)]

//#[cfg(all(not(test), target_arch = "spirv"))]
use spirv_std::glam::*;

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;


#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] in_coord: Vec4,
    output: &mut Vec4,
) {
    *output = vec4(in_coord.x.abs(), 1.0, 1.0, 1.0);
}
