#![allow(dead_code)]
use windows_sys::{
	core::*,
	Win32::UI::WindowsAndMessaging::*
};

macro_rules! wstr {
	($s:literal) => {{
		const INPUT: &[u8] = $s.as_bytes();
		const OUTPUT_LEN: usize = utf16_len(INPUT) + 1;
		const OUTPUT: &[u16; OUTPUT_LEN] = {
			let mut buffer = [0; OUTPUT_LEN];
			let mut input_pos = 0;
			let mut output_pos = 0;
			while let Some((mut code_point, new_pos)) = decode_utf8_char(INPUT, input_pos) {
				input_pos = new_pos;
				if code_point <= 0xffff {
					buffer[output_pos] = code_point as u16;
					output_pos += 1;
				} else {
					code_point -= 0x10000;
					buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
					output_pos += 1;
					buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
					output_pos += 1;
				}
			}
			&{ buffer }
		};
		WStr::with_u16s(OUTPUT)
	}};
}

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
	
	pub fn with_u16s(u16s: &[u16]) -> Self {
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


fn main() {
	unsafe {
		let title = wstr!("こんにちは");
		
		println!("title.len -> {}", title.len());
		println!("title.capacity -> {}", title.capacity());

		MessageBoxW(std::ptr::null_mut(), w!("こんにちは、世界"), title.cstr(), MB_OK);
	}
}
