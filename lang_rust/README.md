isok: true<br>
func:<br>
rets: let int_u8: u8 = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = std::u8::MIN; // 0 || = std::u8::MAX; // 255<br>

isok: true<br>
func:<br>
rets: let int_i8: i8 = // Declare a variable of type i8<br>
args:<br>
ityp:<br>
iter:<br>
exam: = std::i8::MIN; // -128 || = std::i8::MAX; // 127 assign it the maximum value<br>

isok: true<br>
func:<br>
rets: let int_u16: u16 = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = std::u16::MIN; || = std::u16::MAX; // 65535<br>

isok: true<br>
func:<br>
rets: let int_u32: u32 = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = std::u32::MIN; || = std::u32::MAX; // 4294967295<br>

isok: true<br>
func:<br>
rets: let string_slice_option: Option<&str> = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = Some("액션"); || = None;<br>

isok: true<br>
func: .is_some();<br>
rets: let boolean: bool = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = string_slice_option.is_some();<br>

isok: true<br>
func: .is_none();<br>
rets: let boolean: bool = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = string_slice_option.is_none();<br>

isok: true<br>
func: .is_empty();<br>
rets: let boolean: bool = <br>
args:<br>
ityp:<br>
iter: false || true || "". || !"". || byte_arr. || string_slice. || stringmut_vec.<br>
exam: = "".is_empty();<br>

isok: true<br>
func: .len();<br>
rets: let int_usize: usize = <br>
args:<br>
ityp:<br>
iter: "". || "액션". || string_slice.<br>
exam: "".len(); // 0 || "액션".len(); // 6 || string_slice.len(); // 6

isok: true<br>
func: .chars().count();<br>
rets: let int_usize: usize = <br>
args:<br>
ityp:<br>
iter: "". || "액션". || string_slice.<br>
exam: std::usize::MAX; || "".chars().count(); // 0 || "액션".chars().count(); // 2 || string_slice.chars().count(); // 2<br>

isok: true<br>
func: .is_null();<br>
rets: let boolean: bool = <br>
args:<br>
ityp:<br>
iter: ptrconst_u8.<br>
exam: ptrconst_u8.is_null();<br>

isok: true<br>
func:<br>
rets: let ptrconst_u8: *const u8 = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = &string_slice; || = "액션".as_ptr(); // Get a pointer to the first byte of the string<br>

isok: true<br>
func: .as_mut_ptr();<br>
rets: let ptrmut: *mut u8 = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = string_slice.as_mut_ptr();<br>

isok: true<br>
func:<br>
rets: let stringmutref_slice: &mut str = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = &mut stringmut_utf8; // Directly use mutable reference<br>

isok: true<br>
func: .as_mut();<br>
rets: let stringmutref_slice: &mut str = <br>
args:<br>
ityp:<br>
iter:<br>
exam: stringmut_utf8.as_mut();<br>

isok: true<br>
func:<br>
rets: let stringmutref_slice: &mut str = <br>
args:<br>
ityp:<br>
iter:<br>
exam:<br>

isok: true<br>
func:<br>
rets:<br>
args:<br>
ityp:<br>
iter: stringmutref_slice.<br>
exam: stringmutref_slice.make_ascii_uppercase();<br>

isok: true<br>
func: .push_str();<br>
rets:<br>
args:<br>
ityp:<br>
iter: stringmutref_slice.<br>
exam: stringmutref_slice.push_str(string_slice);<br>

isok: true<br>
func: .replace_range();<br>
rets:<br>
args: (0..2, string_slice)<br>
ityp:<br>
iter: stringmutref_slice.<br>
exam: stringmutref_slice.replace_range(0..2, string_slice);<br>

isok: true<br>
func:<br>
rets: let string_slice_static: &'static str = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = "액션";

isok: true<br>
func:<br>
rets:<br>
args: ("{:p}", ptrconst_u8)<br>
ityp:<br>
iter:<br>
exam: println!("{:p}", ptrconst_u8); // Print the pointer address ex: 0x5b903b0fd040<br>

isok: true<br>
func: ::from();<br>
rets: let mut stringmut_utf8: String = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = String::from("액션");<br>

isok: true<br>
func: .push_str();<br>
rets:<br>
args: ("액션")<br>
ityp:<br>
iter: stringmut_utf8. || stringmutref_utf8.<br>
exam: stringmut_utf8.push_str("액션");<br>

isok: true<br>
func: ::new();<br>
rets: let mut stringmut_utf8: String = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = String::new();<br>

isok: true<br>
func:<br>
rets: let stringmutref_utf8: &mut String = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = &mut stringmut_utf8; || = &mut struct.field;<br>

