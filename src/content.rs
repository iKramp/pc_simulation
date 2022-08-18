pub const WIDTH: u32 = 700;
pub const HEIGHT: u32 = 300;
pub const SIZE: i32 = 0;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq)]
pub enum ComponentType {NOTHING, WRITE_TO_WIRE, WIRE, CROSS, READ_FROM_WIRE, AND, OR, XOR, NOT, NAND, XNOR, COMMENT, CLOCK, LATCH, LIGHT, NUM_COMPONENTS}

impl ComponentType{
    pub fn from_u32(val: u32) -> ComponentType{
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
            14 => ComponentType::LIGHT,
            _ => ComponentType::NOTHING,
        }
    }
}

pub const COLORS: [((u8, u8, u8), (u8, u8, u8)); 15] =//dark wires
    [((031, 037, 049), (031, 037, 049)),
        ((085, 062, 071), (255, 113, 113)),
        ((099, 097, 079), (177, 177, 051)),//((099, 097, 079), (251, 251, 074)),
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
        ((061, 085, 081), (110, 251, 183)),
        ((100, 100, 100), (255, 255, 255))];

pub const NAMES: [&str; 15] =  [
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
    "latch",
    "light"
];

#[derive(Clone, Copy)]
pub struct Component{
    pub component_type: ComponentType,
    pub belongs_to: i32,
}

pub struct MiscData{
    pub selected_type: ComponentType,
    pub run_sim: bool,
    pub stopwatch: stopwatch::Stopwatch,
    pub last_time: i64,
    pub last_mouse_pos: (i32, i32),
    pub mouse_pos_on_middle_press: (i32, i32),
    pub shift_pressed: bool,
    pub control_pressed: bool,
    pub copy: bool,
    pub paste: (bool, (i32, i32)),
    pub selection: ((i32, i32), (i32, i32)),
    pub copied_data: Vec<Vec<u8>>
}

impl MiscData{
    pub fn default() -> Self {
        Self{
            selected_type: ComponentType::WIRE,
            run_sim: false,
            stopwatch: stopwatch::Stopwatch::start_new(),
            last_time: 0,
            last_mouse_pos: (0, 0),
            mouse_pos_on_middle_press: (0, 0),
            shift_pressed: false,
            control_pressed: false,
            copy: false,
            paste: (false, (0, 0)),
            selection: ((0, 0), (0, 0)),
            copied_data: vec![],
        }
    }
}

pub(crate) struct LogicComponent{
    pub enabled: bool,
    pub to_update: bool,
    pub gate_type: ComponentType,
    pub elements: Vec<(usize, usize)>,
    pub component_before: Vec<u32>,
    pub component_after: Vec<u32>
}

impl Default for LogicComponent {
    fn default() -> Self {
        Self{
            enabled: false,
            to_update: true,
            gate_type: ComponentType::NOTHING,
            elements: vec![],
            component_before: vec![],
            component_after: vec![],
        }
    }
}

#[allow(non_camel_case_types)]
pub struct ComponentData{
    pub array: Vec<[Component; HEIGHT as usize]>,
    pub to_update: Vec<(usize, usize)>,
    pub(crate) logic_components: Vec<LogicComponent>,
    pub position_on_screen: (f32, f32),
    pub zoom: f32,
}

impl ComponentData{
    pub fn default() -> Self {
        ComponentData{
            array: vec![[Component{component_type: ComponentType::NOTHING, belongs_to: -1}; HEIGHT as usize]; WIDTH as usize],
            to_update: vec![],
            logic_components: vec![],
            position_on_screen: (0.0, 0.0),
            zoom: 1.0
        }
    }

    pub(crate) fn compile_scene(&mut self){
        for i in 0..WIDTH as usize{
            for j in 0..HEIGHT as usize{
                if self.array[i][j].component_type != ComponentType::NOTHING && self.array[i][j].belongs_to == -1{
                    self.new_group(i, j);
                }
            }
        }
        for i in 0..WIDTH as usize{
            for j in 0..HEIGHT as usize{
                if self.array[i][j].component_type != ComponentType::NOTHING {
                    self.link_components(i as i32, j as i32);
                }
            }
        }
    }

