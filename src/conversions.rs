macro_rules! bytes_to_i32 {
  ( $x:expr ) => {
    {
      let bytes = $x;
      ((bytes[0] as i32) << 24) | ((bytes[1] as i32) << 16) | ((bytes[2] as i32) << 8) | (bytes[3] as i32)
    }
  }
}

macro_rules! i32_to_bytes {
  ( $x:expr ) => {
    {
      let mut bytes: [u8; 4] = [0; 4];
      let num = $x as i32;
      bytes[0] = ((num >> 24) & 0xff) as u8;
      bytes[1] = ((num >> 16) & 0xff) as u8;
      bytes[2] = ((num >> 8) & 0xff) as u8;
      bytes[3] = (num & 0xff) as u8;

      bytes
    }
  }
}