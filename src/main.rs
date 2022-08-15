extern crate sdl2;
extern crate stopwatch;
const WIDTH: u32 = 750;
const HEIGHT: u32 = 400;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::EventPump;
use sdl2::libc::time;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::timer::Timer;

#[derive(Clone, Copy)]
enum ComponentType {NOTHING = 0, WRITE_TO_WIRE = 1, WIRE = 2, CROSS = 3, READ_FROM_WIRE = 4, BUFFER = 5, AND = 6, OR = 7, XOR = 8, NOT = 9, NAND = 10, NOR = 11, XNOR = 12, COMMENT = 13, LATCH = 14, NUM_COMPONENTS = 15}
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
            14 => ComponentType::LATCH,
            _ => ComponentType::NOTHING,
        }
    }
}
const COLORS: [((u8, u8, u8), (u8, u8, u8)); 15] =
   [((031, 037, 049), (031, 037, 049)),
    ((085, 062, 071), (255, 113, 113)),
    ((099, 097, 079), (251, 251, 074)),
    ((112, 131, 162), (121, 140, 168)),
    ((051, 078, 107), (119, 202, 255)),
    ((068, 085, 071), (168, 255, 121)),
    ((085, 067, 071), (255, 222, 122)),
    ((062, 082, 099), (121, 255, 255)),
    ((077, 068, 100), (199, 139, 255)),
    ((094, 069, 085), (255, 112, 163)),
    ((094, 072, 059), (255, 184, 000)),
    ((035, 073, 101), (058, 241, 255)),
    ((074, 052, 101), (189, 000, 255)),
    ((067, 072, 079), (067, 072, 079)),
    ((061, 085, 081), (110, 251, 183))];

const NAMES: [&str; 15] =  [
    "nothing",
    "writer",
    "wire",
    "cross",
    "reader",
    "buffer",
    "and",
    "or",
    "xor",
    "not",
    "nand",
    "nor",
    "xnor",
    "comment",
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
    canvas.draw_rect(Rect::new(x as i32 * 2, y as i32 * 2, 2, 2)).expect("failed to draw rect");
}

fn draw_canvas(x:f32, y:f32, zoom: f32, component_data: &mut ComponentData, canvas: &mut WindowCanvas, sim_view: bool){
    if sim_view {
        draw_canvas_components(x, y, zoom, component_data, canvas);
    }else {
        draw_canvas_pixels(x, y, zoom, component_data, canvas);
    }
}

fn draw_canvas_components(x:f32, y:f32, zoom: f32, component_data: &mut ComponentData, canvas: &mut WindowCanvas){
    let color = COLORS[0].0;
    canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
    canvas.clear();
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

fn draw_canvas_pixels(x:f32, y:f32, zoom: f32, component_data: &mut ComponentData, canvas: &mut WindowCanvas){
    for i in 0..component_data.array.len(){
        for j in 0..component_data.array[0].len(){
            let color = get_color(component_data, &component_data.array[i][j]);
            canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
            canvas.draw_rect(Rect::new(i as i32 * 2, j as i32 * 2, 2, 2));
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
        logic_gates: vec![]
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

    draw_canvas(0.0, 0.0, 1.0, &mut component_data, & mut canvas, true);
    main_update(&mut canvas, &mut event_pump, &mut component_data)
}

fn main_update(mut canvas: &mut WindowCanvas, event_pump: &mut EventPump, mut component_data: &mut ComponentData){
    let mut selected_type = ComponentType::WIRE;
    let mut run_sim = false;
    /*let stopwatch = stopwatch::Stopwatch::start_new();
    let mut last_time = stopwatch.elapsed_ms();*/
    'running: loop {
        let mouse_x = event_pump.mouse_state().x() / 2;
        let mouse_y = event_pump.mouse_state().y() / 2;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
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
                        draw_canvas(0.0, 0.0, 1.0, &mut component_data, canvas, false);
                    }else{
                        compile_scene(component_data);
                    }
                    run_sim = !run_sim;
                }
                Event::MouseButtonDown {mouse_btn: MouseButton::Left, ..} => {
                    if run_sim {
                        if component_data.array[mouse_x as usize][mouse_y as usize].component_type as u32 == ComponentType::LATCH as u32 && component_data.array[mouse_x as usize][mouse_y as usize].belongs_to != -1{
                            component_data.logic_gates[component_data.array[mouse_x as usize][mouse_y as usize].belongs_to as usize].enabled = !component_data.logic_gates[component_data.array[mouse_x as usize][mouse_y as usize].belongs_to as usize].enabled;
                            for writer in &component_data.logic_gates[component_data.array[mouse_x as usize][mouse_y as usize].belongs_to as usize].wire_writers{
                                component_data.wire_writers[*writer as usize].to_update = true;
                            }
                        }
                    }

                }
                _ => {}
            }
        }
        if run_sim {
            update_canvas(&mut component_data);
            draw_canvas(0.0, 0.0, 1.0, &mut component_data, canvas, true);
        }else{
            if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
                let x = event_pump.mouse_state().x() / 2;
                let y = event_pump.mouse_state().y() / 2;
                for i in std::cmp::max(x - 5, 0)..std::cmp::min(x + 5, WIDTH as i32) {
                    for j in std::cmp::max(y - 5, 0)..std::cmp::min(y + 5, HEIGHT as i32) {
                        draw_component(i as usize, j as usize, selected_type, false, &mut component_data, canvas);
                    }
                }
            }
            if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Right) {
                let x = event_pump.mouse_state().x() / 2;
                let y = event_pump.mouse_state().y() / 2;
                for i in std::cmp::max(x - 5, 0)..std::cmp::min(x + 5, WIDTH as i32) {
                    for j in std::cmp::max(y - 5, 0)..std::cmp::min(y + 5, HEIGHT as i32) {
                        draw_component(i as usize, j as usize, ComponentType::NOTHING, false, &mut component_data, canvas);
                    }
                }
            }
        }


        canvas.present();
        /*println!("{}", stopwatch.elapsed_ms() - last_time);
        last_time = stopwatch.elapsed_ms();*/
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
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

