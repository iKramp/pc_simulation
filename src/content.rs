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

pub struct WireWriter{
    pub enabled: bool,
    pub to_update: bool,
    pub elements: Vec<(usize, usize)>,
    pub wires: Vec<u32>,
    pub logic_gates: Vec<u32>
}

impl WireWriter {
    pub fn default() -> Self {
        Self{
            enabled: false,
            to_update: true,
            elements: vec![],
            wires: vec![],
            logic_gates: vec![],
        }
    }
}


pub(crate) struct LogicGate{
    pub enabled: bool,
    pub to_update: bool,
    pub gate_type: ComponentType,
    pub elements: Vec<(usize, usize)>,
    pub wire_writers: Vec<u32>,
    pub wire_readers: Vec<u32>
}

impl Default for LogicGate {
    fn default() -> Self {
        Self{
            enabled: false,
            to_update: true,
            gate_type: ComponentType::NOTHING,
            elements: vec![],
            wire_readers: vec![],
            wire_writers: vec![],
        }
    }
}

pub(crate) struct WireReader{
    pub enabled: bool,
    pub to_update: bool,
    pub elements: Vec<(usize, usize)>,
    pub logic_gates: Vec<u32>,
    pub wires: Vec<u32>
}

impl Default for WireReader {
    fn default() -> Self {
        Self{
            enabled: false,
            to_update: true,
            elements: vec![],
            logic_gates: vec![],
            wires: vec![],
        }
    }
}

pub(crate) struct Wire{
    pub enabled: bool,
    pub to_update: bool,
    pub elements: Vec<(usize, usize)>,
    pub wire_readers: Vec<u32>,
    pub wire_writers: Vec<u32>
}

impl Default for Wire {
    fn default() -> Self {
        Self{
            enabled: false,
            to_update: true,
            elements: vec![],
            wire_writers: vec![],
            wire_readers: vec![],
        }
    }
}

#[allow(non_camel_case_types)]
pub struct ComponentData{
    pub array: Vec<[Component; HEIGHT as usize]>,
    pub to_update: Vec<(usize, usize)>,
    pub(crate) wires: Vec<Wire>,
    pub(crate) wire_readers: Vec<WireReader>,
    pub(crate) wire_writers: Vec<WireWriter>,
    pub(crate) logic_gates: Vec<LogicGate>,
    pub position_on_screen: (f32, f32),
    pub zoom: f32,
}

impl ComponentData{
    pub fn default() -> Self {
        ComponentData{
            array: vec![[Component{component_type: ComponentType::NOTHING, belongs_to: -1}; HEIGHT as usize]; WIDTH as usize],
            to_update: vec![],
            wires: vec![],
            wire_readers: vec![],
            wire_writers: vec![],
            logic_gates: vec![],
            position_on_screen: (0.0, 0.0),
            zoom: 1.0
        }
    }

    pub fn translate_mouse_pos(&self, mouse_x: f32, mouse_y: f32) -> (i32, i32){
        (((mouse_x - self.position_on_screen.0) / self.zoom - 0.5).round() as i32, ((mouse_y - self.position_on_screen.1) / self.zoom - 0.5).round() as i32)
    }

    pub fn update_canvas(&mut self){
        let mut lock_array: Vec<usize> = vec![];
        self.lock_latches(&mut lock_array);

        self.update_reader();
        for i in 0..self.wire_readers.len(){
            self.wire_readers[i].to_update = false;
        }
        for i in 0..lock_array.len(){
            self.logic_gates[lock_array[i] as usize].to_update = false;
        }
        self.update_logic();
        for i in 0..self.logic_gates.len(){
            if self.logic_gates[i].gate_type != ComponentType::CLOCK {
                self.logic_gates[i].to_update = false;
            }
        }
        self.update_writer();
        for i in 0..self.wire_writers.len(){
            self.wire_writers[i].to_update = false;
        }
        self.update_wire();
        for i in 0..self.wires.len(){
            self.wires[i].to_update = false;
        }
    }

    fn update_logic(&mut self){
        for i in 0..self.logic_gates.len(){
            if !self.logic_gates[i].to_update{
                continue;
            }
            let previous_state = self.logic_gates[i].enabled;
            let mut should_turn_on = false;

            match self.logic_gates[i].gate_type {
                ComponentType::OR    => { should_turn_on = self.should_or_turn_on   (i); }
                ComponentType::AND   => { should_turn_on = self.should_and_turn_on  (i); }
                ComponentType::XOR   => { should_turn_on = self.should_xor_turn_on  (i); }
                ComponentType::NOT   => { should_turn_on = self.should_not_turn_on  (i); }
                ComponentType::NAND  => { should_turn_on = self.should_nand_turn_on (i); }
                ComponentType::XNOR  => { should_turn_on = self.should_xnor_turn_on (i); }
                ComponentType::CLOCK => { should_turn_on = self.should_clock_turn_on(i); }
                ComponentType::LATCH => { should_turn_on = self.should_latch_turn_on(i); }
                ComponentType::LIGHT => { should_turn_on = self.should_or_turn_on   (i); }
                _ => {}
            }

            if previous_state != should_turn_on{
                self.logic_gates[i].enabled = should_turn_on;
                for j in 0..self.logic_gates[i].wire_writers.len(){
                    self.wire_writers[self.logic_gates[i].wire_writers[j] as usize].to_update = true;
                }
            }
        }
    }

