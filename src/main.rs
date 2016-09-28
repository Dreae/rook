#[macro_use]
mod conversions;

mod cpu;
mod opcodes;

use std::mem;

fn main() {
  let mut c = cpu::RookVM::new();
  opcodes::register_opcodes(&mut c);

  c.registers[0] = 32;
  unsafe {
    c.registers[1] = transmute!(-34, i64, u64);
  }

  c.code[0]  = 0x03u8; // add %r1, %r0
  c.code[1]  = 0x01u8;
  c.code[2]  = 0x20u8; // call 12
  c.code[3]  = 0x00u8;
  c.code[4]  = 0x00u8;
  c.code[5]  = 0x00u8;
  c.code[6]  = 0x00u8;
  c.code[7]  = 0x00u8;
  c.code[8]  = 0x00u8;
  c.code[9]  = 0x00u8;
  c.code[10] = 0x0cu8;
  c.code[11] = 0x00u8; // exit
  c.code[12] = 0x05u8; // mul %r1, r0
  c.code[13] = 0x01u8;
  c.code[14] = 0x21u8; // ret
  c.run();

  unsafe {
    println!("%eax {}", transmute!(c.registers[0], u64, i64));
  }
}