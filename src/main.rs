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
const MAX_ANGLE: f32 = std::f32::consts::FRAC_PI_8/2.0;
const ARM_LENGTH: f32 = 1.0; // make sure to change the arm pos in sync_data if you change this
const ARM_WIDTH:f32 = 0.15; // make sure to change the arm pos in sync_data if you change this
const ANCHOR_POS: [f32; 2] = [0.0, 4.0];
const q_init:[f32;3] = [0.0,std::f32::consts::FRAC_PI_2, -0.2];
const q_goal:[f32;3] =  [0.0, 0.3, 1.0f32]; //[-0.5,0.9, -0.5f32];

fn segment_is_free(
    obstacle_polygons: &Vec<(Cuboid, Isometry2<f32>)>,
    sampled_pos: &[f32;3],
) -> bool {

    let arm  = Cuboid::new(Vector2::new(ARM_LENGTH*0.5, ARM_WIDTH*0.5));

    let joint_from_midlink = Isometry2::new(Vector2::new(0.5*ARM_LENGTH, 0.0), 0.0);

    //Arm is anchor at ANCHOR_POS
    let world_from_joint0 = Isometry2::new(Vector2::new(ANCHOR_POS[0], ANCHOR_POS[1]), sampled_pos[0]);
    let world_from_joint1 = world_from_joint0 * Isometry2::new(Vector2::new(ARM_LENGTH, 0.0), sampled_pos[1]);
    let world_from_joint2 = world_from_joint1 * Isometry2::new(Vector2::new(ARM_LENGTH, 0.0), sampled_pos[2]);

    let world_from_midlink0 =  world_from_joint0 * joint_from_midlink;
    let world_from_midlink1 = world_from_joint1* joint_from_midlink;
    let world_from_midlink2 = world_from_joint2* joint_from_midlink;

    let arm_segs = [world_from_midlink0,world_from_midlink1,world_from_midlink2];

    // println!("arm0_pos:\n{:}", world_from_midlink0.to_matrix());
    // println!("arm1_pos:\n{:}", world_from_midlink1.to_matrix());
    // println!("arm2_pos:\n{:}", world_from_midlink2.to_matrix());

    // assert!(false);

    for i in 0..3{
        let arm_pos = arm_segs[i];
        for (shape, position) in obstacle_polygons.iter() {
            let touching = contact(position, shape, &arm_pos, &arm, 0.0);
            // println!("position {:?} and shape {:?}", position, shape);
            // println!("arm pos {:?} and arm {:?}", arm_pos, arm);
            // println!("{:?}", touching);
            if touching.unwrap() != None {
                return false;
            }
        }
    }

    true
}

fn main() {
    
    let filepath = "src/bin/obstacles.txt";
    let obstacles_coords =
        tests::obstacle_parser_geo(filepath).expect("can't parse for obstacle for geo");
    // println!("obstacles {:?}", obstacles_coords);
    // let (x1, x2, g1, g2) = obstacles_coords[0].clone();
    // let q_init = Point::new(x1 as f32, x2 as f32);
    // let q_goal = Point::new(g1 as f32, g2 as f32);


    let mut obstacle_polygons: Vec<(Cuboid, Isometry2<f32>)> = Vec::new();

    // skip(1) because we already extracted the first line for start and goal cordinates
    for (x0, y0, x1, y1) in obstacles_coords.iter().skip(1).copied() {

        // Location takes into account that cuboid is created at the origin, hence the translation needs to be the mid point
        // of the cuboids.
        let half_x = (x1 as f32 - x0 as f32) / 2.0;
        let half_y = (y1 as f32 - y0 as f32) / 2.0;
        let shape = Cuboid::new(Vector2::new(half_x, half_y));
        let location = Isometry2::new(Vector2::new(x0 as f32 + half_x, y0 as f32 + half_y), 0.0);
        // println!("Box is {shape:?}  at {location:?}");
        obstacle_polygons.push((shape, location));
    }
    // println!("polygons: {:?}", obstacle_polygons);
    if segment_is_free(&obstacle_polygons, &q_goal){
        println!(
            "Goal should be true/ collision free: {}",
            segment_is_free(&obstacle_polygons, &q_goal)
        );
    }else{
        assert!(false)
    }

    if  segment_is_free(&obstacle_polygons, &q_init){
        println!(
            "init should be true/ collision free: {}",
            segment_is_free(&obstacle_polygons, &q_init)
        );
    }else{
        assert!(false)
    }


    // tree_points = np.zeros((MAX_ITERATIONS, 2))

    let mut tree_points = Vec::with_capacity(MAX_ITERATIONS);
    // tree_points.push(q_init);
    tree_points.push(q_init);
    let mut parents = Vec::with_capacity(MAX_ITERATIONS);
    parents.push(0);

    let mut goal_idx = None;

    let mut iter_count = 0.0;
    let mut valid_branch_count = 0.0;
    let mut smallest_abs_diff = 100.0;

    for _ in 0..MAX_ITERATIONS {
        let r: f32 = rand::random();
        let q_rand = if r <= 0.09 {
            // random point becomes the goal point so that the tree grows toward the the goal point
            // print!("q_goal {:?}", q_goal);
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

        //Check and make sure to grow a new branch on the tree according to the maximum max angle change
        let q_new: _ = if dist_square < MAX_ANGLE.powi(2) {
            q_rand
        } else {
            let normalize = MAX_ANGLE / dist_square.sqrt();
            // let addition: f32 = diff.iter().for_each(|b| *b = *b * normalize);
            [diff[0]*normalize + p_closest_to_rand[0],diff[1]*normalize + p_closest_to_rand[1], diff[2]*normalize + p_closest_to_rand[2]]

            // *p_closest_to_rand + diff * SEGMENT_LENGTH / dist_square.sqrt()
        };

        iter_count = iter_count+1.0;

        if segment_is_free(&obstacle_polygons, &q_new) {
            tree_points.push(q_new);
            parents.push(idx);
            valid_branch_count = 1.0+ valid_branch_count;

            let last_sampled_diff = [q_goal[0]- q_new[0],q_goal[1]-q_new[1],q_goal[2]- q_new[2]];
            let abs_diff_square = last_sampled_diff[0].powi(2) + last_sampled_diff[1].powi(2) + last_sampled_diff[2].powi(2);
            let abs_diff = abs_diff_square.sqrt();
            if abs_diff < smallest_abs_diff{
                smallest_abs_diff = abs_diff;
            }
            
            if q_new == q_goal  {
                goal_idx = Some(tree_points.len() - 1);
                println!("last sampled_q {:?} /n",q_new);
                println!("goal point {:?}", q_goal);
                println!("abs_diff_square {:?}", abs_diff);

                break;
            }
        }
    }
    let percentage = valid_branch_count/iter_count;
    println!("percentage: {}", percentage);
    println!("smallest abs diff = {:?}", smallest_abs_diff);

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
        graph_robot_arm(path, ANCHOR_POS,  ARM_LENGTH*25.0f32);
        println!("goal found");
    } else {
        println!("No goal");
    }
}
