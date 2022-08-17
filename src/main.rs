pub mod content;

extern crate sdl2;
extern crate stopwatch;

use crate::content::{HEIGHT, SIZE, WIDTH, ComponentType, COLORS, NAMES, Component, Wire, WireReader, WireWriter, LogicGate, ComponentData};


fn get_color(component_data: &ComponentData, component: &Component) -> (u8, u8, u8){
    if component.belongs_to == -1{
        return COLORS[component.component_type as usize].0;
    }

    return if component.component_type == ComponentType::WIRE {
        if component_data.wires[component.belongs_to as usize].enabled {
            COLORS[ComponentType::WIRE as usize].1
        } else {
            COLORS[ComponentType::WIRE as usize].0
        }
    } else if component.component_type == ComponentType::WRITE_TO_WIRE {
        if component_data.wire_writers[component.belongs_to as usize].enabled {
            COLORS[ComponentType::WRITE_TO_WIRE as usize].1
        } else {
            COLORS[ComponentType::WRITE_TO_WIRE as usize].0
        }
    } else if component.component_type == ComponentType::READ_FROM_WIRE {
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

fn draw_component(x: usize, y: usize, component_type_: ComponentType, turned_on_: bool, component_data: &mut ComponentData, canvas: &mut sdl2::render::WindowCanvas){
    component_data.array[x][y].component_type = component_type_;
    let color;
    if !turned_on_ {
        color = COLORS[component_type_ as usize].0;
    }else {
        color = COLORS[component_type_ as usize].1;
    }

    canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0, color.1, color.2));
    canvas.fill_rect(sdl2::rect::Rect::new(((x as f32 * component_data.zoom + component_data.position_on_screen.0) * 2.0).round() as i32, ((y as f32 * component_data.zoom + component_data.position_on_screen.1.round()) * 2.0).round() as i32, (component_data.zoom * 2.0).round() as u32, (component_data.zoom * 2.0).round() as u32)).expect("failed to draw rect");
}

fn draw_canvas(component_data: &mut ComponentData, canvas: &mut sdl2::render::WindowCanvas, sim_view: bool){
    let color = COLORS[0].0;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0 / 2, color.1 / 2, color.2 / 2));
    canvas.clear();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0, color.1, color.2));
    canvas.fill_rect(sdl2::rect::Rect::new(component_data.position_on_screen.0.round() as i32 * 2, component_data.position_on_screen.1.round() as i32 * 2, (WIDTH as f32 * component_data.zoom * 2.0) as u32, (HEIGHT as f32 * component_data.zoom * 2.0) as u32)).expect("failed to draw");
    if sim_view {
        draw_canvas_components(component_data, canvas);
    }else {
        draw_canvas_pixels(component_data, canvas);
    }
}

fn draw_canvas_components(component_data: &mut ComponentData, canvas: &mut sdl2::render::WindowCanvas){
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

fn draw_canvas_pixels(component_data: &mut ComponentData, canvas: &mut sdl2::render::WindowCanvas){
    for i in 0..component_data.array.len(){
        for j in 0..component_data.array[0].len(){
            if component_data.array[i][j].component_type != ComponentType::NOTHING {
                let color = get_color(component_data, &component_data.array[i][j]);
                canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0, color.1, color.2));
                canvas.fill_rect(sdl2::rect::Rect::new(((i as f32 * component_data.zoom + component_data.position_on_screen.0) * 2.0).round() as i32, ((j as f32 * component_data.zoom + component_data.position_on_screen.1.round()) * 2.0).round() as i32, (component_data.zoom * 2.0).round() as u32, (component_data.zoom * 2.0).round() as u32)).expect("failed to draw rect");
            }
        }
    }
}

pub fn main() {
    let mut component_data = ComponentData::default();
    load_canvas(&mut component_data);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("pc sim", WIDTH * 2, HEIGHT * 2)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    draw_canvas(&mut component_data, & mut canvas, false);
    main_update(&mut canvas, &mut event_pump, &mut component_data);

    save_canvas(&component_data);
}

