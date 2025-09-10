use std::io;

#[derive(Clone, Copy, Debug)]
enum Direction { North, South, East, West }

#[derive(Clone, Copy, Debug)]
struct Step {
    direction: Direction,
    distance: i32,
}

fn parse_step(line: &str) -> Result<Step, &'static str> {
    let mut parts = line.split_whitespace();
    let dir_str = parts.next().ok_or("missing direction")?;
    let dist_str = parts.next().ok_or("missing distance")?;
    if parts.next().is_some() { return Err("too many fields"); }

    let distance: i32 = dist_str.parse().map_err(|_| "distance must be integer")?;
    if distance <= 0 { return Err("distance must be positive"); }

    let direction = match dir_str {
        "N" => Direction::North,
        "S" => Direction::South,
        "E" => Direction::East,
        "W" => Direction::West,
        _ => return Err("direction must be N|S|E|W"),
    };

    Ok(Step { direction, distance })
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    // Posición inicial
    let (mut x, mut y) = (0i32, 0i32);
    println!("{x} {y}");

    loop {
        // === Bucle de validación: no sale hasta obtener un Step válido o EOF ===
        let step = loop {
            line.clear();
            let n = stdin.read_line(&mut line).expect("Failed to read line");
            if n == 0 {
                // EOF: terminar completamente el programa
                return;
            }

            let trimmed = line.trim();

            // 👉 Nuevo: si el usuario escribe CLEAR (sin importar mayúsculas)
            let lowered = trimmed.to_lowercase();
            if lowered == "clear" {
                println!("Exiting program (CLEAR typed).");
                return;
}


            if trimmed.is_empty() {
                eprintln!("missing direction");
                continue;
            }

            match parse_step(trimmed) {
                Ok(s) => break s,
                Err(msg) => {
                    eprintln!("{msg}");
                    continue;
                }
            }
        };

        // Aplicar el paso válido
        match step.direction {
            Direction::North => y += step.distance,
            Direction::South => y -= step.distance,
            Direction::East  => x += step.distance,
            Direction::West  => x -= step.distance,
        }
        println!("{x} {y}");
    }
}
