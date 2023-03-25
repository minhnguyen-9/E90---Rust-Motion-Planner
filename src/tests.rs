use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
// use serde::{Serialize, Deserialize};
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



#[test]
fn test_parse_txt() ->  Result<(), Box<dyn std::error::Error>> {
    let mut lines = BufReader::new(File::open("src/bin/obstacle.txt")?).lines();
    let obstacle_len: usize = lines.next().expect("file is empty, no obstacle")?.parse().expect("Its not a number");
    let mut obstacles = Vec::with_capacity(obstacle_len);
    for _ in 0..obstacle_len {
        let line = lines.next().expect("Declare more obstacle than there are")?;
        let split: Vec<f32>= line.split_whitespace().enumerate().map(|(_, s)| s.parse::<f32>().unwrap()).collect();
        
        // screen axis goes from bottom left to top right
        // 0th and 1st represent bottom left, 2nd and 3rd represent top right
        obstacles.push(Vertex {
            position:(split[0], split[1], 0.0) //bottom left
        });
        obstacles.push(Vertex {
            position:(split[2], split[3], 0.0) // top right
        });
        // obstacles.push(Vertex {
        //     position:(split[0], split[3], 0.0) //top left
        // });
        // obstacles.push(Vertex {
        //     position:(split[2], split[1], 0.0) // bottom right
        // });


    }
    println!("obstacles {:?}", obstacles);
    Ok(())
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
        }
    ];

    let file = File::options().create(true).write(true).open("src/bin/obstacle.json")?;
    serde_json::to_writer_pretty(file, &obstacles)?;
    Ok(())
}

#[test]
fn test_main() -> Result<(), Box<dyn std::error::Error>> {
    let file = BufReader::new(File::open("src/bin/obstacle.txt")?);
    let obstacles: Vec<Obstacle> = serde_json::from_reader(file)?;

    Ok(())

}