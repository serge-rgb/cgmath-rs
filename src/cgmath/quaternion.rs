// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num::{zero, one, cast, sqrt};

use angle::{Angle, Rad, acos, cos, sin, sin_cos};
use array::{Array, build};
use matrix::{Mat3, ToMat3};
use vector::{Vec3, Vector, EuclideanVector};

/// A quaternion in scalar/vector form
#[deriving(Clone, Eq)]
pub struct Quat<S> { s: S, v: Vec3<S> }

array!(impl<S> Quat<S> -> [S, ..4] _4)
approx_eq!(impl<S> Quat<S>)

pub trait ToQuat<S: Float> {
    fn to_quat(&self) -> Quat<S>;
}

impl<S: Float> Quat<S> {
    /// Construct a new quaternion from one scalar component and three
    /// imaginary components
    #[inline]
    pub fn new(w: S, xi: S, yj: S, zk: S) -> Quat<S> {
        Quat::from_sv(w, Vec3::new(xi, yj, zk))
    }

    /// Construct a new quaternion from a scalar and a vector
    #[inline]
    pub fn from_sv(s: S, v: Vec3<S>) -> Quat<S> {
        Quat { s: s, v: v }
    }

    #[inline]
    pub fn look_at(dir: &Vec3<S>, up: &Vec3<S>) -> Quat<S> {
        Mat3::look_at(dir, up).to_quat()
    }

    /// Create a matrix from a rotation around the `x` axis (pitch).
    #[inline]
    pub fn from_angle_x<A: Angle<S>>(theta: A) -> Quat<S> {
        Quat::new(cos(theta.mul_s(cast(0.5).unwrap())), sin(theta), zero(), zero())
    }

    /// Create a matrix from a rotation around the `y` axis (yaw).
    #[inline]
    pub fn from_angle_y<A: Angle<S>>(theta: A) -> Quat<S> {
        Quat::new(cos(theta.mul_s(cast(0.5).unwrap())), zero(), sin(theta), zero())
    }

    /// Create a matrix from a rotation around the `z` axis (roll).
    #[inline]
    pub fn from_angle_z<A: Angle<S>>(theta: A) -> Quat<S> {
        Quat::new(cos(theta.mul_s(cast(0.5).unwrap())), zero(), zero(), sin(theta))
    }

    /// Create a quaternion from a set of euler angles.
    ///
    /// # Parameters
    ///
    /// - `x`: the angular rotation around the `x` axis (pitch).
    /// - `y`: the angular rotation around the `y` axis (yaw).
    /// - `z`: the angular rotation around the `z` axis (roll).
    pub fn from_euler<A: Angle<S>>(x: A, y: A, z: A) -> Quat<S> {
        // http://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Conversion
        let (sx2, cx2) = sin_cos(x.mul_s(cast(0.5).unwrap()));
        let (sy2, cy2) = sin_cos(y.mul_s(cast(0.5).unwrap()));
        let (sz2, cz2) = sin_cos(z.mul_s(cast(0.5).unwrap()));

        Quat::new(cz2 * cx2 * cy2 + sz2 * sx2 * sy2,
                  sz2 * cx2 * cy2 - cz2 * sx2 * sy2,
                  cz2 * sx2 * cy2 + sz2 * cx2 * sy2,
                  cz2 * cx2 * sy2 - sz2 * sx2 * cy2)
    }

    /// Create a quaternion from a rotation around an arbitrary axis
    #[inline]
    pub fn from_axis_angle<A: Angle<S>>(axis: &Vec3<S>, angle: A) -> Quat<S> {
        let half = angle.mul_s(cast(0.5).unwrap());
        Quat::from_sv(cos(half.clone()),
                      axis.mul_s(sin(half)))
    }

    /// The additive identity, ie: `q = 0 + 0i + 0j + 0i`
    #[inline]
    pub fn zero() -> Quat<S> {
        Quat::new(zero(), zero(), zero(), zero())
    }

    /// The multiplicative identity, ie: `q = 1 + 0i + 0j + 0i`
    #[inline]
    pub fn identity() -> Quat<S> {
        Quat::from_sv(one::<S>(), Vec3::zero())
    }

    /// The result of multiplying the quaternion a scalar
    #[inline]
    pub fn mul_s(&self, value: S) -> Quat<S> {
        Quat::from_sv(self.s * value, self.v.mul_s(value))
    }

    /// The result of dividing the quaternion a scalar
    #[inline]
    pub fn div_s(&self, value: S) -> Quat<S> {
        Quat::from_sv(self.s / value, self.v.div_s(value))
    }

    /// The result of multiplying the quaternion by a vector
    #[inline]
    pub fn mul_v(&self, vec: &Vec3<S>) -> Vec3<S>  {
        let tmp = self.v.cross(vec).add_v(&vec.mul_s(self.s.clone()));
        self.v.cross(&tmp).mul_s(cast(2).unwrap()).add_v(vec)
    }

    /// The sum of this quaternion and `other`
    #[inline]
    pub fn add_q(&self, other: &Quat<S>) -> Quat<S> {
        build(|i| self.i(i).add(other.i(i)))
    }

    /// The difference between this quaternion and `other`
    #[inline]
    pub fn sub_q(&self, other: &Quat<S>) -> Quat<S> {
        build(|i| self.i(i).add(other.i(i)))
    }

