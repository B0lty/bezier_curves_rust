use minifb::{Key, Window, WindowOptions, MouseMode, MouseButton};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Bezier Curves Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    // Creates x and y variables
    let mut x: usize;
    let mut y: usize;

    let mut mouse_pos: Vec<f32> = vec![0.0, 0.0];
    
    // Define the color (Format: 0x00RRGGBB)
    let red_color = 0x00FF0000; 
    let green_color = 0x0000FF00; 

    // Define the positions of the four control points for the Bezier curve
    let mut circle_pos_arr: Vec<Vec<f32>> = vec![vec![10.0, 10.0], vec![10.0, 10.0], vec![10.0, 10.0], vec![10.0, 10.0], vec![10.0, 10.0]];

    let mut selected_circle: i32 = -1;

    // Bezier curve resolution
    let bezier_resolution = 2000;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // for i in buffer.iter_mut() {
        //     *i = 22; // write something more funny here!
        // }

        // gets the mouse position
        window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
            mouse_pos[0] = mouse.0 as f32;
            mouse_pos[1] = mouse.1 as f32;
        });


        // resets the buffer to black
        buffer = vec![0; WIDTH * HEIGHT];

        let mut vec_pt: Vec<f32>;
        let mut t: f32;

        // Draw bezier curve
        for i in 0..bezier_resolution {
            t = i as f32/bezier_resolution as f32;

            vec_pt = get_pt_of_nth_degree_bezier(&t, &circle_pos_arr);

            x = vec_pt[0] as usize;
            y = vec_pt[1] as usize;


            if x < WIDTH && y < HEIGHT {
                let index = (y * WIDTH) + x;
                buffer[index] = red_color;
            }
        }


        // only runs the code when the mouse is down
        if window.get_mouse_down(MouseButton::Left) {
            for i in 0..circle_pos_arr.len() {
                if selected_circle == -1 {
                    if is_pt_in_circle(mouse_pos.clone(), circle_pos_arr[i].clone(), 10.0) {
                        circle_pos_arr[i] = mouse_pos.clone();
                        selected_circle = i as i32;
                        println!("{}, {}", circle_pos_arr[i][0], circle_pos_arr[i][1])
                    }
                }
            }

            if selected_circle >= 0 {
                circle_pos_arr[selected_circle as usize] = mouse_pos.clone();
            }
        } else {
            selected_circle = -1;
        };

        // Draw a circle around the mouse position
        let mut vec_pt_circle: Vec<f32>;
        
        for i in 0..circle_pos_arr.len() {
            for angle in 1..360 {
                let a = angle as f32;
                vec_pt_circle = get_circle_pt(a, 10.0, circle_pos_arr[i].clone());

                x = vec_pt_circle[0] as usize;
                y = vec_pt_circle[1] as usize;

                if x < WIDTH && y < HEIGHT {
                    let index = (y * WIDTH) + x;
                    buffer[index] = green_color;
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}



// fn get_bezier_curve_pt(t: &f32, pt0: &Vec<f32>, pt1: &Vec<f32>, pt2: &Vec<f32>, pt3: &Vec<f32>) -> Vec<f32> {
//     let mut v_return = vec!(0.0, 0.0);

//     v_return[0] = (1.0-t).powf(3.0)*pt0[0] + 3.0*(1.0-t).powf(2.0)*t*pt1[0] + 3.0*(1.0-t)*t.powf(2.0)*pt2[0] + t.powf(3.0)*pt3[0];
//     v_return[1] = (1.0-t).powf(3.0)*pt0[1] + 3.0*(1.0-t).powf(2.0)*t*pt1[1] + 3.0*(1.0-t)*t.powf(2.0)*pt2[1] + t.powf(3.0)*pt3[1];

//     return v_return
// }

fn get_circle_pt(theta: f32, radius: f32, pos: Vec<f32>) -> Vec<f32> {
    let mut v_return = vec!(0.0, 0.0);

    let angle_rad = theta * (3.141592653589/180.0);

    v_return[0] = pos[0] - radius*(angle_rad as f32).cos();
    v_return[1] = pos[1] - radius*(angle_rad as f32).sin();

    return v_return
}

fn is_pt_in_circle(pt: Vec<f32>, circle_pos_arr: Vec<f32>, radius: f32) -> bool {
    let dx = pt[0] - circle_pos_arr[0];
    let dy = pt[1] - circle_pos_arr[1];

    return (dx*dx + dy*dy) <= radius*radius;
}

fn get_pt_of_nth_degree_bezier(t: &f32, arr_pts: &Vec<Vec<f32>>) -> Vec<f32> {
    let mut v_return = vec![0.0, 0.0];
    let n = arr_pts.len();

    for i in 0..n {
        v_return[0] += binomial_coefficient(n as u64, i as u64) as f32 * (1.0 - t).powf((n - i as usize) as f32) * t.powf(i as f32) * arr_pts[i][0];
        v_return[1] += binomial_coefficient(n as u64, i as u64) as f32 * (1.0 - t).powf((n - i as usize) as f32) * t.powf(i as f32) * arr_pts[i][1];
    };

    return v_return
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn binomial_coefficient(n: u64, k: u64) -> u64 {
    return factorial(n) / (factorial(k) * factorial(n - k));
}