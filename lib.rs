#![crate_type = "lib"]

#![feature(abi_ptx)]
#![feature(core_intrinsics)]
#![feature(link_llvm_intrinsics)]
#![feature(naked_functions)]

//#![feature(lang_items)]
//#![feature(no_core)]
//#![no_core]

#![no_std]

//extern crate core;

/*#[lang = "copy"]
trait Copy {}

#[lang = "freeze"]
trait Freeze {}

#[lang = "sized"]
trait Sized {}*/

#[allow(improper_ctypes)]
extern "C" {
  #[link_name = "llvm.nvvm.barrier0"]
  fn syncthreads() -> ();

  #[link_name = "llvm.nvvm.read.ptx.sreg.tid.x"]
  fn tid_x() -> i32;

  //#[link_name = "llvm.nvvm.ptr.gen.to.shared.p3i8.p0i8"]
  //fn ptr_gen_to_shared(ptr: *mut u8) -> *mut u8;
}

/*#[no_mangle]
unsafe fn bar(c: i32, x: *mut f32) {
}*/

extern "ptx-kernel" {
  //static shared: *mut u32;
  static shared: [u32; 0];
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn foo() {
//pub unsafe extern "ptx-kernel" fn foo(_x: *mut f32) {
//pub unsafe extern "ptx-kernel" fn foo(c: i32, x: *mut f32) {
  let base: *mut u32 = shared.as_ptr() as *mut u32;
  let p: *mut u32 = base.offset(tid_x() as isize);
  *p = 42;
  /*let mut p = ptr_gen_to_shared(shared);
  *(p) = 0_u8;
  syncthreads();
  *(x) = 3.14_f32;
  core::intrinsics::abort();*/
}
