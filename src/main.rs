pub mod content;
mod render;

extern crate sdl2;
extern crate stopwatch;

use crate::content::{HEIGHT, SIZE, WIDTH, ComponentType, COLORS, NAMES, Component, ComponentData, MiscData};


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
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("pc sim", WIDTH * 2, HEIGHT * 2)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();//code above inits sdl2 somehow, idk what it does

    load_array(&mut component_data);
    draw_canvas(&mut component_data, & mut canvas, false);//initial draw


    main_update(&mut canvas, &mut event_pump, &mut component_data);//loop


    save_array(&component_data);//end of program
}

fn main_update(canvas: &mut sdl2::render::WindowCanvas, event_pump: &mut sdl2::EventPump, mut component_data: &mut ComponentData){
    let mut misc_data = MiscData::default();
    'running: loop {
        misc_data.shift_pressed = event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::LShift);
        misc_data.control_pressed = event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::LCtrl);
        let mouse_x = event_pump.mouse_state().x() / 2;
        let mouse_y = event_pump.mouse_state().y() / 2;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    if misc_data.paste.0  {
                        misc_data.paste.0 = false;
                    }else {
                        break 'running
                    }
                },
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Plus), ..} => {
                    misc_data.selected_type = ComponentType::from_u32(misc_data.selected_type as u32 % (ComponentType::NUM_COMPONENTS as u32 - 1) + 1);
                    println!("{}", NAMES[misc_data.selected_type as usize]);
                },
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Minus), ..} => {
                    if misc_data.selected_type as u32 == 1{
                        misc_data.selected_type = ComponentType::from_u32(ComponentType::NUM_COMPONENTS as u32 - 1);
                    }else {
                        misc_data.selected_type = ComponentType::from_u32(misc_data.selected_type as u32 - 1);
                    }
                    println!("{}", NAMES[misc_data.selected_type as usize]);
                },
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Space), ..} => {
                    if misc_data.run_sim {
                        component_data.clear_compiled_data();
                    }else{
                        misc_data.paste.0 = false;
                        component_data.compile_scene();
                    }
                    misc_data.run_sim = !misc_data.run_sim;
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
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::V), ..} => {
                    if !misc_data.run_sim {
                        if misc_data.control_pressed {
                            misc_data.paste.0 = !misc_data.paste.0;
                            misc_data.paste.1 = component_data.translate_mouse_pos( mouse_x as f32, mouse_y as f32);
                        }
                    }
                }
                sdl2::event::Event::KeyDown {keycode: Some(sdl2::keyboard::Keycode::Delete), ..} => {
                    for i in misc_data.selection.0.0..misc_data.selection.1.0 {
                        for j in misc_data.selection.0.1..misc_data.selection.1.1 {
                            component_data.array[i as usize][j as usize].component_type = ComponentType::NOTHING
                        }
                    }
                }
                sdl2::event::Event::MouseButtonDown {mouse_btn: sdl2::mouse::MouseButton::Left, ..} => {
                    if misc_data.paste.0{
                        paste_selection(&mut component_data, &mut misc_data.copied_data, misc_data.paste.1.0, misc_data.paste.1.1);
                    }else {
                        if misc_data.run_sim {
                            let pos = component_data.translate_mouse_pos(mouse_x as f32, mouse_y as f32);
                            if component_data.array[pos.0 as usize][pos.1 as usize].component_type as u32 == ComponentType::LATCH as u32 && component_data.array[pos.0 as usize][pos.1 as usize].belongs_to != -1 {
                                component_data.logic_gates[component_data.array[pos.0 as usize][pos.1 as usize].belongs_to as usize].enabled = !component_data.logic_gates[component_data.array[pos.0 as usize][pos.1 as usize].belongs_to as usize].enabled;
                                for writer in &component_data.logic_gates[component_data.array[pos.0 as usize][pos.1 as usize].belongs_to as usize].wire_writers {
                                    component_data.wire_writers[*writer as usize].to_update = true;
                                }
                            }
                        } else if misc_data.shift_pressed {
                            misc_data.copy = true;
                            misc_data.selection.0 = component_data.translate_mouse_pos( mouse_x as f32, mouse_y as f32);
                            misc_data.selection.1 = misc_data.selection.0;
                        }
                    }

                }
                sdl2::event::Event::MouseButtonDown {mouse_btn: sdl2::mouse::MouseButton::Middle, ..} => {
                    misc_data.last_mouse_pos.0 = mouse_x;
                    misc_data.last_mouse_pos.1 = mouse_y;
                    misc_data.mouse_pos_on_middle_press.0 = mouse_x;
                    misc_data.mouse_pos_on_middle_press.1 = mouse_y;
                }
                sdl2::event::Event::MouseButtonUp {mouse_btn: sdl2::mouse::MouseButton::Left, ..} => {
                    if misc_data.copy {
                        misc_data.copy = false;
                        prepare_selection(&mut misc_data.selection);
                        copy_selection(component_data, &mut misc_data.selection, &mut misc_data.copied_data);
                    }
                    if misc_data.paste.0 {
                        misc_data.paste.0 = false;
                    }
                }
                sdl2::event::Event::MouseButtonUp {mouse_btn: sdl2::mouse::MouseButton::Middle, ..} => {
                    if misc_data.mouse_pos_on_middle_press.0 == mouse_x && misc_data.mouse_pos_on_middle_press.1 == mouse_y{
                        let pos = component_data.translate_mouse_pos(mouse_x as f32, mouse_y as f32);
                        if component_data.array[pos.0 as usize][pos.1 as usize].component_type != ComponentType::NOTHING{
                            misc_data.selected_type = component_data.array[pos.0 as usize][pos.1 as usize].component_type;
                        }
                    }
                }
                _ => {}
            }
        }
        if misc_data.run_sim /* timed update*/ {
            if misc_data.stopwatch.elapsed_ms() - misc_data.last_time > 1 {
                misc_data.last_time = misc_data.stopwatch.elapsed_ms();
                component_data.update_canvas();
            }

        } else /*draw mode*/ {
            if event_pump.mouse_state().is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
                let pos = component_data.translate_mouse_pos(event_pump.mouse_state().x() as f32 / 2.0, event_pump.mouse_state().y() as f32 / 2.0);
                if misc_data.copy{
                    misc_data.selection.1 = pos;
                }else if !misc_data.paste.0 {
                    for i in std::cmp::max(pos.0 - SIZE, 0)..std::cmp::min(pos.0 + SIZE + 1, WIDTH as i32) {
                        for j in std::cmp::max(pos.1 - SIZE, 0)..std::cmp::min(pos.1 + SIZE + 1, HEIGHT as i32) {
                            draw_component(i as usize, j as usize, misc_data.selected_type, false, &mut component_data, canvas);
                        }
                    }
                }
            }
            if event_pump.mouse_state().is_mouse_button_pressed(sdl2::mouse::MouseButton::Right) {
                if !misc_data.paste.0 {
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
            let delta = ((mouse_x - misc_data.last_mouse_pos.0) as f32, (mouse_y - misc_data.last_mouse_pos.1) as f32);
            component_data.position_on_screen.0 += delta.0;
            component_data.position_on_screen.1 += delta.1;
            component_data.position_on_screen.0 = component_data.position_on_screen.0.clamp(WIDTH as f32 / 2.0 - WIDTH as f32 * component_data.zoom, WIDTH as f32 / 2.0);
            component_data.position_on_screen.1 = component_data.position_on_screen.1.clamp(HEIGHT as f32 / 2.0 - HEIGHT as f32 * component_data.zoom , HEIGHT as f32 / 2.0);
            misc_data.last_mouse_pos.0 = mouse_x;
            misc_data.last_mouse_pos.1 = mouse_y;
        }
        draw_canvas(component_data, canvas, misc_data.run_sim);
        let mut pos = component_data.translate_mouse_pos( mouse_x as f32, mouse_y as f32);

        if misc_data.paste.0 /* draw stuff to paste (hopefully transparent)*/ {
            misc_data.paste.1 = pos;
            draw_to_paste(&component_data, canvas, &misc_data.copied_data, misc_data.paste);
        }

        if !misc_data.run_sim && !misc_data.shift_pressed && !misc_data.paste.0 /*draw drawing cursor square*/ {
            pos.0 = ((pos.0 - SIZE) as f32 * component_data.zoom + component_data.position_on_screen.0) as i32 * 2;
            pos.1 = ((pos.1 - SIZE) as f32 * component_data.zoom + component_data.position_on_screen.1) as i32 * 2;
            let color = COLORS[ComponentType::COMMENT as usize].0;
            canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.0, color.1, color.2, 100));
            canvas.draw_rect(sdl2::rect::Rect::new(pos.0, pos.1, (((SIZE * 4) as f32 + 2.0) * component_data.zoom) as u32, (((SIZE * 4) as f32 + 2.0) * component_data.zoom) as u32)).expect("couldn't draw rect");
        } else if !misc_data.run_sim /*draw selection*/ {
            if misc_data.shift_pressed {
                pos = misc_data.selection.0;
            }
            pos.0 = ((pos.0) as f32 * component_data.zoom + component_data.position_on_screen.0) as i32 * 2;
            pos.1 = ((pos.1) as f32 * component_data.zoom + component_data.position_on_screen.1) as i32 * 2;
            let color = COLORS[ComponentType::COMMENT as usize].0;
            canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.0, color.1, color.2, 100));
            canvas.draw_rect(sdl2::rect::Rect::new(pos.0, pos.1, (((misc_data.selection.1.0 - misc_data.selection.0.0) as f32 * 2.0 + 2.0) * component_data.zoom) as u32, (((misc_data.selection.1.1 - misc_data.selection.0.1) as f32 * 2.0 + 2.0) * component_data.zoom) as u32)).expect("couldn't draw rect");

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

fn save_array(component_data: &ComponentData) {
    let mut temp_arr: Vec<u8> = vec![];
    for column in component_data.array.iter(){
        for element in column.iter(){
            temp_arr.push(element.component_type as u8)
        }
    }
    std::fs::write("C:/Users/Uporabnik/CLionProjects/pc_simulation/canvas.dat", temp_arr).expect("couldn't write to file");
}

fn load_array(component_data: &mut ComponentData) {
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