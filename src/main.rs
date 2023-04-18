use geo::{Contains, Intersects, LineString, Point, Polygon};
// use plotters::{element::BackendCoordOnly, prelude::*};
mod graph3d;
use graph3d::Point2;
// use tests::Obstacle;
mod parry2d_zucker;
mod tests;
mod robot_arm;
// use nalgebra::UnitComplex;
use parry2d::na::{Isometry2, Vector2};
use parry2d::query::contact::{contact, contact_support_map_support_map};
use parry2d::shape::{Ball, Cuboid, Segment};
use robot_arm::graph_robot_arm;

extern crate nalgebra as na;

const MAX_ITERATIONS: usize = 10000;
const SEGMENT_LENGTH: f32 = 0.1;
const POINT_RADIUS: f32 = 0.1;
const MAX_ANGLE: f32 = std::f32::consts::FRAC_PI_8;
const ARM_LENGTH: f32 = 1.0;
const ARM_WIDTH:f32 = 0.15;
const ANCHOR_POS: [f32; 2] = [7.9, 4.0];
// fn point_is_free(obstacle_polygons: &Vec<Polygon<f64>>, point_coords: &Point<f64>) -> bool {
//     let point = Point::new(point_coords.x(), point_coords.y());

//     for p in obstacle_polygons {
//         if p.contains(&point) {
//             return false;
//         }
//     }

//     true
// }


// We want to give a start pose and an goal pos. We then sample diffrent thetas so that we can get to the right orientation 
// where the final pose touches the goal point.

fn point_is_free(
    obstacle_polygons: &Vec<(Cuboid, Isometry2<f32>)>,
    point_coords: &Point<f32>,
) -> bool {
    let point = Ball::new(POINT_RADIUS);
    let point_pos = Isometry2::new(Vector2::new(point_coords.x(), point_coords.y()), 0.0);

    for (shape, position) in obstacle_polygons.iter() {
        let touching = contact(position, shape, &point_pos, &point, 0.0);
        if touching.unwrap() != None {
            return false;
        }
    }

    true
}

// fn segment_is_free(
//     obstacle_polygons: &Vec<Polygon<f64>>,
//     start_coords: &Point<f64>,
//     end_coords: &Point<f64>,
// ) -> bool {
//     let lstr = LineString::from(vec![*start_coords, *end_coords]);

//     for p in obstacle_polygons {
//         if p.intersects(&lstr) {
//             return false;
//         }
//     }

//     true
// }
fn segment_is_free(
    obstacle_polygons: &Vec<(Cuboid, Isometry2<f32>)>,
    start_pos: &[f32;3],
    sampled_pos: &[f32;3],
) -> bool {

    // // let point = Ball::new(POINT_RADIUS);
    // let line = Segment::new(
    //     na::Point2::new(point_from.x(), point_from.y()),
    //     na::Point2::new(point_to.x(), point_to.y()),
    // );

    let arm  = Cuboid::new(Vector2::new(ARM_WIDTH, ARM_LENGTH));
    // for i in 0..3{
    //     let absolute_zeros = Isometry2::new(Vector2::new(0.0, 0.0), point_to[i]);
    // }

    //Arm is anchor at 0, 0
    let arm0_pos = Isometry2::new(Vector2::new(ANCHOR_POS[0], ANCHOR_POS[1]), start_pos[0]);
    let arm1_pos = arm0_pos * Isometry2::new(Vector2::new(0.0, 0.0), start_pos[1]);
    let arm2_pos = arm1_pos * Isometry2::new(Vector2::new(0.0, 0.0), start_pos[2]);
    let arm_segs = [arm0_pos, arm1_pos, arm2_pos];


    for i in 0..3{
        let arm_pos = arm_segs[i];
        for (shape, position) in obstacle_polygons.iter() {
            let touching = contact(position, shape, &arm_pos, &arm, 0.0);
            if touching.unwrap() != None {
                return false;
            }
        }
    }

    true
}