fn main_update(canvas: &mut sdl2::render::WindowCanvas, event_pump: &mut sdl2::EventPump, mut component_data: &mut ComponentData){
    let mut selected_type = ComponentType::WIRE;
    let mut run_sim = false;
    let stopwatch = stopwatch::Stopwatch::start_new();
    let mut last_time = stopwatch.elapsed_ms();
    let mut last_mouse_x = 0;
    let mut last_mouse_y = 0;
    let mut shift_pressed;
    let mut control_pressed;
    let mut copy = false;
    let mut paste = (false, (0, 0));
    let mut selection: ((i32, i32), (i32, i32)) = ((0, 0), (0, 0));
    let mut copied_data: Vec<Vec<u8>> = vec![];
    'running: loop {
        shift_pressed = event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::LShift);
        control_pressed = event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::LCtrl);
        let mouse_x = event_pump.mouse_state().x() / 2;
        let mouse_y = event_pump.mouse_state().y() / 2;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    if paste.0  {
                        paste.0 = false;
                    }else {
                        break 'running
                    }
                },
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Plus), ..} => {
                    selected_type = ComponentType::from_u32(selected_type as u32 % (ComponentType::NUM_COMPONENTS as u32 - 1) + 1);
                    println!("{}", NAMES[selected_type as usize]);
                },
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Minus), ..} => {
                    if selected_type as u32 == 1{
                        selected_type = ComponentType::from_u32(ComponentType::NUM_COMPONENTS as u32 - 1);
                    }else {
                        selected_type = ComponentType::from_u32(selected_type as u32 - 1);
                    }
                    println!("{}", NAMES[selected_type as usize]);
                },
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Space), ..} => {
                    if run_sim {
                        clear_compiled_data(component_data);
                    }else{
                        paste.0 = false;
                        compile_scene(component_data);
                    }
                    run_sim = !run_sim;
                }
                sdl2::event::Event::MouseButtonDown {mouse_btn: sdl2::mouse::MouseButton::Left, ..} => {
                    if paste.0{
                        paste_selection(&mut component_data, &mut copied_data, paste.1.0, paste.1.1);
                    }else {
                        if run_sim {
                            let pos = component_data.translate_mouse_pos(mouse_x as f32, mouse_y as f32);
                            if component_data.array[pos.0 as usize][pos.1 as usize].component_type as u32 == ComponentType::LATCH as u32 && component_data.array[pos.0 as usize][pos.1 as usize].belongs_to != -1 {
                                component_data.logic_gates[component_data.array[pos.0 as usize][pos.1 as usize].belongs_to as usize].enabled = !component_data.logic_gates[component_data.array[pos.0 as usize][pos.1 as usize].belongs_to as usize].enabled;
                                for writer in &component_data.logic_gates[component_data.array[pos.0 as usize][pos.1 as usize].belongs_to as usize].wire_writers {
                                    component_data.wire_writers[*writer as usize].to_update = true;
                                }
                            }
                        } else if shift_pressed {
                            copy = true;
                            selection.0 = component_data.translate_mouse_pos( mouse_x as f32, mouse_y as f32);
                            selection.1 = selection.0;
                        }
                    }

                }
                sdl2::event::Event::MouseButtonDown {mouse_btn: sdl2::mouse::MouseButton::Middle, ..} => {
                    last_mouse_x = mouse_x;
                    last_mouse_y = mouse_y;
                }
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::KpMinus), ..} => {
                    let corner_from_center = ((-component_data.position_on_screen.0 * 2.0 + WIDTH as f32), (-component_data.position_on_screen.1 * 2.0 + HEIGHT as f32));
                    component_data.zoom = component_data.zoom / 2.0;
                    component_data.position_on_screen.0 = WIDTH as f32 / 2.0 - (corner_from_center.0 / 4.0);
                    component_data.position_on_screen.1 = HEIGHT as f32 / 2.0 - (corner_from_center.1 / 4.0);
                }
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::KpPlus), ..} => {
                    let corner_from_center = ((-component_data.position_on_screen.0 * 2.0 + WIDTH as f32), (-component_data.position_on_screen.1 * 2.0 + HEIGHT as f32));
                    component_data.zoom = component_data.zoom * 2.0;
                    component_data.position_on_screen.0 -= corner_from_center.0 / 2.0;
                    component_data.position_on_screen.1 -= corner_from_center.1 / 2.0;
                }
                sdl2::event::Event::MouseButtonUp {mouse_btn: sdl2::mouse::MouseButton::Left, ..} => {
                    if copy {
                        copy = false;
                        prepare_selection(&mut selection);
                        copy_selection(component_data, &mut selection, &mut copied_data);
                    }
                    if paste.0 {
                        paste.0 = false;
                    }
                }
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::V), ..} => {
                    if !run_sim {
                        if control_pressed {
                            paste.0 = !paste.0;
                            paste.1 = component_data.translate_mouse_pos( mouse_x as f32, mouse_y as f32);
                        }
                    }
                }
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Delete), ..} => {
                    for i in selection.0.0..selection.1.0 {
                        for j in selection.0.1..selection.1.1 {
                            component_data.array[i as usize][j as usize].component_type = ComponentType::NOTHING
                        }
                    }
                }
                _ => {}
            }
        }
        if run_sim /* timed update*/ {
            if stopwatch.elapsed_ms() - last_time > 1 {
                last_time = stopwatch.elapsed_ms();
                component_data.update_canvas();
            }

        } else /*draw mode*/ {
            if event_pump.mouse_state().is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
                let pos = component_data.translate_mouse_pos(event_pump.mouse_state().x() as f32 / 2.0, event_pump.mouse_state().y() as f32 / 2.0);
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
            if event_pump.mouse_state().is_mouse_button_pressed(sdl2::mouse::MouseButton::Right) {
                if !paste.0 {
                    let pos = component_data.translate_mouse_pos(event_pump.mouse_state().x() as f32 / 2.0, event_pump.mouse_state().y() as f32 / 2.0);
                    for i in std::cmp::max(pos.0 - SIZE, 0)..std::cmp::min(pos.0 + SIZE + 1, WIDTH as i32) {
                        for j in std::cmp::max(pos.1 - SIZE, 0)..std::cmp::min(pos.1 + SIZE + 1, HEIGHT as i32) {
                            draw_component(i as usize, j as usize, ComponentType::NOTHING, false, &mut component_data, canvas);
                        }
                    }
                }
            }
        }

        if event_pump.mouse_state().is_mouse_button_pressed(sdl2::mouse::MouseButton::Middle) /* move canvas*/ {
            let delta = ((mouse_x - last_mouse_x) as f32, (mouse_y - last_mouse_y) as f32);
            component_data.position_on_screen.0 += delta.0;
            component_data.position_on_screen.1 += delta.1;
            component_data.position_on_screen.0 = component_data.position_on_screen.0.clamp(WIDTH as f32 / 2.0 - WIDTH as f32 * component_data.zoom, WIDTH as f32 / 2.0);
            component_data.position_on_screen.1 = component_data.position_on_screen.1.clamp(HEIGHT as f32 / 2.0 - HEIGHT as f32 * component_data.zoom , HEIGHT as f32 / 2.0);
            last_mouse_x = mouse_x;
            last_mouse_y = mouse_y;
        }
        draw_canvas(component_data, canvas, run_sim);
        let mut pos = component_data.translate_mouse_pos( mouse_x as f32, mouse_y as f32);

        if paste.0 /* draw stuff to paste (hopefully transparent)*/ {
            paste.1 = pos;
            draw_to_paste(&component_data, canvas, &copied_data, paste);
        }

        if !run_sim && !shift_pressed && !paste.0 /*draw drawing cursor square*/ {
            pos.0 = ((pos.0 - SIZE) as f32 * component_data.zoom + component_data.position_on_screen.0) as i32 * 2;
            pos.1 = ((pos.1 - SIZE) as f32 * component_data.zoom + component_data.position_on_screen.1) as i32 * 2;
            let color = COLORS[ComponentType::COMMENT as usize].0;
            canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.0, color.1, color.2, 100));
            canvas.draw_rect(sdl2::rect::Rect::new(pos.0, pos.1, (((SIZE * 4) as f32 + 2.0) * component_data.zoom) as u32, (((SIZE * 4) as f32 + 2.0) * component_data.zoom) as u32)).expect("couldn't draw rect");
        } else if !run_sim /*draw selection*/ {
            if shift_pressed {
                pos = selection.0;
            }
            pos.0 = ((pos.0) as f32 * component_data.zoom + component_data.position_on_screen.0) as i32 * 2;
            pos.1 = ((pos.1) as f32 * component_data.zoom + component_data.position_on_screen.1) as i32 * 2;
            let color = COLORS[ComponentType::COMMENT as usize].0;
            canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.0, color.1, color.2, 100));
            canvas.draw_rect(sdl2::rect::Rect::new(pos.0, pos.1, (((selection.1.0 - selection.0.0) as f32 * 2.0 + 2.0) * component_data.zoom) as u32, (((selection.1.1 - selection.0.1) as f32 * 2.0 + 2.0) * component_data.zoom) as u32)).expect("couldn't draw rect");

        }

        canvas.present();
    }
}

