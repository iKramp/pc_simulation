extern crate sdl2;
extern crate stopwatch;
const WIDTH: u32 = 700;
const HEIGHT: u32 = 300;
const SIZE: i32 = 0;


use std::fs;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::EventPump;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureAccess, WindowCanvas};
use sdl2::video::WindowContext;

#[derive(Clone, Copy)]
enum ComponentType {NOTHING, WRITE_TO_WIRE, WIRE, CROSS, READ_FROM_WIRE, AND, OR, XOR, NOT, NAND, XNOR, COMMENT, CLOCK, LATCH, NUM_COMPONENTS}
impl ComponentType{
    fn from_u32(val: u32) -> ComponentType{
        match val {
            1 => ComponentType::WRITE_TO_WIRE,
            2 => ComponentType::WIRE,
            3 => ComponentType::CROSS,
            4 => ComponentType::READ_FROM_WIRE,
            5 => ComponentType::AND,
            6 => ComponentType::OR,
            7 => ComponentType::XOR,
            8 => ComponentType::NOT,
            9 => ComponentType::NAND,
            10 => ComponentType::XNOR,
            11 => ComponentType::COMMENT,
            12 => ComponentType::CLOCK,
            13 => ComponentType::LATCH,
            _ => ComponentType::NOTHING,
        }
    }
}
const COLORS: [((u8, u8, u8), (u8, u8, u8)); 14] =
   [((031, 037, 049), (031, 037, 049)),
    ((085, 062, 071), (255, 113, 113)),
    ((099, 097, 079), (251, 251, 074)),
    ((112, 131, 162), (121, 140, 168)),
    ((051, 078, 107), (119, 202, 255)),
    ((085, 076, 071), (255, 222, 123)),
    ((062, 082, 099), (121, 255, 255)),
    ((077, 068, 100), (199, 139, 255)),
    ((094, 069, 085), (255, 112, 163)),
    ((094, 072, 059), (255, 184, 000)),
    ((074, 052, 101), (189, 000, 255)),
    ((067, 072, 079), (067, 072, 079)),
    ((085, 040, 069), (255, 000, 078)),
    ((061, 085, 081), (110, 251, 183))];

const NAMES: [&str; 14] =  [
    "nothing",
    "writer",
    "wire",
    "cross",
    "reader",
    "and",
    "or",
    "xor",
    "not",
    "nand",
    "xnor",
    "comment",
    "clock",
    "latch"
];

#[derive(Clone, Copy)]
struct Component{
    component_type: ComponentType,
    belongs_to: i32,
}

struct WireWriter{
    enabled: bool,
    to_update: bool,
    elements: Vec<(usize, usize)>,
    wires: Vec<u32>,
    logic_gates: Vec<u32>
}

struct LogicGate{
    enabled: bool,
    to_update: bool,
    gate_type: ComponentType,
    elements: Vec<(usize, usize)>,
    wire_writers: Vec<u32>,
    wire_readers: Vec<u32>
}

struct WireReader{
    enabled: bool,
    to_update: bool,
    elements: Vec<(usize, usize)>,
    logic_gates: Vec<u32>,
    wires: Vec<u32>
}

struct Wire{
    enabled: bool,
    to_update: bool,
    elements: Vec<(usize, usize)>,
    wire_readers: Vec<u32>,
    wire_writers: Vec<u32>
}

struct ComponentData{
    array: Vec<[Component; HEIGHT as usize]>,
    to_update: Vec<(usize, usize)>,
    wires: Vec<Wire>,
    wire_readers: Vec<WireReader>,
    wire_writers: Vec<WireWriter>,
    logic_gates: Vec<LogicGate>,
    position_on_screen: (f32, f32),
    zoom: f32,
}


fn get_color(component_data: &ComponentData, component: &Component) -> (u8, u8, u8){
    if component.belongs_to == -1{
        return COLORS[component.component_type as usize].0;
    }

    return if component.component_type as u32 == ComponentType::WIRE as u32 {
        if component_data.wires[component.belongs_to as usize].enabled {
            COLORS[ComponentType::WIRE as usize].1
        } else {
            COLORS[ComponentType::WIRE as usize].0
        }
    } else if component.component_type as u32 == ComponentType::WRITE_TO_WIRE as u32 {
        if component_data.wire_writers[component.belongs_to as usize].enabled {
            COLORS[ComponentType::WRITE_TO_WIRE as usize].1
        } else {
            COLORS[ComponentType::WRITE_TO_WIRE as usize].0
        }
    } else if component.component_type as u32 == ComponentType::READ_FROM_WIRE as u32 {
        if component_data.wire_readers[component.belongs_to as usize].enabled {
            COLORS[ComponentType::READ_FROM_WIRE as usize].1
        } else {
            COLORS[ComponentType::READ_FROM_WIRE as usize].0
        }
    } else {
        if component_data.logic_gates[component.belongs_to as usize].enabled {
            COLORS[component.component_type as usize].1
        } else {
            COLORS[component.component_type as usize].0
        }
    }
}

fn draw_component(x: usize, y: usize, component_type_: ComponentType, turned_on_: bool, component_data: &mut ComponentData, canvas: &mut WindowCanvas){
    component_data.array[x][y].component_type = component_type_;
    let color;
    if !turned_on_ {
        color = COLORS[component_type_ as usize].0;
    }else {
        color = COLORS[component_type_ as usize].1;
    }

    canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
    canvas.fill_rect(Rect::new(((x as f32 * component_data.zoom + component_data.position_on_screen.0) * 2.0).round() as i32, ((y as f32 * component_data.zoom + component_data.position_on_screen.1.round()) * 2.0).round() as i32, (component_data.zoom * 2.0).round() as u32, (component_data.zoom * 2.0).round() as u32)).expect("failed to draw rect");
}

