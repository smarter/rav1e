extern crate libc;

use std::mem::uninitialized;
use std::slice;

use bindings::{od_ec_enc, od_ec_enc_done, od_ec_enc_init, od_ec_encode_bool_q15, od_ec_encode_cdf_q15};

pub struct Writer {
    enc: od_ec_enc
}

impl Writer {
    pub fn new() -> Writer {
        unsafe {
            let mut enc: od_ec_enc = uninitialized();
            od_ec_enc_init(&mut enc, 1024);
            Writer { enc: enc }
        }
    }
    pub fn done(&mut self) -> &[u8] {
        let mut nbytes: u32 = 0;
        unsafe {
            let b = od_ec_enc_done(&mut self.enc, &mut nbytes);
            slice::from_raw_parts(b, nbytes as usize)
        }
    }
    pub fn cdf(&mut self, s: u32, cdf: &[u16], nsyms: u32) {
        unsafe {
            od_ec_encode_cdf_q15(&mut self.enc, s as libc::c_int, cdf.as_ptr(), nsyms as libc::c_int);
        }
    }
    pub fn bool(&mut self, val: bool, f: u16) {
        unsafe {
            od_ec_encode_bool_q15(&mut self.enc, val as libc::c_int, f as libc::c_uint);
        }
    }
    fn update_cdf(cdf: &mut [u16], val: u32, nsymbs: usize) {
        let rate = 4 + if cdf[nsymbs] > 31 { 1 } else { 0 } + (31 ^ (nsymbs as u32).leading_zeros());
        let rate2 = 5;
        let mut tmp: i32;
        let diff: i32;
        let tmp0 = 1 << rate2;
        tmp = 32768 - tmp0;
        diff = ((32768 - ((nsymbs as i32) << rate2)) >> rate) << rate;
        for i in 0..(nsymbs - 1) {
            if i as u32 == val {
                tmp -= diff;
            }
            cdf[i as usize] = ((cdf[i as usize] as i32) + ((tmp - (cdf[i as usize] as i32)) >> rate)) as u16;
            tmp -= tmp0;
        }
        if cdf[nsymbs] < 32 {
            cdf[nsymbs] += 1;
        }
    }
    pub fn symbol(&mut self, s: u32, cdf: &mut [u16], nsymbs: usize) {
        self.cdf(s, cdf, nsymbs as u32);
        Writer::update_cdf(cdf, s, nsymbs);
    }
}
