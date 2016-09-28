// Safer than transmute, doesn't rely on architecture endianness
macro_rules! bytes_to_u64 {
  ( $x:expr ) => {
    {
      let bytes = $x;
      ((bytes[0] as u64) << 56) | ((bytes[1] as u64) << 48) | ((bytes[2] as u64) << 40) | ((bytes[3] as u64) << 32) | ((bytes[4] as u64) << 24) | ((bytes[5] as u64) << 16) | ((bytes[6] as u64) << 8)  | (bytes[7] as u64)
    }
  }
}

macro_rules! u64_to_bytes {
  ( $x:expr ) => {
    {
      let mut bytes: [u8; 8] = [0; 8];
      let num = $x;
      bytes[0] = ((num >> 56) & 0xff) as u8;
      bytes[1] = ((num >> 48) & 0xff) as u8;
      bytes[2] = ((num >> 40) & 0xff) as u8;
      bytes[3] = ((num >> 32) & 0xff) as u8;
      bytes[4] = ((num >> 24) & 0xff) as u8;
      bytes[5] = ((num >> 16) & 0xff) as u8;
      bytes[6] = ((num >>  8) & 0xff) as u8;
      bytes[7] = (num & 0xff) as u8;

      bytes
    }
  }
}

macro_rules! transmute {
  ($x:expr, $stype:ty, $dtype:ty) => {
    {
      let num = $x;
      mem::transmute::<$stype,$dtype>(num)
    }
  }
}