fn draw_canvas(component_data: &mut ComponentData, canvas: &mut WindowCanvas, sim_view: bool){
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let color = COLORS[0].0;
    canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
    canvas.fill_rect(Rect::new(component_data.position_on_screen.0.round() as i32 * 2, component_data.position_on_screen.1.round() as i32 * 2, (WIDTH as f32 * component_data.zoom * 2.0) as u32, (HEIGHT as f32 * component_data.zoom * 2.0) as u32)).expect("failed to draw");
    if sim_view {
        draw_canvas_components(component_data, canvas);
    }else {
        draw_canvas_pixels(component_data, canvas);
    }
}

fn draw_canvas_components(component_data: &mut ComponentData, canvas: &mut WindowCanvas){
    for i in 0..component_data.wires.len(){
        for j in 0..component_data.wires[i].elements.len(){
            canvas.set_draw_color(get_color(&component_data, &component_data.array[component_data.wires[i].elements[j].0][component_data.wires[i].elements[j].1]));
            draw_component(component_data.wires[i].elements[j].0, component_data.wires[i].elements[j].1, ComponentType::WIRE, component_data.wires[i].enabled, component_data, canvas)
        }
    }
    for i in 0..component_data.wire_readers.len(){
        for j in 0..component_data.wire_readers[i].elements.len(){
            canvas.set_draw_color(get_color(&component_data, &component_data.array[component_data.wire_readers[i].elements[j].0][component_data.wire_readers[i].elements[j].1]));
            draw_component(component_data.wire_readers[i].elements[j].0, component_data.wire_readers[i].elements[j].1, ComponentType::READ_FROM_WIRE, component_data.wire_readers[i].enabled, component_data, canvas)
        }
    }
    for i in 0..component_data.wire_writers.len(){
        for j in 0..component_data.wire_writers[i].elements.len(){
            canvas.set_draw_color(get_color(&component_data, &component_data.array[component_data.wire_writers[i].elements[j].0][component_data.wire_writers[i].elements[j].1]));
            draw_component(component_data.wire_writers[i].elements[j].0, component_data.wire_writers[i].elements[j].1, ComponentType::WRITE_TO_WIRE, component_data.wire_writers[i].enabled, component_data, canvas)
        }
    }
    for i in 0..component_data.logic_gates.len(){
        for j in 0..component_data.logic_gates[i].elements.len(){
            canvas.set_draw_color(get_color(&component_data, &component_data.array[component_data.logic_gates[i].elements[j].0][component_data.logic_gates[i].elements[j].1]));
            draw_component(component_data.logic_gates[i].elements[j].0, component_data.logic_gates[i].elements[j].1, component_data.array[component_data.logic_gates[i].elements[j].0][component_data.logic_gates[i].elements[j].1].component_type, component_data.logic_gates[i].enabled, component_data, canvas)
        }
    }
}

fn draw_canvas_pixels(component_data: &mut ComponentData, canvas: &mut WindowCanvas){
    for i in 0..component_data.array.len(){
        for j in 0..component_data.array[0].len(){
            if component_data.array[i][j].component_type as u32 != ComponentType::NOTHING as u32 {
                let color = get_color(component_data, &component_data.array[i][j]);
                canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
                canvas.fill_rect(Rect::new(((i as f32 * component_data.zoom + component_data.position_on_screen.0) * 2.0).round() as i32, ((j as f32 * component_data.zoom + component_data.position_on_screen.1.round()) * 2.0).round() as i32, (component_data.zoom * 2.0).round() as u32, (component_data.zoom * 2.0).round() as u32)).expect("failed to draw rect");
            }
        }
    }
}

pub fn main() {
    let mut component_data = ComponentData{
        array: vec![[Component{component_type: ComponentType::NOTHING, belongs_to: -1}; HEIGHT as usize]; WIDTH as usize],
        to_update: vec![],
        wires: vec![],
        wire_readers: vec![],
        wire_writers: vec![],
        logic_gates: vec![],
        position_on_screen: (0.0, 0.0),
        zoom: 1.0
    };
    load_canvas(&mut component_data);
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

    draw_canvas(&mut component_data, & mut canvas, false);
    main_update(&mut canvas, &mut event_pump, &mut component_data);

    save_canvas(&component_data);
}

