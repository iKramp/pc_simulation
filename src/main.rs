extern crate sdl2;
/*const WIDTH: u32 = 750;
const HEIGHT: u32 = 400;*/
const WIDTH: u32 = 750;
const HEIGHT: u32 = 400;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::EventPump;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Clone, Copy)]
enum ComponentType {NOTHING = 0, WRITE_TO_WIRE = 1, WIRE = 2, CROSS = 3, READ_FROM_WIRE = 4, BUFFER = 5, AND = 6, OR = 7, XOR = 8, NOT = 9, NAND = 10, NOR = 11, XNOR = 12, COMMENT = 13, NUM_COMPONENTS = 14}
impl ComponentType{
    fn from_u32(val: u32) -> ComponentType{
        match val {
            1 => ComponentType::WRITE_TO_WIRE,
            2 => ComponentType::WIRE,
            3 => ComponentType::CROSS,
            4 => ComponentType::READ_FROM_WIRE,
            5 => ComponentType::BUFFER,
            6 => ComponentType::AND,
            7 => ComponentType::OR,
            8 => ComponentType::XOR,
            9 => ComponentType::NOT,
            10 => ComponentType::NAND,
            11 => ComponentType::NOR,
            12 => ComponentType::XNOR,
            13 => ComponentType::COMMENT,
            _ => ComponentType::NOTHING,
        }
    }
}
const COLORS: [((u8, u8, u8), (u8, u8, u8)); 14] =
    [((31, 37, 49), (31, 37, 49)),
    ((85, 62, 71), (255, 113, 113)),
    ((99, 97, 79), (251, 251, 74)),
    ((112, 131, 162), (121, 140, 168)),
    ((51, 78, 107), (119, 202, 255)),
    ((68, 85, 71), (168, 255, 121)),
    ((85, 67, 71), (255, 222, 122)),
    ((62, 82, 99), (121, 255, 255)),
    ((77, 68, 100), (199, 139, 255)),
    ((94, 69, 85), (255, 112, 163)),
    ((94, 72, 59), (255, 184, 0)),
    ((35, 73, 101), (58, 241, 255)),
    ((74, 52, 101), (189, 0, 255)),
    ((67, 72, 79), (67, 72, 79))
    ];

#[derive(Clone, Copy)]
struct Component{
    component_type: ComponentType,
    turned_on: bool,
    updated: bool,
    previous_state: bool,
}

struct ComponentData{
    array: Vec<[Component; HEIGHT as usize]>,
    to_update: Vec<(usize, usize)>
}


fn get_color(component: &Component) -> (u8, u8, u8){
    if component.turned_on {
        COLORS[component.component_type as usize].1
    }else {
        COLORS[component.component_type as usize].0
    }
}

fn draw_component(x: usize, y: usize, component_type_: ComponentType, turned_on_: bool, component_data: &mut ComponentData){
    component_data.array[x][y] = Component{
        component_type: component_type_,
        turned_on: turned_on_,
        updated: false,
        previous_state: !turned_on_
    };
}

fn calculate_rgb(h:u16, s:f32, v:f32) -> (u8, u8, u8){
    let c:f32 = v * s;
    let x:f32 = c * (1 as f32 - ((h as f32 / 60.0) % 2 as f32 - 1 as f32).abs());
    let m:f32 = v - c;
    let color: (f32, f32, f32);
    if h < 60 {
        color = (c, x, 0.0);
    }else if h < 120 {
        color = (x, c, 0.0);
    }else if h < 180 {
        color = (0.0, c, x);
    }else if h < 240 {
        color = (0.0, x, c);
    }else if h < 300 {
        color = (x, 0.0, c);
    }else{
        color = (c, 0.0, x);
    }
    (((color.0 + m) * 255.0).round() as u8, ((color.1 + m) * 255.0).round() as u8, ((color.2 + m) * 255.0).round() as u8)
}

pub fn main() {
    let mut component_data = ComponentData{
        array: vec![[Component{component_type: ComponentType::NOTHING, turned_on: false, updated: false, previous_state: true}; HEIGHT as usize]; WIDTH as usize],
        to_update: vec![]
    };
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("pc sim", WIDTH * 2, HEIGHT * 2)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    draw_canvas(0.0, 0.0, 1.0, &mut component_data, & mut canvas);
    main_update(&mut canvas, &mut event_pump, &mut component_data)
}

fn main_update(mut canvas: &mut WindowCanvas, event_pump: &mut EventPump, mut component_data: &mut ComponentData){
    let mut selected_type = ComponentType::WIRE;
    let mut run_sim = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Plus), ..} => {
                    selected_type = ComponentType::from_u32(selected_type as u32 % (ComponentType::NUM_COMPONENTS as u32 - 1) + 1);
                },
                Event::KeyDown {keycode: Some(Keycode::Minus), ..} => {
                    if selected_type as u32 == 1{
                        selected_type = ComponentType::from_u32(ComponentType::NUM_COMPONENTS as u32 - 1);
                    }else {
                        selected_type = ComponentType::from_u32(selected_type as u32 - 1);
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::Space), ..} => {
                    run_sim = !run_sim;
                }
                _ => {}
            }
        }
        if run_sim {
            update_canvas(&mut component_data);
        }else{
            if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
                let x = event_pump.mouse_state().x() / 2;
                let y = event_pump.mouse_state().y() / 2;
                for i in std::cmp::max(x - 5, 0)..std::cmp::min(x + 5, WIDTH as i32) {
                    for j in std::cmp::max(y - 5, 0)..std::cmp::min(y + 5, HEIGHT as i32) {
                        draw_component(i as usize, j as usize, selected_type, false, &mut component_data);
                    }
                }
            }
            if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Right) {
                let x = event_pump.mouse_state().x() / 2;
                let y = event_pump.mouse_state().y() / 2;
                for i in std::cmp::max(x - 5, 0)..std::cmp::min(x + 5, WIDTH as i32) {
                    for j in std::cmp::max(y - 5, 0)..std::cmp::min(y + 5, HEIGHT as i32) {
                        draw_component(i as usize, j as usize, ComponentType::NOTHING, false, &mut component_data);
                    }
                }
            }
        }
        draw_canvas(0.0, 0.0, 1.0, &mut component_data, canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
    }
}

