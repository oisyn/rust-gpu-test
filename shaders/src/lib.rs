#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr, lang_items),
    register_attr(spirv)
)]

use spirv_std::glam::*;

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;


#[spirv(fragment)]
pub fn main_fs(
    output: &mut Vec4,
) {
    *output = vec4(1.0, 0.5, 0.0, 1.0);
}
