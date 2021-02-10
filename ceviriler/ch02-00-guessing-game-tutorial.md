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
     Running `/home/rustdili/projeler/tahmin_oyunu/target/debug/tahmin_oyunu`
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

*Function* kelimesinden türemiş olan `fn` anahtar kellimesi yeni bir işlev bildirirken parantezler `()` işlevin herhangi bir giriş parametresi almadığını, *açılış ayracı* (sağa bakan süslü parantez)  `{` ise işlev gövdesinin başlangıç noktasını gösterir.

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

Tahmin oyunumuza geri dönersek, artık `let mut tahmin` söz diziminin, *içeriği değiştirilebilir olarak saklanan* tahmin adında bir değişken tanımı olduğunu anlıyorsunuzdur. Eşittir `=` işaretinin diğer tarafında, yeni bir dizgi örneği döndürmek amacıyla yararlandığımız `String::new` işlevinden elde edilen ve `tahmin` değişkeninin ilklendirilmesinde kullanılan değer bulunmaktadır. Dizgiler, UTF-8 baytlarıyla kodlanmış, boyutları değiştirilebilen ve standart kütüphane tarafından sağlanan `String` türündeki metin parçalarıdır.

`String::new()` içindeki `::` söz dizimi, `new()`'in String türünün ilişkili işlevi olduğunu gösterir. İlişkili işlevler türe özgü olduklarından, `new` işlevinden dönen dizgi bir `String` örneği olarak değil, bir `String` türü şeklinde elde edilmektedir. Bazı dillerde buna *statik metot* adı verilir.

Bu `new` işlev, yeni ve boş bir dizgi oluşturur. Genellikle `new` olarak adlandırılan ve ilişkili olduğu türün yeni bir değerini oluşturan bu işlevlere Rust'ın birçok türünde rastlayacaksınız.

Özetle `let mut tahmin = String::new();` satırında bir String türünün yeni ve boş bir örneğiyle ilklendirilen değiştirilebilir bir değişken tanımlanmaktadır. Huh!

Hatırlayacağınız gibi daha programımın ilk satırında `use std::io` söz dizimini kullanarak Rust standart kütüphanesinden giriş/çıkış işlevselliğini uygulamış olduğumuzdan artık `io` modülünde bulunan `stdin` işlevini çağırabiliyoruz:

```rust
    io::stdin()
    	.read_line(&mut tahmin)
```

Eğer programın en başına `use std::io` satırını eklememiş olsaydık, `stdin` işlev çağrısını, kod içinde `std::io::stdin` şeklinde yazmamız yetecekti. Bu işlev uçbirimimizde standart girdinin işlenmesi sağlayan `std::io::Stdin` türünün bir örneğini döndürür. 
  

Kodun bir sonraki bölümü olan `.read_line(&mut tahmin)` ifadesindeyse, kullanıcıdan veri almak amacıyla standart girişteki `read_line` metodu çağrılarak, kendisine `&mut tahmin` verisi argüman olarak iletilir.

`read_line` metodunun görevi, kullanıcı tarafından girilen karakterleri standart girişten okumak ve elde edilen veriyi iletilecek olan dizgiye yerleştirmektir. Yöntemin, kullanıcı girdisi eklendikçe dizgi içeriğini değiştirilebilmesi için kendisine iletilen bağımsız değişkenin değiştirilebilir olması gerekmektedir.

`&` işareti bağımsız değişkenin *referans* türünden olduğunu bildirdiğinden, kodun bazı bölümleri tarafından bu değişkenlere, bellekte defalarca kopyalanmaları gerekmeksizin erişimlesi sağlanmış olur. Referanslar dilin güçlü ve karmaşık bir özelliği olmakla birlikte, Rust'ın en önemli avantajlarından biri de bu karmaşık işlevselliği güvenli ve kullanımı kolay hale getirmesidir. Aslında bu programı tamamlayabilmek için çok fazla ayrıntı bilmemize gerek yok. Şimdilik referansların da tıpkı değişmezler gibi varsayılan olarak değişmez kabul edildiğini ve onları değiştirilebilir kılabilmek için `&tahmin` yerine `&mut tahmin` yazmamız  gerektiğini öğrenmemiz yeterlidir. (Referanslar konusu 4.Bölümde ayrıntılı olarak ele alınacaktır.)

### `Result` Türü ile Olası Hataları İşlemek

Kodumuzun bu bölümünde yapılması gereken işleri hebüz bitirmiş sayılmadığımızdan, incelememize `io::stdin` ile başlayan ifadenin üçüncü satırıyla devam ediyoruz. Her ne kadar ayrıymış gibi görünüyor olmasına rağmen, bu satır da tıpkı bir önceki satır gibi, aynı mantıksal kod satırının bir parçasıdır ve koda bir metot eklemektedir:


```rust,ignore
   	.expect("Veri okuma hatası!");
```

Bir metodu `foo()` söz dizimiyle çağırdığınızda uzun ifadeleri mantıksal parçalara ayırmak için genellikle yeni satırlar ve boşluklar eklemeniz gerekir. Kodumuzu aşağıdaki şekilde de yazabilirdik:

```rust,ignore
io::stdin().read_line(&mut tahmin).expect("Veri okuma hatası!");
```

