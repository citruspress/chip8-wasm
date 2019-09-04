use crate::chip8::opcode;
use crate::chip8::opcode::Opcode;
use crate::chip8::Screen;
use rand;
use rand::Rng;

pub struct Cpu {
    i: u16,
    pc: u16,
    register: [u8; 16],
    stack: [u16; 16],
    sp: u8,
    memory: [u8; 4096],
    soundtimer: u8,
    delaytimer: u8,
    pub screen: Screen,
}

const FONT_START: usize = 0x50;
const ROM_START: usize = 0x200;

#[cfg_attr(rustfmt, rustfmt_skip)]
static FONTS: &'static [u8] =
&[
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
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

impl Default for Cpu {
    fn default() -> Self {
        let mut memory = [0u8; 4096];
        for i in 0..FONTS.len() {
            memory[i + FONT_START] = FONTS[i];
        }
        Cpu {
            i: 0,
            pc: 0,
            register: [0u8; 16],
            stack: [0u16; 16],
            sp: 0,
            memory: memory,
            soundtimer: 0,
            delaytimer: 0,
            screen: Screen::new(),
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for i in 0..rom.len() {
            self.memory[(ROM_START + i) as usize] = rom[i];
        }

        self.pc = ROM_START as u16;
    }

    pub fn step(&mut self) {
        let opcode = self.get_opcode();
        self.pc += 2;

        self.execute(opcode);

        if self.delaytimer > 0 {
            self.delaytimer -= 1;
        }

        if self.soundtimer > 0 {
            self.soundtimer -= 1;
        }
    }

    fn get_opcode(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16
    }

    fn execute(&mut self, opcode: u16) {
        let op = opcode::decode(opcode);

        match op {
            Opcode::SYS => (),
            Opcode::CLS => self.screen.clear(),
            Opcode::RET => self.ret(),
            Opcode::JP(address) => self.jump(address),
            Opcode::CALL(address) => self.call(address),
            Opcode::SE(x, kk) => self.skip_equal(x, kk),
            Opcode::SNE(x, kk) => self.skip_not_equal(x, kk),
            Opcode::SER(x, y) => self.skip_register_equal(x, y),
            Opcode::LD(x, kk) => self.load(x, kk),
            Opcode::ADD(x, kk) => self.add(x, kk),
            Opcode::LDR(x, y) => self.load_register(x, y),
            Opcode::OR(x, y) => self.or(x, y),
            Opcode::AND(x, y) => self.and(x, y),
            Opcode::XOR(x, y) => self.xor(x, y),
            Opcode::ADDR(x, y) => self.addr(x, y),
            Opcode::SUBR(x, y) => self.subr(x, y),
            Opcode::SHR(x) => self.shr(x),
            Opcode::SUBN(x, y) => self.subn(x, y),
            Opcode::SHL(x) => self.shl(x),
            Opcode::SNER(x, y) => self.skip_not_equal_registers(x, y),
            Opcode::LDI(nnn) => self.load_i(nnn),
            Opcode::JPR(nnn) => self.jumpr(nnn),
            Opcode::RND(x, kk) => self.rnd(x, kk),
            Opcode::DRW(x, y, n) => self.draw(x, y, n),
            Opcode::SKP(x) => self.skip_when_key_pressed(x),
            Opcode::SKNP(x) => self.skip_when_key_not_pressed(x),
            Opcode::LDDT(x) => self.load_delay_timer(x),
            Opcode::LDK(x) => self.wait_for_keypress(x),
            Opcode::DTLD(x) => self.set_delay_timer(x),
            Opcode::STLD(x) => self.set_sound_timer(x),
            Opcode::ADDI(x) => self.addi(x),
            Opcode::LDF(x) => self.ldf(x),
            Opcode::LDB(x) => self.ldb(x),
            Opcode::LDIR(x) => self.ldir(x),
            Opcode::LDRI(x) => self.ldri(x),
        }
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    fn call(&mut self, address: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    fn skip_equal(&mut self, x: u8, kk: u8) {
        if self.register[x as usize] == kk {
            self.pc += 2;
        }
    }

    fn skip_not_equal(&mut self, x: u8, kk: u8) {
        if self.register[x as usize] != kk {
            self.pc += 2;
        }
    }

    fn skip_register_equal(&mut self, x: u8, y: u8) {
        if self.register[x as usize] == self.register[y as usize] {
            self.pc += 2;
        }
    }

    fn load(&mut self, x: u8, kk: u8) {
        self.register[x as usize] = kk;
    }

    fn add(&mut self, x: u8, kk: u8) {
        self.register[x as usize] += kk;
    }

    fn load_register(&mut self, x: u8, y: u8) {
        self.register[x as usize] = self.register[y as usize];
    }

    fn or(&mut self, x: u8, y: u8) {
        self.register[x as usize] |= self.register[y as usize];
    }

    fn and(&mut self, x: u8, y: u8) {
        self.register[x as usize] &= self.register[y as usize];
    }

    fn xor(&mut self, x: u8, y: u8) {
        self.register[x as usize] ^= self.register[y as usize];
    }

    fn addr(&mut self, x: u8, y: u8) {
        self.register[x as usize] += self.register[y as usize];
    }

    fn subr(&mut self, x: u8, y: u8) {
        self.register[x as usize] -= self.register[y as usize];
    }

    fn shr(&mut self, x: u8) {
        self.register[0xF] = self.register[x as usize] & 1;
        self.register[x as usize] >>= 1;
    }

    fn subn(&mut self, x: u8, y: u8) {
        if self.register[x as usize] > self.register[y as usize] {
            self.register[0xF] = 1;
        }

        self.register[x as usize] =
            self.register[y as usize].wrapping_sub(self.register[x as usize]);
    }

    fn shl(&mut self, x: u8) {
        self.register[0xF] = self.register[x as usize] >> 7;
        self.register[x as usize] <<= 1;
    }

    fn skip_not_equal_registers(&mut self, x: u8, y: u8) {
        if self.register[x as usize] != self.register[y as usize] {
            self.pc += 2;
        }
    }

    fn load_i(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn jumpr(&mut self, nnn: u16) {
        self.pc = nnn + self.register[0] as u16;
    }

    fn rnd(&mut self, x: u8, kk: u8) {
        let mut rng = rand::thread_rng();
        self.register[x as usize] = kk & rng.gen::<u8>();
    }

    fn draw(&mut self, mut x: u8, mut y: u8, n: u8) {
        x = self.register[x as usize];
        y = self.register[y as usize];

        self.register[0xF] = 0;
        let sprite_data = &self.memory[self.i as usize..(self.i + n as u16) as usize];

        if self.screen.draw_sprite(x as usize, y as usize, sprite_data) {
            self.register[0xF] = 1;
        }
    }

    fn skip_when_key_pressed(&mut self, _x: u8) {}

    fn skip_when_key_not_pressed(&mut self, _x: u8) {
        self.pc += 2;
    }

    fn load_delay_timer(&mut self, x: u8) {
        self.register[x as usize] = self.delaytimer;
    }

    fn wait_for_keypress(&mut self, _x: u8) {
        self.pc -= 2;
    }

    fn set_delay_timer(&mut self, x: u8) {
        self.delaytimer = self.register[x as usize];
    }

    fn set_sound_timer(&mut self, x: u8) {
        self.soundtimer = self.register[x as usize];
    }

    fn addi(&mut self, x: u8) {
        self.i += self.register[x as usize] as u16;
    }

    fn ldf(&mut self, x: u8) {
        self.i = self.register[x as usize] as u16 * 5;
    }

    fn ldb(&mut self, x: u8) {
        self.memory[self.i as usize] = self.register[x as usize] / 100;
        self.memory[(self.i + 1) as usize] = (self.register[x as usize] / 10) % 10;
        self.memory[(self.i + 2) as usize] = (self.register[x as usize] % 100) % 10;
    }

    fn ldir(&mut self, x: u8) {
        for i in 0..(x as u16) {
            self.memory[(i + self.i) as usize] = self.register[i as usize];
        }
    }

    fn ldri(&mut self, x: u8) {
        for i in 0..(x as u16) {
            self.register[i as usize] = self.memory[(i + self.i) as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cls_clears_the_screen() {
        let mut cpu = Cpu::default();

        cpu.screen.draw_sprite(0, 0, &vec![0xFF]);
        cpu.execute(0x00E0);

        assert_eq!(0, cpu.screen.get_screen_data()[0]);
    }

    #[test]
    fn ret_decreases_stack_pointer_and_sets_pc() {
        let mut cpu = Cpu::default();

        cpu.sp = 5;
        cpu.stack[4] = 22;

        cpu.execute(0x00EE);

        assert_eq!(22, cpu.pc);
        assert_eq!(4, cpu.sp);
    }

    #[test]
    fn jp_sets_pc_to_address() {
        let mut cpu = Cpu::default();

        cpu.execute(0x1F43);

        assert_eq!(0xF43, cpu.pc);
    }

    #[test]
    fn call_sets_stack_and_sp_and_pc() {
        let mut cpu = Cpu::default();
        cpu.pc = 0x55;

        cpu.execute(0x2F43);

        assert_eq!(0x55, cpu.stack[0]);
        assert_eq!(1, cpu.sp);
        assert_eq!(0xF43, cpu.pc);
    }

    #[test]
    fn skip_equal_skips_next_instruction_if_vx_equals_kk() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0xFF;

        cpu.execute(0x32FF);

        assert_eq!(2, cpu.pc);
    }

    #[test]
    fn skip_equal_does_not_skip_next_instruction_if_vx_does_not_equal_kk() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x44;

        cpu.execute(0x32FF);

        assert_eq!(0, cpu.pc);
    }

    #[test]
    fn skip_not_equal_skips_next_instruction_if_vx_does_not_equal_kk() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x33;

        cpu.execute(0x42FF);

        assert_eq!(2, cpu.pc);
    }

    #[test]
    fn skip_not_equal_does_not_skip_next_instruction_if_vx_equals_kk() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x22;

        cpu.execute(0x4222);

        assert_eq!(0, cpu.pc);
    }

    #[test]
    fn skip_register_equal() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x55;
        cpu.register[5] = 0x55;

        cpu.execute(0x5250);

        assert_eq!(2, cpu.pc);
    }

    #[test]
    fn load_sets_register() {
        let mut cpu = Cpu::default();

        cpu.execute(0x6655);

        assert_eq!(0x55, cpu.register[6]);
    }

    #[test]
    fn add_adds_value_with_register() {
        let mut cpu = Cpu::default();
        cpu.register[4] = 0x10;

        cpu.execute(0x7410);

        assert_eq!(0x20, cpu.register[4]);
    }

    #[test]
    fn load_register_sets_register() {
        let mut cpu = Cpu::default();
        cpu.register[4] = 0x55;

        cpu.execute(0x8240);

        assert_eq!(0x55, cpu.register[2]);
    }

    #[test]
    fn or_stores_bitwise_or_in_vx() {
        let mut cpu = Cpu::default();
        cpu.register[2] = 0b11100000;
        cpu.register[3] = 0b01001111;

        cpu.execute(0x8231);

        assert_eq!(0b11101111, cpu.register[2]);
    }

    #[test]
    fn and_stores_bitwise_and_in_vx() {
        let mut cpu = Cpu::default();
        cpu.register[2] = 0b11100000;
        cpu.register[3] = 0b01001111;

        cpu.execute(0x8232);

        assert_eq!(0b01000000, cpu.register[2]);
    }

    #[test]
    fn xor_stores_bitwise_xor_in_vx() {
        let mut cpu = Cpu::default();
        cpu.register[2] = 0b11100000;
        cpu.register[3] = 0b01001111;

        cpu.execute(0x8233);

        assert_eq!(0b10101111, cpu.register[2]);
    }

    #[test]
    fn addr_adds_two_registers() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x10;
        cpu.register[3] = 0x10;

        cpu.execute(0x8234);

        assert_eq!(0x20, cpu.register[2]);
    }

    #[test]
    fn subr_subtracts_two_registers() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x20;
        cpu.register[3] = 0x10;

        cpu.execute(0x8235);

        assert_eq!(0x10, cpu.register[2]);
    }

    #[test]
    fn shr() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x20;
        cpu.execute(0x8206);

        assert_eq!(0, cpu.register[0xF]);
        assert_eq!(0x10, cpu.register[0x2]);

        cpu.register[2] = 0x21;
        cpu.execute(0x8206);

        assert_eq!(1, cpu.register[0xF]);
        assert_eq!(0x10, cpu.register[0x2]);
    }

    #[test]
    fn subn() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x30;
        cpu.register[3] = 0x10;

        cpu.execute(0x8327);

        assert_eq!(0x0, cpu.register[0xF]);
        assert_eq!(0x20, cpu.register[3]);
        cpu.register[2] = 0x10;
        cpu.register[3] = 0x30;

        cpu.execute(0x8327);

        assert_eq!(0x1, cpu.register[0xF]);
        assert_eq!(0xE0, cpu.register[3]);
    }

    #[test]
    fn shl() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x20;
        cpu.execute(0x820E);

        assert_eq!(0, cpu.register[0xF]);
        assert_eq!(0x40, cpu.register[2]);

        cpu.register[2] = 0xF0;
        cpu.execute(0x820E);

        assert_eq!(1, cpu.register[0xF]);
        assert_eq!(0xF0u8.wrapping_mul(2), cpu.register[2]);
    }

    #[test]
    fn skip_not_equal_registers() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x2;
        cpu.register[3] = 0x3;

        cpu.execute(0x9230);

        assert_eq!(2, cpu.pc);

        cpu.register[3] = 0x2;

        cpu.execute(0x9230);

        assert_eq!(2, cpu.pc);
    }

    #[test]
    fn load_i() {
        let mut cpu = Cpu::default();

        cpu.execute(0xA555);

        assert_eq!(0x555, cpu.i);
    }

    #[test]
    fn jumpr() {
        let mut cpu = Cpu::default();

        cpu.register[0] = 0x10;
        cpu.execute(0xBC23);

        assert_eq!(0xC33, cpu.pc);
    }

    #[test]
    fn rnd() {
        let mut cpu = Cpu::default();

        cpu.execute(0xC222); // Can't assert anything since the result is random for now.
    }

    #[test]
    fn draw_sets_screen_pixels() {
        let mut cpu = Cpu::default();

        cpu.memory[0] = 0xFF;
        cpu.execute(0xD001);

        assert_eq!(0, cpu.register[0xF]);
        let pixels = cpu.screen.get_screen_data();
        assert!(pixels[0..8].iter().all(|pixel| pixel == &255u8));
        assert!(pixels[8..].iter().all(|pixel| pixel == &0u8));
    }

    #[test]
    fn draw_with_collision_toggles_pixels_back_and_sets_vf() {
        let mut cpu = Cpu::default();

        cpu.memory[0] = 0xFF;
        cpu.execute(0xD001);
        cpu.execute(0xD001);

        assert_eq!(1, cpu.register[0xF]);
        let pixels = cpu.screen.get_screen_data();
        assert!(pixels.iter().all(|pixel| pixel == &0u8));
    }

    #[test]
    fn load_delay_timer_loads_delaytimer_value() {
        let mut cpu = Cpu::default();

        cpu.delaytimer = 5;

        cpu.execute(0xF207);

        assert_eq!(5, cpu.register[2]);
    }

    #[test]
    fn set_delay_timer_sets_delaytimer() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x44;
        cpu.execute(0xF215);

        assert_eq!(0x44, cpu.delaytimer);
    }

    #[test]
    fn set_delay_timer_sets_soundtimer() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x44;
        cpu.execute(0xF218);

        assert_eq!(0x44, cpu.soundtimer);
    }

    #[test]
    fn addi() {
        let mut cpu = Cpu::default();

        cpu.register[2] = 0x10;
        cpu.i = 0x20;

        cpu.execute(0xF21E);

        assert_eq!(0x30, cpu.i);
    }
}
