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

Varsayılan haliyle Rust başlatılan her program kapsamına otomatik olarak birkaç türü dahil eder. Bu teknoloji [*prelüd*](https://doc.rust-lang.org/std/prelude/index.html) olarak bilinen ve otomatik içe aktarma veya ön yükleme olarak kavramlaştırabileceğimiz bir teknolojidir. Eğer kullanmak istediğiniz veri türleri bu otomatik içe aktarmaya dahil edilmemişse, bu türleri `use` anahtar sözcüğünü yardımıyla programınıza ithal etmeniz gerekir. Uygulamamızda kullandığımız `std::io` kütüphanesi bize, kullanıcı girdisini kabul etme yeteneği de dahil olmak üzere bir dizi kullanışlı özellik sağlar.

Bölüm 1'den hatırlayacağınız üzere `main` işlevi programın giriş noktasını oluşturur.

```rust
fn main() {
```

*Function* kelimesinden türemiş olan `fn` anahtar kellimesi yeni bir işlev bildirirken parantezler `()` işlevin herhangi bir giriş parametresi almadığını, küme ayracı `{` ise işlev gözdesinin başlangıç noktasını gösterir.

Yine Bölüm 1'den hatırlayacağınız üzere, ekrana bir dizi karakter yazdırmak içinse  `println!` makrosundan yararlanıyoruz.

```rust
    println!("Tuttuğum sayıyı tahmin edin!");

    println!("Lütfen tahmininizi giriniz.");
```

### Değerleri Değişkenlerde Saklamak

Artık aşağıdaki gösterildiği gibi kullanıcı girişini depolayacağımız bir yer oluşturabiliriz:

```rust
    let mut tahmin = String::new();
```

Ve bu satırda program ilgiçleşmeye başlıyor. burada bir değişken oluşturabilmek için `let` anahtar sözcüğü kullanılmaktadır. İşte size başka bir örnek:

```rust, ignore
let foo = bar;
```

Bu satır foo adında yeni bir değişken yaratarak onu bar değeri ile ilklendirir. Rust'ta değişkenlerin varsayılan olarak değişmez oldukları kabul edilir. Bu kavramı 3. Bölümümüz olan ["Değişkenler ve Değişkenlik"](ch03-01-variables-and-mutability.html) başlığı altında ayrıntılarıyla inceleyeceğiz. Aşağıdaki örnek bize, bir değişkeni değiştirilebilir kılmak için değişken adından önce `mut` anahtar kelimesinin kullanılacağını gösterir:

```rust
let foo = 5;    // değişmez
let mut bar = 5;// değişebilir
```

> Not: `//` söz dizimi satır sonuna kadar uzanan bir açıklamanın başlangıcını belirtir. 3. Bölümde ayrıntılarıyla ele alacağımız yorum satırları, buraya yazılanları Rust'ın derleme aşamasında görmezden gelmesini sağlıyor.

Tahmin oyunumuza geri dönersek, artık `let mut tahmin` söz diziminin, *içeriği değiştirilebilir olarak saklanan* tahmin adında bir değişken tanımı olduğunu anlıyorsunuzdur. Eşittir `=` işaretinin diğer tarafında, yeni bir dizgi örneği döndürmek amacıyla yararlandığımız `String::new` işlevinden elde edilen ve `tahmin` değişkenin ilklendirilmesinde kullanılan değer bulunmaktadır. Dizgiler, UTF-8 baytlarıyla kodlanmış, boyutları değiştirilebilen ve standart kütüphane tarafından sağlanan `String` türündeki metin parçalarıdır.

`String::new()` içindeki `::` söz dizimi, `new()`'in String türünün ilişkili işlevi olduğunu gösterir. İlişkili işlevler türe özgü olduklarından, new işlevinden dönen dizgi bir `String` örneği olarak değil, bir  türüşeklinde elde edilmektedir. Bazı dillerde buna *statik metot* adı verilir.

Bu `new` işlev, yeni ve boş bir dizgi oluşturur. Genellikle `new` olarak adlandırılan ve ilişkili olduğu türün yeni bir değerini oluşturan bu işlevlere Rust'ın birçok türünde rastlayacaksınız.

Özetle `let mut tahmin = String::new();` satırında bir String türünün yeni ve boş bir örneğiyle ilklendirilen değiştirilebilir bir değişken tanımlanmaktadır. Huh!

Hatırlayacağınız gibi daha programımın ilk satırında `use std::io` söz dizimini kullanarak Rust standart kütüphanesinden giriş/çıkış işlevselliğini uygulamış olduğumuzdan artık `io` modülünde bulunan `stdin` işlevini çağırabiliyoruz:

```rust
    io::stdin()
    	.read_line(&mut tahmin)
```

Eğer programın en başına `use std::io` satırını eklememiş olsaydık, `stdin` işlev çağrısını, kod içinde `std::io::stdin` şeklinde yazmamız yetecekti. Bu işlev uçbirimimizde standart girdinin işlenmesi sağlayan `std::io::Stdin` türünün bir örneğini döndürür. 
  

Kodun bir sonraki bölümü olan `.read_line(&mut tahmin)` ifadesindeyse, kullanıcıdan veri almak amacıyla standart girişteki `read_line` metodu çağrılarak, kendisine `&mut tahmin` verisi argüman olarak iletilir.

`read_line` metodunun görevi, kullanıcı tarafından girilen karakterleri standart girişten okumak ve elde edilen veriyi iletilecek olan dizgiye yerleştirmektir. Yöntemin, kullanıcı girdisi eklendikçe dizgi içeriğini değiştirilebilmesi için kendisine verilen argümanın değiştirilebilir olması gerekmektedir.
<!-- Kaldım-->
