use libc::{fopen, fgets, fscanf, fclose, FILE, c_char, c_int, c_double};

struct File {
    file_handle: *mut FILE
}

/// This function converts a string into a Vec<i8> which can
/// be used to represent a c-string.
///
/// To turn this into a *mut c_char, use Vec<i8>::as_mut_ptr().
fn to_c_string(string: &str) -> Vec<i8> {
    let bytes: Vec<u8> = String::from(string).into_bytes();
    let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect();

    c_chars.push(0); // null terminator

    c_chars
}

impl File {
    fn open(path: &str) -> Option<Self> {
        let f = unsafe { fopen(to_c_string(path).as_ptr(), to_c_string("r").as_ptr()) };
        if f.is_null() {
            None
        } else {
            Some(File { file_handle: f })
        }

    }

    fn read_string(&mut self) -> Option<String> {
        let mut buf: [i8; 256] = [0; 256];
        let str = unsafe { fgets(buf.as_mut_ptr(), 256, self.file_handle) };
        if str.is_null() {
            None
        } else {
            String::from_utf8(buf.map(|c| c as u8).to_vec()).ok()
        }
    }

    fn read_i64(&mut self) -> Option<i64> {
        let mut ans: c_int = 0;
        let ret = unsafe { fscanf(self.file_handle, to_c_string("%d").as_ptr(), &mut ans) };
        if ret == -1 {
            None
        } else {
            Some(ans.into())
        }
    }

    fn read_f64(&mut self) -> Option<f64> {
        let mut ans: c_double = 0.0;
        let ret = unsafe { fscanf(self.file_handle, to_c_string("%lf").as_ptr(), &mut ans) };
        if ret == -1 {
            None
        } else {
            Some(ans.into())
        }
    }

    fn read_char(&mut self) -> Option<char> {
        let mut ans: c_char = 0;
        let ret = unsafe { fscanf(self.file_handle, to_c_string(" %c").as_ptr(), &mut ans) };
        if ret == -1 || ans == -1 {
            None
        } else {
            Some(ans as u8 as char)
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        println!("Dropping file.");
        unsafe { fclose(self.file_handle) };
    }
}


fn main() {
    let mut file = File::open("data/test_file.txt").expect("Could not open file.");
    let s = file.read_string().unwrap();
    let i = file.read_i64().unwrap();
    let f = file.read_f64().unwrap();
    let c = file.read_char().unwrap();

    println!("{s} {i} {f} {c}");
}
