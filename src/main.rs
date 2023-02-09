use geo::{Contains, EuclideanDistance, Intersects, LineString, Point, Polygon, Line};
use plotters::prelude::*;

// use geo::prelude::*;

// use plotters::prelude::*;

// fn main() {  # Plotters example
//     let root_drawing_area = BitMapBackend::new("images/0.1.png", (1024, 768))
//         .into_drawing_area();

//     root_drawing_area.fill(&WHITE).unwrap();

//     let mut chart = ChartBuilder::on(&root_drawing_area)
//         .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
//         .unwrap();

//     chart.draw_series(LineSeries::new(
//         (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
//         &RED
//     )).unwrap();
// }

const MAX_ITERATIONS: usize = 1000;
const SEGMENT_LENGTH: f64 = 0.1;

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
    let lstr = LineString::from(vec![start_coords.clone(), end_coords.clone()]);

    for p in obstacle_polygons {
        if p.intersects(&lstr) {
            return false;
        }
    }

    true
}

fn main() {
    let rect_coords = vec![(2.0, 2.0, 6.0, 6.0)];

    let q_init = Point::new(1.0, 7.0);
    let q_goal = Point::new(7.0, 1.0);

    let mut obstacle_polygons: Vec<Polygon<f64>> = Vec::new();

    for (x0, y0, x1, y1) in rect_coords {
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

    let mut goal_idx  = None;

    for attempt in 0..MAX_ITERATIONS {
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

        if segment_is_free(&obstacle_polygons, &p_closest_to_rand, &q_new) {
            tree_points.push(q_new);
            parents.push(idx);
            if q_new == q_goal {
                goal_idx = Some(tree_points.len() - 1);
                break;
            }
        }
    }

    // let rects: Vec<Rectangle<f64>> = rect_coords
    //     .iter()
    //     .map(|(x0, y0, x1, y1)| Rectangle::new((*x0, *y0), *x1 - *x0, *y1 - *y0))
    //     .collect();
    // let active_points = &tree_points[..tree_count];

    let root = BitMapBackend::new("image.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart_context = ChartBuilder::on(&root);
    chart_context.caption("Example Plot", ("sans-serif", 30));
    let chart = chart_context.build_cartesian_2d(0..8i32, 0..8i32).unwrap();
    chart.configure_mesh().draw().unwrap();


    let obstacle_style = ShapeStyle {
        color: BLUE.mix(0.6),
        filled: true,
        stroke_width: 2,
    };

    let patches: Vec<Rectangle<_>> = rect_coords.iter().map(|(x_left, y_low, x_right, y_high)| {
        Rectangle::new([(x_left, y_high), (x_right, y_low)], obstacle_style)
    }).collect();
//     chart.draw_series(LineSeries::new(
//         (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
//         &RED
//     )).unwrap();

    chart
        .draw_series(patches.iter())
        .unwrap();

    chart
        .draw_series(tree_points.iter().map(|p| {
            Circle::new((p.x() as i32, p.y() as i32), 3, &BLACK,)
        }))
        .unwrap();

    for (point_idx, parent_idx) in parents.iter().enumerate() {
        if *parent_idx >= 0 {
            let x0 = tree_points[*parent_idx].x() as i32;
            let y0 = tree_points[*parent_idx].y() as i32;
            let x1 = tree_points[point_idx].x() as i32;
            let y1 = tree_points[point_idx].y() as i32;
            chart
                .draw_series(std::iter::once(Line::new((x0, y0), (x1, y1))))
                .unwrap();
        }
    }

    if let Some(goal_idx) = goal_idx {
        let path_points = (0..)
            .map(|idx| {
                let i = goal_idx;
                let p = active_points[i];
                (p[0], p[1])
            })
            .take_while(|_| goal_idx >= 0)
            .collect::<Vec<_>>();
        chart
            .draw_series(std::iter::once((
                Path::new(path_points.into_iter(), &BLACK),
            )))
            .unwrap();
    }

    // chart.configure_mesh().draw().unwrap();



}




// fn plotting() {
//     let root = BitMapBackend::new("image.png", (1024, 1024)).into_drawing_area();
//     root.fill(&WHITE).unwrap();

//     let mut chart = ChartBuilder::on(&root)
//         .caption("Example Plot", ("sans-serif", 30))
//         .build_cartesian_2d(-10.0, 10.0, -10.0, 10.0)
//         .unwrap();

//     let patches: Vec<Rectangle<_>> = rect_coords.iter().map(|(x0, y0, x1, y1)| {
//         Rectangle::new([x0, y0], [(x1 - x0), (y1 - y0)], Shape::Rect)
//     }).collect();

//     chart
//         .draw_series(patches.iter().map(|r| {
//             (r, r.fill(&RED))
//         }))
//         .unwrap();

//     chart
//         .draw_series(active_points[..tree_count].iter().map(|p| {
//             (Circle::new([p[0], p[1]], 3, Fill::Another("black")),)
//         }))
//         .unwrap();

//     for (point_idx, parent_idx) in parents.iter().enumerate() {
//         if *parent_idx >= 0 {
//             let x0 = active_points[*parent_idx][0];
//             let y0 = active_points[*parent_idx][1];
//             let x1 = active_points[point_idx][0];
//             let y1 = active_points[point_idx][1];
//             chart
//                 .draw_series(std::iter::once((Line::new([x0, y0], [x1, y1]),)))
//                 .unwrap();
//         }
//     }

//     if let Some(goal_idx) = goal_idx {
//         let path_points = (0..)
//             .map(|idx| {
//                 let i = goal_idx;
//                 let p = active_points[i];
//                 (p[0], p[1])
//             })
//             .take_while(|_| goal_idx >= 0)
//             .collect::<Vec<_>>();
//         chart
//             .draw_series(std::iter::once((
//                 Path::new(path_points.into_iter(), &BLACK),
//             )))
//             .unwrap();
//     }

//     chart.configure_mesh().draw().unwrap();
// }