
use libc::{c_int, size_t};

#[repr(C)]
#[derive(Debug)]
struct Point {
 	x: f32,
	y: f32,
}

#[repr(C)]
#[derive(Debug)]
struct PointList {
    count: size_t,
    points: *const Point,
}

extern "C" {
    fn foo_none();
    fn foo_sum_ints(a: c_int, b: c_int) -> c_int;
    fn foo_sum_vec(pv: *const c_int, count: size_t) -> c_int;
    fn foo_print_points(points: *const Point, count: size_t);
    fn foo_mult_points(points: *mut Point, count: size_t);
    fn foo_point_list(pl: *const PointList);
}


fn main() {
    println!("Hello, FFI!");

    unsafe {
        foo_none();

        let res = foo_sum_ints(10, 20);
        println!("foo_sum_ints:{}", res);

        let v = vec![10, 20, 30];
        let res = foo_sum_vec(v.as_ptr(), v.len());
        println!("foo_sum_vec:{}", res);

        let points = vec![
            Point{x:10.0, y:20.0},
            Point{x:-2.0, y:40.0},
            Point{x:-45.2, y:90.4},
        ];
        foo_print_points(points.as_ptr(), points.len());

        let mut points2 = vec![
            Point{x:1.0, y:2.0},
            Point{x:3.0, y:4.0},
            Point{x:5.0, y:6.0},
        ];
        foo_mult_points(points2.as_mut_ptr(), points.len());
        for p in &points2 {
            println!("x:{} y:{}", p.x, p.y);
        }

        let pl = PointList{
            count: points.len(),
            points: points.as_ptr(),
        };
        foo_point_list(&pl);

        // let v2 = vec![10, 20, 30];
        // let res = cbar_sum_vec(v2.as_ptr(), v2.len());
        // println!("cbar_sum_vec:{}", res);
    }
}
