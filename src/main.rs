use linear_algebra_library::prelude::*;

fn main() {
    // === VECTORES ===
    let v1 = Vector2::new(3.0, 4.0);
    let v2 = Vector2::new(1.0, 2.0);

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
    println!("v1 + v2 = {}", v1 + v2);
    println!("v1 - v2 = {}", v1 - v2);
    println!("v1 * 2 = {}", v1 * 2.0);
    println!("v1.dot(v2) = {}", v1.dot(&v2));

    // === MATRICES ===
    let m1 = Matrix2::new(1.0, 2.0, 3.0, 4.0);
    println!("\nm1 = \n{}", m1);
    println!("m1 determinant = {}", m1.determinant());

    // Multiplicación matriz × vector
    println!("m1 * v1 = {}", m1 * v1);

    // Inversa
    match m1.inverse() {
        Some(inv) => println!("\nm1 inverse = \n{}", inv),
        None => println!("m1 has no inverse!"),
    }
}
