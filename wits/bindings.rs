mod say {
  #[export_name = "hello"]
  unsafe extern "C" fn __wit_bindgen_hello(arg0: i32, arg1: i32, ) -> i32{
    let len0 = arg1 as usize;
    let result = <super::Say as Say>::hello(String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap());
    let ptr1 = RET_AREA.0.as_mut_ptr() as i32;
    let vec2 = (result.into_bytes()).into_boxed_slice();
    let ptr2 = vec2.as_ptr() as i32;
    let len2 = vec2.len() as i32;
    core::mem::forget(vec2);
    *((ptr1 + 4) as *mut i32) = len2;
    *((ptr1 + 0) as *mut i32) = ptr2;
    ptr1
  }
  pub trait Say {
    fn hello(name: String,) -> String;
  }
  
  #[repr(align(4))]
  struct RetArea([u8; 8]);
  static mut RET_AREA: RetArea = RetArea([0; 8]);
}
