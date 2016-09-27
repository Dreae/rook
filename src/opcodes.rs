use cpu::RookVM;
use std::mem;

pub struct OpCode {
  pub code: u8,
  pub func: fn(&mut RookVM),
}

static OP_CODES: [OpCode; 8] = [
  OpCode {
    code: 0x00,
    func: op_exit,
  },
  OpCode {
    code: 0x01,
    func: op_mov,
  },
  OpCode {
    code: 0x03,
    func: op_add,
  },
  OpCode {
    code: 0x04,
    func: op_sub,
  },
  OpCode {
    code: 0x05,
    func: op_addf,
  },
  OpCode {
    code: 0x06,
    func: op_subf,
  },
  OpCode {
    code: 0x20,
    func: op_call,
  },
  OpCode {
    code: 0x21,
    func: op_ret,
  }
];

fn op_exit(vm: &mut RookVM) {
  vm.running = false;
}

fn op_mov(vm: &mut RookVM) {
  let arg = vm.read_byte();
  let src = vm.nibble_to_register_value(arg & 0x0f);
  let dst = vm.nibble_to_register(arg >> 4);

  *dst = src;
}

fn op_add(vm: &mut RookVM) {
  let arg = vm.read_byte();
  let src = vm.nibble_to_register_value(arg & 0x0f);
  let dst = vm.nibble_to_register(arg >> 4);

  *dst = *dst + src;
}

fn op_sub(vm: &mut RookVM) {
  let arg = vm.read_byte();
  let src = vm.nibble_to_register_value(arg & 0x0f);
  let dst = vm.nibble_to_register(arg >> 4);

  *dst = *dst - src;
}

fn op_addf(vm: &mut RookVM) {
  let arg = vm.read_byte();
  unsafe {
    let src = mem::transmute::<u32, f32>(vm.nibble_to_register_value(arg & 0x0f));
    let dst = mem::transmute::<&mut u32, &mut f32>(vm.nibble_to_register(arg >> 4));

    *dst = *dst + src; 
  }
}

fn op_subf(vm: &mut RookVM) {
  let arg = vm.read_byte();
  unsafe {
    let src = mem::transmute::<u32, f32>(vm.nibble_to_register_value(arg & 0x0f));
    let dst = mem::transmute::<&mut u32, &mut f32>(vm.nibble_to_register(arg >> 4));

    *dst = *dst - src; 
  }
}

fn op_call(vm: &mut RookVM) {
  let addr = bytes_to_i32!(vm.read_bytes(4));
  let bytes = i32_to_bytes!(vm.eip);
  vm.push_bytes(&bytes);
  vm.eip = addr as u32;
}

fn op_ret(vm: &mut RookVM) {
  let mut addr: i32;
  {
    let bytes = vm.pop_bytes(4);
    addr = bytes_to_i32!(bytes);
  }
  vm.eip = addr as u32;
}

pub fn register_opcodes(vm: &mut RookVM) {
  for op_code in OP_CODES.into_iter() {
    vm.add_opcode(op_code.code, op_code.func);
  }
}