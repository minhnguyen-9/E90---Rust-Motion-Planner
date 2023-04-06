use glium::implement_vertex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Obstacle {
    pub bottom_left: Point2,
    pub top_right: Point2,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Vertex {
    position: (f32, f32, f32),
}

implement_vertex!(Vertex, position);

pub fn obstacle_parser_geo(
    filepath: &str,
) -> Result<Vec<(f64, f64, f64, f64)>, Box<dyn std::error::Error>> {
    let mut lines = BufReader::new(File::open(filepath)?).lines();
    //get the start and goal point first
    let first_row: Vec<f64> = lines
        .next()
        .expect("Declare more obstacle than there are")?
        .split_whitespace()
        .enumerate()
        .map(|(_, s)| s.parse::<f64>().unwrap())
        .collect();
    let start_n_goal = (first_row[0], first_row[1], first_row[2], first_row[3]);
    // get the number of obstacles
    let obstacle_len: usize = lines
        .next()
        .expect("file is empty, no obstacle")?
        .parse()
        .expect("Its not a number");
    let mut obstacles = Vec::with_capacity(obstacle_len);
    obstacles.push(start_n_goal); // push the start and goal point
    for _ in 0..obstacle_len {
        let line = lines
            .next()
            .expect("Declare more obstacle than there are")?;
        let split: Vec<f64> = line
            .split_whitespace()
            .enumerate()
            .map(|(_, s)| s.parse::<f64>().unwrap())
            .collect();

        // screen axis goes from bottom left to top right
        // index 0th and 1st represent bottom left, 2nd and 3rd represent top right
        // indecies should go as 0->1->2, 0->3->2, then for the next ones: 4->5->6, 4->7->6
        obstacles.push((split[0], split[1], split[2], split[3]));
    }
    // println!("obstacles {:?}", obstacles);
    Ok(obstacles)
}

pub fn obstacle_parser_glium(filepath: &str) -> Result<Vec<Vertex>, Box<dyn std::error::Error>> {
    let mut lines = BufReader::new(File::open(filepath)?).lines();
    _ = lines
        .next()
        .expect("Fail to skip the start and goal position")?;
    let obstacle_len: usize = lines
        .next()
        .expect("file is empty, no obstacle")?
        .parse()
        .expect("Its not a number");
    let mut obstacles = Vec::with_capacity(obstacle_len);
    for _ in 0..obstacle_len {
        let line = lines
            .next()
            .expect("Declare more obstacle than there are")?;
        let split: Vec<f32> = line
            .split_whitespace()
            .enumerate()
            .map(|(_, s)| s.parse::<f32>().unwrap())
            .collect();

        // screen axis goes from bottom left to top right
        // index 0th and 1st represent bottom left, 2nd and 3rd represent top right
        // indecies should go as 0->1->2, 0->3->2, then for the next ones: 4->5->6, 4->7->6
        obstacles.push(Vertex {
            position: (split[0], split[1], 0.0), //bottom left
        });
        obstacles.push(Vertex {
            position: (split[2], split[3], 0.0), // top right
        });
        obstacles.push(Vertex {
            position: (split[2], split[1], 0.0), // bottom right
        });
        obstacles.push(Vertex {
            position: (split[0], split[3], 0.0), //top left
        });
    }
    println!("obstacles {:?}", obstacles);
    Ok(obstacles)
}

// For writing to and reading fro json files.

#[test]
fn test_write_json() -> Result<(), Box<dyn std::error::Error>> {
    let obstacles = vec![
        Obstacle {
            bottom_left: Point2 { x: 2.0, y: 2.0 },
            top_right: Point2 { x: 4.0, y: 4.0 },
        },
        Obstacle {
            bottom_left: Point2 { x: 5.0, y: 5.0 },
            top_right: Point2 { x: 7.0, y: 7.0 },
        },
    ];

    let file = File::options()
        .create(true)
        .write(true)
        .open("src/bin/obstacle.json")?;
    serde_json::to_writer_pretty(file, &obstacles)?;
    Ok(())
}

#[test]
fn test_main() -> Result<(), Box<dyn std::error::Error>> {
    let file = BufReader::new(File::open("src/bin/obstacle.txt")?);
    let obstacles: Vec<Obstacle> = serde_json::from_reader(file)?;

    Ok(())
}