    fn update_wire(&mut self){
        for i in 0..self.wires.len(){
            if !self.wires[i].to_update{
                continue;
            }
            let previous_state = self.wires[i].enabled;
            let mut should_turn_on = false;
            for j in 0..self.wires[i].wire_writers.len(){
                should_turn_on = should_turn_on || self.wire_writers[self.wires[i].wire_writers[j] as usize].enabled;
            }
            if previous_state != should_turn_on{
                self.wires[i].enabled = should_turn_on;
                for j in 0..self.wires[i].wire_readers.len(){
                    self.wire_readers[self.wires[i].wire_readers[j] as usize].to_update = true;
                }
            }
        }
    }

    fn update_writer(&mut self){
        for i in 0..self.wire_writers.len(){
            if !self.wire_writers[i].to_update{
                continue;
            }
            let previous_state = self.wire_writers[i].enabled;
            let mut should_turn_on = false;
            for j in 0..self.wire_writers[i].logic_gates.len(){
                should_turn_on = should_turn_on || self.logic_gates[self.wire_writers[i].logic_gates[j] as usize].enabled;
            }
            if previous_state != should_turn_on{
                self.wire_writers[i].enabled = should_turn_on;
                for j in 0..self.wire_writers[i].wires.len(){
                    self.wires[self.wire_writers[i].wires[j] as usize].to_update = true;
                }
            }
        }
    }

    fn update_reader(&mut self){
        for i in 0..self.wire_readers.len(){
            if !self.wire_readers[i].to_update{
                continue;
            }
            let previous_state = self.wire_readers[i].enabled;
            let mut should_turn_on = false;
            for j in 0..self.wire_readers[i].wires.len(){
                should_turn_on = should_turn_on || self.wires[self.wire_readers[i].wires[j] as usize].enabled;
            }
            if previous_state != should_turn_on{
                self.wire_readers[i].enabled = should_turn_on;
                for j in 0..self.wire_readers[i].logic_gates.len(){
                    self.logic_gates[self.wire_readers[i].logic_gates[j] as usize].to_update = true;
                }
            }
        }
    }

    fn should_not_turn_on  (&self, gate_index: usize) -> bool {
        for i in 0..self.logic_gates[gate_index].wire_readers.len(){
            if self.wire_readers[self.logic_gates[gate_index].wire_readers[i] as usize].enabled{
                return false;
            }
        }
        return true;
    }

    fn should_or_turn_on   (&self, gate_index: usize) -> bool {
        for i in 0..self.logic_gates[gate_index].wire_readers.len(){
            if self.wire_readers[self.logic_gates[gate_index].wire_readers[i] as usize].enabled{
                return true;
            }
        }
        return false;
    }

    fn should_and_turn_on  (&self, gate_index: usize) -> bool {
        if self.logic_gates[gate_index].wire_readers.len() == 0{
            return false;
        }
        for i in 0..self.logic_gates[gate_index].wire_readers.len(){
            if !self.wire_readers[self.logic_gates[gate_index].wire_readers[i] as usize].enabled{
                return false;
            }
        }
        return true;
    }

    fn should_nand_turn_on (&self, gate_index: usize) -> bool {
        if self.logic_gates[gate_index].wire_readers.len() == 0{
            return true;
        }
        for i in 0..self.logic_gates[gate_index].wire_readers.len(){
            if !self.wire_readers[self.logic_gates[gate_index].wire_readers[i] as usize].enabled{
                return true;
            }
        }
        return false;
    }

    fn should_xor_turn_on  (&self, gate_index: usize) -> bool {
        let mut state = false;
        for i in 0..self.logic_gates[gate_index].wire_readers.len(){
            if self.wire_readers[self.logic_gates[gate_index].wire_readers[i] as usize].enabled{
                state = !state;
            }
        }
        state
    }

    fn should_xnor_turn_on (&self, gate_index: usize) -> bool {
        let mut state = false;
        for i in 0..self.logic_gates[gate_index].wire_readers.len(){
            if self.wire_readers[self.logic_gates[gate_index].wire_readers[i] as usize].enabled{
                state = !state;
            }
        }
        !state
    }

    fn should_clock_turn_on(&self, gate_index: usize) -> bool {
        !self.logic_gates[gate_index].enabled
    }

    fn should_latch_turn_on(&self, gate_index: usize) -> bool {
        for i in 0..self.logic_gates[gate_index].wire_readers.len(){
            if self.wire_readers[self.logic_gates[gate_index].wire_readers[i] as usize].enabled{
                return !self.logic_gates[gate_index].enabled
            }
        }
        self.logic_gates[gate_index].enabled
    }

    fn lock_latches(&mut self, lock_array: &mut Vec<usize>) {
        for i in 0..self.logic_gates.len(){
            if self.logic_gates[i].gate_type == ComponentType::LATCH{
                for j in 0..self.logic_gates[i].wire_readers.len(){
                    if self.wire_readers[self.logic_gates[i].wire_readers[j] as usize].enabled{
                        lock_array.push(i);
                    }
                }
            }
        }
    }
}