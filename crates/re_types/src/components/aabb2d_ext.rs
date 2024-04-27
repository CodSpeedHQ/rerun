use super::AABB2D;

impl From<AABB2D> for emath::Rect {
    #[inline]
    fn from(v: AABB2D) -> Self {
        Self {
            min: emath::pos2(v.min_xy[0] as f32, v.min_xy[1] as f32),
            max: emath::pos2(v.max_xy[0] as f32, v.max_xy[1] as f32),
        }
    }
}
