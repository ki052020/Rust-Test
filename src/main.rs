#![allow(dead_code)]
use windows_sys::{
	core::*,
	Win32::UI::WindowsAndMessaging::*
};

mod wstr;
use wstr::*;
// macro というモジュール名は利用できなかったため、mcr.rs にしている
mod mcr;

fn main() {
	let test = WStr::from_u(&u!("おはよう"));
	unsafe {
		MessageBoxW(null!(), w!("こんにちは、世界"), test.cstr(), MB_OK);
	}
}
