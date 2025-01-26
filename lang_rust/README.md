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


let byte_slice: &[u8] =
let byte_slice: &'static [u8] =
&[]; // An empty byte slice
&[236, 149, 161]; // A non-empty byte slice
&[0b0111_1111u8 as i8] // [127]
&[0b1000_0000u8 as i8] // [-128]
b"액션";
b"\xEC\x95\xA1"; // 액션
"액션".as_bytes();
string_utf8.as_bytes(); // binding only
String::from("액션").as_bytes(); // ERROR


let bytemut_slice: &mut [u8] =
&mut bytemut_arr[0..5];


method let bytemut_slice: &mut [u8] =
bytemut_slice[0] = 255; // Modify directly


rets: let byte_vec: Vec<u8> =
"액션".as_bytes().to_vec();
string_utf8.clone().into_bytes();

func:
rets: let byte_vec: Vec<u8> =
args:
iter: "액션"
exam: = "액션".as_bytes().to_vec();

func: std::os::unix::io::IoSlice;
rets: let slices: &mut [IoSlice<u8>] =
args:
iter: std::os::unix::io::IoSlice
exam: = &mut [IoSlice::new(&[1]),IoSlice::new(&[2, 3]),IoSlice::new(&[4, 5, 6]),];

func: std::borrow::Cow::Borrowed();
rets: let cow: Cow<str> =
args: string_slice
iter: Cow<str>
exam: std::borrow::Cow::Borrowed(string_slice);
// Create a Cow that borrows the string slice

func: .into_owned().into();
rets: let mut cowmut_owned: std::borrow::Cow<str> =
args:
iter: cow.
exam: cow.into_owned().into();

func: .to_mut().make_ascii_uppercase();.
rets:
args:
iter: cowmut_owned.
exam: cowmut_owned.to_mut().make_ascii_uppercase();
 
func: .into_owned();
rets: let string_utf8: String =
args:
iter: cow.
exam: = cow.into_owned();

func: .into_owned().into();
rets: let mut cowmut_owned: Cow<str> =
args:
iter: cow.
exam: = cow.into_owned().into();

func: Cow::Owned(); 
rets: let cow_owned: Cow<str> =
args: string_utf8
iter: Cow<str>
exam: = Cow::Owned(string_utf8);

func: std::fs::File::create()?;
rets: let mut file: std::fs::File =
args: "output.txt"
iter: std::fs::File
exam: = std::fs::File::create("output.txt")?; // Create a new file (or open it if it exists)

func: std::net::TcpStream::connect()?;
rets: let mut stream: std::net::TcpStream =
args: "127.0.0.1:8080"
iter: std::net::TcpStream
exam: = std::net::TcpStream::connect("127.0.0.1:8080")?;

func: .write()?;
rets: let mut file: std::fs::File =
args: byte_slice
iter: file.
exam: = file.write(byte_slice)?;  // Write the data to the file

func: std::io::BufWriter::new();
rets: let mut writer: std::io::BufWriter<File> =
args: file
iter: std::io::BufWriter<File>
exam: = std::io::BufWriter::new(file); // Deklarasi tipe data untuk writer

func: .write_fmt();
rets: let mut writer: std::io::BufWriter<File> =
args:
iter: writer.
exam: = writer.write_fmt(format_args!("{} {}", boolean, string_utf8))?; // Menulis format string ke writer

func: .by_ref();
rets: let filemutref: &mut std::fs::File =
args:
iter: file.
exam: = file.by_ref();

func: .write_all()?;
rets:
args: b"ascii" || &byte_arr
iter: filemutref. || stream.
exam: filemutref.write_all(b"ascii"|| &byte_arr)?;

func: .flush()?;
rets:
args:
iter: stream.
exam: stream.flush()?;

