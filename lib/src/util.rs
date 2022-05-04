pub fn write_color(x: i16, y: i16, z: i16) {
    let x = 255 * x;
    let y = 255 * y;
    let z = 255 * z;
    print!("{} {} {}\n", x, y, z);
}