Ancak böyle bir satırı okumak zor olduğundan, ifadenin daha iyi kavranmasını sağlamak amacıyla onu parçalara ayırmak iyi bir yaklaşımdır. Şimdi bu satırın ne anlama geldiğini inceleyelim. 

Daha önce bahsettiğimiz gibi `read_line` işlevi, kullanıcı tarafından girilen verileri kendisine geçirilen bağımsız değişken içine depolarken, bu işin gerçekleştirilmesi sırasında oluşabilecek hataların izlenebilmesi için `io::Result` türünde bir değer döndürür. Rust standart kitaplığı `Result` adı altında, biri genellenmiş türler için, diğeri alt modüllere özgü sürümlerin yer aldığı `io::Result` olmak üzere birkaç tür bulundurur. 

`Result` türleri genellikle `enums` olarak adlandırılan numaralandırmalardır. Numaralandırmalar, sabit bir değerler kümesine sahip olabilen veri türleri olup bu değerler, *enum varyantları* olarak adlandırılır. Bu türleri 6. Bölümde ayrıntılarıyla ele alacağız.

`Result` türünün `Ok` ve `Err` adında iki varyantı bulunur. Bu varyantların ilki olan `OK`, işlem sonucunun başarılı olması durumunda döndürülen değere ev sahipliği yaparken, işlemin başarısız olması anlamına gelen `Err` varyantında ise bu başarısızlığın nasıl ve neden olduğunu açıklayan bilgiler depolanır.

Bu `Result` türlerinin amacı, hata işleme bilgilerini kodlamaktır. Tüm türlerde olduğu gibi `Result` türü değerleri de kendileri için tanımlanmış ilişkili yöntemlere sahiptir. Bir `io::Result` örneğinin, `expect` adında çağırabileceğimiz bir metodu bulunmaktadır. Bu metot çağrıldığında, `io..Result` örneği `Err` değeriyse `expect` programın çökmesine neden olacak ve kendisine argüman olarak ilettiğiniz mesajı görüntüleyecektir. Eğer `read_line` metodu bir `Err` döndürürse bunun nedeni büyük olasılıkla işletim sisteminden kaynaklanan bir hata olacaktır. Ama `io::Result` örneği bir `Ok` değeriyse, `expect` metodu sadece `Ok` içinde saklanan dönüş değerini alarak kullanabilmeniz için size döndürecektir. Bu durumda bu `Ok` değeri kullanıcı tarafından standart girdiye girilen bayt sayısı olacaktır.

Bununla birlikte `expect` metodunu çağırmadığınız hallerde de programınız derlenecek, fakat aşağıdaki gibi bir uyarı verecektir:

```console
   Compiling tahmin_oyunu v0.1.0 (/home/rustdili/projeler/tahmin_oyunu)
warning: unused `std::result::Result` that must be used
  --> src/main.rs:10:5
   |
10 | /     io::stdin()
11 | |         .read_line(&mut tahmin);
   | |________________________________^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.84s
```

Rust `read_line` tarafından döndürülen `Result` değerini kullanmadığınız konusunda uyarıda bulunarak, programın olası bir hatayı işlemediğini bildirir.

Aslında uyarıyı bastırmanın doğru yolu bir hata işleyici yazmaktan geçiyor olsa da, şu aşamada tek yapmak istediğimiz, bir sorun oluştuğunda programın çömesini sağlamak olduğundan `expect` metodunu kullanıyoruz. Hata işlemek konusunu kitabın 9. Bölümünde ayrıntılarıyla inceleyeceğiz.


### `Println!` Yer Tutucuları ile Değerleri Yazdırmak

Bu aşamada, kodumuzun sonlandığı noktayı gösteren *kapanış ayracı* (sola bakan süslü parantez) haricinde tartışacağımız bir satırımız daha var:

```rust,ignore
    println!("Tahmin ettiğiniz sayı: {}", tahmin);
```

Bu satır kullanıcı girdisini kaydettiğimiz dizgiyi yazdırır. Yer tutucuları temsil eden süslü parantezleri `{}` ise bir değerin yerini tutan yengeç kıskaçları olarak hayal edebilirsiniz. Çok sayıda değeri göstermek için de kullanabileceğiniz süslü parantezlerin ilk çifti, biçimlendirilmiş dizgiden sonraki ilk değeri içerirken, sonraki parantez ikinci değerii bir sonraki üçüncü değeri v.b. gösterir. Böyle bir `println!` çağrısı aşağıdaki gibi görünecektir:

```rust
let x = 5;
let y = 10;

println!("x değeri = {}, y değeri = {}", x, y);
```

Bu kodun çıktısı ekrana `x değeri = 5, y değeri = 10` yazdıracaktır.


### İlk Bölümü Test Etmek

İlk bölümün testini `cargo run` komutuyla çalıştırarak yapalım:

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (/home/rustdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 1.36s
     Running `target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Lütfen tahmininizi giriniz.
6
Tahmin ettiğiniz sayı: 6
```

Klavyeden girdi alıp onu ekrana yazdırabildiğimize göre oyunun ilk bölümünün tamamlanmış demektir. 

<!-- Kaldım-->
