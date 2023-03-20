mod fourth_class;

use fourth_class::sigal_light::Duration;
use fourth_class::sigal_light::TrafficLight;
use fourth_class::sum::sum_u32;
use fourth_class::calcu_space::Square;
use fourth_class::calcu_space::Circle;
use fourth_class::calcu_space::Triangle;
use fourth_class::calcu_space::print_area;
fn main() {
    /*信号灯 */
    // let red_light = TrafficLight::Red;
    // println!("Red light lasts for {} seconds", red_light.duration());

    // let yellow_light = TrafficLight::Yellow;
    // println!("Yellow light lasts for {} seconds", yellow_light.duration());

    // let green_light = TrafficLight::Green;
    // println!("Green light lasts for {} seconds", green_light.duration());

    /* 求和*/
    // let values = [1, 2, 3, u32::MAX - 1, 5];
    // match sum_u32(&values) {
    //     Some(sum) => println!("Sum is {}", sum),
    //     None => println!("Overflow occurred"),
    // }
    /*计算面积 */
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 6.0, height: 8.0 };
    let square = Square { side: 4.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}