    pub(crate) fn clear_compiled_data(&mut self){
        for i in 0..self.array.len(){
            for j in 0..self.array[0].len(){
                self.array[i][j].belongs_to = -1;
            }
        }
        self.logic_components.clear();
    }

    pub fn translate_mouse_pos(&self, mouse_x: f32, mouse_y: f32) -> (i32, i32){
        (((mouse_x - self.position_on_screen.0) / self.zoom - 0.5).round() as i32, ((mouse_y - self.position_on_screen.1) / self.zoom - 0.5).round() as i32)
    }

    pub fn update_canvas(&mut self){
        let mut lock_array: Vec<usize> = vec![];
        self.lock_latches(&mut lock_array);

        self.update_component(ComponentType::READ_FROM_WIRE);//add to func: to_update = false

        for i in 0..lock_array.len(){
            self.logic_components[lock_array[i] as usize].to_update = false;
        }
        let logic_gates = [ComponentType::AND, ComponentType::OR, ComponentType::XOR, ComponentType::NOT, ComponentType::NAND, ComponentType::XNOR, ComponentType::CLOCK, ComponentType::LATCH, ComponentType::LIGHT];
        for gate in logic_gates{
            self.update_component(gate);
        }
        self.update_component(ComponentType::WRITE_TO_WIRE);//add to func: to_update = false
        self.update_component(ComponentType::WIRE);//add to func: to_update = false
    }

    fn update_component(&mut self, component_type: ComponentType){
        for i in 0..self.logic_components.len(){
            if self.logic_components[i].gate_type != component_type || !self.logic_components[i].to_update{
                continue;
            }
            let previous_state = self.logic_components[i].enabled;
            let mut should_turn_on = false;

            match self.logic_components[i].gate_type {
                ComponentType::OR    => { should_turn_on = self.should_or_turn_on   (i); }
                ComponentType::AND   => { should_turn_on = self.should_and_turn_on  (i); }
                ComponentType::XOR   => { should_turn_on = self.should_xor_turn_on  (i); }
                ComponentType::NOT   => { should_turn_on = self.should_not_turn_on  (i); }
                ComponentType::NAND  => { should_turn_on = self.should_nand_turn_on (i); }
                ComponentType::XNOR  => { should_turn_on = self.should_xnor_turn_on (i); }
                ComponentType::CLOCK => { should_turn_on = self.should_clock_turn_on(i); }
                ComponentType::LATCH => { should_turn_on = self.should_latch_turn_on(i); }
                ComponentType::LIGHT => { should_turn_on = self.should_or_turn_on   (i); }
                _ => {
                    for j in 0..self.logic_components[i].component_before.len(){
                        should_turn_on = should_turn_on || self.logic_components[self.logic_components[i].component_before[j] as usize].enabled;
                    }
                }
            }

            if previous_state != should_turn_on{
                self.logic_components[i].enabled = should_turn_on;
                for j in 0..self.logic_components[i].component_after.len(){
                    let index = self.logic_components[i].component_after[j] as usize;
                    self.logic_components[index].to_update = true;
                }
            }
            if self.logic_components[i].gate_type != ComponentType::CLOCK{
                self.logic_components[i].to_update = false;
            }
        }
    }

    fn should_not_turn_on  (&self, gate_index: usize) -> bool {
        for i in 0..self.logic_components[gate_index].component_before.len(){
            if self.logic_components[self.logic_components[gate_index].component_before[i] as usize].enabled{
                return false;
            }
        }
        return true;
    }

    fn should_or_turn_on   (&self, gate_index: usize) -> bool {
        for i in 0..self.logic_components[gate_index].component_before.len(){
            if self.logic_components[self.logic_components[gate_index].component_before[i] as usize].enabled{
                return true;
            }
        }
        return false;
    }

    fn should_and_turn_on  (&self, gate_index: usize) -> bool {
        if self.logic_components[gate_index].component_before.len() == 0{
            return false;
        }
        for i in 0..self.logic_components[gate_index].component_before.len(){
            if !self.logic_components[self.logic_components[gate_index].component_before[i] as usize].enabled{
                return false;
            }
        }
        return true;
    }