fn main() {
    
    let filepath = "src/bin/obstacles.txt";
    // robot_arm_test().expect("blah");

    let obstacles_coords =
        tests::obstacle_parser_geo(filepath).expect("can't parse for obstacle for geo");
    println!("obstacles {:?}", obstacles_coords);

    // let whatever = parry2d_zucker::parry_tst()?;

    // let rect_coords = vec![
    //     (2.0, 2.0, 6.0, 6.0),
    //     (3.0, 1.0, 4.0, 2.0),
    //     (1.0, 1.0, 2.0, 2.0),
    // ]; // These points goes x: left to right 0...8, y: bottom to top 0...8

    // let q_init = Point::new(1.0, 7.0);
    // let q_goal = Point::new(7.0, 1.0);

    let (x1, x2, g1, g2) = obstacles_coords[0].clone();

    // let q_init = Point::new(x1 as f32, x2 as f32);
    // let q_goal = Point::new(g1 as f32, g2 as f32);
    let q_init = [-1.7, 0.0, 1.0f32];
    let q_goal = [1.2,0.2,0.3f32];

    let mut obstacle_polygons: Vec<(Cuboid, Isometry2<f32>)> = Vec::new();

    // skip(1) because we already extracted the first line for start and goal cordinates
    for (x0, y0, x1, y1) in obstacles_coords.iter().skip(1).copied() {
        // let points = vec![
        //     Point::new(x0, y0),
        //     Point::new(x1, y0),
        //     Point::new(x1, y1),
        //     Point::new(x0, y1),
        // ];
        // let polygon = Polygon::new(LineString::from(points), vec![]);

        // Location takes into account that cuboid is created at the origin, hence the translation needs to be the mid point
        // of the cuboids.
        let half_x = (x1 as f32 - x0 as f32) / 2.0;
        let half_y = (y1 as f32 - y0 as f32) / 2.0;
        let shape = Cuboid::new(Vector2::new(half_x, half_y));
        let location = Isometry2::new(Vector2::new(x0 as f32 + half_x, y0 as f32 + half_y), 0.0);
        // println!("Box is {shape:?}  at {location:?}");
        obstacle_polygons.push((shape, location));
    }

    println!("polygons: {:?}", obstacle_polygons);
    println!(
        "should be ?: {}",
        segment_is_free(&obstacle_polygons, &q_init, &q_goal)
    );

    // tree_points = np.zeros((MAX_ITERATIONS, 2))

    let mut tree_points = Vec::with_capacity(MAX_ITERATIONS);
    // tree_points.push(q_init);
    tree_points.push(q_init);
    let mut parents = Vec::with_capacity(MAX_ITERATIONS);
    parents.push(0);

    let mut goal_idx = None;

    for _ in 0..MAX_ITERATIONS {
        // let mut q_rand: Point<f64>;

        let r: f32 = rand::random();
        // let q_rand = [rand::random::<f32>()*std::f32::consts::PI,rand::random::<f32>()*std::f32::consts::PI,rand::random::<f32>()*std::f32::consts::PI];


        let q_rand = if r <= 0.1 {
            // random point becomes the goal point so that the tree grows toward the the goal point
            q_goal
        } else {
            // random point is randon on the plane
            [rand::random::<f32>()*std::f32::consts::PI,rand::random::<f32>()*std::f32::consts::PI,rand::random::<f32>()*std::f32::consts::PI]
        };

        let (idx, (p_closest_to_rand, diff, dist_square)) = tree_points
            .iter()
            .map(|p| {
                //map this point, the diff_vector, and the length of the diff vector
                let diff = [q_rand[0]- p[0],q_rand[1]- p[1],q_rand[2]- p[2]];
                let dist_square = diff[0].powi(2) + diff[1].powi(2) + diff[2].powi(2);
                (p, diff, dist_square)
            })
            .enumerate()
            //min_by compares the 2 elements splitted out by enumerate exp: (idx1,dist1), (idx2,dist2). dist1.partial compare gives you the signs/ordering
            .min_by(|(_, a), (_, b)| a.2.partial_cmp(&b.2).expect("can't compare distances"))
            .expect("no starting points in tree points");

        //Check and make sure to grow a new branch on the tree according to the maximum segment length
        let q_new: _ = if dist_square < SEGMENT_LENGTH.powi(2) {
            q_rand
        } else {
            let normalize = MAX_ANGLE / dist_square.sqrt();
            // let addition: f32 = diff.iter().for_each(|b| *b = *b * normalize);
            [diff[0]*normalize + p_closest_to_rand[0],diff[1]*normalize + p_closest_to_rand[1], diff[2]*normalize + p_closest_to_rand[2]]

            // *p_closest_to_rand + diff * SEGMENT_LENGTH / dist_square.sqrt()
        };

        if segment_is_free(&obstacle_polygons, p_closest_to_rand, &q_new) {
            tree_points.push(q_new);
            parents.push(idx);
            if q_new == q_goal {
                goal_idx = Some(tree_points.len() - 1);
                break;
            }
        }
    }

    // println!("tree points {:?}", tree_points);

    // Retrieve the path from the goal point to start point. Append them to a path array: Goal...start
    if let Some(mut current_idx) = goal_idx {
        let mut path = vec![];
        while current_idx != 0 {
            let p = tree_points[current_idx];
            path.push(p);
            current_idx = parents[current_idx];
        }
        let p = tree_points[current_idx];

        print!("init point {:?}", q_init);
        print!("last point added that is supposed to be init point {:?}", p);
        print!("current_idx {:?}", current_idx);
        path.push(p);

        // 3D graphing with glium
        graph_robot_arm(path);
        println!("goal found");
    } else {
        println!("No goal");
    }
}