fn main_update(mut canvas: &mut WindowCanvas, event_pump: &mut EventPump, mut component_data: &mut ComponentData){
    let mut selected_type = ComponentType::WIRE;
    let mut run_sim = false;
    let stopwatch = stopwatch::Stopwatch::start_new();
    let mut last_time = stopwatch.elapsed_ms();
    let mut last_mouse_x = 0;
    let mut last_mouse_y = 0;
    let mut shift_pressed = false;
    let mut control_pressed = false;
    let mut copy = false;
    let mut paste = (false, (0, 0));
    let mut selection: ((i32, i32), (i32, i32)) = ((0, 0), (0, 0));
    let mut copied_data: Vec<Vec<u8>> = vec![];
    'running: loop {
        shift_pressed = event_pump.keyboard_state().is_scancode_pressed(Scancode::LShift);
        control_pressed = event_pump.keyboard_state().is_scancode_pressed(Scancode::LCtrl);
        let mouse_x = event_pump.mouse_state().x() / 2;
        let mouse_y = event_pump.mouse_state().y() / 2;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    if paste.0  {
                        paste.0 = false;
                    }else {
                        break 'running
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::Plus), ..} => {
                    selected_type = ComponentType::from_u32(selected_type as u32 % (ComponentType::NUM_COMPONENTS as u32 - 1) + 1);
                    println!("{}", NAMES[selected_type as usize]);
                },
                Event::KeyDown {keycode: Some(Keycode::Minus), ..} => {
                    if selected_type as u32 == 1{
                        selected_type = ComponentType::from_u32(ComponentType::NUM_COMPONENTS as u32 - 1);
                    }else {
                        selected_type = ComponentType::from_u32(selected_type as u32 - 1);
                    }
                    println!("{}", NAMES[selected_type as usize]);
                },
                Event::KeyDown {keycode: Some(Keycode::Space), ..} => {
                    if run_sim {
                        clear_compiled_data(component_data);
                    }else{
                        paste.0 = false;
                        compile_scene(component_data);
                    }
                    run_sim = !run_sim;
                }
                Event::MouseButtonDown {mouse_btn: MouseButton::Left, ..} => {
                    if paste.0{
                        paste_selection(&mut component_data, &mut copied_data, paste.1.0, paste.1.1);
                    }else {
                        if run_sim {
                            let pos = translate_mouse_pos(component_data.position_on_screen.0, component_data.position_on_screen.1, component_data.zoom, mouse_x as f32, mouse_y as f32);
                            if component_data.array[pos.0 as usize][pos.1 as usize].component_type as u32 == ComponentType::LATCH as u32 && component_data.array[pos.0 as usize][pos.1 as usize].belongs_to != -1 {
                                component_data.logic_gates[component_data.array[pos.0 as usize][pos.1 as usize].belongs_to as usize].enabled = !component_data.logic_gates[component_data.array[pos.0 as usize][pos.1 as usize].belongs_to as usize].enabled;
                                for writer in &component_data.logic_gates[component_data.array[pos.0 as usize][pos.1 as usize].belongs_to as usize].wire_writers {
                                    component_data.wire_writers[*writer as usize].to_update = true;
                                }
                            }
                        } else if shift_pressed {
                            copy = true;
                            selection.0 = translate_mouse_pos(component_data.position_on_screen.0, component_data.position_on_screen.1, component_data.zoom, mouse_x as f32, mouse_y as f32);
                            selection.1 = selection.0;
                        }
                    }

                }
                Event::MouseButtonDown {mouse_btn: MouseButton::Middle, ..} => {
                    last_mouse_x = mouse_x;
                    last_mouse_y = mouse_y;
                }
                Event::KeyDown {keycode: Some(Keycode::KpMinus), ..} => {
                    let corner_from_center = ((-component_data.position_on_screen.0 * 2.0 + WIDTH as f32), (-component_data.position_on_screen.1 * 2.0 + HEIGHT as f32));
                    component_data.zoom = component_data.zoom / 2.0;
                    component_data.position_on_screen.0 = WIDTH as f32 / 2.0 - (corner_from_center.0 / 4.0);
                    component_data.position_on_screen.1 = HEIGHT as f32 / 2.0 - (corner_from_center.1 / 4.0);
                }
                Event::KeyDown {keycode: Some(Keycode::KpPlus), ..} => {
                    let corner_from_center = ((-component_data.position_on_screen.0 * 2.0 + WIDTH as f32), (-component_data.position_on_screen.1 * 2.0 + HEIGHT as f32));
                    component_data.zoom = component_data.zoom * 2.0;
                    component_data.position_on_screen.0 -= corner_from_center.0 / 2.0;
                    component_data.position_on_screen.1 -= corner_from_center.1 / 2.0;
                }
                Event::MouseButtonUp {mouse_btn: MouseButton::Left, ..} => {
                    if copy {
                        copy = false;
                        prepare_selection(&mut selection);
                        copy_selection(component_data, &mut selection, &mut copied_data);
                    }
                    if(paste.0) {
                        paste.0 = false;
                    }
                }
                Event::KeyDown {keycode: Some(Keycode::V), ..} => {
                    if !run_sim {
                        if control_pressed {
                            paste.0 = !paste.0;
                            paste.1 = translate_mouse_pos(component_data.position_on_screen.0, component_data.position_on_screen.1, component_data.zoom, mouse_x as f32, mouse_y as f32);
                        }
                    }
                }
                _ => {}
            }
        }
        if run_sim /* timed update*/ {
            if stopwatch.elapsed_ms() - last_time > 1 {
                last_time = stopwatch.elapsed_ms();
                update_canvas(&mut component_data);
            }

        } else /*draw mode*/ {
            if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
                let pos = translate_mouse_pos(component_data.position_on_screen.0, component_data.position_on_screen.1, component_data.zoom, event_pump.mouse_state().x() as f32 / 2.0, event_pump.mouse_state().y() as f32 / 2.0);
                if copy{
                    selection.1 = pos;
                }else if !paste.0 {
                    for i in std::cmp::max(pos.0 - SIZE, 0)..std::cmp::min(pos.0 + SIZE + 1, WIDTH as i32) {
                        for j in std::cmp::max(pos.1 - SIZE, 0)..std::cmp::min(pos.1 + SIZE + 1, HEIGHT as i32) {
                            draw_component(i as usize, j as usize, selected_type, false, &mut component_data, canvas);
                        }
                    }
                }
            }
            if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Right) {
                if !paste.0 {
                    let pos = translate_mouse_pos(component_data.position_on_screen.0, component_data.position_on_screen.1, component_data.zoom, event_pump.mouse_state().x() as f32 / 2.0, event_pump.mouse_state().y() as f32 / 2.0);
                    for i in std::cmp::max(pos.0 - SIZE, 0)..std::cmp::min(pos.0 + SIZE + 1, WIDTH as i32) {
                        for j in std::cmp::max(pos.1 - SIZE, 0)..std::cmp::min(pos.1 + SIZE + 1, HEIGHT as i32) {
                            draw_component(i as usize, j as usize, ComponentType::NOTHING, false, &mut component_data, canvas);
                        }
                    }
                }
            }
        }

        if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Middle) /* move canvas*/ {
            let delta = ((mouse_x - last_mouse_x) as f32, (mouse_y - last_mouse_y) as f32);
            component_data.position_on_screen.0 += delta.0;
            component_data.position_on_screen.1 += delta.1;
            last_mouse_x = mouse_x;
            last_mouse_y = mouse_y;
        }
        draw_canvas(component_data, canvas, run_sim);
        let mut pos = translate_mouse_pos(component_data.position_on_screen.0, component_data.position_on_screen.1, component_data.zoom, mouse_x as f32, mouse_y as f32);

        if paste.0 /* draw stuff to paste (hopefully transparent)*/ {
            paste.1 = pos;
            draw_to_paste(&component_data, canvas, &copied_data, paste);
        }

        if !run_sim && !shift_pressed && !paste.0 /*draw drawing cursor square*/ {
            pos.0 = ((pos.0 - SIZE) as f32 * component_data.zoom + component_data.position_on_screen.0) as i32 * 2;
            pos.1 = ((pos.1 - SIZE) as f32 * component_data.zoom + component_data.position_on_screen.1) as i32 * 2;
            let color = COLORS[ComponentType::COMMENT as usize].0;
            canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, 100));
            canvas.fill_rect(Rect::new(pos.0, pos.1, (((SIZE * 4) as f32 + 2.0) * component_data.zoom) as u32, (((SIZE * 4) as f32 + 2.0) * component_data.zoom) as u32)).expect("couldn't draw rect");
        } else if !run_sim /*draw selection*/ {
            if shift_pressed {
                pos = selection.0;
            }
            pos.0 = ((pos.0) as f32 * component_data.zoom + component_data.position_on_screen.0) as i32 * 2;
            pos.1 = ((pos.1) as f32 * component_data.zoom + component_data.position_on_screen.1) as i32 * 2;
            let color = COLORS[ComponentType::COMMENT as usize].0;
            canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, 100));
            canvas.draw_rect(Rect::new(pos.0, pos.1, (((selection.1.0 - selection.0.0) as f32 * 2.0 + 2.0) * component_data.zoom) as u32, (((selection.1.1 - selection.0.1) as f32 * 2.0 + 2.0) * component_data.zoom) as u32)).expect("couldn't draw rect");

        }

        canvas.present();
    }
}

