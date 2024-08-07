use crate::datatypes::Quaternion;

use super::Rotation3D;

impl Rotation3D {
    /// The identity rotation, representing no rotation.
    pub const IDENTITY: Self = Self::Quaternion(Quaternion::IDENTITY);
}

impl From<Quaternion> for Rotation3D {
    #[inline]
    fn from(q: Quaternion) -> Self {
        Self::Quaternion(q)
    }
}

impl From<crate::datatypes::RotationAxisAngle> for Rotation3D {
    #[inline]
    fn from(r: crate::datatypes::RotationAxisAngle) -> Self {
        Self::AxisAngle(r)
    }
}

#[cfg(feature = "glam")]
impl From<Rotation3D> for glam::Quat {
    #[inline]
    fn from(val: Rotation3D) -> Self {
        match val {
            Rotation3D::Quaternion(v) => v.into(),
            Rotation3D::AxisAngle(a) => a.into(),
        }
    }
}

#[cfg(feature = "glam")]
impl From<glam::Quat> for Rotation3D {
    #[inline]
    fn from(val: glam::Quat) -> Self {
        Self::Quaternion(val.into())
    }
}

#[cfg(feature = "mint")]
impl From<Rotation3D> for mint::Quaternion<f32> {
    #[inline]
    fn from(val: Rotation3D) -> Self {
        match val {
            Rotation3D::Quaternion(v) => v.into(),
            Rotation3D::AxisAngle(a) => a.into(),
        }
    }
}

#[cfg(feature = "mint")]
impl From<mint::Quaternion<f32>> for Rotation3D {
    #[inline]
    fn from(val: mint::Quaternion<f32>) -> Self {
        Self::Quaternion(val.into())
    }
}

impl Default for Rotation3D {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

// TODO(#6831): Transitional, to be removed with the rest of this file.
impl From<crate::Rotation3D> for Rotation3D {
    #[inline]
    fn from(r: crate::Rotation3D) -> Self {
        match r {
            crate::Rotation3D::Quaternion(q) => Self::Quaternion(q.0),
            crate::Rotation3D::AxisAngle(a) => Self::AxisAngle(a.0),
        }
    }
}
