pub fn test1() {
    // Creates a mutable i32 vector using macro
    let mut v1 = vec![1, 2, 3];
    println!("Vec1: {:?} len:{}", v1, v1.len());
    print_vi32(&v1);

    // Get a mutable reference to the first element and changes it
    let r0 = &mut v1[0];
    *r0 = 10;
    print_vi32(&v1);

    // Get a mutable reference to the second element and changes it
    let r1 = &mut v1[1];
    *r1 = 20;
    print_vi32_indexed(&v1);

    // Second mutable reference to the same second element !!!!!!
    let r1b = &mut v1[1];
    *r1b = 300;

    // Insert new element
    v1.push(4);
    //*r1b = 400;   // THIS WOULD NOT COMPILE

    // Change third element directly
    v1[2] = 30;
    print_vi32_indexed(&v1);

    // Iterate and changes elements
    for el in &mut v1 {
        *el *= 2;
    }
    print_vi32_indexed(&v1);
}

pub fn test2() {
    // Create new empty mutable vector of f32
    let mut v2: Vec<f32> = Vec::new();

    for i in 0..10 {
        v2.push(i as f32);
    }
    println!("Vec2: {:?} len:{}", v2, v2.len());

    // Iterate and changes elements
    for el in &mut v2 {
        *el *= 10.0;
    }
    println!("Vec2: {:?} len:{}", v2, v2.len());
    println!();
}

pub fn test3() {
    // Creates empty vector of Point{}
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }
    let mut v3: Vec<Point> = Vec::new();

    // Insert initial values
    for i in 0..10 {
        v3.push(Point {
            x: i as f32,
            y: i as f32,
        });
    }
    println!("Vec3: {:?} len:{}", v3, v3.len());

    // Iterates over the vector and change the Points fields
    for p in &mut v3 {
        p.x += 1.0;
        p.y += 2.0;
    }
    println!("Vec3: {:?} len:{}", v3, v3.len());
    println!();
}

pub fn test4() {
    // Creates enum of possible Events
    #[derive(Debug)]
    enum Event {
        Click(f32, f32),
        Key(i32),
        Resize(i32, i32),
    }

    // Creates vector of Events
    let v4 = vec![
        Event::Click(10.0, 30.2),
        Event::Key(12),
        Event::Key(13),
        Event::Resize(800, 600),
    ];
    println!("Vec4: {:?} len:{}", v4, v4.len());

    for ev in &v4 {
        match ev {
            Event::Click(x, y) => println!("Click {}/{}", x, y),
            Event::Key(k) => println!("Key {}", k),
            Event::Resize(w, h) => println!("Resize {}/{}", w, h),
        }
    }
}

// Print vector elements without indices
fn print_vi32(v: &[i32]) {
    for el in v {
        print!("{el} ");
    }
    println!();
}

// Print vector elements with indices
fn print_vi32_indexed(v: &[i32]) {
    for i in 0..v.len() {
        let el = v[i];
        print!("{i}:{el} ");
    }
    println!();
}
