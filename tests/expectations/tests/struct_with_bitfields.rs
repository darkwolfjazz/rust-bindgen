/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq)]
pub struct bitfield {
    pub _bitfield_1: u8,
    pub e: ::std::os::raw::c_int,
    pub _bitfield_2: [u32; 2usize],
}
#[test]
fn bindgen_test_layout_bitfield() {
    assert_eq!(::std::mem::size_of::<bitfield>() , 16usize , concat ! (
               "Size of: " , stringify ! ( bitfield ) ));
    assert_eq! (::std::mem::align_of::<bitfield>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( bitfield ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const bitfield ) ) . e as * const _ as usize }
                , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( bitfield ) , "::" ,
                stringify ! ( e ) ));
}
impl Clone for bitfield {
    fn clone(&self) -> Self { *self }
}
impl bitfield {
    #[inline]
    pub fn a(&self) -> ::std::os::raw::c_ushort {
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        let mask = 1u64 as u8;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_a(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 1u64 as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(&unit_field_val as *const _ as
                                                *const u8,
                                            &mut self._bitfield_1 as *mut _ as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>());
        }
    }
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_ushort {
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        let mask = 2u64 as u8;
        let val = (unit_field_val & mask) >> 1usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 2u64 as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 1usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(&unit_field_val as *const _ as
                                                *const u8,
                                            &mut self._bitfield_1 as *mut _ as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>());
        }
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_ushort {
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        let mask = 4u64 as u8;
        let val = (unit_field_val & mask) >> 2usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_c(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 4u64 as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 2usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(&unit_field_val as *const _ as
                                                *const u8,
                                            &mut self._bitfield_1 as *mut _ as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>());
        }
    }
    #[inline]
    pub fn d(&self) -> ::std::os::raw::c_ushort {
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        let mask = 192u64 as u8;
        let val = (unit_field_val & mask) >> 6usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_d(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 192u64 as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 6usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(&unit_field_val as *const _ as
                                                *const u8,
                                            &mut self._bitfield_1 as *mut _ as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>());
        }
    }
    #[inline]
    pub fn new_bitfield_1(a: ::std::os::raw::c_ushort,
                          b: ::std::os::raw::c_ushort,
                          c: ::std::os::raw::c_ushort,
                          d: ::std::os::raw::c_ushort) -> u8 {
        ({
             ({
                  ({ ({ 0 } | ((a as u16 as u8) << 0usize) & (1u64 as u8)) } |
                       ((b as u16 as u8) << 1usize) & (2u64 as u8))
              } | ((c as u16 as u8) << 2usize) & (4u64 as u8))
         } | ((d as u16 as u8) << 6usize) & (192u64 as u8))
    }
    #[inline]
    pub fn f(&self) -> ::std::os::raw::c_uint {
        let mut unit_field_val: u64 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_2 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u64 as
                                                *mut u8,
                                            ::std::mem::size_of::<u64>())
        };
        let mask = 3u64 as u64;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_f(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 3u64 as u64;
        let val = val as u32 as u64;
        let mut unit_field_val: u64 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_2 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u64 as
                                                *mut u8,
                                            ::std::mem::size_of::<u64>())
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(&unit_field_val as *const _ as
                                                *const u8,
                                            &mut self._bitfield_2 as *mut _ as
                                                *mut u8,
                                            ::std::mem::size_of::<u64>());
        }
    }
    #[inline]
    pub fn g(&self) -> ::std::os::raw::c_uint {
        let mut unit_field_val: u64 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_2 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u64 as
                                                *mut u8,
                                            ::std::mem::size_of::<u64>())
        };
        let mask = 18446744069414584320u64 as u64;
        let val = (unit_field_val & mask) >> 32usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_g(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 18446744069414584320u64 as u64;
        let val = val as u32 as u64;
        let mut unit_field_val: u64 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_2 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u64 as
                                                *mut u8,
                                            ::std::mem::size_of::<u64>())
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 32usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(&unit_field_val as *const _ as
                                                *const u8,
                                            &mut self._bitfield_2 as *mut _ as
                                                *mut u8,
                                            ::std::mem::size_of::<u64>());
        }
    }
    #[inline]
    pub fn new_bitfield_2(f: ::std::os::raw::c_uint,
                          g: ::std::os::raw::c_uint) -> u64 {
        ({ ({ 0 } | ((f as u32 as u64) << 0usize) & (3u64 as u64)) } |
             ((g as u32 as u64) << 32usize) &
                 (18446744069414584320u64 as u64))
    }
}
