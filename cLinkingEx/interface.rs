use libc::{c_int, c_ulong};

//This right here is a interface of teh C function that we want to call, and this is represented in Rust data tyeps
#[link(name = "z")]
extern "C" {
  fn compress(
    dest: *mut u8,
    dest_len: *mut c_ulong,
    src: *const u8,
    src_len: c_ulong,
  ) -> c_int;

  fn compressBound(sourceLen: c_ulong) -> c_ulong;

  fn uncompress(
    dest: *mut u8,
    dest_len: *mut c_ulong,
    src: *const u8,
    src_len: c_ulong,
  ) -> c_int;

}

//We then need to write a Rust warpper function that will call the C function
pub fn compressWraller(source: &[u8]) -> Vec<u8> {
  
  unsafe {
    let source_len = source.len() as c_ulong;
    let dest_len = unsafe { compressBound(source_len) };
    let mut dest = vec![0u8; dest_len as usize];
    let mut dest_len_mut = dest_len;

    let result = unsafe {

      //THIS IS WHERE WE CALL THE C FUNCTION, AND WE PASS THE APPROPRIATE ARGUMENTS TO IT
      compress(
        dest.as_mut_ptr(),
        &mut dest_len_mut,
        source.as_ptr(),
        source_len,
      )
    };

    if result != 0 {
      panic!("Compression failed with error code: {}", result);
    }

    dest.truncate(dest_len_mut as usize);
    dest
  }
}