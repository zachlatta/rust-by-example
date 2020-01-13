#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime (???)
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bot_right: Point,
}

fn rect_area(r: Rectangle) -> f32 {
    let (tx, ty) = (r.top_left.x, r.top_left.y);
    let (bx, by) = (r.bot_right.x, r.bot_right.y);

    let height = ty - by;
    let width = bx - tx;

    height * width
}

fn square(p: Point, len: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: p.x,
            y: p.y + len,
        },
        bot_right: Point {
            x: p.x + len,
            y: p.y,
        },
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };

    println!("point coords: ({}, {})", point.x, point.y);

    let bot_right = Point { x: 5.2, ..point };

    println!("second point: ({}, {})", bot_right.x, bot_right.y);

    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rect = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bot_right: bot_right,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair: {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair: {:?} and {:?}", integer, decimal);

    println!("rect area: {}", rect_area(_rect));

    let square = square(Point { x: 1.0, y: 1.0 }, 3.0);

    println!("square: {:?}", square);
}
