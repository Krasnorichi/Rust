struct Rect {
    top_left: (f32, f32),
    width: f32,
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Rect {
        Rect {
            top_left,
            width,
        }
    }

    fn bottom_right(&self) -> (f32, f32) {
        let (x, y) = self.top_left;
        (x + self.width, y + self.width)
    }

    fn area(&self) -> f32 {
        self.width * self.width
    }

    fn perimeter(&self) -> f32 {
        4.0 * self.width
    }
}

fn main() {
    let rect = Rect::new((0.0, 0.0), 5.0);
    println!("Bottom right corner: {:?}", rect.bottom_right());
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
}


