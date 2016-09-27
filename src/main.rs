mod cpu;
mod opcodes;

#[macro_use]
mod conversions;

use std::mem;

fn main() {
  let mut c = cpu::RookVM::new();
  opcodes::register_opcodes(&mut c);

  unsafe {
    c.registers[0] = mem::transmute::<f32, u32>(32.0);
    c.registers[1] = mem::transmute::<f32, u32>(32.0);
  }
  
  c.code[0] = 5u8;
  c.code[1] = 1u8;
  c.run();

  unsafe {
    println!("%eax {}", mem::transmute::<u32, f32>(c.registers[0]));
  }
}