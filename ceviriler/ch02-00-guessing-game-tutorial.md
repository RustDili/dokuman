# Bir Tahmin Oyunu Programlamak

Haydi birlikte uygulamalı bir proje üzerinde çalışarak Rust'ın derinliklerine dalmaya hazırlanalım! Bu bölüm size Rust bazı temel kavramlarını tanıtarak bunları gerçek bir programda nasıl uygulayacağınızı gösterir. Bölüm boyunca let ve match anahtar kelimelerini, ilişkili metotlar ve işlevleri, harici sandıklar gibi kavramları alıştırma amaçlı ele alacak ve bu kavramları ilerleyen bölümlerde derinlemesine inceleyeceğiz.   

Projemizde klasik bir programlama problemi olan sayı tahmin oyununu kodlayacağız. Program 1 ile 100 arasında rastgele bir sayı oluşturacak ve oyuncudan bu sayıyı tahmin etmesini isteyecektir. Oyuncudan bir tahmin alındığında, alınan tahmin programın oluşturduğu sayı ile karşılaştırılacak, sayı yüksek veya düşükse bu bilgi oyuncu ile paylaşılarak yeniden tahmin etmesi istenecek, doğru sayı bulunduğunda ise bir tebrik mesajı yazdırılacak ve programdan çıkılacaktır.

##  Yeni Bir Proje Oluşturmak

Yeni bir proje oluşturmak için 1. Bölümde oluşturduğumuz *projeler* dizinine giderek aşağıdaki gibi yeni bir proje oluşturun:

```console
$ cargo new tahmin_oyunu
$ cd tahmin_oyunu
```

İlk komutumuz `cargo new` birinci argüman olarak proje adımız olan *tahmin_oyunu* adını alır. İkinci komut ise bizi Cargo tarafından oluşturulan yeni projemizin dizinine yönlendirir. 

Bu dizinde oluşturulan *Cargo.toml* dosyasına göz atalım:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[package]
name = "tahmin_oyunu"
version = "0.1.0"
authors = ["RustDili <rustdili@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies] 
```

Cargo'nun ortamınızdan elde ettiği yazar bilgileri doğru görünmüyorsa, gereken değişikliği gerçekleştirdikten sonra dosyayı yeniden kaydedebilirsiniz.

Birinci bölümden de hatırlayacağınız gibi `cargo new` komutu sizin için bir "Hello, world!" programı oluşturur. Şimdi *tahmin_oyunu* dizininde bulunan *src/main.rs* dosyasına göz atalım:

<span class="filename">Dosya adı: main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Bu "Hello, world!" programını `cargo run` komutu kullanarak tek adımda derleyip çalıştıralım:

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (/home/dogan/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `/home/dogan/projeler/tahmin_oyunu/target/debug/tahmin_oyunu`
Hello, world!
```

Bu uygulamada da sıkça kullanacağımız `run` komutu, bir projeyi çabucak derleyip çalıştırmamız ve bir sonraki derleme adımına hızlıca gitmemiz gerektiğinde oldukça faydalıdır.

Şimdi tahmin oyunu uygulamasının kodlarını yazacağımız *src/main.rs* dosyasını yeniden açalım.

## Giriş Verisinin İşlenmesi

Tahmin oyununun ilk bölümü, kullanıcıdan bir değer girmesini isteyecek ve bu girdiyi işleyerek beklenen biçimde olup olmadığını denetlemekten ibarettir. Oyunu kullanıcının bir tahmin yapmasına izin vererek başlatalım. Örnek 2-1'de yer alan kodu *src/main.rs* dosyasına ekleyelim:

<span class="filename">Dosya adı: main.rs</span>

```rust
use std::io;

fn main() {
    println!("Tuttuğum sayıyı tahmin edin!");

    println!("Lütfen tahmininizi giriniz.");

    let mut tahmin = String::new();

    io::stdin()
    	.read_line(&mut tahmin)
    	.expect("Veri okuma hatası!");

    println!("Tahmin ettiğiniz sayı: {}", tahmin);
}
```

<span class="caption"> Örnek 2-1: Kullanıcıdan bir tahmin verisi alarak bunu yazdıran kod</span>

Bu kod fazlasıyla bilgi içerdiğinden kendisini satır satır inceleyerek gidelim. Öncelikle kullanıcı girdisini yakalamak ve sonucu çıktı olarak yazdırabilmek için io (input/output) kütüphanesini içe aktarmamız gerekir. Bu kitaplık `std` olarak bilinen Rust standart kütüphanesinin bir parçasıdır. 

```rust
use std::io;
```
<!-- Kaldım-->
