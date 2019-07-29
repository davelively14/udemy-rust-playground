struct Foo {
    quax: i32,
    baz: String,
    z: Fuzz,
}

struct Fuzz {
    zed: i32,
}

struct Point2D {
    x: f64,
    y: f64,
}

fn add_points(a: Point2D, b: Point2D) -> Point2D {
    Point2D {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

#[derive(PartialEq)]
enum Animal {
    Dog,
    Cat,
}

pub fn run_examples() {
    let a = Foo {
        quax: 10,
        baz: String::from("Hello, world!"),
        z: Fuzz { zed: 4 },
    };

    println!("Foo");
    println!("- quax: {}", a.quax);
    println!("- baz: {}", a.baz);
    println!("- z.zed: {}", a.z.zed);

    let p1 = Point2D { x: 10.0, y: 20.0 };
    let p2 = Point2D { x: -2.0, y: 30.5 };
    let result = add_points(p1, p2);

    println!("Added two points: {}, {}", result.x, result.y);

    let my_pet = Animal::Dog;
    let other_pet = Animal::Cat;

    assert!(my_pet != other_pet);
}
