
// 定义一个可以计算面积的 trait
pub trait Area {
    fn area(&self) -> f64;
}

// 实现 Circle 类型，并为其实现 Area trait
pub struct Circle {
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 实现 Triangle 类型，并为其实现 Area trait
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 实现 Square 类型，并为其实现 Area trait
pub struct Square {
    pub side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 打印形状的面积
pub fn print_area<T: Area>(shape: &T) {
    println!("Area of the shape is {}", shape.area());
}
