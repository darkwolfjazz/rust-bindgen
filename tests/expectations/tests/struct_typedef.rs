/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq)]
pub struct typedef_named_struct {
    pub has_name: bool,
}
#[test]
fn bindgen_test_layout_typedef_named_struct() {
    assert_eq!(::std::mem::size_of::<typedef_named_struct>() , 1usize , concat
               ! ( "Size of: " , stringify ! ( typedef_named_struct ) ));
    assert_eq! (::std::mem::align_of::<typedef_named_struct>() , 1usize ,
                concat ! (
                "Alignment of " , stringify ! ( typedef_named_struct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const typedef_named_struct ) ) . has_name as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( typedef_named_struct )
                , "::" , stringify ! ( has_name ) ));
}
impl Clone for typedef_named_struct {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq)]
pub struct _bindgen_ty_1 {
    pub no_name: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<_bindgen_ty_1>() , 8usize , concat ! (
               "Size of: " , stringify ! ( _bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<_bindgen_ty_1>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( _bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const _bindgen_ty_1 ) ) . no_name as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( _bindgen_ty_1 ) , "::"
                , stringify ! ( no_name ) ));
}
impl Clone for _bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
impl Default for _bindgen_ty_1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type struct_ptr_t = *mut _bindgen_ty_1;
pub type struct_ptr_ptr_t = *mut *mut _bindgen_ty_1;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum typedef_named_enum { ENUM_HAS_NAME = 1, }
pub const ENUM_IS_ANON: _bindgen_ty_2 = _bindgen_ty_2::ENUM_IS_ANON;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_2 { ENUM_IS_ANON = 0, }
pub type enum_ptr_t = *mut _bindgen_ty_2;
pub type enum_ptr_ptr_t = *mut *mut _bindgen_ty_2;
