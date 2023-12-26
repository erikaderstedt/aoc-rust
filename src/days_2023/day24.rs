// https://adventofcode.com/2023/day/24

use std::str::FromStr;
use itertools::Itertools;
use crate::common::{Solution, parsed_from_each_line};

#[derive(Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone)]
struct Hailstone {
    position: Vec3,
    velocity: Vec3,
}

impl Hailstone {
    fn intersect_path(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let m1 = self.velocity.y / self.velocity.x;
        let b1 = self.position.y - self.position.x * self.velocity.y / self.velocity.x;

        let m2 = other.velocity.y / other.velocity.x;
        let b2 = other.position.y - other.position.x * other.velocity.y / other.velocity.x;

        if m1 == m2 {
            None    // Parallel lines
        } else {
            let x = (b1 - b2) / (m2 - m1);
            let y = m1 * x + b1;
            // At what time?
            let t1 = (x - self.position.x)/self.velocity.x;
            let t2 = (x - other.position.x)/other.velocity.x;
            if t1 >= 0.0 && t2 >= 0.0 {
                Some((x,y))
            } else {
                None
            }
        }
    }
}

const P1_AREA_START: f64 = 200000000000000.0;
const P1_AREA_STOP: f64 = 400000000000000.0;

pub fn solve(input: &str) -> Solution {
    let stones: Vec<Hailstone> = parsed_from_each_line(input);

    let p1: usize = stones.iter()
        .enumerate()
        .map(|(index, stone)|
            stones.iter()
                .skip(index)
                .filter(|other|
                    match stone.intersect_path(other) {
                        Some((x,y)) if 
                            x >= P1_AREA_START && 
                            y >= P1_AREA_START &&
                            x <= P1_AREA_STOP &&
                            y <= P1_AREA_STOP => {
                                true
                                
                            },
                            _ => false,                        
                    })
                .count())
        .sum();

    // Part 2

    // (p1 - p0) = -t (v1 - v0) (where px and vx are position vectors)
    // since t is scalar, these vectors are parallel
    // This means that their cross product is zero.

    // (x1 - x0), (y1 -y0), (z1 - z0) x (vx1 - vx0, vy1 - vy0, vz1 - vz0) give three equations
    // (x2 - x0), (y2 -y0), (z2 - z0) x (vx2 - vx0, vy2 - vy0, vz2 - vz0) give three equations

    // (y1-y0)*(vz1-vz0) - (z1-z0)*(vy1-vy0) = 0  => -vy0*z0 + vy0*z1 + vy1*z0 - vy1*z1 + vz0*y0 - vz0*y1 - vz1*y0 + vz1*y1
    // (z1-z0)*(vx1-vx0) - (x1-x0)*(vz1-vz0) = 0  => -vz0*x0 + vz0*x1 + vz1*x0 - vz1*x1 + vx0*z0 - vx0*z1 - vx1*z0 + vx1*z1
    // (x1-x0)*(vy1-vy0) - (y1-y0)*(vx1-vx0) = 0  => -vx0*y0 + vx0*y1 + vx1*y0 - vx1*y1 + vy0*x0 - vy0*x1 - vy1*x0 + vy1*x1
    // (y2-y0)*(vz2-vz0) - (z2-z0)*(vy2-vy0) = 0  => -vy0*z0 + vy0*z2 + vy2*z0 - vy2*z2 + vz0*y0 - vz0*y2 - vz2*y0 + vz2*y2
    // (z2-z0)*(vx2-vx0) - (x2-x0)*(vz2-vz0) = 0  => -vz0*x0 + vz0*x2 + vz2*x0 - vz2*x2 + vx0*z0 - vx0*z2 - vx2*z0 + vx2*z2
    // (x2-x0)*(vy2-vy0) - (y2-y0)*(vx2-vx0) = 0  => -vx0*y0 + vx0*y2 + vx2*y0 - vx2*y2 + vy0*x0 - vy0*x2 - vy2*x0 + vy2*x2

    // vz0*y0 - vy0*z0:
    // vy0*z1 + vy1*z0 - vy1*z1 - vz0*y1 - vz1*y0 + vz1*y1 = vy0*z2 + vy2*z0 - vy2*z2 - vz0*y2 - vz2*y0 + vz2*y2
    // vx0*z0 - vz0*x0:
    // vz0*x1 + vz1*x0 - vz1*x1 - vx0*z1 - vx1*z0 + vx1*z1 = vz0*x2 + vz2*x0 - vz2*x2 - vx0*z2 - vx2*z0 + vx2*z2
    // vy0*x0 - vx0*y0:
    // vx0*y1 + vx1*y0 - vx1*y1 - vy0*x1 - vy1*x0 + vy1*x1 = vx0*y2 + vx2*y0 - vx2*y2 - vy0*x2 - vy2*x0 + vy2*x2

    // Unknowns
    // x0       y0          z0          vx0         vy0         vz0     
    // 0        -vz1+vz2    vy1-vy2     0           z1-z2       -y1+y2          -vy2*z2 + vz2*y2 +vy1*z1-vz1*y1
    // vz1-vz2  0           -vx1+vx2    -z1+z2      0           x1-x2           -vz2*x2 + vx2*z2 +vz1*x1-vx1*z1
    // -vy1+vy2 vx1-vx2     0           y1-y2       -x1+x2      0               -vx2*y2 + vy2*x2 +vx1*y1-vy1*x1
    // 0        -vz1+vz3    vy1-vy3     0           z1-z3       -y1+y3          -vy3*z3 + vz3*y3 +vy1*z1-vz1*y1
    // vz1-vz3  0           -vx1+vx3    -z1+z3      0           x1-x3           -vz3*x3 + vx3*z3 +vz1*x1-vx1*z1
    // -vy1+vy3 vx1-vx3     0           y1-y3       -x1+x3      0               -vx3*y3 + vy3*x3 +vx1*y1-vy1*x1

    // Translate coordinate system origin to x1,y1,z1 and do Gaussian elimination (after flipping the columns so that we end up with x y z):

    let x1 = stones[0].position.x;
    let y1 = stones[0].position.y;
    let z1 = stones[0].position.z;

    let x2 = stones[1].position.x - x1;
    let y2 = stones[1].position.y - y1;
    let z2 = stones[1].position.z - z1;
    let x3 = stones[2].position.x - x1;
    let y3 = stones[2].position.y - y1;
    let z3 = stones[2].position.z - z1;

    let vx1 = stones[0].velocity.x;
    let vx2 = stones[1].velocity.x;
    let vx3 = stones[2].velocity.x;
    let vy1 = stones[0].velocity.y;
    let vy2 = stones[1].velocity.y;
    let vy3 = stones[2].velocity.y;
    let vz1 = stones[0].velocity.z;
    let vz2 = stones[1].velocity.z;
    let vz3 = stones[2].velocity.z;

    let z = (-vx3*y3 + vy3*x3 - vz2*x2*y3/z2 + vx2*y3 - vy2*x3 + vz2*x3*y2/z2 + (x2*y3 - x3*y2)*(vz3*y3 -vy3*z3 + vy2*z3 - vz2*z3*y2/z2)/(z2*y3 - z3*y2)
    - (vx1-vx3 - (vz1-vz2)*x3/z2 + (x2*y3 - x3*y2)*(vz3-vz1 + z3*(vz1-vz2)/z2)/(z2*y3 - z3*y2) - ((vy3-vy1) + (vz1-vz2)*y3/z2)*((vx1-vx2)*z2 - (vz1-vz2)*x2)/((vy2-vy1)*z2+(vz1-vz2)*y2)) * 
    ((vx3*z3 - vz3*x3 + vz2*x2*z3/z2 - vx2*z3 + (x3*z2 - x2*z3)*(vz3*y3 - vy3*z3 + vy2*z3 - vz2*z3*y2/z2)/(z2*y3 - z3*y2))/
    ((z2*x3 - x2*z3)*(vz3-vz1 + z3*(vz1-vz2)/z2)/(z2*y3 - z3*y2) - (vz1-vz3 - (vz1-vz2)*z3/z2) * ((vx1-vx2)*z2 - (vz1-vz2)*x2)/((-vy1+vy2)*z2+(vz1-vz2)*y2)))) /
    ((-vx1+vx2)*y3/z2 + (vy1-vy2)*x3/z2 + (x2*y3 - x3*y2)*(vy1-vy3 + z3*(-vy1+vy2)/z2)/(z2*y3 - z3*y2)-((-vy1+vy3) + (vz1-vz2)*y3/z2)* (((-vx1+vx2)*y2 + (vy1-vy2)*x2))/((-vy1+vy2)*z2+(vz1-vz2)*y2)
    - (vx1-vx3 + (-vz1+vz2)*x3/z2 + (x2*y3 - x3*y2)*(-vz1+vz3 + z3*(vz1-vz2)/z2)/(z2*y3 - z3*y2) - ((-vy1+vy3) + (vz1-vz2)*y3/z2)*((vx1-vx2)*z2 - (vz1-vz2)*x2)/((-vy1+vy2)*z2+(vz1-vz2)*y2)) * 
    (((-vx1+vx3) + (vx1-vx2)*z3/z2 + (z2*x3 - x2*z3)*(vy1-vy3 + z3*(vy2-vy1)/z2)/(z2*y3 - z3*y2) - (vz1-vz3 - (vz1-vz2)*z3/z2) * (((-vx1+vx2)*y2 + (vy1-vy2)*x2))/((-vy1+vy2)*z2+(vz1-vz2)*y2))/
    ((z2*x3 - x2*z3)*(-vz1+vz3 + z3*(vz1-vz2)/z2)/(z2*y3 - z3*y2) - (vz1-vz3 - (vz1-vz2)*z3/z2) * ((vx1-vx2)*z2 - (vz1-vz2)*x2)/((-vy1+vy2)*z2+(vz1-vz2)*y2))));

    let y = (-vz3*x3 + vx3*z3 + vz2*x2*z3/z2 - vx2*z3 + (x3*z2 - x2*z3)*(-vy3*z3 + vz3*y3 + vy2*z3 - vz2*z3*y2/z2)/(z2*y3 - z3*y2))/((z2*x3 - x2*z3)*(-vz1+vz3 + z3*(vz1-vz2)/z2)/
    (z2*y3 - z3*y2) - (vz1-vz3 - (vz1-vz2)*z3/z2) * ((vx1-vx2)*z2 - (vz1-vz2)*x2)/((-vy1+vy2)*z2+(vz1-vz2)*y2)) - z * (((-vx1+vx3) + (vx1-vx2)*z3/z2 + (z2*x3 - x2*z3)*(vy1-vy3 + z3*(vy2-vy1)/z2)/
    (z2*y3 - z3*y2) - (vz1-vz3 - (vz1-vz2)*z3/z2) * (((-vx1+vx2)*y2 + (vy1-vy2)*x2))/((-vy1+vy2)*z2+(vz1-vz2)*y2))/((z2*x3 - x2*z3)*(-vz1+vz3 + z3*(vz1-vz2)/z2)/
    (z2*y3 - z3*y2) - (vz1-vz3 - (vz1-vz2)*z3/z2) * ((vx1-vx2)*z2 - (vz1-vz2)*x2)/((-vy1+vy2)*z2+(vz1-vz2)*y2)));

    let x = 0.0 - z*(((-vx1+vx2)*y2 + (vy1-vy2)*x2))/((-vy1+vy2)*z2+(vz1-vz2)*y2) - y * ((vx1-vx2)*z2 - (vz1-vz2)*x2)/((-vy1+vy2)*z2+(vz1-vz2)*y2);

    let p2 = (x + y + z + x1 + y1 + z1).round() as i64;

    Solution::new(p1, p2)
}

impl FromStr for Vec3 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x,y,z) = s.split(", ").map(|x| x.parse::<f64>().unwrap()).collect_tuple().unwrap();
        Ok( Vec3 { x, y, z })
    }
}

impl FromStr for Hailstone {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((p, v)) = s.split_once(" @ ") {

            let position = p.parse::<Vec3>()?;
            let velocity = v.parse::<Vec3>()?;
            
            Ok( Hailstone { position, velocity })
        } else {
            Err("Malformed record")
        }
    }
}