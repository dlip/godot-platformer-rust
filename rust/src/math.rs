pub const PI: f32 = 3.14159265358979323846;
pub const TAU: f32 = 6.2831853071795864769252867666;

#[inline(always)]
pub fn lerp(minv: f32, maxv: f32, t: f32) -> f32 {
    minv + t * (maxv - minv)
}
