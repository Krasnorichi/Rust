struct MyVector {
    items: [i32; 100], // массив фиксированного размера
    length: usize,     // текущее количество элементов
}

impl MyVector {
    // создаем новый "вектор"
    fn new() -> MyVector {
        MyVector {
            items: [0; 100], // начальное значение для каждого элемента
            length: 0,       // начинаем с 0 элементов
        }
    }

    // добавляем элемент в "вектор"
    fn push(&mut self, item: i32) {
        if self.length < 100 {
            self.items[self.length] = item;
            self.length += 1;
        } else {
            println!("Vector is full!");
        }
    }

    // выводим "вектор" на экран
    fn print(&self) {
        for i in 0..self.length {
            println!("{}", self.items[i]);
        }
    }
}

fn main() {
    let mut my_vector = MyVector::new();
    my_vector.push(10);
    my_vector.push(20);
    my_vector.push(30);

    my_vector.print();
}