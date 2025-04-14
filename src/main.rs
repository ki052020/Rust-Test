#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]
use windows_sys::{
//	core::*,
//	Win32::Foundation::*,
	Win32::UI::WindowsAndMessaging::*,
	Win32::Graphics::Gdi::*,
};

// macro というモジュール名は利用できなかったため、mcr.rs にしている
mod mcr;
mod std_wnd;
use std_wnd::{*, main_wnd::*};

// -------------------------------------------------------------
fn main() {
	let mut main_wnd_factory: WndFactory<MainWnd> = WndFactory::new();
	
	// メモリ上に生み出すのみ（window リソースは作成しない）
	let main_wnd = main_wnd_factory.yield_new_wnd();
	// window リソースを生成
	main_wnd.create_wnd_rsc();

	unsafe {
		let hwnd = main_wnd.hwnd();
		ShowWindow(hwnd, SW_NORMAL);
		UpdateWindow(hwnd);
		let mut msg = std::mem::zeroed::<MSG>();
		loop {
			if GetMessageW(&mut msg, null!(), 0, 0) == 0 {
				return;
			}
			TranslateMessage(&mut msg);
			DispatchMessageW(&mut msg);
		}
	}
}
