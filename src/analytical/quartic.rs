// Copyright (c) 2015, Mikhail Vorotilov
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::num::Float;
use std::cmp::Ordering;

/// Solves a quartic equation a4*x^4 + a3*x^3 + a2*x^2 + a1*x + a0 = 0.
///
/// Returned roots are ordered.
/// Precision is about 5e-15 for f64, 5e-7 for f32.
///
/// # Examples
///
/// ```
/// use roots::find_roots_quartic;
///
/// let one_root = find_roots_quartic(1f64, 0f64, 0f64, 0f64, 0f64);
/// // Returns [0f64] as 'x^4 = 0' has one root 0
///
/// let two_roots = find_roots_quartic(1f32, 0f32, 0f32, 0f32, -1f32);
/// // Returns [-1f32, 1f32] as 'x^4 - 1 = 0' has roots -1 and 1
/// ```
pub fn find_roots_quartic<F:Float>(a4:F, a3:F, a2:F, a1:F, a0:F) -> Vec<F> {
  // Handle non-standard cases
  let mut roots = if a4 == F::zero() {
    // a4 = 0; a3*x^3 + a2*x^2 + a1*x + a0 = 0; solve cubic equation
    super::cubic::find_roots_cubic(a3, a2, a1, a0)
  } else if a0 == F::zero() {
    // a0 = 0; x^4 + a2*x^2 + a1*x = 0; reduce to cubic and arrange results
    let mut tmp = vec![F::zero()];
    tmp.push_all(super::cubic::find_roots_cubic(a4, a3, a2, a1).as_slice());
    tmp
  } else if a1 == F::zero() && a3 == F::zero() {
    // a1 = 0, a3 =0; a4*x^4 + a2*x^2 + a0 = 0; solve bi-quadratic equation
    super::biquadratic::find_roots_biquadratic(a4, a2, a0)
  } else {
    let _2 = F::one() + F::one();
    let _3 = _2 + F::one();
    let _4 = _2 + _2;
    let _8 = _4 + _4;
    let _16 = _4 * _4;
    let _64 = _8 * _8;
    let _256 = _8 * _8 * _4;

    // a4*x^4 + a3*x^3 + a2*x^2 + a1*x + a0 = 0 => x^4 + a*x^3 + b*x^2 + c*x + d = 0.
    let (a, b, c, d) = (a3/a4, a2/a4, a1/a4, a0/a4);
    // x^4 + a*x^3 + b*x^2 + c*x + d = 0 => y^4 + p*y^2 + q*y + r.
    let _a2 = a*a;
    let subst = -a3/(_4*a4);
    let (p, q, r) = ( (_8*b - _3*_a2)/_8, (_a2*a - _4*a*b + _8*c)/_8, (_256*d - _3*_a2*_a2 - _64*c*a + _16*_a2*b)/_256);

    let y_roots = super::quartic_depressed::find_roots_quartic_depressed(p, q, r);
    let x_roots = y_roots.map_in_place(|y| y+subst);
    x_roots
  };

  roots.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
  roots.dedup();
  roots
}

#[test]
fn test_find_roots_quartic() {
  assert_eq!(find_roots_quartic(1f32, 0f32, 0f32, 0f32, 0f32), [0f32]);
  assert_eq!(find_roots_quartic(1f64, 0f64, 0f64, 0f64, -1f64), [-1f64, 1f64]);
  assert_eq!(find_roots_quartic(1f64, -10f64, 35f64, -50f64, 24f64), [1f64, 2f64, 3f64, 4f64]);

  match find_roots_quartic(3f64, 5f64, -5f64, -5f64, 2f64).as_slice() {
    [x1, x2, x3, x4] => {
      assert_float_eq!(1e-15, x1, -2f64 );
      assert_float_eq!(1e-15, x2, -1f64 );
      assert_float_eq!(1e-15, x3, 0.33333333333333333f64 );
      assert_float_eq!(2e-15, x4, 1f64 );
    },
    _ => { assert!(false); }
  }

  match find_roots_quartic(3f32, 5f32, -5f32, -5f32, 2f32).as_slice() {
    [x1, x2, x3, x4] => {
      assert_float_eq!(5e-7, x1, -2f32 );
      assert_float_eq!(5e-7, x2, -1f32 );
      assert_float_eq!(5e-7, x3, 0.33333333333333333f32 );
      assert_float_eq!(5e-7, x4, 1f32 );
    },
    _ => { assert!(false); }
  }
}