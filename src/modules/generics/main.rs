struct Point {
    x: i32,
    y: i32,
}

// generics in point structure
struct PointV2<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointV3<T, U> {
    x: T,
    y: U,
}

// generics in enum
enum OptionV2<T> {
    Some(T),
    None,
}

// genric in impl
impl<T> PointV2<T> {
    fn new(x: T, y: T) -> Self {
        PointV2 { x, y }
    }
}

impl<T, U> PointV3<T, U> {
    fn mixup<V, W>(self, other: PointV3<V, W>) -> PointV3<T, W> {
        return PointV3 {
            x: self.x,
            y: other.y,
        };
    }
}

// rust generics example
// rust uses trait to make sure the type can do certain operations, like comparison in this case for the `largest` function
// and only accepts types that implement the required traits.
fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn generics_v1() {
    // non growable number list
    let number_list = [1, 2, 1, 4, 5];
    // growable number list
    let mut number_list_v2 = vec![1, 2, 1, 4, 5];
    number_list_v2.push(123);
    println!(
        "The largest number in the non-growable list is: {}",
        get_largest(&number_list)
    );
    println!(
        "The largest number in the growable list is: {}",
        get_largest(&number_list_v2)
    );
}

fn generics_v2() {
    let point_list = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 5, y: 6 },
    ];
    for point in point_list.iter() {
        println!("Point coordinates: ({}, {})", point.x, point.y);
    }
    let point_v2_list = vec![
        PointV2 { x: 1, y: 2 },
        PointV2 { x: 3, y: 4 },
        PointV2 { x: 5, y: 6 },
    ];
    for point in point_v2_list.iter() {
        println!("PointV2 coordinates: ({}, {})", point.x, point.y);
    }
}

fn generics_v3() {
    let some_value: OptionV2<i32> = OptionV2::Some(42);
    let none_value: OptionV2<i32> = OptionV2::None;

    match some_value {
        OptionV2::Some(val) => println!("Got a value: {}", val),
        OptionV2::None => println!("Got nothing"),
    }

    match none_value {
        OptionV2::Some(val) => println!("Got a value: {}", val),
        OptionV2::None => println!("Got nothing"),
    }
}

fn generics_v4() {
    let point = PointV2::new(7, 8);
    println!("PointV2 created using new: ({}, {})", point.x, point.y);
    let some_point = PointV3 { x: 23, y: 76 };

    let some_point_v2 = PointV3::mixup(some_point, PointV3 { x: 12, y: 'b' });
    println!("{:?}",some_point_v2);
}

pub fn generics() {
    println!("");
    println!("This is a demonstration of generics in Rust.");
    generics_v1();
    generics_v2();
    generics_v3();
    generics_v4();
    println!("");
}
