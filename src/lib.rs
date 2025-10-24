//! =============================================
//! Linear Algebra Library (Part 2)
//! =============================================
//! Este archivo principal conecta todos los módulos del proyecto:
//! vector.rs, matrix.rs y prelude.rs.
//!
//! También reexporta las estructuras principales para facilitar su uso.

pub mod vector;
pub mod matrix;
pub mod prelude;

pub use vector::Vector2;
pub use matrix::Matrix2;