fn draw_to_paste(component_data: &ComponentData, canvas: &mut sdl2::render::WindowCanvas, copied_data: &Vec<Vec<u8>>, paste: (bool, (i32, i32))) {
    for i in 0.. copied_data.len(){
        for j in 0..copied_data[i].len(){
            if copied_data[i][j] as u32 != ComponentType::NOTHING as u32 {
                let mut color = COLORS[copied_data[i][j] as usize].0;
                color.0 = (color.0 as f32 * 3.0/4.0) as u8;
                color.1 = (color.1 as f32 * 3.0/4.0) as u8;
                color.2 = (color.2 as f32 * 3.0/4.0) as u8;
                canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.0, color.1, color.2, 100));
                canvas.fill_rect(sdl2::rect::Rect::new((((i as i32 + paste.1.0) as f32 * component_data.zoom + component_data.position_on_screen.0) * 2.0).round() as i32, (((j as i32 + paste.1.1) as f32 * component_data.zoom + component_data.position_on_screen.1.round()) * 2.0).round() as i32, (component_data.zoom * 2.0).round() as u32, (component_data.zoom * 2.0).round() as u32)).expect("failed to draw rect");

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
            if copied_data[i][j] != ComponentType::NOTHING as u8 && (i as i32) + paste_x >= 0 && (i as i32) + paste_x < WIDTH as i32  && (j as i32) + paste_y >= 0 && (j as i32) + paste_y < HEIGHT as i32 {
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
    std::fs::write("C:/Users/Uporabnik/CLionProjects/pc_simulation/canvas.dat", temp_arr).expect("couldn't write to file");
}

fn load_canvas(component_data: &mut ComponentData) {
    if !std::path::Path::new("C:/Users/Uporabnik/CLionProjects/pc_simulation/canvas.dat").exists(){
        return;
    }
    let temp_arr: Vec<u8> = std::fs::read("C:/Users/Uporabnik/CLionProjects/pc_simulation/canvas.dat").unwrap();
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
            if component_data.array[i][j].component_type != ComponentType::NOTHING && component_data.array[i][j].belongs_to == -1{
                new_group(component_data, i, j);
            }
        }
    }
    for i in 0..WIDTH as usize{
        for j in 0..HEIGHT as usize{
            if component_data.array[i][j].component_type != ComponentType::NOTHING {
                link_components(component_data, i as i32, j as i32);
            }
        }
    }
}

