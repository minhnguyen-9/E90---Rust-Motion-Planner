use geo::{Contains, Intersects, LineString, Point, Polygon};
// use plotters::{element::BackendCoordOnly, prelude::*};
mod graph3d;
use graph3d::Point2;

const MAX_ITERATIONS: usize = 10000;
const SEGMENT_LENGTH: f64 = 0.1;

// type Backend<'a> = SVGBackend<'a>;

fn point_is_free(obstacle_polygons: &Vec<Polygon<f64>>, point_coords: &Point<f64>) -> bool {
    let point = Point::new(point_coords.x(), point_coords.y());

    for p in obstacle_polygons {
        if p.contains(&point) {
            return false;
        }
    }

    true
}

fn segment_is_free(
    obstacle_polygons: &Vec<Polygon<f64>>,
    start_coords: &Point<f64>,
    end_coords: &Point<f64>,
) -> bool {
    let lstr = LineString::from(vec![*start_coords, *end_coords]);

    for p in obstacle_polygons {
        if p.intersects(&lstr) {
            return false;
        }
    }

    true
}

fn main() {
    let rect_coords = vec![
        (2.0, 2.0, 6.0, 6.0),
        (3.0, 1.0, 4.0, 2.0),
        (1.0, 1.0, 2.0, 2.0),
    ]; // These points goes x: left to right 0...8, y: bottom to top 0...8

    let q_init = Point::new(1.0, 7.0);
    let q_goal = Point::new(7.0, 1.0);

    let mut obstacle_polygons: Vec<Polygon<f64>> = Vec::new();

    for (x0, y0, x1, y1) in rect_coords.iter().copied() {
        let points = vec![
            Point::new(x0, y0),
            Point::new(x1, y0),
            Point::new(x1, y1),
            Point::new(x0, y1),
        ];

        let polygon = Polygon::new(LineString::from(points), vec![]);
        obstacle_polygons.push(polygon);
    }

    println!("polygons: {:?}", obstacle_polygons[0]);

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
            q_goal
        } else {
            Point::new(rand::random::<f64>() * 8.0, rand::random::<f64>() * 8.0)
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
            //min_by compares the 2 elements splitted out by enumerate exp: (idx1,dist1), (idx2,dist2). d1.partial compare gives you the signs/ordering
            .min_by(|(_, a), (_, b)| a.2.partial_cmp(&b.2).expect("can't compare distances"))
            .expect("no starting points in tree points");

        let q_new: Point<f64> = if dist_square < SEGMENT_LENGTH.powi(2) {
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

    if let Some(mut current_idx) = goal_idx {
        let mut path = vec![];
        while current_idx != 0 {
            let p = tree_points[current_idx];
            path.push(Point2 { x: p.x() as _, y: p.y() as _ });
            current_idx = parents[current_idx];
        }
        let p = tree_points[current_idx];
        path.push(Point2 { x: p.x() as _, y: p.y() as _ });

        // 3D graphing with glium
        graph3d::graph3d(path);
        
        // println!("polygons: {:?}", path);
    } else {
        println!("No goal");
    }


    //2D plotting with plotters

    // let root = Backend::new("image2.svg", (1024, 1024)).into_drawing_area();

    // root.fill(&WHITE).unwrap();

    // let mut chart_context = ChartBuilder::on(&root);
    // chart_context.caption("Example Plot", ("sans-serif", 30));
    // let mut chart = chart_context
    //     .build_cartesian_2d(0.0..8.0f64, 0.0..8.0f64)
    //     .unwrap();
    // // chart.configure_mesh().draw().unwrap();

    // let obstacle_style = ShapeStyle {
    //     color: BLUE.mix(0.6),
    //     filled: true,
    //     stroke_width: 2,
    // };

    // // Draw obstacle patches
    // let patches: Vec<Rectangle<(f64, f64)>> = rect_coords
    //     .iter()
    //     .map(|(x_left, y_low, x_right, y_high)| {
    //         Rectangle::new([(*x_left, *y_high), (*x_right, *y_low)], obstacle_style)
    //     })
    //     .collect();

    // chart
    //     .draw_series::<BackendCoordOnly, Rectangle<(f64, f64)>, _, _>(patches.iter())
    //     .unwrap();

    // chart
    //     .draw_series(
    //         tree_points
    //             .iter()
    //             .map(|p| Circle::new((p.x(), p.y()), 3, &BLACK)),
    //     )
    //     .unwrap();

    // for (point_idx, parent_idx) in parents.iter().enumerate() {
    //     let x0 = tree_points[*parent_idx].x();
    //     let y0 = tree_points[*parent_idx].y();
    //     let x1 = tree_points[point_idx].x();
    //     let y1 = tree_points[point_idx].y();
    //     chart
    //         .draw_series(LineSeries::new([(x0, y0), (x1, y1)], &YELLOW))
    //         .unwrap();
    // }

    // if let Some(mut current_idx) = goal_idx {
    //     let mut path = vec![];
    //     while current_idx != 0 {
    //         let p = tree_points[current_idx];
    //         path.push((p.x(), p.y()));
    //         current_idx = parents[current_idx];
    //     }
    //     let p = tree_points[current_idx];
    //     path.push((p.x(), p.y()));

    //     let example = LineSeries::new(path.iter().map(|p| (p.0, p.1)), &BLUE);
    //     chart.draw_series(example).unwrap();
    // }
    // chart.configure_mesh().draw().unwrap();
}
