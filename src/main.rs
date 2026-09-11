use crate::assets::font;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
pub mod assets;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Bezier Curves - ESC to exit",
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
    let blue_color = 0x000000FF;
    let light_grey_colour = 0x00888888;
    let button_bg_colour = 0x00222244;

    // Define the positions of the four control points for the Bezier curve
    let mut circle_pos_arr: Vec<Vec<f32>> = vec![
        vec![100.0, 100.0],
        vec![400.0, 100.0],
        vec![100.0, 300.0],
        vec![400.0, 300.0]
    ];

    let mut selected_circle: i32 = -1;

    // Bezier curve resolution
    let bezier_resolution = 600;
    let control_line_res = 200;

    // Declaring buttons
    let arr_buttons: Vec<(f32, f32, String)> = vec![
        (WIDTH as f32 - 107.0, 10.0, String::from("add point")),
        (WIDTH as f32 - 107.0, 20.0, String::from("remove point"))
    ];

    let mut button_pressed: bool = false;

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

        // Draw lines between control points
        for i in 1..circle_pos_arr.len() {
            let mut vec_pt: Vec<f32>;
            let mut t: f32;

            for j in 0..control_line_res {
                t = j as f32 / control_line_res as f32;

                vec_pt = get_pt_of_nth_degree_bezier(
                    &t,
                    &vec![circle_pos_arr[i - 1].clone(), circle_pos_arr[i].clone()],
                );

                x = vec_pt[0] as usize;
                y = vec_pt[1] as usize;

                if x < WIDTH && y < HEIGHT {
                    let index = (y * WIDTH) + x;
                    buffer[index] = light_grey_colour;
                }
            }
        }

        let mut vec_pt: Vec<f32>;
        let mut t: f32;

        // Draw bezier curve
        for i in 0..bezier_resolution {
            t = i as f32 / bezier_resolution as f32;

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
                    }
                }
            }

            // Detecting button presses, should be moved into a loop in the future
            if is_pt_in_rect(
                    mouse_pos.clone(), 
                    vec![arr_buttons[0].0.clone(), arr_buttons[0].1.clone()], 
                    vec![(arr_buttons[0].2.len() * font::GLYPH_WIDTH-1) as f32, font::GLYPH_HEIGHT as f32]) {          
                if button_pressed == false {
                    let mut new_pt: Vec<Vec<f32>> = vec![vec![10.0, 10.0]];
                    circle_pos_arr.append(&mut new_pt);
                    button_pressed = true;
                }
            } else if is_pt_in_rect(
                    mouse_pos.clone(), 
                    vec![arr_buttons[1].0.clone(), arr_buttons[1].1.clone()], 
                    vec![(arr_buttons[1].2.len() * font::GLYPH_WIDTH-1) as f32, font::GLYPH_HEIGHT as f32]) {
                if button_pressed == false {
                    circle_pos_arr.pop();
                    button_pressed = true;
                }
            }

            if selected_circle >= 0 {
                circle_pos_arr[selected_circle as usize] = mouse_pos.clone();
            }
        } else {
            selected_circle = -1;
            button_pressed = false;
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
                    if i == 0 || i == circle_pos_arr.len() - 1 {
                        buffer[index] = green_color;
                    } else {
                        buffer[index] = blue_color;
                    }
                }
            }
        }

        // GUI Buttons
        for button in &arr_buttons {
            let text = button.2.clone();
            let mut text_x_offset = button.0.clone()  as usize;
            let text_y_offset = button.1.clone()  as usize;

            for c in text.chars() {
                let letter: [[i32; 7]; 9] = get_letter_glyph(c);

                for y in 0..letter.len() {
                    for x in 0..letter[y].len() {
                        if letter[y][x] == 1 {
                            if x + text_x_offset < WIDTH && y + text_y_offset < HEIGHT {
                                let index = ((y + text_y_offset) * WIDTH) + x + text_x_offset;
                                buffer[index] = red_color;
                            }
                        } else {
                            if x + text_x_offset < WIDTH && y + text_y_offset < HEIGHT {
                                let index = ((y + text_y_offset) * WIDTH) + x + text_x_offset;
                                buffer[index] = button_bg_colour;
                            }
                        }
                    }
                }
                text_x_offset += 7;
            }
        }
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn get_circle_pt(theta: f32, radius: f32, pos: Vec<f32>) -> Vec<f32> {
    let mut v_return = vec![0.0, 0.0];

    let angle_rad = theta * (3.141592653589 / 180.0);

    v_return[0] = pos[0] - radius * (angle_rad as f32).cos();
    v_return[1] = pos[1] - radius * (angle_rad as f32).sin();

    return v_return;
}

fn is_pt_in_circle(pt: Vec<f32>, circle_pos_arr: Vec<f32>, radius: f32) -> bool {
    let dx = pt[0] - circle_pos_arr[0];
    let dy = pt[1] - circle_pos_arr[1];

    return (dx * dx + dy * dy) <= radius * radius;
}

fn get_pt_of_nth_degree_bezier(t: &f32, arr_pts: &Vec<Vec<f32>>) -> Vec<f32> {
    let mut v_return = vec![0.0, 0.0];
    let n = arr_pts.len() - 1;

    for i in 0..arr_pts.len() {
        let basis= binomial_coefficient(n as u128, i as u128) as f64
            * (1.0 - t).powf((n - i) as f32) as f64
            * (t.powf(i as f32)) as f64;
        v_return[0] += (basis * arr_pts[i][0] as f64) as f32;
        v_return[1] += (basis * arr_pts[i][1] as f64) as f32;
    }

    return v_return;
}

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

fn binomial_coefficient(n: u128, k: u128) -> u128 {
    return factorial(n) / (factorial(k) * factorial(n - k));
}

fn get_letter_glyph(c: char) -> [[i32; 7]; 9] {
    if c == 'a' {
        return font::A;
    } else if c == 'b' {
        return font::B;
    } else if c == 'c' {
        return font::C;
    } else if c == 'd' {
        return font::D;
    } else if c == 'e' {
        return font::E;
    } else if c == 'f' {
        return font::F;
    } else if c == 'g' {
        return font::G;
    } else if c == 'h' {
        return font::H;
    } else if c == 'i' {
        return font::I;
    } else if c == 'j' {
        return font::J;
    } else if c == 'k' {
        return font::K;
    } else if c == 'l' {
        return font::L;
    } else if c == 'm' {
        return font::M;
    } else if c == 'n' {
        return font::N;
    } else if c == 'o' {
        return font::O;
    } else if c == 'p' {
        return font::P;
    } else if c == 'q' {
        return font::Q;
    } else if c == 'r' {
        return font::R;
    } else if c == 's' {
        return font::S;
    } else if c == 't' {
        return font::T;
    } else if c == 'u' {
        return font::U;
    } else if c == 'v' {
        return font::V;
    } else if c == 'w' {
        return font::W;
    } else if c == 'x' {
        return font::X;
    } else if c == 'y' {
        return font::Y;
    } else if c == 'z' {
        return font::Z;
    } else {
        return font::SPACE;
    }
}

fn is_pt_in_rect(pt: Vec<f32>, rect_pos: Vec<f32>, rect_dims: Vec<f32>) -> bool {
    for x in rect_pos[0] as i32..(rect_dims[0]+rect_pos[0]) as i32 {
        for y in rect_pos[1] as i32..(rect_dims[1]+rect_pos[1]) as i32 {
            if pt == vec![x as f32, y as f32] {
                return true;
            }
        }
    }
    return false
}