fn draw_to_paste(component_data: &ComponentData, canvas: &mut WindowCanvas, copied_data: &Vec<Vec<u8>>, paste: (bool, (i32, i32))) {
    for i in 0.. copied_data.len(){
        for j in 0..copied_data[i].len(){
            if copied_data[i][j] as u32 != ComponentType::NOTHING as u32 {
                let mut color = COLORS[copied_data[i][j] as usize].0;
                color.0 = (color.0 as f32 * 3.0/4.0) as u8;
                color.1 = (color.1 as f32 * 3.0/4.0) as u8;
                color.2 = (color.2 as f32 * 3.0/4.0) as u8;
                canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, 100));
                canvas.fill_rect(Rect::new((((i as i32 + paste.1.0) as f32 * component_data.zoom + component_data.position_on_screen.0) * 2.0).round() as i32, (((j as i32 + paste.1.1) as f32 * component_data.zoom + component_data.position_on_screen.1.round()) * 2.0).round() as i32, (component_data.zoom * 2.0).round() as u32, (component_data.zoom * 2.0).round() as u32)).expect("failed to draw rect");

            }
        }
    }
}

fn copy_selection(component_data: &ComponentData, selection: &mut ((i32, i32), (i32, i32)), copied_data: &mut Vec<Vec<u8>>) {
    copied_data.resize((selection.1.0 - selection.0.0 + 1) as usize, vec![]);
    for i in 0..(selection.1.0 - selection.0.0 + 1) {
        copied_data[i as usize].resize((selection.1.1 - selection.0.1 + 1) as usize, 0);
        for j in 0..(selection.1.1 - selection.0.1 + 1) {
            copied_data[i as usize][j as usize] = component_data.array[(i + selection.0.0) as usize][(j + selection.0.1) as usize].component_type as u8;
        }
    }
}

fn paste_selection(component_data: &mut ComponentData, copied_data: &mut Vec<Vec<u8>>, paste_x: i32, paste_y: i32) {
    for i in 0.. copied_data.len(){
        for j in 0..copied_data[i].len(){
            if copied_data[i][j] as u32 != ComponentType::NOTHING as u32 && (i as i32) + paste_x >= 0 && (i as i32) + paste_x < WIDTH as i32  && (j as i32) + paste_y >= 0 && (j as i32) + paste_y < HEIGHT as i32 {
                component_data.array[(i as i32 + paste_x) as usize][(j as i32 + paste_y) as usize].component_type = ComponentType::from_u32(copied_data[i][j] as u32);
            }
        }
    }
}

fn prepare_selection(selection: &mut ((i32, i32), (i32, i32))){
    selection.0.0 = std::cmp::min(std::cmp::max(selection.0.0, 0), (WIDTH - 1) as i32);
    selection.0.1 = std::cmp::min(std::cmp::max(selection.0.1, 0), (WIDTH - 1) as i32);
    selection.1.0 = std::cmp::min(std::cmp::max(selection.1.0, 0), (WIDTH - 1) as i32);
    selection.1.1 = std::cmp::min(std::cmp::max(selection.1.1, 0), (WIDTH - 1) as i32);
    if selection.0.0 > selection.1.0{
        let temp = selection.0.0;
        selection.0.0 = selection.1.0;
        selection.1.0 = temp;
    }
    if selection.0.1 > selection.1.1{
        let temp = selection.0.1;
        selection.0.1 = selection.1.1;
        selection.1.1 = temp;
    }
}

fn save_canvas(component_data: &ComponentData) {
    let mut temp_arr: Vec<u8> = vec![];
    for column in component_data.array.iter(){
        for element in column.iter(){
            temp_arr.push(element.component_type as u8)
        }
    }
    fs::write("C:/Users/Uporabnik/CLionProjects/ray_tracer_or_pc_simulation/canvas.dat", temp_arr).expect("couldn't write to file");
}

fn load_canvas(component_data: &mut ComponentData) {
    let temp_arr: Vec<u8> = fs::read("C:/Users/Uporabnik/CLionProjects/ray_tracer_or_pc_simulation/canvas.dat").unwrap();
    if temp_arr.len() != (WIDTH * HEIGHT) as usize {
        return;
    }
    for column in component_data.array.iter_mut().enumerate(){
        for mut element in column.1.iter_mut().enumerate(){
            element.1.component_type = ComponentType::from_u32(temp_arr[(column.0 as u32 * HEIGHT + element.0 as u32) as usize] as u32);
        }
    }
}

fn clear_compiled_data(component_data: &mut ComponentData){
    for i in 0..component_data.array.len(){
        for j in 0..component_data.array[0].len(){
            component_data.array[i][j].belongs_to = -1;
        }
    }
    component_data.wires.clear();
    component_data.wire_writers.clear();
    component_data.wire_readers.clear();
    component_data.logic_gates.clear();
}

