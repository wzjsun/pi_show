use cg::Matrix;

use component::math::*;
use component::component_def::Transform;
/// `Transform` is used to store and manipulate the postiion, rotation and scale
/// of the object. We use a left handed, y-up world coordinate system.

impl Default for Transform {
    fn default() -> Self {
        Transform {
            scale: Vector3::new(1.0, 1.0, 1.0),
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            rot_ref: Vector3::new(0.0, 0.0, 0.0),
            dirty: false
        }
    }
}

// impl ::std::ops::Mul for Transform {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self {
//         Transform {
//             position: self.rotation * (rhs.position * self.scale) + self.position,
//             rotation: self.rotation * rhs.rotation,
//             scale: self.scale * rhs.scale,
//         }
//     }
// }
 
impl Transform {
    /// Returns a transform that "un-does" this one.
    // #[inline]
    // pub fn inverse(self) -> Option<Self> {
    //     if self.scale <= ::std::f32::EPSILON {
    //         None
    //     } else {
    //         let s = 1.0 / self.scale;
    //         let r = self.rotation.invert();
    //         let d = r.rotate_vector(self.position) * -s;

    //         Some(Transform {
    //             scale: s,
    //             rotation: r,
    //             position: d,
    //         })
    //     }
    // }

    /// Transforms direction from local space to transform's space.
    ///
    /// This operation is not affected by scale or position of the transform. The returned
    /// vector has the same length as direction.
    #[inline]
    pub fn transform_direction<T>(&self, v: T) -> Vector3
    where
        T: Into<Vector3>,
    {
        self.rotation * v.into()
    }

    /// Transforms vector from local space to transform's space.
    ///
    /// This operation is not affected by position of the transform, but is is affected by scale.
    /// The returned vector may have a different length than vector.
    // #[inline]
    // pub fn transform_vector<T>(&self, v: T) -> Vector3<f32>
    // where
    //     T: Into<Vector3<f32>>,
    // {
    //     self.rotation * (v.into() * self.scale)
    // }

    // /// Transforms points from local space to transform's space.
    // #[inline]
    // pub fn transform_point<T>(&self, v: T) -> Vector3<f32>
    // where
    //     T: Into<Vector3<f32>>,
    // {
    //     self.rotation * (v.into() * self.scale) + self.position
    // }

    /// Returns the up direction in transform's space, which is looking down the positive y-axis.
    #[inline]
    pub fn up(&self) -> Vector3 {
        self.transform_direction(Vector3::new(0.0, 1.0, 0.0))
    }

    /// Returns the forward direction in transform's space, which is looking down the positive z-axis.
    #[inline]
    pub fn forward(&self) -> Vector3 {
        self.transform_direction(Vector3::new(0.0, 0.0, 1.0))
    }

    /// Returns the right direction in transform's space, which is looking down the positive x-axis.
    #[inline]
    pub fn right(&self) -> Vector3 {
        self.transform_direction(Vector3::new(1.0, 0.0, 0.0))
    }

    // Returns the view matrix from world space to view space.
    #[inline]
    pub fn view_matrix(&self) -> Matrix4 {
        // M = ( T * R ) ^ -1
        let it = Matrix4::from_translation(-self.position);
        let ir = Matrix4::from(self.rotation).transpose();
        ir * it
    }

    /// Returns the matrix representation.
    #[inline]
    pub fn matrix(&self) -> Matrix4 {
        // M = T * R * S
        let r: Matrix3 = self.rotation.into();
        let s: Matrix3 = self.rotation.into();
        let mut m: Matrix4 = (r * s).into();
        m.w = self.position.extend(1.0);
        m
    }
}