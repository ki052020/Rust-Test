// -------------------------------------------------------------
struct GDIObj {
	gdi_obj: HGDIOBJ
}

impl GDIObj {
	fn new() -> Self {
		Self {
			gdi_obj: null!()
		}
	}
	
	fn with(obj: HGDIOBJ) -> Self {
		Self {
			gdi_obj: obj
		}
	}
	
	fn set_once(&mut self, obj: HGDIOBJ) {
		if self.gdi_obj != null!() {
			panic!("!!! self.gdi_obj != null!()");
		}
		self.gdi_obj = obj;
	}
	
	fn get(&self) -> HGDIOBJ {
		#[cfg(debug_assertions)]
		if self.gdi_obj == null!() {
			panic!("!!! self.gdi_obj == null!()");
		}
		
		self.gdi_obj
	}
}

impl Drop for GDIObj {
	fn drop(&mut self) {
		if self.gdi_obj == null!() { return; }
		
		unsafe {
			DeleteObject(self.gdi_obj);
		}
	}
}

// -------------------------------------------------------------
struct InitOnce<T: Default> {
	val: T,
	is_inited: bool,
}

impl<T: Default> InitOnce<T> {
	fn new() -> Self {
		Self {
			val: T::default(),
			is_inited: false,
		}
	}
	
	fn with(val: T) -> Self {
		Self {
			val,
			is_inited: true,
		}
	}
}
