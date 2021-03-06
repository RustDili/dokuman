# Bir Tahmin Oyunu Programlamak

Haydi hep birlikte uygulamalı bir proje üzerinde çalışarak Rust'ın derinliklerine inmeye başlayalım! Bu bölüm size Rust'ın temel kavramlarından bazılarını tanıtacak ve bu kavramları gerçek bir programda nasıl kullanacağınızı gösterecektir. Bölüm boyunca let ve match anahtar kelimelerini, ilişkili metotlar ve işlevleri, harici sandıklar gibi kavramları alıştırma amaçlı ele alacak ve bu kavramları ilerleyen bölümlerde derinlemesine inceleyeceğiz.   

Projemizde klasik bir programlama problemi olan sayı tahmin oyununu kodlayacağız. Program 1 ile 100 arasında rastgele bir sayı oluşturacak ve oyuncudan bu sayıyı tahmin etmesini isteyecektir. Oyuncudan bir tahmin girdisi alındığında, alınan tahmin değeri, programın oluşturduğu sayı ile karşılaştırılacak, sayı yüksek veya düşükse bu bilgi oyuncu ile paylaşılarak yeniden tahmin girmesi istenecek, doğru sayı bulunduğunda ise bir tebrik mesajı yazdırılarak programdan çıkılacaktır.

##  Yeni Bir Proje Oluşturmak

Yeni bir proje oluşturmak için 1. Bölümde oluşturduğumuz *projeler* dizinine giderek aşağıdaki komutları uygulayın:

```console
$ cargo new tahmin_oyunu
$ cd tahmin_oyunu
```

İlk komutumuz `cargo new` birinci argüman olarak projeye verdiğimiz *tahmin_oyunu* adını alır. İkinci komut ise bizi Cargo tarafından oluşturulan yeni proje  dizinine yönlendirir. 

Bu dizinde Cargo tarafından oluşturulmuş bulunan *Cargo.toml* dosyasına göz atalım:

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

Cargo'nun ortamınızdan elde ettiği yazar bilgileri doğru görünmüyorsa, yazar bilgilerini düzenleyerek dosyayı yeniden kaydedebilirsiniz.

Birinci bölümden de hatırlayacağınız gibi `cargo new` komutu sizin için hazır bir "Hello, world!" programı oluşturuyordu. Şimdi *tahmin_oyunu* dizininde oluşturulması gereken bu dosyayı inceleyelim:

<span class="filename">Dosya adı: main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Ve bu programı `cargo run` komutu kullanarak tek seferde derleyip çalıştıralım:

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (/home/dogan/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `/home/rustdili/projeler/tahmin_oyunu/target/debug/tahmin_oyunu`
Hello, world!
```

Sıklıkla kullanılan `run` komutu, bir projeyi çabucak derleyip çalıştırmamız ve bir sonraki derleme adımına hızlıca gitmemiz gerektiğinde oldukça faydalıdır.

Haydi oyun kodlarının yer alacağı *src/main.rs* dosyasını yeniden açarak kodlamaya başlayalım!

## Kullanıcı Girdisinin İşlenmesi

Tahmin oyununun ilk bölümünde, kullanıcılardan bir değer girmesini isteyecek ve bu girdiyi alarak beklenen biçimde olup olmadığını denetleyeceğiz. Programımıza kullanıcılardan bir tahmin girdisi alacak kodları yazarak başlayacağız. Örnek 2-1'de yer alan kodu *src/main.rs* dosyasına ekleyelim:

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

Bu aşama yoğun bilgi içerdiğinden kodları satır satır inceleyerek gidelim. İlk etapta kullanıcı girdisini elde etmek ve değerini yazdırabilmek için Rust standart kütüphanesi `std`'nin bir parçası olan `io` (input/output) kütüphanesini içe aktarmamız gerekir.

```rust
use std::io;
```

Varsayılan haliyle Rust başlatılan her program kapsamına otomatik olarak birkaç türü dahil eder. Bu teknoloji [*prelüd*](https://doc.rust-lang.org/std/prelude/index.html) olarak bilinen ve *otomatik içe aktarma* veya *ön yükleme* olarak kavramlaştırabileceğimiz bir teknolojidir. Eğer kullanmak istediğiniz veri türleri bu ön yükleme modülüne dahil edilmemişse, bunları `use` anahtar sözcüğü kullanarak programınıza dahil etmeniz gerekecektir. Uygulamamızda kullandığımız `std::io` kütüphanesi, kullanıcı girdisini kabul etme yeteneği de dahil bir dizi kullanışlı özellikle birlikte gelir.

Birinci bölümden hatırlayacağınız üzere `main()` işlevi programın giriş noktasını oluşturur.

```rust
fn main() {
```

*Function* kelimesinin kısaltılmışı olan `fn` söz dizimi yeni bir işlev bildirirken, içi boş parantezler `()` işlevin herhangi bir giriş parametresi almadığını, *açılış ayracı* olarak da bilinen sağa bakan süslü parantez `{` ise işlev gövdesinin başlangıç noktasını gösterir.

Yine 1. Bölüm'den hatırlayacağınız üzere, bir dizi karakterin ekrana yazdırılması amacıyla `println!` makrosunu kullanıyorduk.

```rust
    println!("Tuttuğum sayıyı tahmin edin!");

    println!("Lütfen tahmininizi giriniz.");
```

Yukarıdaki kodda ise oyun hakkında bilgi verilerek kullanıcılardan bir sayı girmesi istenmektedir.

### Değerleri Değişkenlerde Saklamak

Sırada aşağıda gösterildiği gibi kullanıcı girdisini depolayacağımız bir yer oluşturmak var:

```rust
    let mut tahmin = String::new();
```

Çok şeyin gerçekleştiği bu satırda program ilginçleşmeye başlıyor. Bu satırın değişken oluşturmak için kullanılan bir `let` ifadesiyle başladığına dikkat edin. İşte size başka bir örnek:
<!--Kaldım-->
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

Programımızın ilk bölümünü `cargo run` komutunu kullanarak test edelim:

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

Klavyeden girdi alıp onu ekrana yazdırabildiğimize göre oyunun ilk bölümü tamamlanmış demektir. 

### Gizli Sayıyı Oluşturmak

Programımızın bu bölümünde kullanıcının tahmin etmeye çalışacağı gizli sayıyı oluşturmamız gerekiyor. Oluşturulacak bu gizli sayı, oyunun tekrar tekrar oynanmasını eğlenceli kılabilmek için her defasında farklı bir numaradan oluşturulmalıdır. Oyunun zorlaşmaması için tahmin edilecek sayıyı, 1 ile 100 arasındaki sayılardan tesadüfü şekilde seçmemiz yerinde olacaktır. Ancak bu rastgele sayıyı oluşturmamızı sağlayacak bir işlev Rust'ın standart kitaplığında  yer almadığından, yine Rust ekibi tarafından sağlanan [`rand`](https://crates.io/crates/rand) adlı harici sandıktan yararlanacağız.

### İlave İşlevler İçin Sandıkları Kullanmak

Sandıklar, Rust kaynak kodu dosyalarından oluşan birer koleksiyondur. Şu anda geliştirmekte olduğumuz bu proje bile,aslında çalıştırılabilir bir ikili *(binary)* sandıktır. Bize harici olarak sunulan `rand` sandığı başka programlarda kullanılması amaçlanan kodları içeren bir *kütüphane sandığı*dır.

Harici sandıkların kullanımı `Cargo` aracının kolaylaştırıcı özelliklerinin ön plana çıktığı en iyi yerlerden biridir. kodlarımızda `rand` sandığı işlevselliğini kullanmadan önce `Cargo.toml` dosyasının bu bağımlılığı içerecek şekilde güncellenmesi gerekir. Bunu gerçekleştirebilmek için aşağıdaki satırları, `Cargo.toml` dosyasında yer alan 
`[dependencies]` başlığının altına doğru biçimde ekleyelim:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[dependencies]
rand = "0.8.3"
```

`Cargo.toml` dosyasındaki bir bölüm başlığını takip eden her şey, başka bir bölüm başlayana kadar devam eden bölümün bir parçasıdır. Bu dosyanın bağımlılıklar `[dependencies]` başlıklı bölümü, projenizin çalışabilmesi için hangi harici sandıklara ve bu sandıkların hangi sürümlerine ihtiyaç duyulduğunu bildirdiğiniz yerdir. Şu aşamada projemizde kullanacağımız`rand` sandığı sürümünü `0.8.3` olarak belirleyeceğiz. Cargo, sürüm numaralarını bildirmekte standart olarak kullanılan [anlamsal sürümleme(http://semver.org/)]yi (SemVer olarak da adlandırılır.) yorumlamayı bildiğinden `0.8.3`'ün aslında `^0.8.3`'ün kısaltması olduğunu anlar. Bağımlılık olarak bildirdiğimiz `rand` sandığının sürüm numarası `0.8.3` ise projemizin en az `0.8.3` olan ancak `0.9.0`'ın altında kalan herhangi bir sürümle çalışabileceği anlamına gelmektedir. Bu durumda Cargo, `0.8.3`'den `0.9.0`'a kadar olası sandık sürümlerinin `0.8.3` sürümüyle uyumlu bir genel API'ye sahip olduğunu varsayar ve projemizin derlenebilmesi için gereken en son sürümü edinerek projemizin çalışmasını sağlar. Bununla birlikte `0.9.0` veya daha sonraki herhangi bir sürümün aşağıdaki örneklerin kullandığı API ile aynı API'ye sahip olacağı gareanti edilmez.

Şimdi projemizi kodlarımızda hiç bir değişiklik yapmadan tıpkı Örnek 2-2'de gösterildiği gibi oluşturalım.

```console
$ cargo build
    Updating crates.io index
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded cfg-if v1.0.0
  Downloaded rand v0.8.3
  Downloaded rand_core v0.6.2
  Downloaded getrandom v0.2.2
  Downloaded libc v0.2.86
  Downloaded 7 crates (698.0 KB) in 1.37s
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppüylend_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling tahmin_oyunu v0.1.0 (/home/rustdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 1m 21s
```

<span class="caption">Örnek 2-2: Rand sandığını bağımlılık olarak ekledikten sonra `cargo build` komutu ile elde edeceğimiz çıktı</span>

Bu aşamada tam olarak aynı sürüm numaralarını ve aynı satırlarda göremeyebilirsiniz. Anlamsal sürümleme *(SemVer)* sayesinde kodumuzla uyumlu olan sürümler işletim sistemimize bağlı olarak farklı sıra ve satırlarda olabilir.  

Artık harici bir bağlantımız olduğuna göre Cargo, [Crates.io](https://crates.io/)'daki verilerin bir kopyası olan *kayıt defteri*nden, ihtiyaç duyduğumuz tüm bağımlılıkların en son sürümlerini alır. Crates.io, Rust ekosistemindeki geliştiricilerin açık kaynak projelerini başkaları ile paylaşmak amacıyla sandıklar şeklinde yayınladıkları bir çevrimiçi kaynaktır.   

Kayıt güncellendikten sonra Cargo, `[dependencies]` bölümünü kontrol ederek henüz sahip olmadığımız sandıkları indirir. Bu aşamada Cargo'nun, bağımlılık olarak sadece rand kütüphanesini eklememize rağmen, bu kütüphane ile çalışabilmemiz için gerekli diğer sandıkları da yüklediğine şahit olacaksınız. Gerekli sandıklar indirildikten sonra Rust önce bu sandıkları derleyecek, arkasından projemizi mevcut bağımlılıklar ile yenicen oluşturacaktır.  

Bu aşamada projenizde hiçbir değişiklik yapmadan yeniden `cargo build` komutunu çalıştırırsanız terminalinizde `Finished` satırının dışında herhangi bir çıktı alamazsınız. Bu eylemsizlik Cargo'nun, kodda bir değişiklik yapılmadığını, bağımlılıkların indirilip derlendiğini ve *Cargo.toml* dosyasında yeni bir ekleme bulunmadığını bilmesinden kaynaklanır. Bu durumda Cargo yapılacak bir şey bulunmadığını9 bildiğinden programı derlemez ve süreci sonlandırır.

Eğer *src/main.rs* dosyasını açarak üzerinde basit bir değişiklik yaparak yeniden kaydedip derlerseniz yalnızca iki satırdan oluşan aşağıdaki çıktıyı görürsünüz: 

```console
$ cargo build
   Compiling tahmin_oyunu v0.1.0 (/home/rustdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
```

Bu satırlar derlemenin sadece *src/main.rs* dosyasındaki küçük değişiklikler ile güncellenerek gerçekleştirdildiğini göstermektedir. Bağımlılıkların değişmediğini, projemizin daha önce indirilip derlenen bağımlılıklarla kullanılabileceğini bilen Cargo, kodu sadece değişen kısmıyla yeniden oluşturur.   


#### `Cargo.lock` Dosyası ile Derlemeleri Tekrarlamak

Cargo, siz aksini belirtinceye kadar, kodunuzun her derlenişinde aynı bağımlılık sürümlerini kullanarak aynı yapının yeniden oluşturulmasını sağlayan bir mekanizmaya sahiptir. Peki ama `rand` sandığı, yakın gelecekte önemli bir hatanın giderildiği ancak kodumuzun bozulmasına neden olan başka bir düzeltmeyi içeren `0.8.4` sürüm numarasıyla yeniden yayınlanırsa ne olur?  

Bunun cevabı, `cargo build` komutunu ilk çalıştırdığınızda *tahmin_oyunu* dizininde oluşturulan *Cargo.lock* dosyasında bulunmaktadır. Bir projeyi ilk kez derlediğinizde kriterlere uyan tüm bağımlılık sürümleri Cargo tarafından belirlenerek *Cargo.lock* dosyasına yazılır. Daha sonraki bir zamanda projeyi yeniden derlemeniz gerektiğinde Cargo, *Cargo.lock* dosyasının halihazırda var olduğunu görecek ve tüm sürüm oluşturma işlemlerini yapmak yerine, orada belirtilmiş sürümleri kullanacaktır. Bu ise otomatik olarak tekrarlanabilir derlemelere sahip olmanızı sağlar. Başka bir ifadeyle, *Cargo.lock* dosyası sayesinde projeniz açık bir biçimde yeniden güncellenene kadar `0.8.3` sürümünde kalmaya devam eder.

#### Bir Sandığı Yeni Bir Sürüme Güncellemek

Bir sandığı güncellemek istediğinizde Cargo size, *Cargo.lock* dosyasını yok sayacak ve *Cargo.toml* dosyanızdaki kriterlerinize uygun en son sürümleri bulmanızı sağlayacak `update` adında bir komut daha sağlar. Süreç başarıyla tamamlanırsa güncellenen bu sürümler *Cargo.lock* dosyasına yazılacaktır.

Sürüm güncelleme için `cargo update` komutunu kullanıldığında varsayılan olarak sadece `0.8.3`'ten büyük `0.9.0`'dan küçük olan sürümler aranacaktır. Eğer `rand` sandığı için `0.8.4` ve `0.9.0` olmak üzere iki yeni sürüm yayınlanmışsa `update` komutunu çalıştırdığınızda aşağıdaki gibi bir çıktı görürsünüz:

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

Bu aşamada *Cargo.lock* dosyanızda halihazırda kullanmakta olduğunuz rand sandık sürümünün `0.8.4` olduğunu belirten bir değişiklik yapıldığını da görebilirsiniz.

Eğer rand sandığının `0.9.0` veya `0.9.x` sürümlerinden birini kullanmak isterseniz, *Cargo.toml* dosyanızı aşağıdaki şekilde güncellemeniz gerekecektir:

```toml
[dependencies]
rand = "0.9.0"
```

Bundan sonra `cargo build` komutunu çalıştırdığınızda, Cargo mevcut sandıkların kayıtlarını güncelleyerek `rand` kütüphanesi gereksinimlerini bildirdiğiniz yeni sürüme göre yeniden değerlendirecektir.

[Cargo ekosistemi](http://doc.crates.io/) hakkında söylenecek çok şey olmasına rağmen bunları, konuyu ayrıntılarıyla tartışacağımız 14. Bölüme saklayacağız. Şimdilik Cargo'nun kütüphanelerin yeniden kullanımını kolaylaştırdığını ve geliştiricilere, bir dizi paketi bir araya getirerek küçük projeler yazma olanağı sağladığını bilmemiz yeterlidir.  

### Rastgele Sayı Üretmek

Rastgele sayı üretiminde kullanacağımız `rand` sandığını *Cargo.toml* dosyasına eklediğimize göre artık bu sandığı kullanmaya başlayabiliriz. Yapacağımız ilk şey *src/main.rs* dosyamızı örnek 2-3'te olduğu gibi güncellemektir.

<span class="filename">Dosya adı: src/main.rs</span>

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Tuttuğum sayıyı tahmin edin!");
    
    let gizli_sayi = rand::thread_rng().gen_range(1..101);
    
    println!("Gizli sayı: {}", gizli_sayi);

    println!("Lütfen tahmininizi giriniz.");
    
    let mut tahmin = String::new();

    io::stdin()
    	.read_line(&mut tahmin)
    	.expect("Veri okuma hatası!");

    println!("Tahmin ettiğiniz sayı: {}", tahmin);
}
```

<span class="caption">Örnek 2-3: Rastgele sayı üretmek için eklenen kodlar.</span>

İlk önce projemizin kapsam alanına `use rand::Rng` şeklinde bir `use` satırı ekliyoruz. Rand kütüphanesinin `Rng` özelliği *(trait)* rastgele sayı üreteçlerinin uyguladığı metotları tanımladığından, bu yöntemleri kullanabilmemiz için kapsama dahil edilmek zorundadır. Özellikler konusunu 10. Bölümde etraflıca inceleyeceğiz.

Hemen ardından ilk ekran çıktısını üreten satırdan sonra iki satır daha ekleyeceğiz. Bu satırlardan ilki olan `rand::thread_rng()` işlevinde, işletim sistemi tarafından başlatılan ve geçerli olan iş parçacığına özgü kullanılan rastgele sayı üreteci başlatılacak ve üretilecek olan sayı `gizli_sayi` adlı değişkende saklanacaktır. Bu sayının üretiminde ise `rand::Rng` olarak kapsama alanına dahil ettiğimiz `Rng` özelliğinde tanımlanmış `gen_range()` metodundan yararlanılacaktır. Kendisine verilen bir aralığa göre rasgele sayı üreten `gen_range()` metodunda kullanılan aralık ifadesi `başlangıç..bitiş` şeklinde olup, başlangıç olarak verilen alt sınır değeri kapsanmakta, bitiş olarak verilen üst sınır değeri ise hariç tutulmaktadır. Bu nedenle 1 ile 100 arasındaki sayılar arasından birini rastgele olarak talep edebilmemiz için metoda ileteceğimiz aralık değerlerini, aralığa dahil edilecek olan 1 ile aralığa dahil edilmeyecek olan üst sayı sınırını bildiren 101 olarak bildirmemiz gerekir. Eğer bu ifade biçimi size karışık geliyorsa, aynı işi yapan ve hem başlangıç hem de bitiş değerlerini aralığa dahil olarak gösterebileceğiniz `1..=100` şeklindeki gösterimi `gen_range()` metoduna aralık olarak iletebilirsiniz.

> Bir sandığın hangi özellik, metot ve işlevlerinin kullanılabileceğini her zaman bilemeyebilirsiniz.
> Sandıkların nasıl kullanılması gerektiğine dair talimatlar o sandığa ait belgelerde yer almaktadır.
> Cargo'nun bir başka güzel özelliği de, tüm bağımlılıklarınız tarafından sağlanan dökümantasyonu yerel 
> olarak oluşturup, tarayıcınızda uyumlu ollartak çalıştıracak olan `cargo doc --open` komutunu sağlamasıdır.
> örneğin `rand` sandığındaki bulunan diğer işlevler hakkında bilgilenmek istiyorsanız, `cargo doc --open`
> komutunu çalıştırarak sol kenar çubuğunda yer alan `rand` seçeneğine tıklamanız yeterli olacaktır.     

Eklediğimiz ikinci satır ise `gizli_sayi` değişkenini yazdırmak için kullanılacaktır. Programı geliştirme aşamasında test amaçlı kullanacağımız bu satır, programımızın nihai sürümünde yer almayacaktır. Fakat daha başlatılır başlatılmaz tahmin edilmesi istenen gizli sayıyı gösteren bir programın da oyun olduğunu pek iddia edemeyiz.

Programı birkaç kez çalıştırmayı deneyin:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 38
Lütfen tahmininizi giriniz.
4
Tahmin ettiğiniz sayı: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 50
Lütfen tahmininizi giriniz.
5
Tahmin ettiğiniz sayı: 5
```

Programınız her çalıştırıldığında 1 ile 100 arasında farklı bir sayı gösteriyorsa bu konuda başarılı olduk demektir!

## Tahmin Sayısının Gizli Sayı ile Karşılaştırılması

Artık kullanıcıdan alınan bir tahmin sayısı ve tasadüfi olarak üretilen bir sayıya sahip olduğumuza göre bunları rahatlıkla karşılaştırabiliriz. Bu aşama Örnek 2-4'te gösterilmekte olup, bu kod daha sonra açıklayacağımız nedenlerden ötürü henüz derlenmemektedir. 

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
// --Kesilen bölüm--
    println!("Tahmin ettiğiniz sayı: {}", tahmin);

    match tahmin.cmp(&gizli_sayi) {
        Ordering::Less => println!("Sayınız küçük!"),
        Ordering::Greater => println!("Sayınız büyük!"),
        Ordering::Equal => println!("Bildiniz!"),
    }
}
```

<span class="caption">Örnek 2-4: Karşılaştırılan iki sayıdan dönen olası değerleri işlemek.</span>

Bu koda eklediğimiz ilk yenilik `std::cmp::Ordering;` adındaki bir türü standart kütüphaneden kod kapsamımıza aktaran yeni bir `use` deyiminin kullanılmış olmasıdır. Tıpkı `Result` türü gibi bir başka `enum` türü olan `Ordering` türü de, `less`, `Greater`, `Equal` şeklinde üç karşılaştırma varyantından oluşur ve bunlar iki değeri karşılaştırırken ortaya çıkan üç olası sonucu temsil etmekte kullanılırllar.

Kodumuza eklediğimiz ikinci yenilik ise, bu `enum` türünü kullanmak amacıyla kodun en alt kısmına yerleştirdiğimiz beş tane yeni satırdan oluşan bir eşleme ifadesidir. Bu eşleme ifadesinde kullandığımız `cmp` metodu, birbiriyle karşılaştırılabilecek her şey için uygulanabilen bir işlevsellik olup, iki değerin karşılaştırılması amacıyla kullanılır. Karşılaştırılması istenen değerin referansını alarak çalışan bu metot, `tahmin` değişkeni içindeki değeri `gizli_sayı` değişkenindeki değer ile karşılaştıracak ve `use` anahtar kelimesiyle kod kapsamına aldığımız `Ordering` türünün varyantlarından uygun olan birini döndürecektir. Elde ettiğimiz dönüş değeriyle ne yapılacağına ise `tahmin` ve `gizli_sayi` değerlerini karşılaştıran `cmp` çağrısından döndürülecek olası sonuçlarla eşleştirdiğimiz ifadelerle karar veriyoruz. 

Dilimize *eşleme* olarak çevirebileceğimiz [`match`](ch06-02-match.html) olası durumları ifade eden dallardan meydana gelir. Bu dallar, bir örüntü *(kalıp, şablon)* ve eşleme ifadesinin başlangıcında belirtilen değerin bu örüntüyle eşleşmesi halinde yürütülecek olan koddan ibarettir. Eşleştirilecek değeri alan Rust bunu sırasıyla her dalın örüntüsüyle karşılaştıracak ve eşleşen daldaki kodu işletecektir. Rust'ın `match` yapısı ve örüntüleri, kodunuzda karşılaşabileceğiniz çeşitli durumları ifade etmenize ve olası her durumun ele alındığından emin olmanızı sağlayan güçlü özelliklerdir. Bu özellikler sırasıyla 6. ve 18. bölümlerde ayrıntılı biçimde ele alınacaktır.

Burada kullanılan eşleme ifadesinin nasıl çalışacağını hayal etmeye çalışalım. Kullanıcının tahmin ettiği sayının 50, rasgele üretilen sayının da 38 olduğunu varsayalım. Kod 50 ile 38 sayılarını karşılaştırdığında, 50 sayısı 38'den büyük olduğundan `cmp` metodu `Ordering::Greater` döndürecek ve `match` ifadesi `Ordering::Greater` değerini alarak her dalın örüntüsünü teker teker kontrol edilmeye başlayacaktır. İlk dalın `Ordering::Less` örüntüsü kontrol edildiğinde bu değerin `Ordering::Greater` ile eşleşmediği görülecek ve bu daldaki kodlar yok sayılarak hemen arkasından bir sonraki dal kontrol edilecektir. Bir sonraki dal incelendiğinde, dalın `Ordering::Greater` örüntüsünün `match` ifademizin almış olduğu `Ordering::Greater` değeriyle aynı olduğu görülecek ve bu koldaki kodlar çalıştırılarak ekrana `Sayınız büyük` yazdırılacaktır. Bu aşamada `match` ifadesi artık bir eşleme yapılmış olduğundan son dala bakmaya gerek duymayacak ve çalışmasını sonlandıracaktır.

Ancak Örnek 2-4'teki kodumuzu derlemeye çalıştığımızda henüz derlenmediğine şahit olacaksınız:

```console
$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |
   = note: expected reference `&String`
              found reference `&{integer}`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game`

To learn more, run the command again with --verbose.
```

Çıktıyı dikkatlice incelediğimizde derleyicinin bize aldığımız hatanın temelinde *tür uyumsuzluğu*nun yattığını bildirdiğini görüyoruz. Rust statik ve güçlü bir tür sistemine sahip olmasına rağmen aynı zamanda tür çıkarım özelliğine de sahip bir programlama dili olduğundan, tahmin değişkenini `let mut tahmin = String::new()` olarak bildirdiğimizde `tahmin` değişkeninin `String` türünde olacağını varsayarak bizi değişkenin türünü açıkça belirtmemiz için zorlamaz. Fakat programımızın rastgele ürettiği `gizli_sayi` değişkenimiz ise sayısal bir değer içermektedir. Rust'ta 1 ile 100 arasındaki sayılartı gösterebilecek belirli sayısal türler vardır. Bunlar, işaretli 32 bitlik sayı türlerini gösteren `i32`, işaretsiz 32 bitlik sayı türlerini göstermek için kullanılan `u32`, işaretli 64 bitlik sayı türlerini gösteren `i64` ve benzerleri gibi sayı türleridir. Eğer kodun farklı bir noktasında, türün farklı olduğunun değerlendirilebileceği şekilde tür bilgisi girilmedikçe, Rust varsayılan tamsayı türünü `i32` olarak varsayacağından `gizli_sayi` değişken türü otomatik olarak `i32` olarak atanacaktır. Dolayısıyla kodu derlemeye kalkıştığımızda aldığımız hatanın nedeni Rust'ın bir `String` türü ile bir sayı türünü karşılaştıramamasıdır.

Bu sorunu çözebilmek için programın kullanıcı girdisi olarak okuduğu `String` türünü bir gerçek bir sayı türüne dönüştürerek onu sayısal değere sahip olan `gizli_sayi` değişkeniyle karşılaştırmamız gerekir. Bunu `main()` işlevine ekleyeğimiz tek satır kod ile gerçekleştirebiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
// --Kesilen bölüm--
    let mut tahmin = String::new();

    io::stdin()Ek bölümler
    	.read_line(&mut tahmin)
    	.expect("Veri okuma hatası!");

    let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı türü girin!");

    println!("Tahmin ettiğiniz sayı: {}", tahmin);

    match tahmin.cmp(&gizli_sayi) {
        Ordering::Less => println!("Sayınız küçük!"),
        Ordering::Greater => println!("Sayınız büyük!"),
        Ordering::Equal => println!("Bildiniz!"),
    }
}
```

Eklediğimiz yeni satır aşağıda yer almaktadır:

```rust,ignore
let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı türü giriniz!");
```

Bu satır programımızda `tahmin` adında yeni bir değişken oluşturur. Hatırlayacağınız üzere programımızda halihazırda kullanılmakta olan bir `tahmin` değişkeni zaten vardı. O halde bu satırla oluşturulan `tahmin` değişkeni ne anlama gelmektedir? Rust bir değişkeni, aynı adlı başka bir değişkenle değiştirmemize izin verir. Gölgeleme olarak adlandırılan bu özellik, bir değeri olduğu türden başka bir türe çevirmek istediğiniz durumlarda oldukça kullanışlıdır. Bu özellik örneğin `tahmin_str` ve `tahmin` gibi iki benzersiz değişken oluşturmak yerine `tahmin` değişken adını yeniden kullanmamıza izin verir. *Gölgeleme konusu 3. Bölümde ayrıntılarıyla ele alınmaktadır.*

Yeniden oluşturduğumuz `tahmin` değişkenini `tahmin.trim().parse()` ifadesine Ek bölümlerbağladığımızda, ifade içindeki `tahmin` `String` türündeki kullanıcı girdisini içeren orjinal `tahmin` değişkenini gösterir. Bir `String` örneğine uygulanan `trim` metodu ise kendisine iletilen dizginin baş ve sonunda bulunan beyaz boşlukları temizler. Her ne kadar `u32` türü yalnızca sayısal karakterler içeriyor olsa da kullanıcının `read_line` işlemini yerine getirmek için enter tuşuna basması gereklidir. Kullanıcı enter tuşuna bastığındaysa dizgiye yeni bir satır eklenecektir. Örneğin kullanıcının tahmin ettiği sayıyı 5 olarak yazıp enter tuşuna bastığınını düşünelim. Bu gerçekleştiğinde `tahmin` içindeki veri `5\n` olarak görünecek, enter tuşuna basılmasından kaynaklı `tahmin` dizgisine İngilizce karşılığı "newline" olan ve *yeni bir satırı* temsil eden `\n` karakteri eklenecektir. İşte `trim` metodunu kullanmakla `\n` karakterini temizleyerek girdinin 5 olmasını sağlamış oluyoruz.

Dizgilerle kullanılan [`parse`](https://github.com/rust-lang/book/blob/master/std/primitive.str.html#method.parse) metodu ise, bir karakter dizisini bir sayı türüne ayrıştırır.Bu metot çeşitli sayı türlerini ayrıştırabildiğinden yapmak istediğimizi Rust'a `let tahmin: u32 ` şeklinde açıkça bildirmemiz gerekir. `tahmin` değişkeninin hemen arkasından gelen `(:)` iki nokta üst üste ise bildirdiğimiz değişkene tür açıklaması ekleyeceğimizi gösterir. Rust'ta birkaç yerleşik sayısal tür  bulunur ve burada kullandığımız `u32` ise işaretsiz 32 bitlik bir tamsayıyı olup küçük bir pozitif sayı için gayet iyi bir tercihtir. Diğer sayı türlerini 3. Bölümde inceleyeceğiz bilgisine ek olarak, bu örnek programa eklediğimiz `u32` tür açıklaması ve bunun `gizli_sayi` ile karşılaştırılması Rust'ın `gizli_sayi` değişken türünü de `u32` olarak çıkarsayacağı anlamına gelmektedir. Bu da artık karşılaştırma işleminin aynı türden iki değer arasında gerçekleştirileceü, anlamına gelmektedir!

Eğer dizgi içeriğinde `A👍%` şeklinde bir değer bulunuyorsa, bu değeri başarılı şekilde bir sayıya dönüştürmenin herhangi bir yolu olmadığından ayrıştırma çağrısı kolaylıkla bir hataya neden olabilir. O nedenle bu metot başarısız olma olasılığına karşı, daha önce [*`Result` Türü ile Olası Hataları İşlemek*](#result-türü-ile-olası-hataları-i̇şlemek) başlığında incelediğimiz gibi ve `read_line` metoduna benzer şekilde bir `Result` türü döndürür. Döndürülen `Result` türünü yine aynı şekilde `expect` metodunu kullanarak değerlendireceğiz. Eğer `parse` metoduyla dizgiden bir sayı elde edilemez ve `Result` türü `Err` varyantını döndürürse `expect` çağrısı programı çökertecek ve kendisine parametre olarak ilettiğimiz *Lütfen bir sayı türü giriniz!* mesajını gösterecektir. Fakat `parse` metodu başarılı olur ve bir sayı üretebilirse, `Result` türü `Ok` varyantını döndüreceğinden `expect` çağrısından da `Ok` varyantı içinde depolanan bu değer döndürülmüş olacaktır.   

Haydi şimdi programımız yeniden çalıştıralım!

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (/home/rustdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 1.53s
     Running `target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 26
Lütfen tahmininizi giriniz.
76
Tahmin ettiğiniz sayı: 76
Sayınız büyük!
```

Programımızın kullanıcı girdisi olarak alınan 76 sayısının önünde boşluklar olmasına rağmen tahmin değerinin 76 olduğunu rahatça fark ediyor olması oldukça güzel! Lütfen programınızı "Sayınız küçük!", "Sayınız büyük!" ve "Bildiniz!" seçeneklerini üretecek şekilde birkaç defa çalıştırarak gözlemleyin.

Bu aşamada oyunun çoğu kısmı doğru çalışıyor gibi görünse de şu an için tek bir tahmin yapılabiliyor olması dikkatinizi çekmiştir. Haydi bu durumu oyuna bir döngü ekleyerek değiştirelim! 

## Döngü ile Çok Sayıda Tahmine İzin Vermek

Bir anahtar kelime olan `loop` sonsuz bir döngü oluşturmakta kullanılır. Şimdi bu döngüyü oyunumuzda kullanıcılara daha fazla tahmin şansı verebilmek için kodumuza ekleyeceğiz.

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
    // --Kesilen bölüm--
    println!("Gizli sayı: {}", gizli_sayi);

    loop {
        println!("Lütfen tahmininizi giriniz.");

        // --Kesilen bölüm--
        
        match tahmin.cmp(&gizli_sayi) {
            Ordering::Less => println!("Sayınız küçük!"),
            Ordering::Greater => println!("Sayınız büyük!"),
            Ordering::Equal => println!("Bildiniz!"),
        }
    }
}
```

Kolaylıkla fark edeceğiniz gibi tahmin giriş isteminden itibaren her şeyi döngü kapsamına taşıdık. Şimdi döngü içinde yer alan her satırı dört boşluk daha girintiledikten sonra programınızı çalıştırın. Programımız bu aşamada tam olarak istediğimiz şeyi yapmakla beraber, kullanıcının çıkmasına izin vermeden sonsuza kadar tahmin bekleyen yeni bir sorunla karşılaşıyor ve kullanıcılarımız oturumlarını kapatamıyor gibi görünüyor değil mi?

Aslında kullanıcılar *ctrl+d* klavye kısayolunu kullanarak programı her zaman sonlandırabilecek olmalarına rağmen bu doyumsuz canavardan kaçmanın bir başka yolu daha var. [Tahmin Sayısının Gizli Sayı ile Karşılaştırılması](#tahmin-sayısının-gizli-sayı-ile-karşılaştırılması) başlığında tartıştığımız `parse` konusundan hatırlayacağınız gibi programdan çıkabilmek için, kullanıcının sayı olmayan bir tahmin verisi girmesiyle programın çökecek olmasından yararlanabiliriz.

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 35
Lütfen tahmininizi giriniz.
45
Tahmin ettiğiniz sayı: 45
Sayınız büyük!
Lütfen tahmininizi giriniz.
20
Tahmin ettiğiniz sayı: 20
Sayınız küçük!
Lütfen tahmininizi giriniz.
45
Tahmin ettiğiniz sayı: 45
Sayınız büyük!
Lütfen tahmininizi giriniz.
çıkış
thread 'main' panicked at 'Lütfen bir sayı türü girin!: ParseIntError { kind: InvalidDigit }', src/main.rs:21:49
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

*Çıkış* gibi sayı olmayan herhangi bir ifadenin girilmesi programdan çıkılmasına yetiyor gibi görünse de bu mekanizma, "Tahmin sayısının doğru girilmesi halinde programın otomatik olarak sonlanması" talebimizi henüz karşılamıyor.

### Doğru Tahmin Sonrası Oyunu Sonlandırmak

Kullanıcının doğru tahmin yaparak oyunu kazanması durumunda, oyunu sonlandırarak programdan çıkılmasını sağlayan `break` anahtar kelimesini kodlarımıza ekleyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
       
        // --Kesilen bölüm--
        match tahmin.cmp(&gizli_sayi) {
            Ordering::Less => println!("Sayınız küçük!"),
            Ordering::Greater => println!("Sayınız büyük!"),
            Ordering::Equal => {
                println!("Bildiniz!");
                break;
            }
        }
    }
}
```

Kullanıcın doğru tahmini yaptığı ve "Bildiniz!" mesajının ekran yazdırıldığı satırın ardına eklenen `break` ifadesi programın döngüden çıkmasını sağlar. Döngü `main` işlevinin son bölümü olduğundan döngüden çıkmak aynı zamanda programdan çıkmak anlamına da gelmektedir. 

### Geçersiz Veri Girişlerini İşlemek

Oyunun davranışını daha da iyileştirebilmek adına, kullanıcı sayısal olmayan bir değer girdiğinde programı çökertmek yerine, programın sayısal olmayan değerleri yok saymasını sağlayarak kullanıcının doğru sayı tahminine devam etmesini sağlayabiliriz. Bunu aşağıda yer alan Örnek 2-5'te gösterildiği gibi `tahmin` değişkenini `String` türünden `u32` türüne dönüştüren satırı değiştirerek gerçekleştirebiliriz.

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
// --Kesilen bölüm--
  
        io::stdin()
            .read_line(&mut tahmin)
            .expect("Veri okuma hatası!");

        let tahmin: u32 = match tahmin.trim().parse() {
            Ok(sayi) => sayi,
            Err(_) => continue,
        };

        println!("Tahmin ettiğiniz sayı: {}", tahmin);
        
        // --Kesilen bölüm--
      
```

<span class="caption">Örnek 2-5: Sayı olmayan bir tahmin girildiğinde programı çökertmek yerine yeni bir tahminin istenmesi</span>

Bir `expect` çağrısını `match` ifadesiyle değiştirmek, genellikle programı çökerten bir hatadan düzgün şekilde işlenmiş bir hataya geçmek için kullanılan tekniktir. Ayrıştırma işlemini gerçekleştiren `parse` metodunun bir `Result` türü döndürdüğünü ve bu türün `OK` veya `Err` varyantlarına sahip bir `enum` türü olduğunu unutmayın. Tıpkı `cmp` metodunun `Ordering` türünden döndürdüğü sonuç değerlerini işlediğimiz gibi burada da bir `match` ifadesi kullandığımıza dikkat edin.

Dizgi `parse` metoduyla başarılı biçimde bir sayıya dönüştürülebildiğinde elde edilen sayıyı içeren bir `Ok` değeri döndürülür. Bu `Ok` değeri ilk dalın örüntüsüyle eşleşecek `match` ifadesi yalnızca `parse` ile oluşturulan `sayi` değerini döndürecek ve `Ok` değerinini içine yerleştirecek ve böylelikle bu sayı yeni oluşturduğumuz `tahmin` değişkeninde yerini alacaktır.

Dizgi `parse` metodunda sayıya dönüştürülemediğindeyse hata hakkında detaylı bilgi içeren `Err` değeri döndürülücektir. Bu değer `match` ifadesinin ilk dalı olan `Ok(sayi)` örüntüsüyle değil ikinci dalın örüntüsü olan `Err(_)` kalıbıyla eşleşecektir. Bu kalıpta yer alan alt çizgi `_` ise her şeyin kapsanmasını isteyen bir değer olup, `Err` varyantındaki değerin ne olduğuna bakılmaksızın tüm `Err`  değerlerinin bu dal ile eşleştirileceğini söylemektedir. Program ikinci dalı çalıştırmakla bu dalda bulunan ve döngünün bir sonraki yinelemesine devam ederek yeni bir tahmin verisi istemesini sağlayan `continue` ifadesi işletilecek, böylelikle `parse` metodunun karşılaşabileceği tüm olası hatalar göz ardı edilmiş olacaktır.    

Bu aşamada artık programımızdaki her şey beklendiği gibi çalışacaktır. Deneyelim:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 90
Lütfen tahmininizi giriniz.
10
Tahmin ettiğiniz sayı: 10
Sayınız küçük!
Lütfen tahmininizi giriniz.
99
Tahmin ettiğiniz sayı: 99
Sayınız büyük!
Lütfen tahmininizi giriniz.
foo
Lütfen tahmininizi giriniz.
90
Tahmin ettiğiniz sayı: 90
Bildiniz!
```

Harika! Küçük bir ince ayar daha yaptıktan sonra oyunumuzu bitireceğiz. Programın halen gizli numarayı ekrana yazdırdığını hatırlıyorsunuz değil mi? Kodlarımız test aşamasında gayet güzel çalışıyorken `gizli_sayi`'nın açık seçik ortada olması oyunun tüm eğlencesini bozuyor. Bunu düzeltebilmek için `gizli_sayi`'yı ekrana bastıran `println!` satırını silmemiz yeterli olacaktır. Aşağıda yer alan Örnek 2-6 kodun tam ve hatasız çalışan halini göstermektedir.

<span class="filename">Dosya adı: src/main.rs</span>

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Tuttuğum sayıyı tahmin edin!");
    
    let gizli_sayi = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Lütfen tahmininizi giriniz.");
    
        let mut tahmin = String::new();

        io::stdin()
            .read_line(&mut tahmin)
        	.expect("Veri okuma hatası!");


        let tahmin: u32 = match tahmin.trim().parse() {
            Ok(sayi) => sayi,
            Err(_) => continue,
        };
        

        println!("Tahmin ettiğiniz sayı: {}", tahmin);

        match tahmin.cmp(&gizli_sayi) {
            Ordering::Less => println!("Sayınız küçük!"),
            Ordering::Greater => println!("Sayınız büyük!"),
            Ordering::Equal => {
                println!("Bildiniz!");
                break;
            }
        }
    }
}
```

<span class="caption">Örnek 2-6: Tahmin oyunu programının son hali</span>

## Özet

Başarıyla çalışan bir sayı tahmin oyunu oluşturduğunuz için teşekkürler!

Bu proje size; `let`, `match`, *metotlar*, *ilişkili işlevler*, harici sandıkların kullanılması gibi birçok Rust kavramını size tanıtmanın uygulamalı bir yolu olmakla beraber, sonraki bölümlerinde bu kavramlarhakkında daha çok şey öğreneceksiniz. Kitabın 3. Bölümü; değişkenler, veri türleri ve işlevler gibi çoğu programlama dili tarafından kullanılan kavramları kapsayacak ve bunların Rust ile nasıl kullanıldığını gösterecektir. 4. Bölümde ise Rust'ı diğer dillerden ayıran önemli bir özellik olan mülkiyet kavramı incelenecek, 5. Bölümdeyse yapı ve metot söz dizimleri tartışılacak, 6. bölümdeyse `enum` türünün çalışması irdelenecektir.
