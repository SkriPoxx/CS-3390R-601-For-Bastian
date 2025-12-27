use clap::Parser;
use image::{ImageBuffer, Rgb, RgbImage};
use num::complex::Complex;

/// Máximo de iteraciones por punto del plano complejo.
const MAX_ITERATIONS: u32 = 256;

/// Argumentos de línea de comandos EXACTOS como en la consigna.
#[derive(Parser, Debug)]
#[command(
    name = "mandelbrot",
    about = "Genera un PNG del conjunto de Mandelbrot a partir de un rectángulo en el plano complejo",
    arg_required_else_help = true
)]
struct Args {
    /// Límite izquierdo (x_min)
    #[arg(long = "x_min")]
    x_min: f64,

    /// Límite inferior (y_min)
    #[arg(long = "y_min")]
    y_min: f64,

    /// Límite derecho (x_max)
    #[arg(long = "x_max")]
    x_max: f64,

    /// Límite superior (y_max)
    #[arg(long = "y_max")]
    y_max: f64,

    /// Ancho en píxeles
    #[arg(long)]
    width: u32,

    /// Alto en píxeles
    #[arg(long)]
    height: u32,

    /// Archivo PNG de salida (opcional)
    #[arg(long, default_value = "mandelbrot.png")]
    out: String,
}

/// Devuelve cuántas iteraciones tarda en escapar (o MAX_ITERATIONS si no escapa).
fn mandelbrot_escape_iterations(c: Complex<f64>) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITERATIONS {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    MAX_ITERATIONS
}

/// Convierte un píxel (col, fila) a un punto (x, y) del plano complejo según el rectángulo dado.
fn map_pixel_to_complex(
    col: u32,
    row: u32,
    image_width: u32,
    image_height: u32,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Complex<f64> {
    let x = x_min + (col as f64) * (x_max - x_min) / (image_width as f64 - 1.0);
    let y = y_min + (row as f64) * (y_max - y_min) / (image_height as f64 - 1.0);
    Complex::new(x, y)
}

/// Renderiza el fractal y guarda un PNG en `output_path`.
fn render_mandelbrot_to_png(
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
    image_width: u32,
    image_height: u32,
    output_path: &str,
) {
    let mut image: RgbImage = ImageBuffer::new(image_width, image_height);

    for row in 0..image_height {
        for col in 0..image_width {
            let c = map_pixel_to_complex(col, row, image_width, image_height, x_min, x_max, y_min, y_max);
            let iterations = mandelbrot_escape_iterations(c);

            // Escala de grises simple: negro = dentro; más claro = escapó antes.
            let shade = if iterations >= MAX_ITERATIONS {
                0
            } else {
                (255.0 * (iterations as f32 / MAX_ITERATIONS as f32)) as u8
            };

            image.put_pixel(col, row, Rgb([shade, shade, shade]));
        }
    }

    image.save(output_path).expect("No se pudo guardar el PNG");
    println!("✅ Guardado: {output_path}");
}

fn main() {
    let args = Args::parse();

    // Validaciones simples (cap. 3: if + println!) en lugar de assert! (cap. 11).
    if !(args.x_min < args.x_max && args.y_min < args.y_max) {
        println!("Error: se requiere x_min < x_max y y_min < y_max");
        return;
    }
    if !(args.width > 1 && args.height > 1) {
        println!("Error: width y height deben ser > 1");
        return;
    }

    render_mandelbrot_to_png(
        args.x_min,
        args.y_min,
        args.x_max,
        args.y_max,
        args.width,
        args.height,
        &args.out,
    );
}