fn draw_canvas(x:f32, y:f32, zoom: f32, component_data: &mut ComponentData, canvas: &mut WindowCanvas){
    for column in component_data.array.iter_mut().enumerate(){
        for point in column.1.iter_mut().enumerate(){
            if point.1.turned_on != point.1.previous_state {
                let color = get_color(&point.1);
                canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
                canvas.draw_rect(Rect::new((column.0 * 2) as i32, (point.0 * 2) as i32, 2, 2)).expect("couldn't draw a rect");
                point.1.previous_state = point.1.turned_on;
            }
        }
    }
}

fn update_canvas(mut component_data: &mut ComponentData){
    for i in 0..WIDTH{
        for j in 0..HEIGHT{
            if component_data.array[i as usize][j as usize].component_type as u32 == ComponentType::READ_FROM_WIRE as u32 {
                component_data.array[i as usize][j as usize].turned_on = false;
            }
            component_data.array[i as usize][j as usize].updated = false;
        }
    }
    update_read(&mut component_data);
    //update gate
    for i in 0..WIDTH{
        for j in 0..HEIGHT{
            if component_data.array[i as usize][j as usize].component_type as u32 == ComponentType::WIRE as u32 || component_data.array[i as usize][j as usize].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32 {
                component_data.array[i as usize][j as usize].turned_on = false;
            }
        }
    }
    update_write(&mut component_data);
    update_wire(&mut component_data);
}

fn update_read(mut component_data: &mut ComponentData){
    for i in 0..WIDTH as i32{
        for j in 0..HEIGHT as i32{
            if component_data.array[i as usize][j as usize].component_type as u32 == ComponentType::READ_FROM_WIRE as u32{
                let before = component_data.array[i as usize][j as usize].turned_on;
                let turned_on =
                    turns_read_on(&component_data, i + 1, j) ||
                    turns_read_on(&component_data, i - 1, j) ||
                    turns_read_on(&component_data, i, j + 1) ||
                    turns_read_on(&component_data, i, j - 1);
                if before != turned_on {
                    draw_component(i as usize, j as usize, component_data.array[i as usize][j as usize].component_type, turned_on, component_data)
                }
            }
        }
    }
}

fn turns_read_on(component_data: &ComponentData, x: i32, y: i32) -> bool{
    if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32{
        return false;
    }
    (component_data.array[x as usize][y as usize].component_type as u32 == ComponentType::WIRE as u32 || component_data.array[x as usize][y as usize].component_type as u32 == ComponentType::READ_FROM_WIRE as u32) && component_data.array[x as usize][y as usize].turned_on == true
}

fn update_wire(mut component_data: &mut ComponentData){
    for i in 0..WIDTH as i32{
        for j in 0..HEIGHT as i32{
            if component_data.array[i as usize][j as usize].component_type as u32 == ComponentType::WIRE as u32{
                let before = component_data.array[i as usize][j as usize].turned_on;
                let turned_on =
                    turns_wire_on(&component_data, i + 1, j) ||
                    turns_wire_on(&component_data, i - 1, j) ||
                    turns_wire_on(&component_data, i, j + 1) ||
                    turns_wire_on(&component_data, i, j - 1);
                if before != turned_on {
                    draw_component(i as usize, j as usize, component_data.array[i as usize][j as usize].component_type, turned_on, component_data)
                }
            }
        }
    }
}

fn turns_wire_on(component_data: &ComponentData, x: i32, y: i32) -> bool{
    if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32{
        return true;
    }
    (/*component_data.array[x as usize][y as usize].component_type as u32 == ComponentType::WIRE as u32 || */component_data.array[x as usize][y as usize].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32) && component_data.array[x as usize][y as usize].turned_on == true
}

fn update_write(mut component_data: &mut ComponentData){
    for i in 0..WIDTH as i32{
        for j in 0..HEIGHT as i32{
            if component_data.array[i as usize][j as usize].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32{
                let before = component_data.array[i as usize][j as usize].turned_on;
                let turned_on =
                    turns_write_on(&component_data, i + 1, j) ||
                    turns_write_on(&component_data, i - 1, j) ||
                    turns_write_on(&component_data, i, j + 1) ||
                    turns_write_on(&component_data, i, j - 1);
                if before != turned_on {
                    draw_component(i as usize, j as usize, component_data.array[i as usize][j as usize].component_type, turned_on, component_data)
                }
            }
        }
    }
}

fn turns_write_on(component_data: &ComponentData, x: i32, y: i32) -> bool{
    if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32{
        return true;
    }
    (component_data.array[x as usize][y as usize].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32 ||
    (component_data.array[x as usize][y as usize].component_type as u32 > 4 && (component_data.array[x as usize][y as usize].component_type as u32) < 13)) &&
    component_data.array[x as usize][y as usize].turned_on == true
}

