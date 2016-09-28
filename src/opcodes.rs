use cpu::RookVM;
use std::mem;
use std::ops::{Div, Mul, Add, Sub, Rem, BitAnd, BitOr, BitXor, Shl, Shr};

pub struct OpCode {
  pub code: u8,
  pub func: fn(&mut RookVM),
}

macro_rules! expr {
  ($e:expr) => {
    $e
  }
}

macro_rules! read_args {
    ($vm:ident, $arg1:ident, $arg2:ident, $argt:ty) => {
      {
        let arg = $vm.read_byte();
        unsafe {
          $arg1 = transmute!($vm.nibble_to_register_value(arg & 0x0f), u64, $argt);
          $arg2 = transmute!($vm.nibble_to_register(arg >> 4), &mut u64, &mut $argt);
        }
      }
    }
}

macro_rules! math_op {
  ($name:ident, $op:ident, $argt:ty, $castt:ty) => {
    fn $name(vm: &mut RookVM) {
      let src: $argt;
      let dst: &mut $argt;
      read_args!(vm, src, dst, $argt);

      *dst = (*dst).$op(src as $castt);
    }
  };

  ($name:ident, $op:ident, $argt:ty) => {
    fn $name(vm: &mut RookVM) {
      let src: $argt;
      let dst: &mut $argt;
      read_args!(vm, src, dst, $argt);

      *dst = (*dst).$op(src);
    }
  };
}

macro_rules! bin_op {
  ($name:ident, $op:ident) => {
    fn $name(vm: &mut RookVM) {
      let arg = vm.read_byte();
      let src = vm.nibble_to_register_value(arg & 0x0f);
      let dst = vm.nibble_to_register(arg >> 4);

      *dst = (*dst).$op(src);
    }
  };
}

static OP_CODES: [OpCode; 21] = [
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
    func: op_mul,
  },
  OpCode {
    code: 0x06,
    func: op_div,
  },
  OpCode {
    code: 0x07,
    func: op_mod,
  },
  OpCode {
    code: 0x08,
    func: op_pow,
  },
  OpCode {
    code: 0x09,
    func: op_or,
  },
  OpCode {
    code: 0x0a,
    func: op_xor,
  },
  OpCode {
    code: 0x0b,
    func: op_and,
  },
  OpCode {
    code: 0x0c,
    func: op_shl,
  },
  OpCode {
    code: 0x0d,
    func: op_shr,
  },
  OpCode {
    code: 0x13,
    func: op_addf,
  },
  OpCode {
    code: 0x14,
    func: op_subf,
  },
  OpCode {
    code: 0x15,
    func: op_mulf,
  },
  OpCode {
    code: 0x16,
    func: op_divf,
  },
  OpCode {
    code: 0x17,
    func: op_modf,
  },
  OpCode {
    code: 0x18,
    func: op_powf,
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

math_op!(op_add, add, i64);
math_op!(op_sub, sub, i64);
math_op!(op_mul, mul, i64);
math_op!(op_div, div, i64);
math_op!(op_mod, rem, i64);
math_op!(op_pow, pow, i64, u32);

math_op!(op_addf, add, f64);
math_op!(op_subf, sub, f64);
math_op!(op_mulf, mul, f64);
math_op!(op_divf, div, f64);
math_op!(op_modf, rem, f64);
math_op!(op_powf, powf, f64);

bin_op!(op_and, bitand);
bin_op!(op_or, bitor);
bin_op!(op_xor, bitxor);
bin_op!(op_shl, shl);
bin_op!(op_shr, shr);

fn op_call(vm: &mut RookVM) {
  let addr = bytes_to_u64!(vm.read_bytes(8));
  let bytes = u64_to_bytes!(vm.eip);
  vm.push_bytes(&bytes);
  vm.eip = addr;
}

fn op_ret(vm: &mut RookVM) {
  let mut addr: u64;
  {
    let bytes = vm.pop_bytes(8);
    addr = bytes_to_u64!(bytes);
  }
  vm.eip = addr;
}

pub fn register_opcodes(vm: &mut RookVM) {
  for op_code in OP_CODES.into_iter() {
    vm.add_opcode(op_code.code, op_code.func);
  }
}