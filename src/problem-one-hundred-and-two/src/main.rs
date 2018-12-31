use std::fs;
use std::io;
use std::time::SystemTime;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

fn main() -> Result<(), io::Error> {
    let triangles = fs::read_to_string("p102_triangles.txt")?;

    let benchmark_start = SystemTime::now();
    let mut triangle_encloses_origin_sum: u16 = 0;

    for triangle_line in triangles.lines() {
        let lines: Vec<f32> = triangle_line
            .split(',')
            .map(|x| x.parse::<f32>().unwrap())
            .collect();
        debug_assert!(lines.len() == 6);

        let triangle = Triangle {
            a: Point {
                x: lines[0],
                y: lines[1],
            },
            b: Point {
                x: lines[2],
                y: lines[3],
            },
            c: Point {
                x: lines[4],
                y: lines[5],
            },
        };

        if is_origin_in_triangle(triangle) {
            triangle_encloses_origin_sum += 1;
        }
    }

    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();

    println!(
        "Sum: {} (took {:?})",
        triangle_encloses_origin_sum, benchmark_duration
    );

    Ok(())
}

fn is_origin_in_triangle(triangle: Triangle) -> bool {
    // see https://stackoverflow.com/a/34093754 (adjusted/optimized for always having P(0|0))

    let p0 = triangle.a;
    let p1 = triangle.b;
    let p2 = triangle.c;

    let difference_x_p2_p1 = p2.x - p1.x;
    let difference_y_p1_p2 = p1.y - p2.y;

    let delta = difference_y_p1_p2 * (p0.x - p2.x) + difference_x_p2_p1 * (p0.y - p2.y);
    let s = difference_y_p1_p2 * (-p2.x) + difference_x_p2_p1 * -p2.y;
    let t = (p2.y - p0.y) * (-p2.x) + (p0.x - p2.x) * -p2.y;

    if delta < 0.0 {
        s <= 0.0 && t <= 0.0 && s + t >= delta
    } else {
        s >= 0.0 && t >= 0.0 && s + t <= delta
    }
}

#[cfg(test)]
mod tests {
    use super::{is_origin_in_triangle, Point, Triangle};

    #[test]
    fn test_is_origin_in_triangle() {
        assert!(is_origin_in_triangle(Triangle {
            a: Point { x: 1.0, y: 2.0 },
            b: Point { x: -3.0, y: 1.0 },
            c: Point { x: 0.5, y: -3.0 }
        }));
    }
}
