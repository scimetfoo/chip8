pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_RAM_ADDR: u16 = 0x200;

pub struct Emulator {
    program_counter: u16,
    ram: [u8; RAM_SIZE],
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    v_registers: [u8; NUM_REGISTERS],
    i_registers: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            program_counter: START_RAM_ADDR,
            ram: [0; RAM_SIZE],
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            v_registers: [0; NUM_REGISTERS],
            i_registers: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
