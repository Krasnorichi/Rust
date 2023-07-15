struct Rect {
    top_left: (f32, f32),
    width: f32,
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Rect {
        Rect { top_left, width }
    }

    fn bottom_right(&self) -> (f32, f32) {
        let (x, y) = self.top_left;
        let bottom_right = (x + self.width, y - self.width);
        bottom_right
    }

    fn area(&self) -> f32 {
        let area = self.width * self.width;
        area
    }

    fn perimeter(&self) -> f32 {
        let perimeter = 4.0 * self.width;
        perimeter
    }
    
}

fn main() {
    let adam_rect = Rect::new((60.0, 90.0), 60.0);
    println!("Координаты нижнего угла: {:?}", adam_rect.bottom_right());
    println!("Площадь равна {}", adam_rect.area());
    println!("Периметр равен {}", adam_rect.perimeter());
}