isok: true<br>
func: ::new();<br>
rets: let mut stringmut_vec: Vec<&str> = // Create a new empty vector that holds string slices<br>
args:<br>
ityp:<br>
iter:<br>
exam: = Vec::new(); // Create an empty vector<br>

isok: true<br>
func: ::new();<br>
rets: let hashmapmut: HashMap<&str, u8> = // Declare a HashMap with String keys and i32 values<br>
args:<br>
ityp:<br>
iter:<br>
exam: = std::collections::HashMap::new();<br>

isok: true<br>
func: ::new();<br>
rets: let hasher: Hasher<&str, u8> = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = std::hash::Hasher::new();<br>

isok: true<br>
func: .get();<br>
rets: if let Some() = <br>
args: (string_utf8)<br>
ityp:<br>
iter:<br>
exam: hashmapmut.get("one");<br>

isok: true<br>
func: .insert();<br>
rets:<br>
args:<br>
iter:<br>
ityp:<br>
iter:<br>
exam: hashmapmut.insert(string_utf8, 255);<br>

isok: true<br>
func: .push();<br>
rets:<br>
args: ("") || ("액션") || (string_slice)<br>
ityp:<br>
iter: stringmut_vec.<br>
exam: stringmut_vec.push("액션");<br>

isok: true<br>
func: ::new();<br>
rets: let mut bufmut: bytes::BytesMut = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = bytes::BytesMut::new();<br>

isok: true<br>
func: .put_slice();<br>
rets:<br>
args: (b"액션")<br>
ityp:<br>
iter: bufmut.<br>
exam: bufmut.put_slice(b"액션");<br>

isok: true<br>
func: .extend_from_slice();<br>
rets:<br>
args:<br>
ityp:<br>
iter:<br>
exam: bufmut.extend_from_slice(b"액션");<br>

isok: true<br>
func: .extend_from_slice();<br>
rets:<br>
args:<br>
ityp:<br>
iter:<br>
exam:<br>

isok: true<br>
func: .freeze();<br>
rets: let bytes: bytes::Bytes = // Freeze the buffer to create an immutable Bytes instance<br>
args:<br>
ityp: : bytes::BytesMut = <br>
iter: bufmut.<br>
exam: bufmut.freeze();<br>

isok: true<br>
func:<br>
rets: let char_utf8: char = <br>
args:<br>
ityp:<br>
iter:<br>
exam: '액'; // char Literal || string_utf8.chars().nth(0).unwrap(); // '액'<br>

isok: true<br>
func: .into();<br>
rets: let string_utf8: String = <br>
args:<br>
ityp:<br>
iter:<br>
exam: string_utf8.into(); // owned

isok: true<br>
func: .to_owned();<br>
rets: let string_utf8: String = <br>
args:<br>
ityp:<br>
iter: string_slice.<br>
exam: string_slice.to_owned();<br>

isok: true<br>
func: .to_string();<br>
rets: let string_utf8: String = <br>
args:<br>
ityp:<br>
iter: "액션". || string_slice.<br>
exam: "액션".to_string(); // string literal<br>

isok: true<br>
func:<br>
rets: let string_utf8: String = <br>
args: (string_slice) || "액션"<br>
ityp:<br>
iter:<br>
exam: String::from("액션");<br>

isok: true<br>
func: .expect();<br>
rets: let string_utf8: String = <br>
args: (bytevec_u8)<br>
ityp: : Vec<u8> =<br>
iter:<br>
exam: String::from_utf8(bytevec_u8).expect("Invalid UTF-8 Sequence");<br>

isok: true<br>
func: .to_string();<br>
rets:<br>
args:<br>
ityp:<br>
iter: (byte_arr) || (&byte_arr) || ("".as_bytes()) || ("액션".as_bytes()) || (string_slice.as_bytes())<br>
exam: String::from_utf8_lossy(byte_arr).to_string();<br>

isok: true<br>
func: .into_owned();<br>
rets:<br>
args: (byte_arr)<br>
ityp:<br>
iter: (byte_arr)<br>
exam: String::from_utf8_lossy(byte_arr).into_owned();<br>

isok: true<br>
func:<br>
rets: let string_utf8: String = <br>
args: (stringmut_utf8, string_utf8)<br>
ityp:<br>
iter:<br>
exam: std::mem::replace(stringmut_utf8, string_utf8);<br>

isok: true<br>
func: .as_str();<br>
rets: let string_slice: &str = <br>
args:<br>
ityp:<br>
iter:<br>
exam: string_utf8.as_str(); // binding only<br>

isok: false<br>
func: .as_str();<br>
rets: let string_slice: &str = <br>
args:<br>
ityp:<br>
iter:<br>
exam: String::from("액션").as_str(); // ERROR<br>

isok: true<br>
func: .as_ref();<br>
rets: let string_slice: &str = <br>
args:<br>
ityp:<br>
iter: string_utf8.<br>
exam: string_utf8.as_ref();<br>

