//! =============================================
//! Módulo: Matrix2
//! =============================================
//! Define la estructura Matrix2 y operaciones como
//! multiplicación, determinante e inversión.

use std::fmt;
use std::ops::Mul;
use num::Float;

use crate::vector::Vector2;

/// Representa una matriz 2x2 genérica
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix2<T: num::Float> {
    pub a: T, pub b: T,
    pub c: T, pub d: T,
}


impl<T: Float> Matrix2<T> {
    /// Crea una nueva matriz con los valores dados
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }

    /// Calcula el determinante de la matriz
    pub fn determinant(&self) -> T {
        (self.a * self.d) - (self.b * self.c)
    }

    /// Calcula la matriz inversa (si existe)
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == T::zero() {
            None
        } else {
            let inv_det = T::one() / det;
            Some(Self {
                a: self.d * inv_det,
                b: -self.b * inv_det,
                c: -self.c * inv_det,
                d: self.a * inv_det,
            })
        }
    }
}

// =============================================
// Implementaciones de multiplicaciones
// =============================================

impl<T: Float> Mul<Vector2<T>> for Matrix2<T> {
    type Output = Vector2<T>;

    fn mul(self, vec: Vector2<T>) -> Vector2<T> {
        Vector2::new(
            self.a * vec.x + self.b * vec.y,
            self.c * vec.x + self.d * vec.y,
        )
    }
}

impl<T: Float> Mul<Matrix2<T>> for Matrix2<T> {
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

// =============================================
// Formateo visual (Display)
// =============================================

impl<T: Float + fmt::Display> fmt::Display for Matrix2<T> {
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
    fn test_determinant() {
        let m = Matrix2::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m.determinant(), -2.0);
    }

    #[test]
    fn test_inverse() {
        let m = Matrix2::new(4.0, 7.0, 2.0, 6.0);
        let inv = m.inverse().unwrap();
        let expected = Matrix2::new(0.6, -0.7, -0.2, 0.4);
        assert!((inv.a - expected.a).abs() < 1e-6);
        assert!((inv.b - expected.b).abs() < 1e-6);
        assert!((inv.c - expected.c).abs() < 1e-6);
        assert!((inv.d - expected.d).abs() < 1e-6);
    }
}