fn link_components(component_data: &mut ComponentData, x: i32, y: i32){
    let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    if component_data.array[x as usize][y as usize].component_type == ComponentType::WIRE{
        for direction in directions{
            if !are_coordinates_in_bounds(x + direction.0, y + direction.1){
                continue;
            }
            if component_data.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type == ComponentType::READ_FROM_WIRE{
                link_wire_read(component_data, x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
            }
        }
    }
    if component_data.array[x as usize][y as usize].component_type == ComponentType::READ_FROM_WIRE{
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
            if component_data.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type == ComponentType::WRITE_TO_WIRE{
                link_logic_write(component_data, x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
            }
        }
    }
    if component_data.array[x as usize][y as usize].component_type == ComponentType::WRITE_TO_WIRE{
        for direction in directions{
            if (x + direction.0 as i32) < 0 ||
                x + direction.0 as i32 == WIDTH as i32 ||
                (y + direction.1 as i32) < 0 ||
                y + direction.1 as i32 > HEIGHT as i32{
                continue;
            }
            if component_data.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type == ComponentType::WIRE{
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
    if component_data.array[x][y].component_type == ComponentType::WIRE {
        new_wire_group(component_data, x, y);
    }else if component_data.array[x][y].component_type == ComponentType::READ_FROM_WIRE {
        new_wire_reader_group(component_data, x, y);
    }else if component_data.array[x][y].component_type == ComponentType::WRITE_TO_WIRE {
        new_wire_writer_group(component_data, x, y);
    }else {
        new_logic_gate_group(component_data, x, y);
    }
}

fn new_wire_group(component_data: &mut ComponentData, x: usize, y: usize){
    component_data.wires.push(Wire::default());
    let wire_index = component_data.wires.len() - 1;
    let wire: &mut Wire = &mut component_data.wires[wire_index];
    wire.elements.push((x, y));
    component_data.array[x][y].belongs_to = wire_index as i32;
    let mut index = 0;
    while index < wire.elements.len(){
        let x_ = wire.elements[index].0 as i32;
        let y_ = wire.elements[index].1 as i32;
        let sides: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for side in sides{
            if are_coordinates_in_bounds(x_ + side.0, y_ + side.1) && component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == ComponentType::WIRE && component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to == -1{
                wire.elements.push(((x_ + side.0) as usize, (y_ + side.1) as usize));
                component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to = wire_index as i32;
            }
            if are_coordinates_in_bounds(x_ + side.0 * 2, y_ + side.1 * 2) && component_data.array[(x_ + (side.0 * 2)) as usize][(y_ + (side.1 * 2)) as usize].component_type == ComponentType::WIRE && component_data.array[(x_ + (side.0 * 2)) as usize][(y_ + (side.1 * 2)) as usize].belongs_to == -1 &&
                component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == ComponentType::CROSS{
                wire.elements.push(((x_ + (side.0 * 2)) as usize, (y_ + (side.1 * 2)) as usize));
                component_data.array[(x_ + (side.0 * 2)) as usize][(y_ + (side.1 * 2)) as usize].belongs_to = wire_index as i32;
            }
        }
        index += 1;
    }
}

fn new_wire_reader_group(component_data: &mut ComponentData, x: usize, y: usize){
    component_data.wire_readers.push(WireReader::default());
    let wire_reader_index = component_data.wire_readers.len() - 1;
    let wire_reader: &mut WireReader = &mut component_data.wire_readers[wire_reader_index];
    wire_reader.elements.push((x, y));
    component_data.array[x][y].belongs_to = wire_reader_index as i32;
    let mut index = 0;
    while index < wire_reader.elements.len(){
        let x_ = wire_reader.elements[index].0 as i32;
        let y_ = wire_reader.elements[index].1 as i32;
        let sides: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for side in sides{
            if are_coordinates_in_bounds(x_ + side.0, y_ + side.1) && component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == ComponentType::READ_FROM_WIRE && component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to == -1{
                wire_reader.elements.push(((x_ + side.0) as usize, (y_ + side.1) as usize));
                component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to = wire_reader_index as i32;
            }
        }
        index += 1;
    }
}

fn new_wire_writer_group(component_data: &mut ComponentData, x: usize, y: usize){
    component_data.wire_writers.push(WireWriter::default());
    let wire_writer_index = component_data.wire_writers.len() - 1;
    let wire_writer: &mut WireWriter = &mut component_data.wire_writers[wire_writer_index];
    wire_writer.elements.push((x, y));
    component_data.array[x][y].belongs_to = wire_writer_index as i32;
    let mut index = 0;
    while index < wire_writer.elements.len(){
        let x_ = wire_writer.elements[index].0 as i32;
        let y_ = wire_writer.elements[index].1 as i32;
        let sides: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for side in sides{
            if are_coordinates_in_bounds(x_ + side.0, y_ + side.1) && component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == ComponentType::WRITE_TO_WIRE && component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to == -1{
                wire_writer.elements.push(((x_ + side.0) as usize, (y_ + side.1) as usize));
                component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to = wire_writer_index as i32;
            }
        }
        index += 1;
    }
}

fn new_logic_gate_group(component_data: &mut ComponentData, x: usize, y: usize){
    let component_type_index = component_data.array[x][y].component_type;
    let logic_gate_index = component_data.logic_gates.len();
    component_data.logic_gates.push(LogicGate::default());
    component_data.logic_gates[logic_gate_index].gate_type = component_type_index;
    let logic_gate: &mut LogicGate = &mut component_data.logic_gates[logic_gate_index];
    logic_gate.elements.push((x, y));
    component_data.array[x][y].belongs_to = logic_gate_index as i32;
    let mut index = 0;
    while index < logic_gate.elements.len(){
        let x_ = logic_gate.elements[index].0 as i32;
        let y_ = logic_gate.elements[index].1 as i32;
        let sides: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for side in sides{
            if are_coordinates_in_bounds(x_ + side.0, y_ + side.1) && component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == component_type_index && component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to == -1{
                logic_gate.elements.push(((x_ + side.0) as usize, (y_ + side.1) as usize));
                component_data.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to = logic_gate_index as i32;
            }
        }
        index += 1;
    }
}

fn are_coordinates_in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < WIDTH as i32 && y < HEIGHT as i32
}