use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use serde::{Serialize, Deserialize};
use glium::implement_vertex;

#[derive(Serialize, Deserialize)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Obstacle {
    pub bottom_left : Point2,
    pub top_right: Point2,
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Vertex {
    position: (f32, f32, f32),
}

implement_vertex!(Vertex, position);

pub fn obstacle_parser_find_connecting_indices(length: usize) -> Result<Vec<u16>, Box<dyn std::error::Error>>{
    let mut connecting_indices = Vec::with_capacity(length);
    let mut i = 0;
    for _each_obstacle in 0..length{
        connecting_indices.push(i as u16);
        connecting_indices.push(i+1);
        connecting_indices.push(i+2);

        connecting_indices.push(i);
        connecting_indices.push(i+3);
        connecting_indices.push(i+1);
 
        i = i + 4;


    }
    println!("obstacles {:?}", connecting_indices);
    Ok(connecting_indices)
}

pub fn obstacle_parser_glium(filepath: &str) ->  Result<Vec<Vertex>, Box<dyn std::error::Error>> {
    let mut lines = BufReader::new(File::open(filepath)?).lines();
    _ = lines.next().expect("Fail to skip the start and goal position")?;
    let obstacle_len: usize = lines.next().expect("file is empty, no obstacle")?.parse().expect("Its not a number");
    let mut obstacles = Vec::with_capacity(obstacle_len);
    for _ in 0..obstacle_len {
        let line = lines.next().expect("Declare more obstacle than there are")?;
        let split: Vec<f32>= line.split_whitespace().enumerate().map(|(_, s)| s.parse::<f32>().unwrap()).collect();
        
        // screen axis goes from bottom left to top right
        // index 0th and 1st represent bottom left, 2nd and 3rd represent top right
        // indecies should go as 0->1->2, 0->3->1, then for the next ones: 4->5->6, 4->7->5
        obstacles.push(Vertex {
            position:(split[0]*25.0 -100.0, split[1]*25.0 -100.0, 0.0) //bottom left
        });
        obstacles.push(Vertex {
            position:(split[2]*25.0 -100.0, split[3]*25.0 -100.0, 0.0) // top right
        });
        obstacles.push(Vertex {
            position:(split[2]*25.0 -100.0, split[1]*25.0 -100.0, 0.0) // bottom right
        });
        obstacles.push(Vertex {
            position:(split[0]*25.0 -100.0, split[3]*25.0 -100.0, 0.0) //top left
        });
        


    }
    // println!("obstacles {:?}", obstacles);
    Ok(obstacles)
}



