#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
}
use glium::implement_vertex;

implement_vertex!(Vertex, position);

// These points goes x: left to right 0...8, y: bottom to top 0...8
// -100 + point*25: Formula to transform from obstacle (only for obstacle) in 2d cordinates to glium cordinates

// x location of the vertex goes from left to right of the screen: -100 .... 100
// y location of vertex goes from top to bottom: 100...-100

// Trinagle vertex connects clockwise

//Robot is practically at the center of the screen, its translation will be given by the path
pub const ROBOT: [Vertex; 4] = [
    Vertex {
        position: (0.0, 0.0, 0.0),
    }, // dummy vector because in the original model indices
    // start at 1
    Vertex {
        position: (-1.00, 1.3457, -1.10804),
    },
    Vertex {
        position: (-1.12, -1.5557, -1.10804),
    },
    Vertex {
        position: (1.34, 0.7457, -1.10804),
    },
];

// Trying for the obstacle course
pub const VERTICES: [Vertex; 5] = [
    Vertex {
        position: (0.0, 0.0, 0.0),
    }, // dummy vector because in the original model indices
    // start at 1
    Vertex {
        position: (-50.00, -50.000, -1.10804),
    },
    Vertex {
        position: (50.00, 50.000, -1.10804),
    },
    Vertex {
        position: (50.00, -50.00, -1.10804),
    },
    Vertex {
        position: (-50.00,50.00,-1.10804),
    },
];

//(2.0, 2.0, 6.0, 6.0),  1,2,3 - 1,4,2
// (3.0, 1.0, 4.0, 2.0),
// (1.0, 1.0, 2.0, 2.0),

// -100 + point*25: Formula to transform from obstacle (only for obstacle) in 2d cordinates to glium cordinates
 
#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32),
}

implement_vertex!(Normal, normal);

pub const NORMALS: [Normal; 11] = [
    Normal {
        normal: (0.0, 0.0, 0.0),
    }, // dummy vector because in the original model indices
    // start at 1
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966824, 0.255443, 0.0),
    },
    Normal {
        normal: (-0.092052, 0.995754, 0.0),
    },
    Normal {
        normal: (0.68205, 0.731305, 0.0),
    },
    Normal {
        normal: (0.870301, 0.492521, -0.0),
    },
    Normal {
        normal: (-0.893014, -0.256345, -0.369882),
    },
    Normal {
        normal: (-0.893437, 0.255997, -0.369102),
    },
    Normal {
        normal: (-0.0838771, 0.995843, -0.0355068),
    },
    Normal {
        normal: (0.629724, 0.73186, 0.260439),
    },
    Normal {
        normal: (0.629724, 0.73186, 0.260439),
    },
];

pub const NORMALS2: [Normal; 4] = [
    Normal {
        normal: (0.0, 0.0, 0.0),
    }, // dummy vector because in the original model indices
    // start at 1
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966824, 0.255443, 0.0),
    },
    Normal {
        normal: (-0.966824, 0.255443, 0.0),
    },
];

// pub const INDICES: [u16; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9u16];
pub const INDICES: [u16; 6] = [1, 2, 3, 1, 4, 2u16];

pub const INDICES2: [u16; 3] = [1, 2, 3u16];


// pub const VERTICES: [Vertex; 10] = [
//     Vertex {
//         position: (0.0, 0.0, 0.0),
//     }, // dummy vector because in the original model indices
//     // start at 1
//     Vertex {
//         position: (10.6266, 10.3457, -1.10804),
//     },
//     Vertex {
//         position: (10.0714, 30.4443, -1.10804),
//     },
//     Vertex {
//         position: (30.7155, 20.1438, -1.10804),
//     },
//     Vertex {
//         position: (40.6266, 10.3457, -1.10804),
//     },
//     Vertex {
//         position: (40.0714, 30.4443, -1.10804),
//     },
//     Vertex {
//         position: (60.7155, 20.1438, -1.10804),
//     },
//     Vertex {
//         position: (40.6266, 50.3457, -1.10804),
//     },
//     Vertex {
//         position: (40.0714, 70.4443, -1.10804),
//     },
//     Vertex {
//         position: (100.7155, -60.1438, -1.10804),
//     },
// ];

// pub const PATHS: [Vertex; 4] = [
//     Vertex {
//         position: (0.0, 0.0, 0.0),
//     }, // dummy vector because in the original model indices
//     // start at 1
//     Vertex {
//         position: (5.6266, 10.3457, -1.10804),
//     },
//     Vertex {
//         position: (5.0714, 30.4443, -1.10804),
//     },
//     Vertex {
//         position: (25.7155, 20.1438, -1.10804),
//     },
// ];

// pub const ROBOT: [Vertex; 4] = [
//     Vertex {
//         position: (0.0, 0.0, 0.0),
//     }, // dummy vector because in the original model indices
//     // start at 1
//     Vertex {
//         position: (5.6266, 10.3457, -1.10804),
//     },
//     Vertex {
//         position: (5.0714, 30.4443, -1.10804),
//     },
//     Vertex {
//         position: (25.7155, 20.1438, -1.10804),
//     },
// ];
