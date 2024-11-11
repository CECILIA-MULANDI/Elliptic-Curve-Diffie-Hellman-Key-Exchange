use sha1::{Sha1,Digest};
#[derive(Debug)]
struct Point {
    x: u128,
    y: u128,
}

#[derive(Debug)]
struct EllipticCurve {
    a: u128,
    b: u128,
    p: u128,
}

fn double_add_algorithm(point_p: Point, curve: &EllipticCurve, mut scalar: u128) -> Point {
    let mut point_q = point_p;
    let mut point_r = Point { x: 0, y: 0 };

    while scalar > 0 {
        if scalar % 2 == 1 {
            point_r = point_addition(&point_r, &point_q, curve);
        }
        point_q = point_doubling(&point_q, curve);
        scalar /= 2; 
    }
    point_r
}

fn point_addition(point1: &Point, point2: &Point, curve: &EllipticCurve) -> Point {
    if point1.x == 0 && point1.y == 0 {
        return Point { x: point2.x, y: point2.y };
    }
    if point2.x == 0 && point2.y == 0 {
        return Point { x: point1.x, y: point1.y };
    }
    if point1.x == point2.x && point1.y == point2.y {
        return point_doubling(point1, curve);
    }

    let dy = ((point2.y + curve.p - point1.y) % curve.p);
    let dx = mod_inverse(((point2.x + curve.p - point1.x) % curve.p) as i128, curve.p as i128) as u128;
    let slope = (dy * dx) % curve.p;

    let x3 = (slope * slope + curve.p - point1.x + curve.p - point2.x) % curve.p;
    let y3 = (slope * (point1.x + curve.p - x3) % curve.p + curve.p - point1.y) % curve.p;

    Point { x: x3, y: y3 }
}

fn point_doubling(point1: &Point, curve: &EllipticCurve) -> Point {
    if point1.x == 0 && point1.y == 0 || point1.y == 0 {
        return Point { x: 0, y: 0 };
    }

    let dy = (3 * point1.x * point1.x + curve.a) % curve.p;
    let dx = mod_inverse((2 * point1.y % curve.p) as i128, curve.p as i128) as u128;
    let slope = (dy * dx) % curve.p;

    let x3 = (slope * slope + curve.p - 2 * point1.x) % curve.p;
    let y3 = (slope * (point1.x + curve.p - x3) % curve.p + curve.p - point1.y) % curve.p;

    Point { x: x3, y: y3 }
}


fn mod_inverse(a: i128, b: i128) -> i128 {
    let (gcd, x, _) = extended_euclidean_algorithm(a, b);
    if gcd != 1 {
        panic!("No modular inverse exists for {} mod {}", a, b);
    }
    (x % b + b) % b
}

fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_euclidean_algorithm(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (gcd, x, y)
    }
}

fn hash_point(point:&Point)->String{
    let mut hasher = Sha1::new();
    hasher.update(&point.x.to_string().as_bytes());
    // hasher.update(&point.y.to_le_bytes());
    let result = hasher.finalize();
    format!("{:x}", result) 
}

fn main() {
    let curve = EllipticCurve {
        a: 497,
        b: 1768,
        p: 9739,
    };
    let result = double_add_algorithm(Point { x: 815, y: 3190 }, &curve, 1829);
    println!("Result: {:?}", result);

    println!("The secret key is: {:?}",hash_point(&result));
}
