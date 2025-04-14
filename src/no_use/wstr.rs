
// -------------------------------------------------------------
// null 終端を持つ utf16 文字列
pub struct WStr {
	len: usize,
	capacity: usize,
	buf: Vec<u16>,
}

// -------------------------------------------------------------
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
	
	pub fn cstr(&self) -> *const u16 {
		self.buf.as_ptr()
	}
	
	pub fn len(&self) -> usize {
		self.len
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}
}
