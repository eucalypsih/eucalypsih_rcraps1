Berikut adalah beberapa varian dari io::ErrorKind yang umum digunakan:
* NotFound: Menunjukkan bahwa file atau direktori yang dicari tidak ditemukan.
* PermissionDenied: Menunjukkan bahwa akses ke file atau direktori ditolak.
* ConnectionRefused: Menunjukkan bahwa koneksi ke server ditolak.
* ConnectionReset: Menunjukkan bahwa koneksi yang sudah ada telah direset.
* TimedOut: Menunjukkan bahwa operasi I/O telah melebihi batas waktu yang ditentukan.
* UnexpectedEof: Menunjukkan bahwa akhir file yang tidak terduga telah tercapai saat membaca data.

futures-core = "0.3"
futures = "0.3"
Trait `Future` adalah inti dari pemrograman asinkron di Rust.
Trait `Stream` digunakan untuk menangani aliran data asinkron.
Trait `Sink` digunakan untuk mengirimkan data ke aliran.

`Output`: Tipe yang akan dihasilkan oleh future ketika selesai.
`poll`: Metode ini digunakan untuk memeriksa apakah future sudah siap untuk menghasilkan nilai. Jika belum, Anda dapat menyimpan Waker untuk memberi tahu kapan future siap untuk dipoll lagi.

`futures_core::future::Future` adalah trait yang mendefinisikan sebuah objek yang dapat dipoll untuk menghasilkan nilai di masa depan. Trait ini merupakan bagian dari crate `futures`, yang menyediakan berbagai utilitas untuk pem编程 asinkron di Rust. Trait Future memungkinkan Anda untuk menulis kode asinkron yang lebih bersih dan terstruktur.
Struct Delay: Ini adalah struct yang menyimpan waktu kapan delay akan selesai.
Implementasi Future: Dalam implementasi `poll`, kita memeriksa apakah waktu saat ini sudah melewati waktu yang ditentukan. Jika sudah, kita mengembalikan `Poll::Ready(())`. Jika belum, kita menyimpan `Waker` dan mengembalikan `Poll::Pending`.
Polling: Di dalam `main`, kita membuat instance dari `Delay` dan melakukan polling dalam loop. Jika `poll` mengembalikan `Poll::Ready`, kita keluar dari loop.

```rust
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::future::Future;
use std::time::{Duration, Instant};
use std::thread;

struct Delay {
    when: Instant,
}

impl Delay {
    fn new(duration: Duration) -> Self {
        Delay {
            when: Instant::now() + duration,
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            // Jika belum siap, simpan waker untuk diberitahu nanti
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    let delay = Delay::new(Duration::from_secs(2));

    // Simulasi polling
    let mut delay = Box::pin(delay);
    let waker = futures::task::noop_waker(); // Waker yang tidak melakukan apa-apa
    let mut context = Context::from_waker(&waker);

    loop {
        match delay.as_mut().poll(&mut context) {
            Poll::Ready(()) => {
                println!("Delay finished!");
                break;
            }
            Poll::Pending => {
                println!("Still waiting...");
                thread::sleep(Duration::from_millis(500)); // Simulasi waktu tunggu
            }
        }
    }
}

```

