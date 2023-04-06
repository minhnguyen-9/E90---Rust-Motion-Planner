use geo::{Contains, Intersects, LineString, Point, Polygon};
// use plotters::{element::BackendCoordOnly, prelude::*};
mod graph3d;
use graph3d::Point2;
// use tests::Obstacle;
mod parry2d_zucker;
mod tests;
use nalgebra::UnitComplex;
use parry2d::na::{Isometry2, Vector2};
use parry2d::query::contact::{contact, contact_support_map_support_map};
use parry2d::shape::{Ball, Cuboid, Segment};

extern crate nalgebra as na;

const MAX_ITERATIONS: usize = 10000;
const SEGMENT_LENGTH: f32 = 0.1;
const POINT_RADIUS: f32 = 0.1;

// type Backend<'a> = SVGBackend<'a>;

// fn point_is_free(obstacle_polygons: &Vec<Polygon<f64>>, point_coords: &Point<f64>) -> bool {
//     let point = Point::new(point_coords.x(), point_coords.y());

//     for p in obstacle_polygons {
//         if p.contains(&point) {
//             return false;
//         }
//     }

//     true
// }

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
    point_from: &Point<f32>,
    point_to: &Point<f32>,
) -> bool {
    // let point = Ball::new(POINT_RADIUS);
    let line = Segment::new(
        na::Point2::new(point_from.x(), point_from.y()),
        na::Point2::new(point_to.x(), point_to.y()),
    );
    // let point_pos = Isometry2::new(Vector2::new(point_coords.x(), point_coords.y()), 0.0);
    let absolute_zeros = Isometry2::new(Vector2::new(0.0, 0.0), 0.0);

    for (shape, position) in obstacle_polygons.iter() {
        let touching = contact(position, shape, &absolute_zeros, &line, 0.0);
        if touching.unwrap() != None {
            return false;
        }
    }

    true
}

fn main() {
    let filepath = "src/bin/obstacles.txt";
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
    let q_init = Point::new(x1 as f32, x2 as f32);
    let q_goal = Point::new(g1 as f32, g2 as f32);

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
        let half_x = (x1 as f32 - x2 as f32) / 2.0;
        let half_y = (y1 as f32 - y0 as f32) / 2.0;
        let shape = Cuboid::new(Vector2::new(half_x, half_y));
        let location = Isometry2::new(Vector2::new(x0 as f32 + half_x, y0 as f32 + half_y), 0.0);

        obstacle_polygons.push((shape, location));
    }

    println!("polygons: {:?}", obstacle_polygons);

    println!(
        "should be True: {}",
        point_is_free(&obstacle_polygons, &q_init)
    );
    println!(
        "should be True: {}",
        point_is_free(&obstacle_polygons, &q_goal)
    );
    println!(
        "should be False: {}",
        segment_is_free(&obstacle_polygons, &q_init, &q_goal)
    );

    // tree_points = np.zeros((MAX_ITERATIONS, 2))

    let mut tree_points = Vec::with_capacity(MAX_ITERATIONS);
    tree_points.push(q_init);
    let mut parents = Vec::with_capacity(MAX_ITERATIONS);
    parents.push(0);

    let mut goal_idx = None;

    for _ in 0..MAX_ITERATIONS {
        // let mut q_rand: Point<f64>;

        let r: f64 = rand::random();

        let q_rand = if r <= 0.01 {
            // random point becomes the goal point so that the tree grows toward the the goal point
            q_goal
        } else {
            // random point is randon on the plane
            Point::new(rand::random::<f32>() * 8.0, rand::random::<f32>() * 8.0)
        };

        let (idx, (p_closest_to_rand, diff, dist_square)) = tree_points
            .iter()
            .map(|p| {
                //map this point, the diff_vector, and the length of the diff vector
                let diff = q_rand - *p;
                let dist_square = diff.dot(diff);
                (p, diff, dist_square)
            })
            .enumerate()
            //min_by compares the 2 elements splitted out by enumerate exp: (idx1,dist1), (idx2,dist2). dist1.partial compare gives you the signs/ordering
            .min_by(|(_, a), (_, b)| a.2.partial_cmp(&b.2).expect("can't compare distances"))
            .expect("no starting points in tree points");

        //Check and make sure to grow a new branch on the tree according to the maximum segment length
        let q_new: Point<f32> = if dist_square < SEGMENT_LENGTH.powi(2) {
            q_rand
        } else {
            *p_closest_to_rand + diff * SEGMENT_LENGTH / dist_square.sqrt()
            // active_points[idx] + &diff[idx] * SEGMENT_LENGTH / dist[idx];
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

    // Retrieve the path from the goal point to start point. Append them to a path array: Goal...start
    if let Some(mut current_idx) = goal_idx {
        let mut path = vec![];
        while current_idx != 0 {
            let p = tree_points[current_idx];
            path.push(Point2 {
                x: p.x() as _,
                y: p.y() as _,
            });
            current_idx = parents[current_idx];
        }
        let p = tree_points[current_idx];
        path.push(Point2 {
            x: p.x() as _,
            y: p.y() as _,
        });

        // 3D graphing with glium
        graph3d::graph3d(path);

        // println!("polygons: {:?}", path);
    } else {
        println!("No goal");
    }
}