    fn should_nand_turn_on (&self, gate_index: usize) -> bool {
        if self.logic_components[gate_index].component_before.len() == 0{
            return true;
        }
        for i in 0..self.logic_components[gate_index].component_before.len(){
            if !self.logic_components[self.logic_components[gate_index].component_before[i] as usize].enabled{
                return true;
            }
        }
        return false;
    }

    fn should_xor_turn_on  (&self, gate_index: usize) -> bool {
        let mut state = false;
        for i in 0..self.logic_components[gate_index].component_before.len(){
            if self.logic_components[self.logic_components[gate_index].component_before[i] as usize].enabled{
                state = !state;
            }
        }
        state
    }

    fn should_xnor_turn_on (&self, gate_index: usize) -> bool {
        let mut state = false;
        for i in 0..self.logic_components[gate_index].component_before.len(){
            if self.logic_components[self.logic_components[gate_index].component_before[i] as usize].enabled{
                state = !state;
            }
        }
        !state
    }

    fn should_clock_turn_on(&self, gate_index: usize) -> bool {
        !self.logic_components[gate_index].enabled
    }

    fn should_latch_turn_on(&self, gate_index: usize) -> bool {
        for i in 0..self.logic_components[gate_index].component_before.len(){
            if self.logic_components[self.logic_components[gate_index].component_before[i] as usize].enabled{
                return !self.logic_components[gate_index].enabled
            }
        }
        self.logic_components[gate_index].enabled
    }

    fn lock_latches(&self, lock_array: &mut Vec<usize>) {
        for i in 0..self.logic_components.len(){
            if self.logic_components[i].gate_type == ComponentType::LATCH{
                for j in 0..self.logic_components[i].component_before.len(){
                    if self.logic_components[self.logic_components[i].component_before[j] as usize].enabled{
                        lock_array.push(i);
                    }
                }
            }
        }
    }

    fn new_group(&mut self, x: usize, y: usize){
        if self.array[x][y].component_type == ComponentType::WIRE {
            self.new_wire_group(x, y);
        }else if self.array[x][y].component_type == ComponentType::READ_FROM_WIRE {
            self.new_wire_reader_group(x, y);
        }else if self.array[x][y].component_type == ComponentType::WRITE_TO_WIRE {
            self.new_wire_writer_group(x, y);
        }else {
            self.new_logic_gate_group(x, y);
        }
    }

    fn new_wire_group(&mut self, x: usize, y: usize){
        self.logic_components.push(LogicComponent::default());
        let index = self.logic_components.len() - 1;
        self.logic_components[index].gate_type = ComponentType::WIRE;
        let wire_index = self.logic_components.len() - 1;
        let wire: &mut LogicComponent = &mut self.logic_components[wire_index];
        wire.elements.push((x, y));
        self.array[x][y].belongs_to = wire_index as i32;
        let mut index = 0;
        while index < wire.elements.len(){
            let x_ = wire.elements[index].0 as i32;
            let y_ = wire.elements[index].1 as i32;
            let sides: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for side in sides{
                if ComponentData::are_coordinates_in_bounds(x_ + side.0, y_ + side.1) && self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == ComponentType::WIRE && self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to == -1{
                    wire.elements.push(((x_ + side.0) as usize, (y_ + side.1) as usize));
                    self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to = wire_index as i32;
                }
                if ComponentData::are_coordinates_in_bounds(x_ + side.0 * 2, y_ + side.1 * 2) && self.array[(x_ + (side.0 * 2)) as usize][(y_ + (side.1 * 2)) as usize].component_type == ComponentType::WIRE && self.array[(x_ + (side.0 * 2)) as usize][(y_ + (side.1 * 2)) as usize].belongs_to == -1 &&
                    self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == ComponentType::CROSS{
                    wire.elements.push(((x_ + (side.0 * 2)) as usize, (y_ + (side.1 * 2)) as usize));
                    self.array[(x_ + (side.0 * 2)) as usize][(y_ + (side.1 * 2)) as usize].belongs_to = wire_index as i32;
                }
            }
            index += 1;
        }
    }

