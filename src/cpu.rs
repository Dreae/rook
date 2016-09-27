pub struct RookVM {
  pub registers: [u32; 15],
  pub eip: u32,
  pub esp: u32,
  pub ebp: u32,
  pub running: bool,

  stack: [u8; 4098],
  heap: [u8; 4098],
  code: [u8; 4098],
  opcodes: [Option<fn(&mut RookVM)>; 512],
}

impl RookVM {
  pub fn new() -> RookVM {
    RookVM {
      registers: [0; 15],
      eip: 0,
      esp: 0,
      ebp: 0,
      running: false,

      stack: [0; 4098],
      heap: [0; 4098],
      code: [0; 4098],
      opcodes: [None; 512],
    }
  }

  pub fn read_byte(&mut self) -> u8 {
    let byte = self.code[self.eip as usize];
    self.eip += 1;

    byte
  }

  pub fn pop_byte(&mut self) -> u8 {
    self.esp -= 1;

    self.stack[(self.ebp + self.esp) as usize]
  }

  pub fn push_byte(&mut self, byte: u8) {
    self.stack[(self.ebp + self.esp) as usize] = byte;
    self.esp += 1;
  }

  pub fn run(&mut self) {
    self.running = true;
    while self.running {
      let byte = self.read_byte();
      let opcode = self.opcodes[byte as usize];
      match opcode {
        Some(func) => func(self),
        None => println!("Error undefined opcode {}", byte),
      };
    }
  }

  pub fn read_bytes(&mut self, num: u32) -> &[u8] {
    let slice = &self.code[self.eip as usize .. (self.eip + num) as usize];
    self.eip += num;

    slice
  } 

  pub fn pop_bytes(&mut self, num: u32) -> &[u8] {
    let slice = &self.stack[(self.ebp + (self.esp - num)) as usize .. (self.ebp + self.esp) as usize];
    self.esp -= num;

    slice
  }

  pub fn push_bytes(&mut self, bytes: &[u8]) {
    for byte in bytes {
      self.push_byte(*byte);
    }
  }

  pub fn nibble_to_register(&mut self, nibble: u8) -> &mut u32 {
    if nibble < 14 {
      &mut self.registers[nibble as usize]
    } else if nibble == 14 {
      &mut self.esp
    } else {
      &mut self.ebp
    }
  }
}