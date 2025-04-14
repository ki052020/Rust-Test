use windows_sys::{
//	core::*,
	Win32::Foundation::*,
	Win32::UI::WindowsAndMessaging::*,
	Win32::Graphics::Gdi::*,
};

use super::{u, null};
pub mod main_wnd;

////////////////////////////////////////////////////////////////
pub struct WndClassSetter {
	class_name: &'static [u16],
	wnd_proc: WNDPROC,

	style: WNDCLASS_STYLES,
	brush_background: HBRUSH,
}

pub trait WndBaseInfo {
	fn wnd_class_setter() -> WndClassSetter;
	fn new() -> Self;
}

////////////////////////////////////////////////////////////////
pub struct WndFactory<T: WndBaseInfo> {
	wnds: Vec<Box<T>>,
}

impl<T: WndBaseInfo> WndFactory<T> {
	pub fn new() -> Self {
		unsafe {
			let setter = T::wnd_class_setter();
			let mut wc = std::mem::zeroed::<WNDCLASSW>();
			
			wc.style = setter.style;
			wc.lpfnWndProc = setter.wnd_proc;
			// GetModuleHandle(NULL) で HINSTANCE のハンドルは取れるらしいが、NULL でも良いらしい
			//wc.hInstance = NULL;
			wc.hIcon = LoadIconW(null!(), IDI_APPLICATION);
			wc.hCursor = LoadCursorW(null!(), IDC_ARROW);
			wc.hbrBackground = setter.brush_background;
			wc.lpszClassName = setter.class_name.as_ptr();

			if RegisterClassW(&wc) == 0 {
				panic!("!!! RegisterClassW(&wc) == 0");
			}
		}
		
		Self {
			wnds: Vec::new(),
		}
	}
	
	// メモリ上に生み出すのみ（window リソースは作成しない）
	pub fn yield_new_wnd<'a>(&'a mut self) -> &'a mut T {
		self.wnds.push(Box::new(T::new()));
		self.wnds.last_mut().unwrap().as_mut()
	}
}

////////////////////////////////////////////////////////////////
pub struct StdWnd {
	hwnd_std_wnd: HWND,
}

impl StdWnd {
	fn new() -> Self {
		Self {
			hwnd_std_wnd: null!(),
		}
	}
	
	fn get_wnd_proc() -> WNDPROC {
		Some(Self::wnd_proc)
	}
	
	fn hwnd(&self) -> HWND {
		self.hwnd_std_wnd
	}

	// -------------------------------------------------------------
	fn create_wnd_rsc(&mut self, wnd_class_name: &[u16], wnd_title: &[u16]) {
		if self.hwnd_std_wnd != null!() {
			panic!("!!! self.hwnd_std_wnd != null!()");
		}

		unsafe {
			let hwnd = CreateWindowExW(
				0,
				wnd_class_name.as_ptr(),
				wnd_title.as_ptr(),
				WS_OVERLAPPEDWINDOW,
				100, 100,  // left, top
				500, 500,  // width, height
				null!(),  // hWndParent
				null!(),  // hMenu
				null!(),  // hInstance
				null!()   // lpParam
			);
			
			if hwnd == null!() {
				panic!("!!! hwnd == null!()");
			}
			
			self.hwnd_std_wnd = hwnd;
		}
	}
	
	// -------------------------------------------------------------
	unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
		match msg {
			WM_DESTROY => {
				PostQuitMessage(0);
				return 0;
			},
			_ => return DefWindowProcW(hwnd, msg, w_param, l_param),
		};
	}
}
