pub struct RookVM {
  pub registers: [u64; 15],
  pub eip: u64,
  pub esp: u64,
  pub ebp: u64,
  pub running: bool,
  pub code: [u8; 4098],

  stack: [u8; 4098],
  heap: [u8; 4098],
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

  pub fn add_opcode(&mut self, code: u8, func: fn(&mut RookVM)) {
    self.opcodes[code as usize] = Some(func);
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

  pub fn read_bytes(&mut self, num: u64) -> &[u8] {
    let slice = &self.code[self.eip as usize .. (self.eip + num) as usize];
    self.eip += num;

    slice
  } 

  pub fn pop_bytes(&mut self, num: u64) -> &[u8] {
    let slice = &self.stack[(self.ebp + (self.esp - num)) as usize .. (self.ebp + self.esp) as usize];
    self.esp -= num;

    slice
  }

  pub fn push_bytes(&mut self, bytes: &[u8]) {
    for byte in bytes {
      self.push_byte(*byte);
    }
  }

  pub fn nibble_to_register(&mut self, nibble: u8) -> &mut u64 {
    if nibble < 14 {
      &mut self.registers[nibble as usize]
    } else if nibble == 14 {
      &mut self.esp
    } else {
      &mut self.ebp
    }
  }

  pub fn nibble_to_register_value(&self, nibble: u8) -> u64 {
    if nibble < 14 {
      self.registers[nibble as usize]
    } else if nibble == 14 {
      self.esp
    } else {
      self.ebp
    }
  }
}