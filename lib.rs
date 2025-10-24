// =============================================
// Linear Algebra Library (2D Vectors & Matrices)
// =============================================

// Importamos los traits necesarios para sobrecargar operadores y formatear texto
use std::fmt;
use std::ops::{Add, Sub, Mul};
use std::convert::From;

// =============================================
// Definición de Vector2
// =============================================



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,  // Componente X del vector
    pub y: f32,  // Componente Y del vector
}

impl Vector2 {
    /// Crea un nuevo vector con los valores x e y dados
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

// Implementamos suma de vectores usando el operador +
impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Suma cada componente
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Implementamos resta de vectores usando el operador -
impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // Resta cada componente
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Implementamos multiplicación vector × escalar (f32)
impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        // Multiplica cada componente por el escalar
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Implementamos conversión de una tupla (x, y) → Vector2
impl From<(f32, f32)> for Vector2 {
    fn from(tuple: (f32, f32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

// Implementamos Display para mostrar el vector de forma legible
impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// =============================================
// Definición de Matrix2
// =============================================

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix2 {
    pub a: f32, // Elemento fila 1, columna 1
    pub b: f32, // Elemento fila 1, columna 2
    pub c: f32, // Elemento fila 2, columna 1
    pub d: f32, // Elemento fila 2, columna 2
}

impl Matrix2 {
    /// Crea una nueva matriz con los valores dados
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }
}

// Implementamos multiplicación matriz × escalar
impl Mul<f32> for Matrix2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            a: self.a * scalar,
            b: self.b * scalar,
            c: self.c * scalar,
            d: self.d * scalar,
        }
    }
}

// Implementamos multiplicación matriz × vector
impl Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    fn mul(self, vec: Vector2) -> Vector2 {
        // Multiplicación estándar de matriz 2x2 por vector 2D
        Vector2 {
            x: self.a * vec.x + self.b * vec.y,
            y: self.c * vec.x + self.d * vec.y,
        }
    }
}

// Implementamos multiplicación matriz × matriz
impl Mul<Matrix2> for Matrix2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
        }
    }
}

// Implementamos conversión de una tupla (a, b, c, d) → Matrix2
impl From<(f32, f32, f32, f32)> for Matrix2 {
    fn from(tuple: (f32, f32, f32, f32)) -> Self {
        Self {
            a: tuple.0,
            b: tuple.1,
            c: tuple.2,
            d: tuple.3,
        }
    }
}

// Implementamos Display para mostrar la matriz con formato
impl fmt::Display for Matrix2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "| {:.2}  {:.2} |\n| {:.2}  {:.2} |",
            self.a, self.b, self.c, self.d
        )
    }
}

// =============================================
// Pruebas unitarias
// =============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_addition() {
        let v1 = Vector2::new(2.0, 3.0);
        let v2 = Vector2::new(1.0, 2.0);
        assert_eq!(v1 + v2, Vector2::new(3.0, 5.0));
    }

    #[test]
    fn test_matrix_vector_multiplication() {
        let m = Matrix2::new(1.0, 2.0, 3.0, 4.0);
        let v = Vector2::new(2.0, 1.0);
        assert_eq!(m * v, Vector2::new(4.0, 10.0));
    }
}