    fn new_wire_reader_group(&mut self, x: usize, y: usize){
        self.logic_components.push(LogicComponent::default());
        let index = self.logic_components.len() - 1;
        self.logic_components[index].gate_type = ComponentType::READ_FROM_WIRE;
        let wire_reader_index = self.logic_components.len() - 1;
        let wire_reader: &mut LogicComponent = &mut self.logic_components[wire_reader_index];
        wire_reader.elements.push((x, y));
        self.array[x][y].belongs_to = wire_reader_index as i32;
        let mut index = 0;
        while index < wire_reader.elements.len(){
            let x_ = wire_reader.elements[index].0 as i32;
            let y_ = wire_reader.elements[index].1 as i32;
            let sides: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for side in sides{
                if ComponentData::are_coordinates_in_bounds(x_ + side.0, y_ + side.1) && self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == ComponentType::READ_FROM_WIRE && self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to == -1{
                    wire_reader.elements.push(((x_ + side.0) as usize, (y_ + side.1) as usize));
                    self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to = wire_reader_index as i32;
                }
            }
            index += 1;
        }
    }

    fn new_wire_writer_group(&mut self, x: usize, y: usize){
        self.logic_components.push(LogicComponent::default());
        let index = self.logic_components.len() - 1;
        self.logic_components[index].gate_type = ComponentType::WRITE_TO_WIRE;
        let wire_writer_index = self.logic_components.len() - 1;
        let wire_writer: &mut LogicComponent = &mut self.logic_components[wire_writer_index];
        wire_writer.elements.push((x, y));
        self.array[x][y].belongs_to = wire_writer_index as i32;
        let mut index = 0;
        while index < wire_writer.elements.len(){
            let x_ = wire_writer.elements[index].0 as i32;
            let y_ = wire_writer.elements[index].1 as i32;
            let sides: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for side in sides{
                if ComponentData::are_coordinates_in_bounds(x_ + side.0, y_ + side.1) && self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == ComponentType::WRITE_TO_WIRE && self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to == -1{
                    wire_writer.elements.push(((x_ + side.0) as usize, (y_ + side.1) as usize));
                    self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to = wire_writer_index as i32;
                }
            }
            index += 1;
        }
    }

