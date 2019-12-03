use std::fs::read_to_string;

type Point = (i64, i64);

enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    orientation: Orientation,
    distance: i64,
}

fn manhattan_distance(a: &Point, b: &Point) -> u64 {
    (a.0 - b.0).abs() as u64 + (a.1 - b.1).abs() as u64
}

fn generate_ranges(path: &Vec<Instruction>) -> Vec<Point> {
    let mut loc: Point = (0, 0);
    let mut ranges = vec![loc];
    for ins in path {
        match ins.orientation {
            Orientation::Up => ranges.push((loc.0, loc.1 + ins.distance)),
            Orientation::Down => ranges.push((loc.0, loc.1 - ins.distance)),
            Orientation::Left => ranges.push((loc.0 - ins.distance, loc.1)),
            Orientation::Right => ranges.push((loc.0 + ins.distance, loc.1)),
        }
        loc = *ranges.last().unwrap();
    }
    ranges
}

fn overlap(p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> Option<Point> {

    let p0_x = p0.0 as f64;
    let p0_y = p0.1 as f64;

    let p1_x = p1.0 as f64;
    let p1_y = p1.1 as f64;

    let p2_x = p2.0 as f64;
    let p2_y = p2.1 as f64;

    let p3_x = p3.0 as f64;
    let p3_y = p3.1 as f64;

    let s1_x = p1_x - p0_x;
    let s1_y = p1_y - p0_y;
    let s2_x = p3_x - p2_x;
    let s2_y = p3_y - p2_y;

    let det = -s2_x * s1_y + s1_x * s2_y;
    let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / det;
    let t = ( s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / det;

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        let ix = (p0_x + (t * s1_x)) as i64;
        let iy = (p0_y + (t * s1_y)) as i64;
        Some((ix, iy))
    }
    else {
        None
    }
}

fn min_manhattan_intersection(path1: &Vec<Instruction>, path2: &Vec<Instruction>) -> (Point, u64) {
    let ranges1 = generate_ranges(path1);
    let ranges2 = generate_ranges(path2);
    let mut inters = vec![];

    for (p0, p1) in ranges1.iter().zip(ranges1[1..].iter()) {
        for (p2, p3) in ranges2.iter().zip(ranges2[1..].iter()) {
            let o = overlap(p0, p1, p2, p3);
            if o.is_some() && o != Some((0, 0)) {
                inters.push(o.unwrap())
            }
        }
    }

    let min = *inters.iter().min_by(|inter1, inter2| manhattan_distance(&(0, 0), inter1).cmp(&manhattan_distance(&(0, 0), inter2))).unwrap();
    (min, manhattan_distance(&(0, 0), &min))
}

fn min_steps_intersection(path1: &Vec<Instruction>, path2: &Vec<Instruction>) -> (Point, u64) {
    let ranges1 = generate_ranges(path1);
    let ranges2 = generate_ranges(path2);
    let mut inters = vec![];

    let mut steps_a = 0;
    for (p0, p1) in ranges1.iter().zip(ranges1[1..].iter()) {
        steps_a += manhattan_distance(p0, p1);
        
        let mut steps_b = 0;
        for (p2, p3) in ranges2.iter().zip(ranges2[1..].iter()) {
            steps_b += manhattan_distance(p2, p3);

            let o = overlap(p0, p1, p2, p3);
            if o.is_some() && o != Some((0, 0)) {
                let dist = steps_a + steps_b - manhattan_distance(p1, &o.unwrap()) - manhattan_distance(p3, &o.unwrap());
                inters.push((o.unwrap(), dist))
            }
        }
    }

    *inters.iter().min_by_key(|(_, dist)| dist).unwrap()
}

fn input_to_instructions(input: String) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            line.split(',').map(|s| {
                Instruction {
                    orientation: match s.chars().next().unwrap() {
                        'U' => Orientation::Up,
                        'D' => Orientation::Down,
                        'L' => Orientation::Left,
                        'R' => Orientation::Right,
                        _ => unreachable!(),
                    },
                    distance: s[1..].parse().unwrap()
                }
            }).collect()
        })
        .collect()
}

pub fn main() {
    let input = read_to_string("input/day3/input1.txt").unwrap();
    let ins = input_to_instructions(input);
    println!("{:?}", min_manhattan_intersection(&ins[0], &ins[1]));
    println!("{:?}", min_steps_intersection(&ins[0], &ins[1]));
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day3_test1() {
        let ins0 = &input_to_instructions("R8,U5,L5,D3".to_string())[0];
        let ins1 = &input_to_instructions("U7,R6,D4,L4".to_string())[0];
        assert_eq!(6, min_manhattan_intersection(ins0, ins1).1);
	}

	#[test]
	fn day3_test2() {
        let ins0 = &input_to_instructions("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string())[0];
        let ins1 = &input_to_instructions("U62,R66,U55,R34,D71,R55,D58,R83".to_string())[0];
        assert_eq!(159, min_manhattan_intersection(ins0, ins1).1);
	}

	#[test]
	fn day3_test3() {
        let ins0 = &input_to_instructions("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string())[0];
        let ins1 = &input_to_instructions("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string())[0];
        assert_eq!(135, min_manhattan_intersection(ins0, ins1).1);
    }
    
    #[test]
    fn day3_test4() {
        let ins0 = &input_to_instructions("R8,U5,L5,D3".to_string())[0];
        let ins1 = &input_to_instructions("U7,R6,D4,L4".to_string())[0];
        assert_eq!(30, min_steps_intersection(ins0, ins1).1);
    }
    
    #[test]
    fn day3_test5() {
        let ins0 = &input_to_instructions("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string())[0];
        let ins1 = &input_to_instructions("U62,R66,U55,R34,D71,R55,D58,R83".to_string())[0];
        assert_eq!(610, min_steps_intersection(ins0, ins1).1);
    }
    
    #[test]
    fn day3_test6() {
        let ins0 = &input_to_instructions("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string())[0];
        let ins1 = &input_to_instructions("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string())[0];
        assert_eq!(410, min_steps_intersection(ins0, ins1).1);
	}
}