    /// The the result of multipliplying the quaternion by `other`
    pub fn mul_q(&self, other: &Quat<S>) -> Quat<S> {
        Quat::new(self.s * other.s - self.v.x * other.v.x - self.v.y * other.v.y - self.v.z * other.v.z,
                  self.s * other.v.x + self.v.x * other.s + self.v.y * other.v.z - self.v.z * other.v.y,
                  self.s * other.v.y + self.v.y * other.s + self.v.z * other.v.x - self.v.x * other.v.z,
                  self.s * other.v.z + self.v.z * other.s + self.v.x * other.v.y - self.v.y * other.v.x)
    }

    #[inline]
    pub fn mul_self_s(&mut self, s: S) {
        self.each_mut(|_, x| *x = x.mul(&s))
    }

    #[inline]
    pub fn div_self_s(&mut self, s: S) {
        self.each_mut(|_, x| *x = x.div(&s))
    }

    #[inline]
    pub fn add_self_q(&mut self, other: &Quat<S>) {
        self.each_mut(|i, x| *x = x.add(other.i(i)));
    }

    #[inline]
    pub fn sub_self_q(&mut self, other: &Quat<S>) {
        self.each_mut(|i, x| *x = x.sub(other.i(i)));
    }

    #[inline]
    pub fn mul_self_q(&mut self, other: &Quat<S>) {
        *self = self.mul_q(other);
    }

    /// The dot product of the quaternion and `other`
    #[inline]
    pub fn dot(&self, other: &Quat<S>) -> S {
        self.s * other.s + self.v.dot(&other.v)
    }

    /// The conjugate of the quaternion
    #[inline]
    pub fn conjugate(&self) -> Quat<S> {
        Quat::from_sv(self.s.clone(), -self.v.clone())
    }

    /// The squared magnitude of the quaternion. This is useful for
    /// magnitude comparisons where the exact magnitude does not need to be
    /// calculated.
    #[inline]
    pub fn magnitude2(&self) -> S {
        self.s * self.s + self.v.length2()
    }

    /// The magnitude of the quaternion
    ///
    /// # Performance notes
    ///
    /// For instances where the exact magnitude of the quaternion does not need
    /// to be known, for example for quaternion-quaternion magnitude comparisons,
    /// it is advisable to use the `magnitude2` method instead.
    #[inline]
    pub fn magnitude(&self) -> S {
        sqrt(self.magnitude2())
    }

    /// The normalized quaternion
    #[inline]
    pub fn normalize(&self) -> Quat<S> {
        self.mul_s(one::<S>() / self.magnitude())
    }

    /// Normalised linear interpolation
    ///
    /// # Return value
    ///
    /// The intoperlated quaternion
    pub fn nlerp(&self, other: &Quat<S>, amount: S) -> Quat<S> {
        self.mul_s(one::<S>() - amount).add_q(&other.mul_s(amount)).normalize()
    }
}

impl<S: Float> Quat<S> {
    /// Spherical Linear Intoperlation
    ///
    /// Perform a spherical linear interpolation between the quaternion and
    /// `other`. Both quaternions should be normalized first.
    ///
    /// # Return value
    ///
    /// The intoperlated quaternion
    ///
    /// # Performance notes
    ///
    /// The `acos` operation used in `slerp` is an expensive operation, so unless
    /// your quarternions a far away from each other it's generally more advisable
    /// to use `nlerp` when you know your rotations are going to be small.
    ///
    /// - [Understanding Slerp, Then Not Using It]
    ///   (http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/)
    /// - [Arcsynthesis OpenGL tutorial]
    ///   (http://www.arcsynthesis.org/gltut/Positioning/Tut08%20Interpolation.html)
    pub fn slerp(&self, other: &Quat<S>, amount: S) -> Quat<S> {
        use std::num::cast;

        let dot = self.dot(other);
        let dot_threshold = cast(0.9995).unwrap();

        // if quaternions are close together use `nlerp`
        if dot > dot_threshold {
            self.nlerp(other, amount)
        } else {
            // stay within the domain of acos()
            let robust_dot = dot.clamp(&-one::<S>(), &one::<S>());

            let theta: Rad<S> = acos(robust_dot.clone());   // the angle between the quaternions
            let theta: Rad<S> = theta.mul_s(amount);        // the fraction of theta specified by `amount`

            let q = other.sub_q(&self.mul_s(robust_dot))
                         .normalize();

            self.mul_s(cos(theta.clone()))
                .add_q(&q.mul_s(sin(theta)))
        }
    }
}

impl<S: Float> ToMat3<S> for Quat<S> {
    /// Convert the quaternion to a 3 x 3 rotation matrix
    fn to_mat3(&self) -> Mat3<S> {
        let x2 = self.v.x + self.v.x;
        let y2 = self.v.y + self.v.y;
        let z2 = self.v.z + self.v.z;

        let xx2 = x2 * self.v.x;
        let xy2 = x2 * self.v.y;
        let xz2 = x2 * self.v.z;

        let yy2 = y2 * self.v.y;
        let yz2 = y2 * self.v.z;
        let zz2 = z2 * self.v.z;

        let sy2 = y2 * self.s;
        let sz2 = z2 * self.s;
        let sx2 = x2 * self.s;

        Mat3::new(one::<S>() - yy2 - zz2, xy2 + sz2, xz2 - sy2,
                  xy2 - sz2, one::<S>() - xx2 - zz2, yz2 + sx2,
                  xz2 + sy2, yz2 - sx2, one::<S>() - xx2 - yy2)
    }
}

impl<S: Float> Neg<Quat<S>> for Quat<S> {
    #[inline]
    fn neg(&self) -> Quat<S> {
        Quat::from_sv(-self.s, -self.v)
    }
}

impl<S> ToStr for Quat<S> {
    fn to_str(&self) -> ~str {
        fmt!("%? + %?i + %?j + %?k", self.s, self.v.x, self.v.y, self.v.z)
    }
}
