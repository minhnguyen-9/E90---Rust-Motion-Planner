mod graph3d;
mod robot_arm;
mod tests;
use parry2d::na::{Isometry2, Vector2};
use parry2d::query::contact::contact;
use parry2d::shape::Cuboid;
use robot_arm::graph_robot_arm;

const MAX_ITERATIONS: usize = 10000 + 10000;
const MAX_ANGLE: f32 = std::f32::consts::FRAC_PI_2 / 10.0;
const ARM_LENGTH: f32 = 1.0; // make sure to change the arm pos in sync_data if you change this
const ARM_WIDTH: f32 = 0.15; // make sure to change the arm pos in sync_data if you change this
const ANCHOR_POS: [f32; 2] = [0.0, 4.0];
const Q_INIT: [f32; 3] = [0.0, std::f32::consts::FRAC_PI_2, -0.2];
const Q_GOAL: [f32; 3] = [0.5, -0.9, -1.1f32];

// collision free pose that can NOT be transversed [0.5,-0.9, -1.1f32]; // collision-free pose that can be transversef[0.0, 0.3, 1.0f32]; // collided pose[-0.5,0.9, -0.5f32];

fn segment_is_free(
    obstacle_polygons: &Vec<(Cuboid, Isometry2<f32>)>,
    sampled_pos: &[f32; 3],
) -> bool {
    let arm = Cuboid::new(Vector2::new(ARM_LENGTH * 0.5, ARM_WIDTH * 0.5));

    //Arm is anchor at ANCHOR_POS, each segment is rotated by given theta1, theta2, theta3
    let world_from_joint0 =
        Isometry2::new(Vector2::new(ANCHOR_POS[0], ANCHOR_POS[1]), sampled_pos[0]);
    let world_from_joint1 =
        world_from_joint0 * Isometry2::new(Vector2::new(ARM_LENGTH, 0.0), sampled_pos[1]);
    let world_from_joint2 =
        world_from_joint1 * Isometry2::new(Vector2::new(ARM_LENGTH, 0.0), sampled_pos[2]);
    
    let joint_from_midlink = Isometry2::new(Vector2::new(0.5 * ARM_LENGTH, 0.0), 0.0);
    // transforming each segment by midlink because cuboid rendering starts at the center of the cuboid
    let world_from_midlink0 = world_from_joint0 * joint_from_midlink;
    let world_from_midlink1 = world_from_joint1 * joint_from_midlink;
    let world_from_midlink2 = world_from_joint2 * joint_from_midlink;

    let arm_segs = [
        world_from_midlink0,
        world_from_midlink1,
        world_from_midlink2,
    ];

    //iter through each arm piece and see if it touches any obstacle
    for i in 0..3 {
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
    let obstacles_coords =
        tests::obstacle_parser_geo(filepath).expect("can't parse for obstacle for geo");
    //Init a Vector to store obstacles 
    let mut obstacle_polygons: Vec<(Cuboid, Isometry2<f32>)> = Vec::new();

    // Skip(1) because we already extracted the first line for start and goal cordinates
    for (x0, y0, x1, y1) in obstacles_coords.iter().skip(1).copied() {
        // Location takes into account that cuboid is created at the origin, hence the translation needs to be the mid point
        // of the cuboids.
        let half_x = (x1 as f32 - x0 as f32) / 2.0;
        let half_y = (y1 as f32 - y0 as f32) / 2.0;
        let shape = Cuboid::new(Vector2::new(half_x, half_y));
        let location = Isometry2::new(Vector2::new(x0 as f32 + half_x, y0 as f32 + half_y), 0.0);
        obstacle_polygons.push((shape, location));
    }
    // Check if the goal point is collision-free
    if segment_is_free(&obstacle_polygons, &Q_GOAL) {
        println!(
            "Goal should be true/ collision free: {}",
            segment_is_free(&obstacle_polygons, &Q_GOAL)
        );
    } else {
        println! {"Goal pose is NOT collision free!"}
        std::process::exit(1);
    }

    if segment_is_free(&obstacle_polygons, &Q_INIT) {
        println!(
            "init should be true/ collision free: {}",
            segment_is_free(&obstacle_polygons, &Q_INIT)
        );
    } else {
        println! {"Init pose is NOT collision free!"}
        // assert!(false)
        std::process::exit(1);
    }

    let mut tree_points = Vec::with_capacity(MAX_ITERATIONS);
    // tree_points.push(Q_INIT);
    tree_points.push(Q_INIT);
    let mut parents = Vec::with_capacity(MAX_ITERATIONS);
    parents.push(0);

    let mut goal_idx = None;

    for _ in 0..MAX_ITERATIONS {
        let r: f32 = rand::random();
        let q_rand = if r <= 0.09 {
            // random point becomes the goal point so that the tree grows toward the the goal point
            // print!("Q_GOAL {:?}", Q_GOAL);
            Q_GOAL
        } else {
            // random point is randon on the plane

            let rand_range = |lo: f32, hi: f32| rand::random::<f32>() * (hi - lo) + lo;
            let rand_angle = || rand_range(-std::f32::consts::PI, std::f32::consts::PI);
            [rand_angle(), rand_angle(), rand_angle()]
        };

        let (idx, (p_closest_to_rand, diff, dist_square)) = tree_points
            .iter()
            .map(|p| {
                let diff = [q_rand[0] - p[0], q_rand[1] - p[1], q_rand[2] - p[2]];
                let dist_square = diff[0].powi(2) + diff[1].powi(2) + diff[2].powi(2);
                (p, diff, dist_square)
            })
            .enumerate()
            //min_by compares the 2 elements splitted out by enumerate exp: (idx1,dist1), (idx2,dist2). dist1.partial compare gives you the signs/ordering
            .min_by(|(_, a), (_, b)| a.2.partial_cmp(&b.2).expect("can't compare distances"))
            .expect("no starting points in tree points");

        //Check and make sure to grow a new branch on the tree according to the maximum max angle change
        let q_new = if dist_square < MAX_ANGLE.powi(2) {
            q_rand
        } else {
            // *p_closest_to_rand + diff * SEGMENT_LENGTH / dist_square.sqrt()

            let normalize = MAX_ANGLE / (dist_square.sqrt());
            [
                diff[0] * normalize + p_closest_to_rand[0],
                diff[1] * normalize + p_closest_to_rand[1],
                diff[2] * normalize + p_closest_to_rand[2],
            ]
        };

        if segment_is_free(&obstacle_polygons, &q_new) {
            tree_points.push(q_new);
            parents.push(idx);

            if q_new == Q_GOAL {
                goal_idx = Some(tree_points.len() - 1);
                println!("last sampled_q {:?} ", q_new);
                println!("goal point {:?}", Q_GOAL);
                break;
            }
        }
    }

    // Retrieve the path from the goal point to start point. Append them to a path array: Goal...start
    if let Some(mut current_idx) = goal_idx {
        let mut path = vec![];
        while current_idx != 0 {
            let p = tree_points[current_idx];
            path.push(p);
            current_idx = parents[current_idx];
        }
        let p = tree_points[current_idx];

        println!("init point {:?}", Q_INIT);
        println!("last point added that is supposed to be init point {:?}", p);
        path.push(p);

        // visualization graphing with glium
        graph_robot_arm(path, ANCHOR_POS, ARM_LENGTH * 25.0f32, 1.0);
        println!("goal found");
    } else {
        println!("No goal");
        graph_robot_arm(tree_points, ANCHOR_POS, ARM_LENGTH * 25.0f32, 2.0);
    }
}
