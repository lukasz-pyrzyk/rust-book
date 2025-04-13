fn main() {
    //let width1 = 30;
    //let height1 = 50;

    // let area = area((width1, height1));

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::new(30, 50);

    let area = rect.area();
    println!("The area of the rectangle is {area} square pixels.");


}

// fn area(rectangle: &Rectangle) -> u32 {
   // rectangle.width * rectangle.height
//}

//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

// fn area(dimensions: (u32, u32)) -> u32 {
   // dimensions.0 * dimensions.1
//}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn new(width: u32, height: u32) -> Rectangle{
        Rectangle {
            width,
            height,
        }
    }
}