isok: true<br>
func: "";<br>
rets: let string_slice: &str = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = ""; || "액션"; // directly use a string literal || <br>

isok: true<br>
func: [];<br>
rets: let byte_arr: [u8; 0] = // Define an empty byte slice<br>
args:<br>
ityp:<br>
iter:<br>
exam: = [];<br>

isok: true<br>
func: &[];<br>
rets: let byte_arr: &[] = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = &[];<br>

isok: true<br>
func: [];<br>
rets: let byte_arr: [u8; 6] = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = [236, 149, 161, 236, 133, 152]; // 액션 Action<br>

isok: true<br>
func:<br>
rets: let bytestatic_arr: &'static [i8] =<br>
args:<br>
ityp:<br>
iter: &[0b0111_1111u8 as i8]; || 
&[0b1000_0000u8 as i8];<br>
exam: = &[0b0111_1111u8 as i8]; // [127] || = &[0b1000_0000u8 as i8]; // [-128]<br>

isok: true<br>
func:<br>
rets: let bytestatic_arr: &'static [u8] =<br>
args:<br>
ityp:<br>
iter: &[]; || &[236, 149, 161]; || b"액션"; || b"\xEC\x95\xA1";<br>
exam: = &[]; // An empty byte slice || = &[236, 149, 161]; // A non-empty byte slice<br>

isok: true<br>
func: .as_bytes();<br>
rets: let byte_arr: &[u8] = <br>
args:<br>
ityp: : &str = || : String =<br>
iter: "액션". || string_slice. || string_utf8.<br>
exam: string_slice.as_bytes(); // binding onlyct<br>

isok: true<br>
func: = &[];<br>
rets: let byteslice_arr: &[&str] = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = &["액", "션", "액션"];

isok: false<br>
func: .as_bytes();<br>
rets:<br>
args:<br>
ityp:<br>
iter:<br>
exam: String::from("액션").as_bytes(); // ERROR<br>

isok: true<br>
func:<br>
rets: let bytemut_slice: &mut [u8] =<br>
args:<br>
ityp:<br>
iter:<br>
exam: &mut bytemut_arr[0..5];<br>

isok: true<br>
func: bytemut_slice[]<br>
rets: let bytemut_slice: &mut [u8] = <br>
args:<br>
ityp: : &mut [u8] =<br>
iter: bytemut_slice: &mut [u8]<br>
exam: bytemut_slice[0] = 255; // Modify directly<br>

isok: true<br>
func: = vec![];<br>
rets: let bytevec_u8: Vec<u8> = <br>
args:<br>
ityp:<br>
iter:<br>
exam: = vec![236, 149, 161, 236, 133, 152]; // 액션<br>

isok: true<br>
func: .clone().into_bytes();<br>
rets: let bytevec_u8: Vec<u8> = <br>
args:<br>
ityp: : String =<br>
iter: String::from(); || string_utf8.<br>
exam: = string_utf8.clone().into_bytes();<br>

isok: true<br>
func: .to_vec();<br>
rets: let bytevec_u8: Vec<u8> = <br>
args:<br>
ityp: : &str =<br>
iter: : "". || "액션". || string_slice.<br>
exam: = "액션".as_bytes().to_vec();<br>

isok: true<br>
func: .finish();<br>
rets:<br>
args:<br>
ityp:<br>
iter:<br>
exam:<br>

rets: let iter: std::str::Chars = // membuat iterator dari string<br>
exam: = string_utf8.chars();<br>

rets: let iter: std::collections::hash_map::Iter<&str, u8> = // membuat iterator dari HashMap<br>
exam: = hashmap.iter();<br>

rets: let bytevec_tuple: Vec<(&str, u8)> = <br>
exam: = Vec::from_iter(iter);<br>

rets: let iter: std::ops::Range<u8> = // membuat iterator dari rentang<br>
exam: = 1..255;

rets: let iter = <br>
exam: = bytevec_u8.iter().map(|&x| x * x);<br>

isok: true<br>
func: .iter();<br>
rets: let iter: std::slice::Iter<u8> = <br>
args:<br>
ityp:<br>
iter:<br
exam: = byte_arr.iter(); || bytevec_u8.iter();<br>

rets: let iter: std::slice::Iter<u8> = <br>
exam: = byte_arr.into_iter();<br>

isok: true<br>
func: ::from_iter();<br>
rets: let bytevec_u8: Vec<&u8> = <br>
args: (iter)<br>
ityp:<br>
iter:<br>
exam: = Vec::from_iter(iter);<br>

isok: true<br>
func: ::from_iter();<br>
rets: let set: std::collections::HashSet<&u8> = <br>
args: (iter)<br>
ityp:<br>
iter:<br>
exam: = std::collections::HasSet::from_iter(iter_u8);<br>

