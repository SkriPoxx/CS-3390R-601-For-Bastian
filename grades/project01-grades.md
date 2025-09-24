# Project 1 by Bastian Bagnara - Grading Results

## Grading Rubric:
1. Branch 'project01' exists: Found (1/1 points)
2. Directory 'project01' exists: Not found (0/1 points)
3. cargo check: Not checked (no project in correct directory) (0/1 points)
4. cargo build: Not checked (no project in correct directory) (0/2 points)
5. Program testing:
   - Not tested (no project in correct directory) (0/5 points)

**Total points: 1 out of 10 possible points**

## Notes:
- The project01 branch exists in your repository
- The required 'project01' directory was not found at the root of the repository
- A Rust project was found in the 'prime_factor_count' directory which appears to be your submission
- **CRITICAL ISSUES WITH RECENT CHANGES:**
  - You merged a "Project_2_NESW" branch into your project01 branch, which is completely inappropriate
  - No actual changes were made to your project01 assignment, so no regrading is necessary
  - The branch name "Project_2_NESW" does not follow the required format of "project##" (should be "project02")
  - You have committed unnecessary files to the repository root including Cargo.toml, Cargo.lock, and the entire target/ directory
  - The target/ directory should NEVER be committed as it contains regenerable binaries and wastes repository space
  - Your repository structure is now a mess with mixed project files in the root directory

## Resubmission Instructions:
To resubmit this assignment for full grading:

1. Create the project01 directory at the root of your repository:
   ```bash
   mkdir project01
   ```

2. Move your existing project into the project01 directory:
   ```bash
   git mv prime_factor_count/* project01/
   git mv prime_factor_count/.* project01/ 2>/dev/null || true
   rmdir prime_factor_count
   ```

3. Commit these changes:
   ```bash
   git add project01
   git commit -m "Moved project to correct project01 directory"
   ```

4. Push the changes to your project01 branch:
   ```bash
   git push
   ```

The assignment will be automatically regraded once these steps are completed.

**IMPORTANT:** I will accept one more resubmission if you want a grade on this assignment. If you need assistance with rearranging the structure of both project01 and project02, please reach out to me. I will not do the work for you, but I will walk you through the steps to clean up your repository structure.