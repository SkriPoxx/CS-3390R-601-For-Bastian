# Project 04 Grade - Bastian Bagnara

## Repository Structure
- **Branch and directory `project04` exist**: 0 / 1 point
- No `project04` branch found in repository

## Compilation
- **`cargo check` and `cargo build` pass**: 0 / 1 point
- No project04 branch to test compilation

## Unit Tests
- **Unit tests for core functionality**: 0 / 2 points
- No project04 branch to test unit tests

## Program Functionality
- **Test Case 1 (Default Mandelbrot)**: 0 / 2 points (0 for PNG generation, 0 for image content)
- **Test Case 2 (Zoomed region)**: 0 / 2 points (0 for PNG generation, 0 for image content)
- **Test Case 3 (Different aspect ratio)**: 0 / 2 points (0 for PNG generation, 0 for image content)
- No project04 branch to test program functionality

## Summary
**Total points**: 0 out of 10 possible points

## Issues to Address for Resubmission
1. **Missing project04 branch**: No project04 branch found in repository
2. **Missing submission**: Project04 assignment has not been submitted

## Resubmission Instructions
1. **Repository structure**: Create a `project04` branch with a `project04` directory containing your Rust project
2. **Dependencies**: Add appropriate crates to `Cargo.toml` (suggested: `clap`, `num`, `image`)
3. **Command line arguments**: Implement all required arguments (`--x_min`, `--y_min`, `--x_max`, `--y_max`, `--width`, `--height`)
4. **Mandelbrot algorithm**: Implement the iteration `z <- (z * z) + c` with proper convergence testing
5. **PNG generation**: Use appropriate image generation approach to create valid PNG files
6. **Unit tests**: Add comprehensive tests for core functionality
7. **Build artifacts**: Remove `Cargo.lock` and `target` directory from repository (add to `.gitignore`)
8. **Image verification**: Ensure generated PNG files contain recognizable Mandelbrot set patterns
9. **Bonus implementation**: Consider implementing aspect ratio preservation for bonus points