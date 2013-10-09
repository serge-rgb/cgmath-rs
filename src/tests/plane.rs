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

use cgmath::plane::*;
use cgmath::point::*;

#[test]
fn test_from_points() {
    assert_eq!(Plane::from_points(Point3::new(5f32, 0f32,  5f32),
                                  Point3::new(5f32, 5f32,  5f32),
                                  Point3::new(5f32, 0f32, -1f32)), Some(Plane::from_abcd(-1f32, 0f32, 0f32, 5f32)));

    assert_eq!(Plane::from_points(Point3::new(0f32, 5f32, -5f32),
                                  Point3::new(0f32, 5f32,  0f32),
                                  Point3::new(0f32, 5f32,  5f32)), None);     // The points are parallel
}
