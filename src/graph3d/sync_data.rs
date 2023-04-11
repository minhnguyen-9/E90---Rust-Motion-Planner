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
        position: (-50.00, 50.00, -1.10804),
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

pub const NORMALS: [Normal; 21 ] = [
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

pub const CIRCLE: [Vertex; 35] = [
    Vertex {
        position: (0.0, 0.0, 0.0),
    }, // dummy vector because in the original model indices
    // start at 1
    Vertex {
        position: (0.0, 0.0, 0.0),
    },
    Vertex {
        position: (1.0, 0.0, 0.0), 
    },
    Vertex {
        position: (0.9800665778412416, 0.19866933079506122, 0.0),
    },
    Vertex {
        position: (0.9210609940028851, 0.3894183423086505, 0.0),
    },
    Vertex {
        position: (0.8253356149096782, 0.5646424733950355, 0.0),

    },
    Vertex {
        position:(0.6967067093471655, 0.7173560908995228, 0.0) ,
    },
    Vertex {
        position: (0.5403023058681398, 0.8414709848078965, 0.0),
    },
    Vertex {
        position:  (0.3623577544766736, 0.9320390859672263, 0.0), 
        
    },
    Vertex {
        position: (0.169967142900241, 0.9854497299884601, 0.0),
    },
    Vertex {
        position: (-0.029199522301288593, 0.9995736030415052, 0.0) ,
    },
    Vertex {
        position:   (-0.2272020946930869, 0.9738476308781953, 0.0),
    },
    Vertex {
        position: (-0.4161468365471422, 0.9092974268256818, 0.0),
    },
    Vertex {
        position: (-0.5885011172553455, 0.8084964038195903, 0.0) ,
    },
    Vertex {
        position:   (-0.7373937155412454, 0.675463180551151, 0.0), 
    },
    Vertex {
        position: (-0.8568887533689473, 0.5155013718214642, 0.0),
    },
    Vertex {
        position: (-0.9422223406686583, 0.33498815015590466, 0.0),
    },
    Vertex {
        position: (-0.9899924966004455, 0.14112000805986677, 0.0),

    },
    Vertex {
        position: (-0.998294775794753, -0.05837414342758053, 0.0),
    },
    Vertex {
        position: (-0.9667981925794609, -0.2555411020268321, 0.0),
    },
    Vertex {
        position: (-0.8967584163341465, -0.4425204432948533, 0.0), 

    },
    Vertex {
        position: (-0.7909677119144161, -0.61185789094272, 0.0) ,
    },
    Vertex {
        position: (-0.6536436208636113, -0.7568024953079288, 0.0),
    },
    Vertex {
        position: (-0.4902608213406987, -0.8715757724135886, 0.0), 
    },
    Vertex {
        position: (-0.30733286997841847, -0.9516020738895163, 0.0) ,
    },
    Vertex {
        position: (-0.1121525269350531, -0.9936910036334646, 0.0),
    },
    Vertex {
        position: (0.08749898343944816, -0.9961646088358406, 0.0), 
    },
    Vertex {
        position: (0.28366218546322797, -0.9589242746631379, 0.0),
    },
    Vertex {
        position: (0.4685166713003787, -0.8834546557201524, 0.0),
    },
    Vertex {
        position: (0.634692875942636, -0.772764487555986, 0.0), 
    },
    Vertex {
        position: (0.7755658785102513, -0.6312666378723195, 0.0) ,
    },
    Vertex {
        position: (0.8855195169413201, -0.464602179413755, 0.0),
    },
    Vertex {
        position: (0.9601702866503667, -0.2794154981989233, 0.0),
    },
    Vertex {
        position:  (0.9965420970232177, -0.08308940281749375, 0.0),
    },
    Vertex {
        position: (1.0, 0.0, 0.0), 
    },
];

pub const CIRCLE_INDICES: [u16; 34] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34u16];

pub const CIRCLE_NORMALS: [Normal; 34] = [
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
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
    Normal {
        normal: (-0.966742, -0.255752, 0.0),
    },
];
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
