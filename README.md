Library of well known algorithms for numerical root finding.

[![License](https://img.shields.io/badge/License-BSD%202--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)[![Build Status](https://travis-ci.org/vorot/roots.svg)](https://travis-ci.org/vorot/roots)[![Crates.io](https://img.shields.io/crates/v/roots.svg)](https://crates.io/crates/roots)

## Features

- Iterative approximation:
   - [Newton-Raphson](http://en.wikipedia.org/wiki/Newton%27s_method) method
   - [Secant](http://en.wikipedia.org/wiki/Secant_method) method
   - [Regula falsi](http://en.wikipedia.org/wiki/False_position_method) method (with Illinois modification)
   - [Brent-Dekker](http://en.wikipedia.org/wiki/Brent%27s_method) method
- Solving polynomial equations
   - [Linear](http://en.wikipedia.org/wiki/Linear_equation) equation (editors' choice)
   - [Quadratic](http://en.wikipedia.org/wiki/Quadratic_equation) equation
   - [Cubic](http://en.wikipedia.org/wiki/Cubic_function) equation
   - [Quartic](http://en.wikipedia.org/wiki/Quartic_function) equation

## Usage

```rust
extern crate roots;
use roots::Roots;
use roots::find_roots_cubic;
use roots::find_root_brent;
use roots::find_root_secant;

// Find the root of a complex function in the area determined by a simpler polynom
fn find_solution(enormous_function: &Fn(f64)->f64, root_area_polynom:(f64,f64,f64,f64)) -> Option<f64> {
  // de-structure polynom coefficients
  match root_area_polynom {
    (a3,a2,a1,a0) => {
      // Find root area by solving the polynom
      match find_roots_cubic(a3,a2,a1,a0) {
        // Try to find the root by one of iterative methods
        Roots::Three(roots) => {
          // Three roots found, normal case
          find_root_brent(roots[0],roots[2],enormous_function, &mut 1e-8f64).ok()
        },
        Roots::Two(roots) => {
          // Two roots found, High precision required
          find_root_brent(roots[0],roots[1],enormous_function,&mut 1e-15f64).ok()
        },
        Roots::One(roots) => {
          // One root found, Low precision is enough
          find_root_secant(roots[0]-1f64,roots[0]+1f64,enormous_function,&mut 1e-3f64).ok()
        },
        _ => None,
      }
    },
    _ => None,
  }
}
```
