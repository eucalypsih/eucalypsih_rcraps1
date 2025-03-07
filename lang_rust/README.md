let boolean: bool
true
false
"".is_empty();
!"액션".is_empty();
byte_slice.is_empty();
string_slice.is_empty();
ptrconst_u8.is_null();



let int_usize: usize
std::usize::MAX;
"".len(); // 0
"".chars().count(); // 0
"액션".len(); // 6
"액션".chars().count(); // 2
string_slice.len(); // 6
string_slice.chars().count(); // 2




let ptrconst: *const u8 =
"액션".as_ptr(); // Get a pointer to the first byte of the string





let ptrmut: *mut u8 =
string_slice.as_mut_ptr();







let strmutref: &mut str =
&mut stringmut_utf8; // Directly use mutable reference
stringmut_utf8.as_mut();





method let strmutref: &mut str =
strmutref.make_ascii_uppercase();
strmutref.push_str(string_slice);
strmutref.replace_range(0..2, string_slice);






let stringsta_slice: &'static str
"액션";



println!("{:p}", ptrconst_u8); // Print the pointer address ex: 0x5b903b0fd040




let mut stringmut_utf8: String =
String::new();
stringmut_utf8.push_str();


let stringmutref_utf8: &mut String =
&mut struct.field


let mut _bytes_mut: BytesMut =
bytes::BytesMut::new();
_bytes_mut.extend_from_slice(b"액션");


let char_utf8: char =
'액'; // char Literal
string_utf8.chars().nth(0).unwrap(); // '액'


let string_utf8: String =
string_utf8.into(); // owned
"액션".to_string(); // string literal
string_slice.to_string();
string_slice.to_owned();
String::from("액션");
String::from(string_slice);
String::from_utf8(byte_vec).expect("Invalid UTF-8 Sequence");
String::from_utf8_lossy(byte_arr).to_string();
String::from_utf8_lossy(&byte_arr).to_string();
String::from_utf8_lossy("액션".as_bytes()).to_string();
String::from_utf8_lossy(string_slice.as_bytes()).to_string();
String::from_utf8_lossy(byte_arr).into_owned();


let string_utf8: String =
std::mem::replace(stringmut_utf8, string_utf8);


let string_slice: &str =
"";
"액션"; // directly use a string literal
string_utf8.as_ref();
string_utf8.as_str(); // binding only
String::from("액션").as_str(); // ERROR



let byte_arr: [u8; 6] =
[236, 149, 161, 236, 133, 152]; // 액션 Action

method let byte_arr: [u8; 6] =
&byte_arr


isok: true<br>
func:<br>
rets: let byte_slice: &'static [i8] =<br>
args:<br>
ityp:<br>
iter: &[0b0111_1111u8 as i8]; || &[0b1000_0000u8 as i8];<br>
exam: = &[0b0111_1111u8 as i8]; // [127] || = &[0b1000_0000u8 as i8]; // [-128]<br>

isok: true<br>
func:<br>
rets: let byte_slice: &'static [u8] =<br>
args:<br>
ityp:<br>
iter: &[]; || &[236, 149, 161]; || b"액션"; || b"\xEC\x95\xA1";<br>
exam: = &[]; // An empty byte slice || = &[236, 149, 161]; // A non-empty byte slice<br>

isok: true<br>
func: .as_bytes();<br>
rets: let byte_slice: &[u8] =
args:<br>
ityp: : &str = || : String =<br>
iter: "액션". || string_slice. || string_utf8.<br>
exam: string_slice.as_bytes(); // binding onlyct<br>

isok: false<br>
func: .as_bytes();<br>
rets:<br>
args:<br>
ityp:<br>
iter:<br>
exam: String::from("액션").as_bytes(); // ERROR

isok: true<br>
func:<br>
rets: let bytemut_slice: &mut [u8] =<br>
args:<br>
ityp:<br>
iter:<br>
exam: &mut bytemut_arr[0..5];<br>

isok: true<br>
func: bytemut_slice[]<br>
rets: let bytemut_slice: &mut [u8] =<br>
args:<br>
ityp: : &mut [u8] =<br>
iter: bytemut_slice: &mut [u8]<br>
exam: bytemut_slice[0] = 255; // Modify directly<br>

isok: true<br>
func: .clone().into_bytes();<br>
rets: let byte_vec: Vec<u8> =<br>
args:<br>
ityp: : String =<br>
iter: String::from(); || string_utf8.<br>
exam: = string_utf8.clone().into_bytes();<br>

isok: true<br>
func:<br>
rets: let byte_vec: Vec<u8> =<br>
args:<br>
ityp: : &str =<br>
iter: : "". || "액션". || string_slice.<br>
exam: = "액션".as_bytes().to_vec();<br>

isok: true<br>
func: std::os::unix::io::IoSlice;<br>
rets: let slices: &mut [IoSlice<u8>] =<br>
args:<br>
ityp:<br>
iter: std::os::unix::io::IoSlice<br>
exam: = &mut [IoSlice::new(&[1]),IoSlice::new(&[2, 3]),IoSlice::new(&[4, 5, 6]),];<br>

isok: true<br>
func: std::borrow::Cow::Borrowed();<br>
rets: let cow: Cow<str> =<br>
args: string_slice<br>
ityp:<br>
iter: Cow<str><br>
exam: = std::borrow::Cow::Borrowed(string_slice); // Create a Cow that borrows the string slice<br>

isok: true<br>
func: .into_owned().into();<br>
rets: let mut cowmut_owned: std::borrow::Cow<str> =<br>
args:<br>
ityp:<br>
iter: cow.<br>
exam: = cow.into_owned().into();<br>

isok: true<br>
func: .to_mut().make_ascii_uppercase();<br>
rets:<br>
args:<br>
ityp:<br>
iter: cowmut_owned.<br>
exam: cowmut_owned.to_mut().make_ascii_uppercase();<br>

isok: true<br>
func: .into_owned();<br>
rets: let string_utf8: String =<br>
args:<br>
ityp:<br>
iter: cow.<br>
exam: = cow.into_owned();<br>

isok: true<br>
func: .into_owned().into();<br>
rets: let mut cowmut_owned: Cow<str> =<br>
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
args: byte_slice<br>
ityp:<br>
iter: file.<br>
exam: = file.write(byte_slice)?;  // Write the data to the file<br>

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

