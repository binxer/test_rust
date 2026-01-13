use std::clone;

struct Point {
    x: i32,
    y: i32,
}

fn newPoint<'a>(Point: &'a Point) -> &'a Point {
    &Point
}
fn main() {
    let mut p = Point { x: 1, y: 2 };
    match &mut p {
        &mut Point { ref mut x, ref y } => {
            *x += 1;
        }
    }
    //comment
    println!("Point({}, {})", p.x, p.y);
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = numbers.iter().fold(0, |a, &num| a + num);
    println!("Sum: {}", sum);
}
