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
    let rect = Rect::new((60.0, 90.0), 60.0);
    println!("Координаты нижнего угла: {:?}", rect.bottom_right());
    println!("Площадь равна {}", rect.area());
    println!("Периметр равен {}", rect.perimeter());
}

#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn bottom_right_assert() {
        let rect = Rect::new((1.0, 1.0), 1.0);
        assert_eq!(rect.bottom_right(), (2.0 as f32, 0.0 as f32));
    }

    #[test]
    fn area_assert() {
        let rect = Rect::new((10.0, 15.0), 10.0);
        assert_eq!(rect.area(), 100.0 as f32);
    }

    #[test]
    fn perimeter_assert() {
        let rect = Rect::new((10.0, 15.0), 10.0);
        assert_eq!(rect.perimeter(), 40.0 as f32);
    }



}
