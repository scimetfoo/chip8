pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_RAM_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
        let mut emulator = Self::default_emulator_state();
        emulator.initialize_ram();
        emulator
    }

    pub fn reset(&mut self) {
        *self = Self::default_emulator_state();
        self.initialize_ram();
    }

    fn default_emulator_state() -> Self {
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

    fn initialize_ram(&mut self) {
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    fn push(&mut self, val: u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.program_counter as usize] as u16;
        let lower_byte = self.ram[(self.program_counter + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.program_counter += 2;
        op
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {}
            self.sound_timer -= 1;
        }
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        //self.execute(op);
    }
}

#[cfg(test)]
mod tests {
    use crate::Emulator;

    #[test]
    fn stack_ops() {
        let mut emulator = Emulator::new();
        Emulator::push(&mut emulator, 1);
        Emulator::push(&mut emulator, 2);
        Emulator::pop(&mut emulator);

        assert_eq!(emulator.stack[0..3], [1u16, 2, 0]);
    }

    #[test]
    fn test_fetch_op() {
        let mut emulator = Emulator::new();
        assert_eq!(emulator.fetch(), 0);
    }
}
