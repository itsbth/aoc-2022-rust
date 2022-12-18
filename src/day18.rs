use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Voxel {
    x: i32,
    y: i32,
    z: i32,
}

pub fn generator(input: &str) -> Vec<Voxel> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(lines(
        (x: i32) "," (y: i32) "," (z: i32) => Voxel { x, y, z }
    ));
    parser.parse(input).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FaceDirection {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl FaceDirection {
    // returns the position of the face in the direction of w
    fn get_position(self, x: i32, y: i32, z: i32) -> (i32, i32, i32) {
        match self {
            FaceDirection::Front => (x, y, z),
            FaceDirection::Right => (x, y, z),
            FaceDirection::Top => (x, y, z),
            FaceDirection::Back => (x + 1, y, z),
            FaceDirection::Left => (x, y + 1, z),
            FaceDirection::Bottom => (x, y, z + 1),
        }
    }
    // "normalize" it to always be one of the "front-facing" faces (Front, Right, Top)
    fn normalize(self) -> Self {
        match self {
            FaceDirection::Front => FaceDirection::Front,
            FaceDirection::Back => FaceDirection::Front,
            FaceDirection::Left => FaceDirection::Right,
            FaceDirection::Right => FaceDirection::Right,
            FaceDirection::Top => FaceDirection::Top,
            FaceDirection::Bottom => FaceDirection::Top,
        }
    }

    const DIRECTIONS: [FaceDirection; 6] = [
        FaceDirection::Front,
        FaceDirection::Back,
        FaceDirection::Left,
        FaceDirection::Right,
        FaceDirection::Top,
        FaceDirection::Bottom,
    ];
}

// faces are the 6 sides of a cube. (x, y, z) is the position of the cube, w is the direction of the face.
// faces do not have a direction, so the back face of (x, y, z) is the same as the front face of (x, y, z + 1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Face {
    x: i32,
    y: i32,
    z: i32,
    w: FaceDirection,
}

pub fn part_1(input: &[Voxel]) -> usize {
    let mut faces = HashSet::new();
    // we want to get the area of the faces of the cubes
    // as one face is shared by at most 2, we can use xor to get the faces that are not shared
    for voxel in input {
        for dir in FaceDirection::DIRECTIONS.iter() {
            let (x, y, z) = dir.get_position(voxel.x, voxel.y, voxel.z);
            let face = Face {
                x,
                y,
                z,
                w: dir.normalize(),
            };
            if faces.contains(&face) {
                faces.remove(&face);
            } else {
                faces.insert(face);
            }
        }
    }
    faces.len()
}

const DIRECTIONS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
    (0, 0, -1),
    (0, -1, 0),
    (-1, 0, 0),
];

pub fn part_1_alt(input: &[Voxel]) -> usize {
    let input: HashSet<_> = input.iter().collect();
    let mut overlapping = 0;
    for voxel in input.iter() {
        for (dx, dy, dz) in DIRECTIONS[..3].iter() {
            let neighbor = Voxel {
                x: voxel.x + dx,
                y: voxel.y + dy,
                z: voxel.z + dz,
            };
            if input.contains(&neighbor) {
                overlapping += 1;
            }
        }
    }
    input.len() * 6 - 2 * overlapping
}

// part 2 was total surface, but we want the exterior surface
pub fn part_2(input: &[Voxel]) -> usize {
    let input: HashSet<_> = input.iter().collect();
    let extents = input.iter().fold(
        (
            std::i32::MAX,
            std::i32::MAX,
            std::i32::MAX,
            std::i32::MIN,
            std::i32::MIN,
            std::i32::MIN,
        ),
        |(min_x, min_y, min_z, max_x, max_y, max_z), voxel| {
            (
                min_x.min(voxel.x),
                min_y.min(voxel.y),
                min_z.min(voxel.z),
                max_x.max(voxel.x),
                max_y.max(voxel.y),
                max_z.max(voxel.z),
            )
        },
    );
    // convert to inclusive ranges
    let extents = (
        (extents.0 - 1)..=(extents.3 + 1),
        (extents.1 - 1)..=(extents.4 + 1),
        (extents.2 - 1)..=(extents.5 + 1),
    );
    let mut queue = Vec::new();
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut faces = 0;
    queue.push((*extents.0.start(), *extents.1.start(), *extents.2.start()));
    while let Some((x, y, z)) = queue.pop() {
        if visited.contains(&(x, y, z)) {
            continue;
        }
        visited.insert((x, y, z));
        for dir in DIRECTIONS {
            let (nx, ny, nz) = (x + dir.0, y + dir.1, z + dir.2);
            if !extents.0.contains(&nx)
                || !extents.1.contains(&ny)
                || !extents.2.contains(&nz)
                || visited.contains(&(nx, ny, nz))
            {
                continue;
            }
            // if it's a voxel, bump the face count and don't add it to the queue
            // otherwise, add it to the queue
            if input.contains(&Voxel {
                x: nx,
                y: ny,
                z: nz,
            }) {
                faces += 1;
            } else {
                queue.push((nx, ny, nz));
            }
        }
    }
    faces
}
