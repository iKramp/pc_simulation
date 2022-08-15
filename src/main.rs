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
    belongs_to: i32,
}

struct WireWriter{
    enabled: bool,
    to_update: bool,
    elements: Vec<(usize, usize)>,
    wires: Vec<u32>
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
    logic_gates: Vec<u32>
}

struct Wire{
    enabled: bool,
    to_update: bool,
    elements: Vec<(usize, usize)>,
    wire_readers: Vec<u32>
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
                    if run_sim {
                        clear_compiled_data(component_data);
                        draw_canvas(0.0, 0.0, 1.0, &mut component_data, canvas, false);
                    }else{
                        compile_scene(component_data);
                    }
                    run_sim = !run_sim;
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
        to_update: false,
        elements: vec![],
        wire_readers: vec![]
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
        to_update: false,
        elements: vec![],
        logic_gates: vec![]
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
        to_update: false,
        elements: vec![],
        wires: vec![]
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
        to_update: false,
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
    //update_read(&mut component_data);
    //update gate
    //update_write(&mut component_data);
    for i in 0..WIDTH as usize{
        if component_data.array[i][0].component_type as u32 == ComponentType::WIRE as u32{
            let wire_index = component_data.array[i][0].belongs_to;
            component_data.wires[wire_index as usize].enabled = true;
            component_data.wires[wire_index as usize].to_update = true;
        }
    }
    //update_wire(&mut component_data);
}

