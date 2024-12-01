use std::ops::{Add, BitOrAssign};

#[derive(Debug)]
enum Instruction {
    Push(Value),
    Add,
    Jmpos(i32),
    Ret,
}

impl Instruction {
    fn from_str(input: &str) -> Self {
        let mut split_input = input.split(' ');
        match split_input.next().unwrap() {
            "push" => Self::Push(
                match split_input.next().unwrap().to_ascii_lowercase().as_str() {
                    "x" => Value::X,
                    "y" => Value::Y,
                    "z" => Value::Z,
                    v => Value::Num(v.parse().unwrap()),
                },
            ),
            "add" => Self::Add,
            "jmpos" => Self::Jmpos(split_input.next().unwrap().parse().unwrap()),
            "ret" => Self::Ret,
            v => panic!("Unrecognised instruction: {}", v),
        }
    }
}

#[derive(Debug)]
enum Value {
    X,
    Y,
    Z,
    Num(i32),
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn run_program(program: &[Instruction], point: Point) -> i32 {
    let mut pc = 0;
    let mut stack = vec![];

    loop {
        // dbg!(pc, &program[pc as usize], &stack);
        match &program[pc as usize] {
            Instruction::Push(value) => match value {
                Value::X => stack.push(point.x),
                Value::Y => stack.push(point.y),
                Value::Z => stack.push(point.z),
                Value::Num(v) => stack.push(*v),
            },
            Instruction::Add => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x + y);
            }
            Instruction::Jmpos(v) => {
                if stack.pop().unwrap() >= 0 {
                    pc += v
                }
            }
            Instruction::Ret => return stack.pop().unwrap(),
        }
        pc += 1;
    }
}

impl Add for Point {
    type Output = Option<Point>;

    fn add(self, rhs: Self) -> Self::Output {
        let new_point = Point {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        };
        if new_point.x < 0 || new_point.x >= 30 {
            return None;
        }
        if new_point.y < 0 || new_point.y >= 30 {
            return None;
        }
        if new_point.z < 0 || new_point.z >= 30 {
            return None;
        }
        Some(new_point)
    }
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn index<'a, T>(&self, grid: &'a [[[T; 30]; 30]; 30]) -> &'a T {
        &grid[self.x as usize][self.y as usize][self.z as usize]
    }

    fn index_mut<'a, T>(&self, grid: &'a mut [[[T; 30]; 30]; 30]) -> &'a mut T {
        &mut grid[self.x as usize][self.y as usize][self.z as usize]
    }
}

const CARDINALS: [Point; 6] = [
    Point { x: 1, y: 0, z: 0 },
    Point { x: -1, y: 0, z: 0 },
    Point { x: 0, y: 1, z: 0 },
    Point { x: 0, y: -1, z: 0 },
    Point { x: 0, y: 0, z: 1 },
    Point { x: 0, y: 0, z: -1 },
];

fn grow_cloud(
    grid: &[[[bool; 30]; 30]; 30],
    considered: &mut [[[bool; 30]; 30]; 30],
    point: Point,
) -> Vec<Point> {
    let mut included = vec![];
    if !*point.index(grid) {
        return included;
    }

    // New point is part of a cloud
    included.push(point);
    for offset in CARDINALS {
        let new_point = point + offset;
        let new_point = match new_point {
            Some(np) => np,
            None => {
                // The point where we would grow to is out of range.
                continue;
            }
        };
        if *new_point.index(considered) {
            // We've already considered this point, move on.
            continue;
        }
        // Regardless of whether it's in this a cloud or not, we've considered it now.
        new_point.index_mut(considered).bitor_assign(true);
        if *new_point.index(grid) {
            // New point is part of the cloud
            included.append(&mut grow_cloud(grid, considered, new_point));
        }
    }

    included
}

fn main() {
    let program_text = include_str!("../input_program.txt");

    let program: Vec<Instruction> = program_text.lines().map(Instruction::from_str).collect();

    let mut grid = [[[false; 30]; 30]; 30];

    let mut calibration_number = 0;
    for x in 0..30 {
        for y in 0..30 {
            for z in 0..30 {
                let grid_value = run_program(&program, Point::new(x, y, z));
                calibration_number += grid_value;
                if grid_value > 0 {
                    grid[x as usize][y as usize][z as usize] = true;
                }
            }
        }
    }
    println!("Calibration number: {calibration_number}");

    let mut clouds = 0;
    let mut considered_points = [[[false; 30]; 30]; 30];
    for x in 0..30 {
        for y in 0..30 {
            for z in 0..30 {
                if considered_points[x][y][z] {
                    // We've already considered this point, move on.
                    continue;
                }
                considered_points[x][y][z] = true;
                let cloud = grow_cloud(
                    &grid,
                    &mut considered_points,
                    Point::new(x as i32, y as i32, z as i32),
                );
                if !cloud.is_empty() {
                    clouds += 1;
                }
            }
        }
    }
    println!("Clouds: {clouds}");
}
