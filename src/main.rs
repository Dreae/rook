mod cpu;
mod opcodes;

#[macro_use]
mod conversions;

fn main() {
  let mut c = cpu::RookVM::new();
  c.push_byte(2);
  println!("%esp {}", c.esp);

  let byte = c.pop_byte();
  println!("Got {}", byte);

  c.push_byte(0);
  c.push_byte(0);
  c.push_byte(0);
  c.push_byte(10);
  println!("%esp {}", c.esp);

  println!("int: {}", bytes_to_i32!(c.pop_bytes(4)));

  println!("%esp {}", c.esp);

  {
    let register = c.nibble_to_register(1);
    *register = 2;  
  }
  
  println!("%ebx {}", c.registers[1]);
  let bytes = i32_to_bytes!(10);
  println!("bytes[3] {}", bytes[3]);
}