    fn new_logic_gate_group(&mut self, x: usize, y: usize){
        let component_type_index = self.array[x][y].component_type;
        let logic_gate_index = self.logic_components.len();
        self.logic_components.push(LogicComponent::default());
        self.logic_components[logic_gate_index].gate_type = component_type_index;
        let logic_gate: &mut LogicComponent = &mut self.logic_components[logic_gate_index];
        logic_gate.elements.push((x, y));
        self.array[x][y].belongs_to = logic_gate_index as i32;
        let mut index = 0;
        while index < logic_gate.elements.len(){
            let x_ = logic_gate.elements[index].0 as i32;
            let y_ = logic_gate.elements[index].1 as i32;
            let sides: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for side in sides{
                if ComponentData::are_coordinates_in_bounds(x_ + side.0, y_ + side.1) && self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].component_type == component_type_index && self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to == -1{
                    logic_gate.elements.push(((x_ + side.0) as usize, (y_ + side.1) as usize));
                    self.array[(x_ + side.0) as usize][(y_ + side.1) as usize].belongs_to = logic_gate_index as i32;
                }
            }
            index += 1;
        }
    }

    fn link_components(&mut self, x: i32, y: i32){
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        if self.array[x as usize][y as usize].component_type == ComponentType::WIRE{
            for direction in directions{
                if !ComponentData::are_coordinates_in_bounds(x + direction.0, y + direction.1){
                    continue;
                }
                if self.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type == ComponentType::READ_FROM_WIRE{
                    self.link_wire_read(x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
                }
            }
        }
        if self.array[x as usize][y as usize].component_type == ComponentType::READ_FROM_WIRE{
            for direction in directions{
                if !ComponentData::are_coordinates_in_bounds(x + direction.0, y + direction.1){
                    continue;
                }
                if self.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type as u32 > 4 && (self.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type as u32) < ComponentType::NUM_COMPONENTS as u32{
                    self.link_read_logic(x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
                }
            }
        }
        if self.array[x as usize][y as usize].component_type as u32 > 4 && (self.array[x as usize][y as usize].component_type as u32) < ComponentType::NUM_COMPONENTS as u32{
            for direction in directions{
                if !ComponentData::are_coordinates_in_bounds(x + direction.0, y + direction.1){
                    continue;
                }
                if self.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type == ComponentType::WRITE_TO_WIRE{
                    self.link_logic_write(x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
                }
            }
        }
        if self.array[x as usize][y as usize].component_type == ComponentType::WRITE_TO_WIRE{
            for direction in directions{
                if !ComponentData::are_coordinates_in_bounds(x + direction.0, y + direction.1){
                    continue;
                }
                if self.array[(x + direction.0) as usize][(y + direction.1) as usize].component_type == ComponentType::WIRE{
                    self.link_write_wire(x as usize, y as usize, (x + direction.0) as usize, (y + direction.1) as usize);
                }
            }
        }
    }

    fn link_wire_read(&mut self, x1: usize, y1: usize, x2: usize, y2: usize){
        if !self.logic_components[self.array[x1][y1].belongs_to as usize].component_after.contains(&(self.array[x2][y2].belongs_to as u32)){
            self.logic_components[self.array[x1][y1].belongs_to as usize].component_after.push(self.array[x2][y2].belongs_to as u32);
        }
        if !self.logic_components[self.array[x2][y2].belongs_to as usize].component_before.contains(&(self.array[x1][y1].belongs_to as u32)){
            self.logic_components[self.array[x2][y2].belongs_to as usize].component_before.push(self.array[x1][y1].belongs_to as u32);
        }
    }

    fn link_read_logic(&mut self, x1: usize, y1: usize, x2: usize, y2: usize){
        if !self.logic_components[self.array[x1][y1].belongs_to as usize].component_after.contains(&(self.array[x2][y2].belongs_to as u32)){
            self.logic_components[self.array[x1][y1].belongs_to as usize].component_after.push(self.array[x2][y2].belongs_to as u32);
        }
        if !self.logic_components[self.array[x2][y2].belongs_to as usize].component_before.contains(&(self.array[x1][y1].belongs_to as u32)){
            self.logic_components[self.array[x2][y2].belongs_to as usize].component_before.push(self.array[x1][y1].belongs_to as u32);
        }
    }

    fn link_logic_write(&mut self, x1: usize, y1: usize, x2: usize, y2: usize){
        if !self.logic_components[self.array[x1][y1].belongs_to as usize].component_after.contains(&(self.array[x2][y2].belongs_to as u32)){
            self.logic_components[self.array[x1][y1].belongs_to as usize].component_after.push(self.array[x2][y2].belongs_to as u32);
        }
        if !self.logic_components[self.array[x2][y2].belongs_to as usize].component_before.contains(&(self.array[x1][y1].belongs_to as u32)){
            self.logic_components[self.array[x2][y2].belongs_to as usize].component_before.push(self.array[x1][y1].belongs_to as u32);
        }
    }

    fn link_write_wire(&mut self, x1: usize, y1: usize, x2: usize, y2: usize){
        if !self.logic_components[self.array[x1][y1].belongs_to as usize].component_after.contains(&(self.array[x2][y2].belongs_to as u32)){
            self.logic_components[self.array[x1][y1].belongs_to as usize].component_after.push(self.array[x2][y2].belongs_to as u32);
        }
        if !self.logic_components[self.array[x2][y2].belongs_to as usize].component_before.contains(&(self.array[x1][y1].belongs_to as u32)){
            self.logic_components[self.array[x2][y2].belongs_to as usize].component_before.push(self.array[x1][y1].belongs_to as u32);
        }
    }

    pub(crate) fn are_coordinates_in_bounds(x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < WIDTH as i32 && y < HEIGHT as i32
    }
}