isok: true<br>
func: std::os::unix::io::IoSlice;<br>
rets: let slices: &mut [IoSlice<u8>] = <br>
args:<br>
ityp:<br>
iter: std::os::unix::io::IoSlice<br>
exam: = &mut [IoSlice::new(&[1]),IoSlice::new(&[2, 3]),IoSlice::new(&[4, 5, 6]),];<br>

isok: true<br>
func: std::borrow::Cow::Borrowed();<br>
rets: let cow: Cow<str> =<br>
args: string_slice<br>
ityp:<br>
iter: std::borrow::Cow<str><br>
exam: = std::borrow::Cow::Borrowed(string_slice); // Create a Cow that borrows the string slice<br>

isok: true<br>
func: .into_owned().into();<br>
rets: let mut cowmut_owned: std::borrow::Cow<str> = <br>
args:<br>
ityp:<br>
iter: cow.<br>
exam: = cow.into_owned().into();<br>

isok: true<br>
func: .to_mut().make_ascii_uppercase();<br>
rets:<br>
args:<br>
ityp: : std::borrow::Cow<str> = <br>
iter: cowmut_owned.<br>
exam: cowmut_owned.to_mut().make_ascii_uppercase();<br>

isok: true<br>
func: ::from();<br>
rets: let mut stringmut_utf8: String = <br>
args:<br>
ityp:<br>
iter:<br>
exam: String::from("액션");<br>

isok: true<br>
func: .borrow_mut();<br>
rets: let stringmutref_utf8: &mut String = // Borrow a mutable reference to the underlying String<br>
args:<br>
ityp:<br>
iter: <B: std::borrow::BorrowMut<String>>(mut s: B)<br>
exam: = s.borrow_mut();<br>

isok: true<br>
func: .into_owned();<br>
rets: let string_utf8: String = <br>
args:<br>
ityp:<br>
iter: cow.<br>
exam: = cow.into_owned();<br>

isok: true<br>
func: .into_owned().into();<br>
rets: let mut cowmut_owned: std::borrow::Cow<str> =<br>
args:<br>
ityp:<br>
iter: cow.<br>
exam: = cow.into_owned().into();<br>

isok: true<br>
func: Cow::Owned();<br>
rets: let cow_owned: Cow<str> =<br>
args: string_utf8<br>
iryp:<br>
iter: Cow<str><br>
exam: = Cow::Owned(string_utf8);<br>

isok:<br>
func: std::fs::File::create()?;<br>
rets: let mut file: std::fs::File =<br>
args: "output.txt"<br>
ityp:<br>
iter: std::fs::File<br>
exam: = std::fs::File::create("output.txt")?; // Create a new file (or open it if it exists)<br>

isok:<br>
func: std::net::TcpStream::connect()?;<br>
rets: let mut stream: std::net::TcpStream =<br>
args: "127.0.0.1:8080"<br>
ityp:<br>
iter: std::net::TcpStream<br>
exam: = std::net::TcpStream::connect("127.0.0.1:8080")?;<br>

isok:<br>
func: .write()?;<br>
rets: let mut file: std::fs::File =<br>
args: byte_arr<br>
ityp:<br>
iter: file.<br>
exam: = file.write(byte_arr)?;  // Write the data to the file<br>

isok:<br>
func: std::io::BufWriter::new();<br>
rets: let mut writer: std::io::BufWriter<File> =<br>
args: file<br>
ityp:<br>
iter: std::io::BufWriter<File><br>
exam: = std::io::BufWriter::new(file); // Deklarasi tipe data untuk writer<br>

isok:<br>
func: .write_fmt();<br>
rets: let mut writer: std::io::BufWriter<File> =<br>
args:<br>
ityp:<br>
iter: writer.<br>
exam: = writer.write_fmt(format_args!("{} {}", boolean, string_utf8))?; // Menulis format string ke writer<br>

isok:<br>
func: .by_ref();<br>
rets: let filemutref: &mut std::fs::File =<br>
args:<br>
ityp:<br>
iter: file.<br>
exam: = file.by_ref();<br>

isok:<br>
func: .write_all()?;<br>
rets:<br>
args: b"ascii" || &byte_arr<br>
ityp:<br>
iter: filemutref. || stream.<br>
exam: filemutref.write_all(b"ascii"|| &byte_arr)?;<br>

isok:<br>
func: .flush()?;<br>
rets:<br>
args:<br>
ityp:<br>
iter: stream.<br>
exam: stream.flush()?;<br>

isok: true<br>
func: .hash();<br>
rets: impl std::hash::Hash for <br>
args:<br>
ityp:<br>
iter:<br>
exam:<br>