fn new_group(mut component_data: &mut ComponentData, x: usize, y: usize){
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
        component_data.logic_gates[i].to_update = false;
    }
    update_writer(&mut component_data);
    for i in 0..component_data.wire_writers.len(){
        component_data.wire_writers[i].to_update = false;
    }
    for i in 0..WIDTH as usize{
        if component_data.array[i][0].component_type as u32 == ComponentType::WIRE as u32{
            let wire_index = component_data.array[i][0].belongs_to;
            component_data.wires[wire_index as usize].enabled = true;
            component_data.wires[wire_index as usize].to_update = true;
        }
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
                component_data.wire_readers[component_data.wires[i].wire_readers[i] as usize].to_update = true;
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
            should_turn_on = should_turn_on || component_data.logic_gates[component_data.wire_writers[i].wires[j] as usize].enabled;
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
            ComponentType::BUFFER => { should_turn_on = should_or_turn_on(component_data, i); }
            ComponentType::OR => { should_turn_on = should_or_turn_on(component_data, i); }
            ComponentType::AND => { should_turn_on = should_and_turn_on(component_data, i); }
            ComponentType::XOR => { should_turn_on = should_xor_turn_on(component_data, i); }
            ComponentType::NOT => { should_turn_on = should_not_turn_on(component_data, i); }
            ComponentType::NOR => { should_turn_on = should_not_turn_on(component_data, i); }
            ComponentType::NAND => { should_turn_on = should_nand_turn_on(component_data, i); }
            ComponentType::XNOR => { should_turn_on = should_xnor_turn_on(component_data, i); }
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

fn should_not_turn_on(component_data: &mut ComponentData, gate_intex: usize) -> bool {
    for i in 0..component_data.logic_gates[gate_intex].wire_readers.len(){
        if component_data.wire_readers[component_data.logic_gates[gate_intex].wire_readers[i] as usize].enabled{
            return false;
        }
    }
    return true;
}

fn should_or_turn_on(component_data: &mut ComponentData, gate_intex: usize) -> bool {
    for i in 0..component_data.logic_gates[gate_intex].wire_readers.len(){
        if component_data.wire_readers[component_data.logic_gates[gate_intex].wire_readers[i] as usize].enabled{
            return true;
        }
    }
    return false;
}

fn should_and_turn_on(component_data: &mut ComponentData, gate_intex: usize) -> bool {
    if component_data.logic_gates[gate_intex].wire_readers.len() == 0{
        return false;
    }
    for i in 0..component_data.logic_gates[gate_intex].wire_readers.len(){
        if !component_data.wire_readers[component_data.logic_gates[gate_intex].wire_readers[i] as usize].enabled{
            return false;
        }
    }
    return true;
}

fn should_nand_turn_on(component_data: &mut ComponentData, gate_intex: usize) -> bool {
    if component_data.logic_gates[gate_intex].wire_readers.len() == 0{
        return true;
    }
    for i in 0..component_data.logic_gates[gate_intex].wire_readers.len(){
        if !component_data.wire_readers[component_data.logic_gates[gate_intex].wire_readers[i] as usize].enabled{
            return true;
        }
    }
    return false;
}

fn should_xor_turn_on(component_data: &mut ComponentData, gate_intex: usize) -> bool {
    let mut state = false;
    for i in 0..component_data.logic_gates[gate_intex].wire_readers.len(){
        if component_data.wire_readers[component_data.logic_gates[gate_intex].wire_readers[i] as usize].enabled{
            state = !state;
        }
    }
    state
}

fn should_xnor_turn_on(component_data: &mut ComponentData, gate_intex: usize) -> bool {
    let mut state = false;
    for i in 0..component_data.logic_gates[gate_intex].wire_readers.len(){
        if component_data.wire_readers[component_data.logic_gates[gate_intex].wire_readers[i] as usize].enabled{
            state = !state;
        }
    }
    !state
}