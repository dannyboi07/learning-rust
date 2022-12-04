fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }

    fn y(&self) -> &Y1 {
        &self.y
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    println!("Hello, world!");
    let arr: [i32; 5] = [1, 2, 3, 4, 7];

    let largest_int = largest(&arr);
    println!("largest integer: {}", largest_int);

    let char_arr: [char; 6] = ['d', 'a', 'n', 'i', 'e', 'l'];
    let largest_char = largest(&char_arr);
    println!("largest char: {}", largest_char);

    let point: Point<i32, i32> = Point { x: 67, y: 32 };
    println!("self.x: {}, self.y: {}", point.x(), point.y());

    let mixed_up = point.mixup(Point{x: 13, y: 9});
    println!("mixup.x: {}, mixup.y: {}", mixed_up.x(), mixed_up.y());


    println!("self.x: {}, self.y: {}", point.x(), point.y());
}
