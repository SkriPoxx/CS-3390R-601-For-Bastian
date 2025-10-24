// =============================================
// Ejemplo de uso: Linear Algebra Library
// =============================================

// Importamos las estructuras desde la librería
use linear_algebra_library::{Vector2, Matrix2};

fn main() {
    // Creamos dos vectores 2D
    let v1 = Vector2::new(3.0, 4.0);
    let v2 = Vector2::new(1.0, 2.0);

    // Creamos una matriz 2x2
    let m1 = Matrix2::new(1.0, 2.0, 3.0, 4.0);

    // Mostramos resultados de operaciones
    println!("v1 + v2 = {}", v1 + v2);
    println!("m1 * v1 = {}", m1 * v1);
}
