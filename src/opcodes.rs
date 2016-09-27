use cpu::RookVM;

pub struct OpCode {
  pub mnemonic: &'static str,
  pub code: u8,
  pub handler: fn(&mut RookVM),
}