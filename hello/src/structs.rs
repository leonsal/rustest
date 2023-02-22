
#[derive(Debug, Clone)]
pub struct Vec2f32 {
    pub x: f32,
    pub y: f32,
}

impl Vec2f32 {

    const ZERO: Vec2f32 = Vec2f32{x:0., y:0.};

    pub fn new(x: f32, y: f32) -> Vec2f32 {

        Vec2f32{x, y}
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn add_vec2(&mut self, other: &Vec2f32)  {
        self.x += other.x;
        self.y += other.y;
    }
}

pub fn test1() {
    let mut v1 = Vec2f32::new(1.0, 2.0);
    println!("v1.norm:{}", v1.norm());

    let v2 = Vec2f32::new(2.0, 5.0);
    v1.add_vec2(&v2);
    //println!("v1:{}", v1);
}

//pub struct Vec2<T> {
//    pub x: T ,
//    pub y: T,
//}
//
//impl<T> Vec2<T> {
//
//    pub fn new(x: T, y: T) -> Vec2<T> {
//
//        Vec2{x, y}
//    }
//
//    pub fn norm(&self) -> T {
//        (self.x * self.x + self.y * self.y).sqrt()
//    }
//
//}
//
//pub fn test2() {
//
//
//
//
//}


