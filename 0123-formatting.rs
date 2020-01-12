use std::fmt;

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({r}, {g}, {b}) 0x{r:02X}{g:02X}{b:02X}",
            r = self.r,
            g = self.g,
            b = self.b
        )
    }
}

fn main() {
    for color in [
        Color {
            r: 128,
            g: 255,
            b: 90,
        },
        Color { r: 0, g: 3, b: 254 },
        Color { r: 0, g: 0, b: 0 },
    ]
    .iter()
    {
        println!("{}", color)
    }
}