fn compile_scene(component_data: &mut ComponentData){
    for i in 0..WIDTH as usize{
        for j in 0..HEIGHT as usize{
            if component_data.array[i][j].component_type as u32 != ComponentType::NOTHING as u32 && component_data.array[i][j].belongs_to == -1{
                new_group(component_data, i, j);
            }
        }
    }
    for i in 0..WIDTH as usize{
        for j in 0..HEIGHT as usize{
            if component_data.array[i][j].component_type as u32 != ComponentType::NOTHING as u32 {
                link_components(component_data, i as i32, j as i32);
            }
        }
    }
}

fn link_components(component_data: &mut ComponentData, x: i32, y: i32){
    let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    if component_data.array[x as usize][y as usize].component_type as u32 == ComponentType::WIRE as u32{
        for direction in directions{
            if (x + direction.0) < 0 ||
                x + direction.0 == WIDTH as i32 ||
                (y + direction.1) < 0 ||
                y + direction.1 > HEIGHT as i32{
                continue;
            }
            if component_data.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type as u32 == ComponentType::READ_FROM_WIRE as u32{
                link_wire_read(component_data, x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
            }
        }
    }
    if component_data.array[x as usize][y as usize].component_type as u32 == ComponentType::READ_FROM_WIRE as u32{
        for direction in directions{
            if (x + direction.0) < 0 ||
                x + direction.0 == WIDTH as i32 ||
                (y + direction.1) < 0 ||
                y + direction.1 > HEIGHT as i32{
                continue;
            }
            if component_data.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type as u32 > 4 && (component_data.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type as u32) < ComponentType::NUM_COMPONENTS as u32{
                link_read_logic(component_data, x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
            }
        }
    }
    if component_data.array[x as usize][y as usize].component_type as u32 > 4 && (component_data.array[x as usize][y as usize].component_type as u32) < ComponentType::NUM_COMPONENTS as u32{
        for direction in directions{
            if (x + direction.0 as i32) < 0 ||
                x + direction.0 as i32 == WIDTH as i32 ||
                (y + direction.1 as i32) < 0 ||
                y + direction.1 as i32 > HEIGHT as i32{
                continue;
            }
            if component_data.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32{
                link_logic_write(component_data, x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
            }
        }
    }
    if component_data.array[x as usize][y as usize].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32{
        for direction in directions{
            if (x + direction.0 as i32) < 0 ||
                x + direction.0 as i32 == WIDTH as i32 ||
                (y + direction.1 as i32) < 0 ||
                y + direction.1 as i32 > HEIGHT as i32{
                continue;
            }
            if component_data.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type as u32 == ComponentType::WIRE as u32{
                link_write_wire(component_data, x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
            }
        }
    }
}

fn link_wire_read(component_data: &mut ComponentData, x1: usize, y1: usize, x2: usize, y2: usize){
    if !component_data.wires[component_data.array[x1][y1].belongs_to as usize].wire_readers.contains(&(component_data.array[x2][y2].belongs_to as u32)){
        component_data.wires[component_data.array[x1][y1].belongs_to as usize].wire_readers.push(component_data.array[x2][y2].belongs_to as u32);
    }
    if !component_data.wire_readers[component_data.array[x2][y2].belongs_to as usize].wires.contains(&(component_data.array[x1][y1].belongs_to as u32)){
        component_data.wire_readers[component_data.array[x2][y2].belongs_to as usize].wires.push(component_data.array[x1][y1].belongs_to as u32);
    }
}

fn link_read_logic(component_data: &mut ComponentData, x1: usize, y1: usize, x2: usize, y2: usize){
    if !component_data.wire_readers[component_data.array[x1][y1].belongs_to as usize].logic_gates.contains(&(component_data.array[x2][y2].belongs_to as u32)){
        component_data.wire_readers[component_data.array[x1][y1].belongs_to as usize].logic_gates.push(component_data.array[x2][y2].belongs_to as u32);
    }
    if !component_data.logic_gates[component_data.array[x2][y2].belongs_to as usize].wire_readers.contains(&(component_data.array[x1][y1].belongs_to as u32)){
        component_data.logic_gates[component_data.array[x2][y2].belongs_to as usize].wire_readers.push(component_data.array[x1][y1].belongs_to as u32);
    }
}

fn link_logic_write(component_data: &mut ComponentData, x1: usize, y1: usize, x2: usize, y2: usize){
    if !component_data.logic_gates[component_data.array[x1][y1].belongs_to as usize].wire_writers.contains(&(component_data.array[x2][y2].belongs_to as u32)){
        component_data.logic_gates[component_data.array[x1][y1].belongs_to as usize].wire_writers.push(component_data.array[x2][y2].belongs_to as u32);
    }
    if !component_data.wire_writers[component_data.array[x2][y2].belongs_to as usize].logic_gates.contains(&(component_data.array[x1][y1].belongs_to as u32)){
        component_data.wire_writers[component_data.array[x2][y2].belongs_to as usize].logic_gates.push(component_data.array[x1][y1].belongs_to as u32);
    }
}

fn link_write_wire(component_data: &mut ComponentData, x1: usize, y1: usize, x2: usize, y2: usize){
    if !component_data.wire_writers[component_data.array[x1][y1].belongs_to as usize].wires.contains(&(component_data.array[x2][y2].belongs_to as u32)){
        component_data.wire_writers[component_data.array[x1][y1].belongs_to as usize].wires.push(component_data.array[x2][y2].belongs_to as u32);
    }
    if !component_data.wires[component_data.array[x2][y2].belongs_to as usize].wire_writers.contains(&(component_data.array[x1][y1].belongs_to as u32)){
        component_data.wires[component_data.array[x2][y2].belongs_to as usize].wire_writers.push(component_data.array[x1][y1].belongs_to as u32);
    }
}

fn new_group(component_data: &mut ComponentData, x: usize, y: usize){
    if component_data.array[x][y].component_type as u32 == ComponentType::WIRE as u32 {
        new_wire_group(component_data, x, y);
    }else if component_data.array[x][y].component_type as u32 == ComponentType::READ_FROM_WIRE as u32 {
        new_wire_reader_group(component_data, x, y);
    }else if component_data.array[x][y].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32 {
        new_wire_writer_group(component_data, x, y);
    }else {
        new_logic_gate_group(component_data, x, y);
    }
}

fn new_wire_group(component_data: &mut ComponentData, x: usize, y: usize){
    component_data.wires.push(Wire{
        enabled: false,
        to_update: true,
        elements: vec![],
        wire_readers: vec![],
        wire_writers: vec![]
    });
    let wire_index = component_data.wires.len() - 1;
    let wire: &mut Wire = &mut component_data.wires[wire_index];
    wire.elements.push((x, y));
    component_data.array[x][y].belongs_to = wire_index as i32;
    let mut index = 0;
    while index < wire.elements.len(){
        let x_ = wire.elements[index].0;
        let y_ = wire.elements[index].1;
        if x_ > 0 && component_data.array[x_ - 1][y_].component_type as u32 == ComponentType::WIRE as u32 && component_data.array[x_ - 1][y_].belongs_to == -1{
            wire.elements.push((x_ - 1, y_));
            component_data.array[x_ - 1][y_].belongs_to = wire_index as i32;
        }
        if y_ > 0 && component_data.array[x_][y_ - 1].component_type as u32 == ComponentType::WIRE as u32 && component_data.array[x_][y_ - 1].belongs_to == -1{
            wire.elements.push((x_, y_ - 1));
            component_data.array[x_][y_ - 1].belongs_to = wire_index as i32;
        }
        if x_ < (WIDTH - 1) as usize && component_data.array[x_ + 1][y_].component_type as u32 == ComponentType::WIRE as u32 && component_data.array[x_ + 1][y_].belongs_to == -1{
            wire.elements.push((x_ + 1, y_));
            component_data.array[x_ + 1][y_].belongs_to = wire_index as i32;
        }
        if y_ < (HEIGHT - 1) as usize && component_data.array[x_][y_ + 1].component_type as u32 == ComponentType::WIRE as u32 && component_data.array[x_][y_ + 1].belongs_to == -1{
            wire.elements.push((x_, y_ + 1));
            component_data.array[x_][y_ + 1].belongs_to = wire_index as i32;
        }
        if y_ < (HEIGHT - 2) as usize && component_data.array[x_][y_ + 2].component_type as u32 == ComponentType::WIRE as u32 && component_data.array[x_][y_ + 2].belongs_to == -1 &&
            component_data.array[x_][y_ + 1].component_type as u32 == ComponentType::CROSS as u32{
            wire.elements.push((x_, y_ + 2));
            component_data.array[x_][y_ + 2].belongs_to = wire_index as i32;
        }
        if y_ > 1 && component_data.array[x_][y_ - 2].component_type as u32 == ComponentType::WIRE as u32 && component_data.array[x_][y_ - 2].belongs_to == -1 &&
            component_data.array[x_][y_ - 1].component_type as u32 == ComponentType::CROSS as u32{
            wire.elements.push((x_, y_ - 2));
            component_data.array[x_][y_ - 2].belongs_to = wire_index as i32;
        }
        if x_ < (WIDTH - 2) as usize && component_data.array[x_ + 2][y_].component_type as u32 == ComponentType::WIRE as u32 && component_data.array[x_ + 2][y_].belongs_to == -1 &&
            component_data.array[x_ + 1][y_].component_type as u32 == ComponentType::CROSS as u32{
            wire.elements.push((x_ + 2, y_));
            component_data.array[x_ + 2][y_].belongs_to = wire_index as i32;
        }
        if x_ > 1 && component_data.array[x_ - 2][y_].component_type as u32 == ComponentType::WIRE as u32 && component_data.array[x_ - 2][y_].belongs_to == -1 &&
            component_data.array[x_ - 1][y_].component_type as u32 == ComponentType::CROSS as u32{
            wire.elements.push((x_ - 2, y_));
            component_data.array[x_ - 2][y_].belongs_to = wire_index as i32;
        }
        index += 1;
    }
}

fn new_wire_reader_group(component_data: &mut ComponentData, x: usize, y: usize){
    component_data.wire_readers.push(WireReader{
        enabled: false,
        to_update: true,
        elements: vec![],
        logic_gates: vec![],
        wires: vec![]
    });
    let wire_reader_index = component_data.wire_readers.len() - 1;
    let wire_reader: &mut WireReader = &mut component_data.wire_readers[wire_reader_index];
    wire_reader.elements.push((x, y));
    component_data.array[x][y].belongs_to = wire_reader_index as i32;
    let mut index = 0;
    while index < wire_reader.elements.len(){
        let x_ = wire_reader.elements[index].0;
        let y_ = wire_reader.elements[index].1;
        if x_ > 0 && component_data.array[x_ - 1][y_].component_type as u32 == ComponentType::READ_FROM_WIRE as u32 && component_data.array[x_ - 1][y_].belongs_to == -1{
            wire_reader.elements.push((x_ - 1, y_));
            component_data.array[x_ - 1][y_].belongs_to = wire_reader_index as i32;
        }
        if y_ > 0 && component_data.array[x_][y_ - 1].component_type as u32 == ComponentType::READ_FROM_WIRE as u32 && component_data.array[x_][y_ - 1].belongs_to == -1{
            wire_reader.elements.push((x_, y_ - 1));
            component_data.array[x_][y_ - 1].belongs_to = wire_reader_index as i32;
        }
        if x_ < (WIDTH - 1) as usize && component_data.array[x_ + 1][y_].component_type as u32 == ComponentType::READ_FROM_WIRE as u32 && component_data.array[x_ + 1][y_].belongs_to == -1{
            wire_reader.elements.push((x_ + 1, y_));
            component_data.array[x_ + 1][y_].belongs_to = wire_reader_index as i32;
        }
        if y_ < (HEIGHT - 1) as usize && component_data.array[x_][y_ + 1].component_type as u32 == ComponentType::READ_FROM_WIRE as u32 && component_data.array[x_][y_ + 1].belongs_to == -1{
            wire_reader.elements.push((x_, y_ + 1));
            component_data.array[x_][y_ + 1].belongs_to = wire_reader_index as i32;
        }
        index += 1;
    }
}

fn new_wire_writer_group(component_data: &mut ComponentData, x: usize, y: usize){
    component_data.wire_writers.push(WireWriter{
        enabled: false,
        to_update: true,
        elements: vec![],
        wires: vec![],
        logic_gates: vec![]
    });
    let wire_writer_index = component_data.wire_writers.len() - 1;
    let wire_writer: &mut WireWriter = &mut component_data.wire_writers[wire_writer_index];
    wire_writer.elements.push((x, y));
    component_data.array[x][y].belongs_to = wire_writer_index as i32;
    let mut index = 0;
    while index < wire_writer.elements.len(){
        let x_ = wire_writer.elements[index].0;
        let y_ = wire_writer.elements[index].1;
        if x_ > 0 && component_data.array[x_ - 1][y_].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32 && component_data.array[x_ - 1][y_].belongs_to == -1{
            wire_writer.elements.push((x_ - 1, y_));
            component_data.array[x_ - 1][y_].belongs_to = wire_writer_index as i32;
        }
        if y_ > 0 && component_data.array[x_][y_ - 1].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32 && component_data.array[x_][y_ - 1].belongs_to == -1{
            wire_writer.elements.push((x_, y_ - 1));
            component_data.array[x_][y_ - 1].belongs_to = wire_writer_index as i32;
        }
        if x_ < (WIDTH - 1) as usize && component_data.array[x_ + 1][y_].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32 && component_data.array[x_ + 1][y_].belongs_to == -1{
            wire_writer.elements.push((x_ + 1, y_));
            component_data.array[x_ + 1][y_].belongs_to = wire_writer_index as i32;
        }
        if y_ < (HEIGHT - 1) as usize && component_data.array[x_][y_ + 1].component_type as u32 == ComponentType::WRITE_TO_WIRE as u32 && component_data.array[x_][y_ + 1].belongs_to == -1{
            wire_writer.elements.push((x_, y_ + 1));
            component_data.array[x_][y_ + 1].belongs_to = wire_writer_index as i32;
        }
        index += 1;
    }
}

fn new_logic_gate_group(component_data: &mut ComponentData, x: usize, y: usize){
    let component_type_index = component_data.array[x][y].component_type as u32;
    component_data.logic_gates.push(LogicGate{
        enabled: false,
        to_update: true,
        gate_type: ComponentType::from_u32(component_type_index),
        elements: vec![],
        wire_readers: vec![],
        wire_writers: vec![]
    });
    let logic_gate_index = component_data.logic_gates.len() - 1;
    let logic_gate: &mut LogicGate = &mut component_data.logic_gates[logic_gate_index];
    logic_gate.elements.push((x, y));
    component_data.array[x][y].belongs_to = logic_gate_index as i32;
    let mut index = 0;
    while index < logic_gate.elements.len(){
        let x_ = logic_gate.elements[index].0;
        let y_ = logic_gate.elements[index].1;logic_gate_index;
        if x_ > 0 && component_data.array[x_ - 1][y_].component_type as u32 == ComponentType::from_u32(component_type_index) as u32 && component_data.array[x_ - 1][y_].belongs_to == -1{
            logic_gate.elements.push((x_ - 1, y_));
            component_data.array[x_ - 1][y_].belongs_to = logic_gate_index as i32;
        }
        if y_ > 0 && component_data.array[x_][y_ - 1].component_type as u32 == ComponentType::from_u32(component_type_index) as u32 && component_data.array[x_][y_ - 1].belongs_to == -1{
            logic_gate.elements.push((x_, y_ - 1));
            component_data.array[x_][y_ - 1].belongs_to = logic_gate_index as i32;
        }
        if x_ < (WIDTH - 1) as usize && component_data.array[x_ + 1][y_].component_type as u32 == ComponentType::from_u32(component_type_index) as u32 && component_data.array[x_ + 1][y_].belongs_to == -1{
            logic_gate.elements.push((x_ + 1, y_));
            component_data.array[x_ + 1][y_].belongs_to = logic_gate_index as i32;
        }
        if y_ < (HEIGHT - 1) as usize && component_data.array[x_][y_ + 1].component_type as u32 == ComponentType::from_u32(component_type_index) as u32 && component_data.array[x_][y_ + 1].belongs_to == -1{
            logic_gate.elements.push((x_, y_ + 1));
            component_data.array[x_][y_ + 1].belongs_to = logic_gate_index as i32;
        }
        index += 1;
    }
}

fn update_canvas(mut component_data: &mut ComponentData){
    update_reader(&mut component_data);
    for i in 0..component_data.wire_readers.len(){
        component_data.wire_readers[i].to_update = false;
    }
    update_logic(&mut component_data);
    for i in 0..component_data.logic_gates.len(){
        if component_data.logic_gates[i].gate_type as u32 != ComponentType::CLOCK as u32 {
            component_data.logic_gates[i].to_update = false;
        }
    }
    update_writer(&mut component_data);
    for i in 0..component_data.wire_writers.len(){
        component_data.wire_writers[i].to_update = false;
    }
    update_wire(&mut component_data);
    for i in 0..component_data.wires.len(){
        component_data.wires[i].to_update = false;
    }
}

fn update_wire(component_data: &mut ComponentData){

    for i in 0..component_data.wires.len(){
        if !component_data.wires[i].to_update{
            continue;
        }
        let previous_state = component_data.wires[i].enabled;
        let mut should_turn_on = false;
        for j in 0..component_data.wires[i].wire_writers.len(){
            should_turn_on = should_turn_on || component_data.wire_writers[component_data.wires[i].wire_writers[j] as usize].enabled;
        }
        if previous_state != should_turn_on{
            component_data.wires[i].enabled = should_turn_on;
            for j in 0..component_data.wires[i].wire_readers.len(){
                component_data.wire_readers[component_data.wires[i].wire_readers[j] as usize].to_update = true;
            }
        }
    }
}

fn update_reader(component_data: &mut ComponentData){
    for i in 0..component_data.wire_readers.len(){
        if !component_data.wire_readers[i].to_update{
            continue;
        }
        let previous_state = component_data.wire_readers[i].enabled;
        let mut should_turn_on = false;
        for j in 0..component_data.wire_readers[i].wires.len(){
            should_turn_on = should_turn_on || component_data.wires[component_data.wire_readers[i].wires[j] as usize].enabled;
        }
        if previous_state != should_turn_on{
            component_data.wire_readers[i].enabled = should_turn_on;
            for j in 0..component_data.wire_readers[i].logic_gates.len(){
                component_data.logic_gates[component_data.wire_readers[i].logic_gates[j] as usize].to_update = true;
            }
        }
    }
}

fn update_writer(component_data: &mut ComponentData){
    for i in 0..component_data.wire_writers.len(){
        if !component_data.wire_writers[i].to_update{
            continue;
        }
        let previous_state = component_data.wire_writers[i].enabled;
        let mut should_turn_on = false;
        for j in 0..component_data.wire_writers[i].logic_gates.len(){
            should_turn_on = should_turn_on || component_data.logic_gates[component_data.wire_writers[i].logic_gates[j] as usize].enabled;
        }
        if previous_state != should_turn_on{
            component_data.wire_writers[i].enabled = should_turn_on;
            for j in 0..component_data.wire_writers[i].wires.len(){
                component_data.wires[component_data.wire_writers[i].wires[j] as usize].to_update = true;
            }
        }
        //component_data.wire_writers[i].enabled = true;
    }
}

fn update_logic(component_data: &mut ComponentData){
    for i in 0..component_data.logic_gates.len(){
        if !component_data.logic_gates[i].to_update{
            continue;
        }
        let previous_state = component_data.logic_gates[i].enabled;
        let mut should_turn_on = false;

        match component_data.logic_gates[i].gate_type {
            ComponentType::OR    => { should_turn_on = should_or_turn_on   (component_data, i); }
            ComponentType::AND   => { should_turn_on = should_and_turn_on  (component_data, i); }
            ComponentType::XOR   => { should_turn_on = should_xor_turn_on  (component_data, i); }
            ComponentType::NOT   => { should_turn_on = should_not_turn_on  (component_data, i); }
            ComponentType::NAND  => { should_turn_on = should_nand_turn_on (component_data, i); }
            ComponentType::XNOR  => { should_turn_on = should_xnor_turn_on (component_data, i); }
            ComponentType::CLOCK => { should_turn_on = should_clock_turn_on(component_data, i); }
            ComponentType::LATCH => { should_turn_on = should_latch_turn_on(component_data, i); }
            _ => {}
        }

        if previous_state != should_turn_on{
            component_data.logic_gates[i].enabled = should_turn_on;
            for j in 0..component_data.logic_gates[i].wire_writers.len(){
                component_data.wire_writers[component_data.logic_gates[i].wire_writers[j] as usize].to_update = true;
            }
        }
    }
}

fn should_not_turn_on(component_data: &mut ComponentData, gate_index: usize) -> bool {
    for i in 0..component_data.logic_gates[gate_index].wire_readers.len(){
        if component_data.wire_readers[component_data.logic_gates[gate_index].wire_readers[i] as usize].enabled{
            return false;
        }
    }
    return true;
}

fn should_or_turn_on(component_data: &mut ComponentData, gate_index: usize) -> bool {
    for i in 0..component_data.logic_gates[gate_index].wire_readers.len(){
        if component_data.wire_readers[component_data.logic_gates[gate_index].wire_readers[i] as usize].enabled{
            return true;
        }
    }
    return false;
}

fn should_and_turn_on(component_data: &mut ComponentData, gate_index: usize) -> bool {
    if component_data.logic_gates[gate_index].wire_readers.len() == 0{
        return false;
    }
    for i in 0..component_data.logic_gates[gate_index].wire_readers.len(){
        if !component_data.wire_readers[component_data.logic_gates[gate_index].wire_readers[i] as usize].enabled{
            return false;
        }
    }
    return true;
}

fn should_nand_turn_on(component_data: &mut ComponentData, gate_index: usize) -> bool {
    if component_data.logic_gates[gate_index].wire_readers.len() == 0{
        return true;
    }
    for i in 0..component_data.logic_gates[gate_index].wire_readers.len(){
        if !component_data.wire_readers[component_data.logic_gates[gate_index].wire_readers[i] as usize].enabled{
            return true;
        }
    }
    return false;
}

fn should_xor_turn_on(component_data: &mut ComponentData, gate_index: usize) -> bool {
    let mut state = false;
    for i in 0..component_data.logic_gates[gate_index].wire_readers.len(){
        if component_data.wire_readers[component_data.logic_gates[gate_index].wire_readers[i] as usize].enabled{
            state = !state;
        }
    }
    state
}

fn should_xnor_turn_on(component_data: &mut ComponentData, gate_index: usize) -> bool {
    let mut state = false;
    for i in 0..component_data.logic_gates[gate_index].wire_readers.len(){
        if component_data.wire_readers[component_data.logic_gates[gate_index].wire_readers[i] as usize].enabled{
            state = !state;
        }
    }
    !state
}

fn should_clock_turn_on(component_data: &mut ComponentData, gate_index: usize) -> bool {
    !component_data.logic_gates[gate_index].enabled
}

fn should_latch_turn_on(component_data: &mut ComponentData, gate_index: usize) -> bool {
    for i in 0..component_data.logic_gates[gate_index].wire_readers.len(){
        if component_data.wire_readers[component_data.logic_gates[gate_index].wire_readers[i] as usize].enabled{
            return !component_data.logic_gates[gate_index].enabled
        }
    }
    component_data.logic_gates[gate_index].enabled
}

fn translate_mouse_pos(canvas_x: f32, canvas_y: f32, zoom: f32, mouse_x: f32, mouse_y: f32) -> (i32, i32){
    (((mouse_x - canvas_x) / zoom - 0.5).round() as i32, ((mouse_y - canvas_y) / zoom - 0.5).round() as i32)
}