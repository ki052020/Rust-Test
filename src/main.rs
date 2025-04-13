#![allow(dead_code)]
use windows_sys::{
	core::*,
	Win32::UI::WindowsAndMessaging::*
};

// null 終端を持つ文字列
struct WStr {
	len: usize,
	capacity: usize,
	buf: Vec<u16>,
}

impl WStr {
	pub fn new() -> Self {
		WStr {
			len: 0,
			capacity: 0,
			buf: Vec::new(),
		}
	}
	
	pub fn with_capacity(capacity: usize) -> Self {
		WStr {
			len: 0,
			capacity,
			buf: Vec::with_capacity(capacity),
		}
	}
	
	pub fn from_u(u16s: &[u16]) -> Self {
		WStr {
			len: u16s.len() - 1,
			capacity: u16s.len(),
			buf: u16s.to_vec(),
		}
	}
	
	pub fn cstr(self) -> *const u16 {
		self.buf.as_ptr()
	}
	
	pub fn len(&self) -> usize {
		self.len
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}
}

macro_rules! u {
	($str:literal) => {{
		const INPUT: &[u8] = $str.as_bytes();
		const OUTPUT_LEN: usize = utf16_len(INPUT) + 1;
		
		// コンパイル時定数になるように const fn を利用しているけれど、
		// const fn にする必要はないと思う。const OUTPUT = ... でよいかも？
		const fn crt_output() -> [u16; OUTPUT_LEN] {
			let mut ret_ary = [0; OUTPUT_LEN];
			let mut idx_src = 0;
			let mut idx_dst = 0;
			while let Some((mut code, idx_src_new)) = decode_utf8_char(INPUT, idx_src) {
				idx_src = idx_src_new;
				if code <= 0xffff {
					ret_ary[idx_dst] = code as u16;
					idx_dst += 1;
				} else {
					code -= 0x10000;
					ret_ary[idx_dst] = 0xd800 + (code >> 10) as u16;
					ret_ary[idx_dst + 1] = 0xdc00 + (code & 0x3ff) as u16;
					idx_dst += 2;
				}
			}
			ret_ary
		}
		crt_output()
	}};
}

fn main() {
	// ??? test1 は正しく動作する。test2 は、動作が不安定
	let test1 = u!("おはよう");
//	let test2 = WStr::from_u(&u!("おはよう"));
	unsafe {
		MessageBoxW(std::ptr::null_mut(), w!("こんにちは、世界"), test1.as_ptr(), MB_OK);
//		MessageBoxW(std::ptr::null_mut(), w!("こんにちは、世界"), test2.cstr(), MB_OK);
	}
}
