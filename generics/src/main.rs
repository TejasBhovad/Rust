struct Point<T, U> {
    x: T,
    y: U,
}
// can take any value
impl<X, Y> Point<X, Y> {
    fn x(&self) -> &X {
        &self.x
    }
}
// takes only float values
impl Point<f64, f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

struct Point_two<T, U> {
    x: T,
    y: U,
}

// x is set to type of point_two, of the point_two method is called in
// y is set to y values of the point_two being passed in
impl<T, U> Point_two<T, U> {
    fn mixup<V, W>(self, other: Point_two<V, W>) -> Point_two<T, W> {
        Point_two {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("Largest number is {}", largest);
    let char_list = vec!['c', 'd', 'w', 'a'];
    let largest_char = get_largest_generic(char_list);
    println!("Largest character is {}", largest_char);

    let p1 = Point { x: 4, y: 5 };
    let p2 = Point { x: 0.5, y: 3.75 };

    let pt_1 = Point_two { x: 5, y: 10.4 };
    let pt_2 = Point_two { x: "hello", y: 'x' };

    let pt_3 = pt_1.mixup(pt_2);
    println!("pt_3.x = {}, pt_3.y ={}", pt_3.x, pt_3.y);
}

fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

//  get largest X from X list
fn get_largest_generic<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
