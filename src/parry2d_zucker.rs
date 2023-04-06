extern crate nalgebra as na;

use na::{Isometry2, Point2, Vector2};
use parry2d::query::contact::contact;
use parry2d::shape::{Ball, Cuboid, Segment};
// use parry2d::math::Point;

#[test]
fn parry_tst() -> Result<(), Box<dyn std::error::Error>> {
    // 4 x 2 box centered at origin
    let mybox = Cuboid::new(Vector2::new(3.0, 1.0));

    // 0.1 radius ball at origin
    let myball = Ball::new(1.0);

    let line = Segment::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0));

    // let's put our box rotated 90 degrees and centered at 2, 0
    let T_world_from_box = Isometry2::new(Vector2::new(2.0, 0.0), std::f32::consts::FRAC_PI_2); //0.0);

    // leave the ball at the origin
    let T_world_from_ball = Isometry2::new(Vector2::new(0.0, 0.0), 0.0);

    // relative transformation of ball from box
    let T_ball_from_box = T_world_from_ball.inverse() * T_world_from_box;

    // see if these are closer than 0 units to each other
    let foo = contact(&T_world_from_ball, &myball, &T_world_from_ball, &line, 0.0);

    println!("Querying collision");
    println!("Ball is {myball:?} at {T_world_from_ball:?}");
    println!("Box is {mybox:?} at {T_world_from_box:?}");
    println!("Result is {foo:?}");

    Ok(())
}
