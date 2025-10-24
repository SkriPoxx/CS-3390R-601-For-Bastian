//! =============================================
//! Módulo: Vector2
//! =============================================
//! Define la estructura Vector2 y operaciones como suma, resta,
//! multiplicación por escalar y producto punto (dot product).

use std::fmt;
use std::ops::{Add, Sub, Mul};
use num::Float;

/// Representa un vector bidimensional genérico (x, y)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Vector2<T> {
    /// Crea un nuevo vector con los valores x e y dados
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
}

// =============================================
// Implementaciones de operadores
// =============================================

impl<T: Float> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Float> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Float> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// =============================================
// Implementación del formateo visual (Display)
// =============================================

impl<T: Float + fmt::Display> fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
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
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);
        assert_eq!(v1 + v2, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector2::new(1.0, 3.0);
        let v2 = Vector2::new(2.0, 4.0);
        assert_eq!(v1.dot(&v2), 14.0);
    }

    #[test]
    fn test_scalar_multiplication() {
        let v = Vector2::new(2.0, 3.0);
        let result = v * 2.0;
        assert_eq!(result, Vector2::new(4.0, 6.0));
    }
}
