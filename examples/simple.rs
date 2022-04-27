//! A simple render.

static H: u16 = 256;
static W: u16 = 256;

fn main() {
    println!("P3\n{} {}\n255\n", W, H);

    for x in (0..H).rev() {
        eprint!("\rScanlines remaining: {}", x);
        for y in 0..W {
            let b = (0.25 * 255.) as u16;

            println!("{} {} {}\n", y, x, b);
        }
    }

    eprint!("\nDone.\n");
}