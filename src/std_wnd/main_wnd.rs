use super::*;

// -------------------------------------------------------------
pub struct MainWnd {
	std_wnd: StdWnd,
}

impl MainWnd {	
	const WND_CLASS_NAME: &'static [u16] = &u!("MainWnd");
	const ID_TEST_BUTTON: i32 = 100;

	pub fn create_wnd_rsc(&mut self) {
		self.std_wnd.create_wnd_rsc(Self::WND_CLASS_NAME, &u!("メインウィンドウ"));
		
		// テストコード
		TEST_create_child_wnd(self.std_wnd.hwnd());
	}
	
	pub fn hwnd(&self) -> HWND {
		self.std_wnd.hwnd()
	}
}

// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// テストコード
#[allow(non_snake_case)]
fn TEST_create_child_wnd(hwnd: HWND) {
	unsafe {
		let hwnd = CreateWindowExW(
			0, u!("EDIT").as_ptr(),
			u!("").as_ptr(),  // title
			WS_CHILD | WS_VISIBLE | WS_BORDER,
			10, 10,  // left, top
			200, 200, // width, height
			hwnd,  // hWndParent
			null!(),  // hMenu
			null!(),  // hInstance
			null!()   // lpParam
		);
		
		if hwnd == null!() {
			panic!("!!! hwnd == null!()");
		}
	}
	
	unsafe {
		let hwnd = CreateWindowExW(
			0, u!("BUTTON").as_ptr(),
			u!("テスト").as_ptr(),  // title
			WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON as u32,
			220, 10,  // left, top
			100, 25, // width, height
			hwnd,  // hWndParent
			MainWnd::ID_TEST_BUTTON as HMENU,  // hMenu
			null!(),  // hInstance
			null!()   // lpParam
		);
		
		if hwnd == null!() {
			panic!("!!! hwnd == null!()");
		}
	}
}
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// -------------------------------------------------------------
impl WndBaseInfo for MainWnd {
	fn wnd_class_setter() -> WndClassSetter {
		WndClassSetter {
			class_name: MainWnd::WND_CLASS_NAME,
			wnd_proc: StdWnd::get_wnd_proc(),
			
			style: CS_HREDRAW | CS_VREDRAW,
			brush_background: unsafe { GetStockObject(WHITE_BRUSH) as HBRUSH }
		}
	}
	
	fn new() -> MainWnd {
		MainWnd {
			std_wnd: StdWnd::new(),
		}
	}
}
