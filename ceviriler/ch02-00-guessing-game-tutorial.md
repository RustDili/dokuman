# Bir Tahmin Oyunu Programlamak

Haydi hep birlikte uygulamalı bir proje üzerinde ilerleyerek Rust'ın derinliklerini kavramaya çalışalım! Bu bölümde sizlere Rust'ın temel kavramlarından bazıları tanıtılacak ve bu kavramların gerçek bir programda nasıl kullanılacağı gösterilecektir. Bölüm boyunca `let` ve `match` anahtar kelimeleri, ilişkili metotlar ve işlevler, harici sandıklar gibi kavramlar alıştırma amaçlı ele alınacak ve ilerleyen bölümlerde bu kavramlar derinlemesine incelenecektir.   

Projemizde klasik bir programlama problemi olan sayı tahmin oyununu kodlayacağız. Program 1 ile 100 arasında rastgele bir sayı oluşturacak ve oyuncudan bu sayıyı tahmin etmesini isteyecektir. Oyuncu tahmin ettiği sayıyı girdiğinde bu değer, programın oluşturduğu sayı ile karşılaştırılacak, sayı yüksek veya düşükse bu bilgi oyuncu ile paylaşılarak yeniden tahmin girilmesi istenecek, doğru sayı bulunduğunda bir tebrik mesajı yazdırılarak programdan çıkılacaktır.

##  Yeni Bir Proje Oluşturmak

Yeni bir proje oluşturmak için 1. Bölümde oluşturduğumuz *projeler* dizinine giderek aşağıdaki komutları uygulayın:

```console
$ cargo new tahmin_oyunu
$ cd tahmin_oyunu
```

İlk satırdaki `cargo new` komutu argüman olarak projeye verdiğimiz *tahmin_oyunu* adını alır. İkinci satırdaki `cd tahmin_oyunu` komutu bizi, Cargo tarafından oluşturulan bu yeni dizine yönlendirir. 

Cargo tarafından otomatik oluşturulan *Cargo.toml* dosyasına göz atalım:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Eğer, Cargo'nun ortamınızdan elde ettiği yazar bilgileri doğru görünmüyorsa, bu bilgileri düzenledikten sonra dosyayı yeniden kaydedin.

Birinci bölümden hatırlayacağınız gibi `cargo new` komutu size hazır bir "Hello, world!" programı sunar. `src/main.rs` dosyasını kontrol edelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Ve bu programı `cargo run` komutu kullanarak tek seferde derleyip çalıştıralım:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

Sıklıkla kullanılan `run` komutu, bir projeyi çabucak derleyip çalıştırmamız ve bir sonraki derleme adımına hızlıca gitmemiz gerektiğinde oldukça faydalıdır.

Programımızı oluşturacağımız *src/main.rs* dosyasını yeniden açarak kodlamaya başlayalım!

## Kullanıcı Girdisinin İşlenmesi

Tahmin oyununun ilk bölümünde, kullanıcılardan bir değer girmesini isteyecek ve bu girdiyi işleyerek beklenen biçimde olup olmadığını denetleyeceğiz. Programımıza kullanıcıların girdiği tahmin verisini alacak kodları yazarak başlayacağız. Örnek 2-1'de yer alan kodu *src/main.rs* dosyasına ekleyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption"> Örnek 2-1: Kullanıcıdan alınan tahmin verisini yazdıran kod</span>

Bu aşama yoğun bilgi içerdiğinden kodun satır satır incelenmesi öğretici olacaktır. İlk etapta kullanıcı girdisini elde etmek ve değerini yazdırabilmek için Rust standart kütüphanesi `std`'nin bir parçası olan `io` (input/output) kütüphanesini içe aktarmamız gerekir.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Varsayılan haliyle Rust başlatılan her program kapsamına otomatik olarak birkaç türü dahil eder. Bu teknoloji [*prelüd*][prelude] <!-- ignore --> olarak teleffuz edebileceğimiz ve *otomatik içe aktarılan* veya başlangıç için gerekli olan program özelliklerinini *ön yüklemesi* olarak kavramlaştırabileceğimiz bir teknolojidir. Eğer kullanmak istediğiniz veri türleri bu ön yükleme modülüne dahil edilmemişse, bunları `use` anahtar sözcüğü kullanarak programınıza dahil etmeniz gerekecektir. Uygulamamızda kullandığımız `std::io` kütüphanesi, kullanıcı girdisini kabul etme yeteneği dahil bir dizi kullanışlı özellikle birlikte gelir.

Birinci bölümden hatırlayacağınız üzere `main()` işlevi programın giriş noktasını oluşturur.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

*Function* kelimesinin kısaltılmışı olan `fn` söz dizimi yeni bir işlev bildirirken, içi boş parantezler `()` işlevin herhangi bir giriş parametresi almadığını, *açılış ayracı* olarak da bilinen sağa bakan süslü parantez `{` ise işlev gövdesinin başlangıç noktasını gösterir.

Yine 1. Bölüm'den hatırlayacağınız üzere, bir dizi karakterin ekrana yazdırılması amacıyla `println!` makrosunu kullanıyorduk.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Yukarıdaki kodda ise oyun hakkında bilgi verilerek kullanıcılardan bir sayı girmesi istenmektedir.

### Değerleri Değişkenlerde Saklamak

Sırada aşağıda gösterildiği gibi kullanıcı girdisini depolayacağımız bir yer oluşturmak var:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Çok şeyin gerçekleştiği bu satırda program ilginçleşmeye başlıyor. Bu satırın *değişken* oluşturmak için kullanılan bir `let` ifadesiyle başladığına dikkat edin. İşte size başka bir örnek:

```rust,ignore
let foo = bar;
```

Bu satır `foo` adında yeni bir değişken oluşturarak onu `bar` değerine bağlar. Rust'ta değişkenlerin varsayılan olarak değişmez oldukları kabul edilir. Bu kavramı 3. Bölümümüz olan ["Değişkenler ve Değişkenlik"][variables-and-mutability] <!-- ignore --> başlığı altında ayrıntılarıyla inceleyeceğiz. Aşağıdaki örnek bize, bir değişkeni değiştirilebilir kılmak için `mut` anahtar kelimesinin nasıl kullanılacağını göstermektedir:

```rust,ignore
let foo = 5;    // değişmez
let mut bar = 5;// değişebilir
```

> Not: `//` söz dizimi satır sonuna kadar uzanan bir açıklamanın başlangıcını belirtir. 
> Rust tarafından derleme aşamasında görmezden gelinen bu satırları 3. Bölümde tartışacağız.

Tahmin oyunumuza geri dönersek, artık `let mut tahmin` söz diziminin, *içeriği değiştirilebilir olarak saklanan* tahmin adında bir değişken tanımı olduğunu anlıyorsunuzdur. Eşittir `=` işaretinin diğer tarafında, yeni bir dizgi örneği döndürmek amacıyla yararlandığımız `String::new()` işlevinden elde edilen ve `tahmin` değişkeninin ilklendirilmesinde kullanılan değer bulunmaktadır. Dizgiler, UTF-8 baytlarıyla kodlanmış, boyutları değiştirilebilen ve standart kütüphane tarafından sağlanan [`String`][string]<!-- ignore --> türündeki metin parçalarıdır.

`String::new()` içindeki `::` söz dizimi, `new()`'in işlevinin `String` türünün ilişkili işlevi yani bu türün metodu olduğunu göstermektedir. İlişkili işlevler türe özgü olduklarından, `new` işlevinden dönen dizgi `String` örneği olarak değil, `String` türü şeklinde elde edilmektedir. Bazı dillerde buna *statik metot* adı verilir.

`new` işlevi, yeni ve boş bir dizgi oluşturur. Genellikle `new` olarak adlandırılan ve ilişkili olduğu türün yeni bir değerini oluşturan bu işlevlerle Rust'ın birçok türünde karşılaşacaksınız.

Özetle `let mut tahmin = String::new();` satırında bir String türünün yeni ve boş bir örneğiyle ilklendirilen değiştirilebilir bir değişken tanımlanmaktadır.

Hatırlayacağınız gibi programın ilk satırında `use std::io` söz dizimini kullanarak Rust standart kütüphanesinden giriş/çıkış işlevselliğini uygulamıştık. Şimdiyse `io` modülünde bulunan `stdin` işlevini çağıracağız:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Eğer programın en başına `use std::io` satırını eklememiş olsaydık, `stdin` işlev çağrısını, kod içinde `std::io::stdin` şeklinde yazmamız gerekecekti. Bu işlev uçbirimimizde standart girdinin işlenmesi sağlayan [`std::io::Stdin`][iostdin]<!-- ignore --> türünün bir örneğini döndürür. 

Kodun bir sonraki bölümü olan `.read_line(&mut tahmin)` ifadesindeyse, kullanıcıdan veri almak amacıyla standart girişteki [`read_line`][read_line]<!-- ignore --> metodu çağrılarak, kendisine `&mut tahmin` verisi argüman olarak iletilir.

`read_line` metodunun görevi, kullanıcı tarafından girilen karakterleri standart girişten okumak ve elde edilen veriyi iletilecek olan dizgiye yerleştirmektir. Yöntemin, kullanıcı girdisi eklendikçe dizgi içeriğini değiştirilebilmesi için, kendisine iletilen argümanın değişebilir olması gerekmektedir.

`&` işareti, bağımsız değişkenin *referans* türünden olduğunu bildirdiğinden, kodun bazı bölümleri tarafından bu değişkenlere, bellekte defalarca kopyalanmaları gerekmeksizin erişilmesi sağlanmış olur. Referanslar dilin güçlü ve karmaşık özelliklerinden olmakla birlikte, Rust'ın önemli avantajlarından biri de, bu karmaşık işlevselliği, güvenilir ve kullanışlı hale getirmesidir. Aslında bu aşamada programı tamamlayabilmek için daha fazla ayrıntıya ihtiyacınız yok. Şimdilik referansların da tıpkı değişkenler gibi varsayılan olarak değişmez kabul edildiğini ve onları değiştirilebilir kılabilmek için `&tahmin` yerine `&mut tahmin` yazmamız  gerektiğini öğrenmemiz yeterlidir. (Referanslar konusu 4.Bölümde ayrıntılı olarak ele alınacaktır.)

### `Result` Türü ile Olası Hataları İşlemek
<!-- Kaldım -->
İncelememize `io::stdin` ile başlayan ifadenin üçüncü satırıyla devam edelim. Her ne kadar ayrı bir satırmış gibi görünmesine rağmen, bu satır da tıpkı bir önceki satır gibi, aynı mantıksal kod satırının parçası olup koda `expect` metodunu eklemektedir:


```rust,ignore
   	.expect("Veri okuma hatası!");
```

Bir metodu `foo()` söz dizimiyle çağırdığınızda, uzun ifadeleri mantıksal parçalara bölebilmeniz için genellikle yeni satırlar ve boşluklar eklemeniz gerekir. Oysa kodumuzu aşağıdaki şekilde de yazabilirdik:

```rust,ignore
io::stdin().read_line(&mut tahmin).expect("Veri okuma hatası!");
```

Ancak böyle bir satırı okumak zor olduğundan, ifadenin daha iyi kavranmasını sağlamak amacıyla onu parçalara ayırmak mantıklıdır. Şimdi bu satırın ne anlama geldiğini inceleyelim.

Daha önce bahsettiğimiz gibi `read_line` işlevi, kullanıcı tarafından girilen verileri kendisine geçirilen bağımsız değişken içine depolarken, bu işin gerçekleştirilmesi sırasında oluşabilecek hataların izlenebilmesi için `io::Result` türünde bir değer döndürür. Rust standart kitaplığı `Result` adı altında, biri genellenmiş türler için, diğeri alt modüllere özgü sürümlerin yer aldığı `io::Result` için olmak üzere birkaç tür bulundurur. 

`Result` türleri genellikle *enums* olarak adlandırılan [numaralandırmalardır](https://github.com/RustDili/book/blob/master/src/ch06-00-enums.html). Numaralandırmalar, sabit değerler kümesine sahip olabilen veri türleri olup bu değerler, *numaralandırmaya ait varyantlar* olarak adlandırılır. Bu türleri 6. Bölümde ayrıntılarıyla ele alacağız.

`Result` türünün `Ok` ve `Err` adında iki varyantı bulunur. Bu varyantların ilki olan `Ok`, işlem sonucunun başarılı olması durumunda döndürülen değere ev sahipliği yaparken, işlemin başarısız olması anlamına gelen `Err` varyantında ise bu başarısızlığın nasıl ve neden olduğunu açıklayan bilgiler depolanır.

`Result` türlerinin amacı, hata işleme bilgilerini kodlamaktır. Tüm türlerde olduğu gibi `Result` türü değerleri de kendileri için tanımlanmış ilişkili yöntemlere sahiptir. Mesela bir `io::Result` örneğinin, `expect` adında bir metodu bulunmaktadır. Bu metot çağrıldığında, `io..Result` örneği `Err` değerindeyse `expect` programın çökmesine neden olacak ve kendisine argüman olarak ilettiğiniz mesajı görüntüleyecektir. `read_line` metodunun `Err` varyantını döndürmesi genellikle işletim sisteminden kaynaklanan bir hatadır. Bununla birlikte `io::Result` örneği `Ok` değerindeyse, `expect` metodu sadece `Ok` içinde saklanan dönüş değerini alacak ve kullanmanız için size döndürecektir. Bu durumda döndürülen `Ok` değeri kullanıcı tarafından standart girdiye iletilen bayt sayısından ibaret olacaktır.

Bu aşamada `expect` metodunu çağırmasanız bile programınız derlenir fakat aşağıdaki gibi bir uyarı alırsınız:

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

Rust `read_line` tarafından döndürülen `Result` değerini kullanmadığınızı hatırlatarak, programın olası bir hatayı işlemediğini bildirmektedir.

Her ne kadar bir hata işleyici yazarak uyarıları bastırabilecek olsak bile, bu noktada yapmak istediğimiz şey, sorun oluştuğunda programın çökmesini sağlamak olduğundan `expect` metodunu kullanmak zorundayız. Hata işlemek konusunu kitabın 9. Bölümünde ayrıntılarıyla inceleyeceğiz.
<!-- Kaldım-->

### `Println!` Yer Tutucuları ile Değerleri Yazdırmak

Kodun sonlandığı noktayı gösteren *kapanış ayracı* (sola bakan süslü parantez) haricinde değerlendirilmesi gereken bir satırımız daha var:

```rust,ignore
    println!("Tahmin ettiğiniz sayı: {}", tahmin);
```

Bu satır kullanıcı girdisini kaydettiğimiz dizgiyi ekrana yazdırabilmek için vardır. Yer tutucuları temsil eden süslü parantezleri `{}` ise bir değerin yerini tutan yengeç kıskaçlarına benzetebilirsiniz. Çok sayıda değerin gösterilmesi amacıyla da kullanabileceğiniz bu parantezlerin ilk çifti, biçimlendirilmiş dizgiden sonraki ilk değeri içerirken, sonraki parantez ikinci değeri, bir sonraki üçüncü değeri gösterecektir. İki farklı değişkenin değerlerini ekrana yazdıran örnek `println!` çağrısı aşağıdakine benzeyecektir:

```rust
let x = 5;
let y = 10;

println!("x değeri = {}, y değeri = {}", x, y);
```

Bu örneğin çıktısı ekrana `x değeri = 5, y değeri = 10` yazdırmaktadır.


### İlk Bölümü Test Etmek

Artık `cargo run` komutunu kullanarak programımızın ilk bölümünü test edebiliriz:

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

Programımızın bu bölümünde kullanıcının tahmin etmeye çalışacağı gizli sayıyı oluşturmaya çalışacağız. Oyunu eğlenceli hale getirip tekrar oynanmasını sağlayabilmek amacıyla her defasında bu gizli sayıyı değiştirmemiz ve oyunu kolaylaştırmak için, tahmin edilecek sayıyı, 1 ile 100 aralığıyla sınırlayarak, bu sayıların seçimini tesadüfü şekilde yapmamız gerekecektir. Rastgele sayı oluşturabileceğimiz bir işlev Rust'ın standart kitaplığında bulundurulmadığından, yine Rust ekibi tarafından sağlanan [`rand`](https://crates.io/crates/rand) adlı harici sandıktan yararlanacağız.

### İlave İşlevler İçin Sandıkları Kullanmak

Sandıklar, Rust kaynak kodu dosyalarından oluşan birer koleksiyondur. Şu anda geliştirmekte olduğumuz bu proje bile, aslında çalıştırılabilir bir ikili *(binary)* sandıktır. Bize harici olarak sunulan `rand` sandığı başka programlarda kullanılması amaçlanan kodları içeren bir *kütüphane sandığı*dır.

Harici sandıkların kullanımı, `Cargo` aracının kolaylaştırıcı özelliklerinin ön plana çıktığı yerlerden biridir. kodlarımıza `rand` sandığı işlevselliğini uygulamadan önce `Cargo.toml` dosyasının bu bağımlılığı içerecek şekilde güncellenmesi gerekir. Bunu gerçekleştirebilmek için aşağıdaki satırları, `Cargo.toml` dosyasında yer alan `[dependencies]` başlığının altına doğru şekilde ekleyelim:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[dependencies]
rand = "0.8.3"..
```

`Cargo.toml` dosyasındaki bölüm başlıklarını takip eden her şey, başka bir bölüm başlayana kadar devam eden bölümün parçasıdır. Bu dosyanın bağımlılıklar `[dependencies]` başlıklı bölümü, projenizin çalışabilmesi için ihtiyaç duyduğu sandıkları ve bu sandıkların ilgili sürümlerini bildirdiğiniz yer olduğundan projemizde kullanacağımız `rand` sandığı sürümünü `0.8.3` olarak belirleyeceğiz. Cargo, sürüm numaralarını bildirmekte standart olarak kullanılan [anlamsal sürümleme](http://semver.org/) sistemini -SemVer olarak da adlandırılır- yorumlamayı bildiğinden, `0.8.3`'ün aslında `^0.8.3`'ün kısaltması olduğunu anlar. Bağımlılık olarak bildirdiğimiz `rand` sandığının sürüm numarası `0.8.3`, projemizin en az `0.8.3` olan ancak `0.9.0`'ın altında kalan herhangi bir sürümle çalışabileceği anlamına gelmektedir. Bu durumda Cargo, `0.8.3`'den `0.9.0`'a kadar olan olası sandık sürümlerinin, `0.8.3` sürümüyle uyumlu genel API'ye sahip olduğunu varsayarak, projemizin derlenebilmesi için gereken en son sürümü ediner ve projemizin çalışmasını sağlar. Bununla birlikte `0.9.0` veya daha sonraki herhangi bir sürümün aşağıdaki örneklerin kullandığı API ile aynı API'ye sahip olacağı garanti edilmez.

Tıpkı Örnek 2-2'de gösterildiği haliyle kodlarımızda hiç bir değişikliğe gitmeden projemizi oluşturalım.

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

<span class="caption">Örnek 2-2: Bağımlılık olarak eklenen `rand` sandığı sonrasında `cargo build` komutuyla elde edilen çıktı.</span>

Derleme esnasında oluşan çıktı işletim sisteminize bağlı olarak değişebileceğinden derlenen paket adları ve sürüm numaraları ekranınızda farklı sırayla yansıtılabilir. Bununla birlikte yüklenen sürümler *anlamsal sürümleme* sayesinde kodumuzla uyumlu olacaktır.

Artık harici bir bağlantımız olduğuna göre Cargo, [Crates.io](https://crates.io/)'daki verilerin bir kopyası olan *kayıt defteri*nden, ihtiyaç duyduğumuz tüm bağımlılıkların en son sürümlerini çekecektir. Crates.io, Rust ekosistemindeki geliştiricilerin açık kaynak projelerini başkaları ile paylaşmak amacıyla sandıklar şeklinde yayınladıkları çevrimiçi bir kaynaktır.   

Kayıt defteri güncellendikten sonra Cargo, `[dependencies]` bölümünü kontrol ederek henüz sahip olmadığımız sandıkları indirir. Bu aşamada Cargo'nun, bağımlılık olarak sadece `rand` kütüphanesini eklememize rağmen, bu kütüphane ile çalışabilmemiz için gerekli diğer sandıkları da yüklediğini göreceksiniz. Gerekli sandıklar indirildikten sonra Rust önce bu sandıkları derleyecek, arkasından projemizi mevcut bağımlılıklar ile yeniden oluşturacaktır.  

Bu aşamada projenizde hiçbir değişiklik yapmayıp `cargo build` komutunu yeniden çalıştırırsanız, uçbiriminizde `Finished` satırını görürsünüz. Bu eylemsizlik Cargo'nun; bağımlılıkların indirilip derlendiğini, kodda değişiklik yapılmadığını ve *Cargo.toml* dosyasının aynı kaldığını bilmesinden kaynaklanır. Bu durumda yapacak bir şey olmadığını fark eden Cargo programı derlemeden süreci sonlandırır.

Fakat *src/main.rs* dosyasını açıp üzerinde basit bir değişiklik yaparak kaydedip derlerseniz, yalnızca iki satırdan oluşan aşağıdaki çıktıyla karşılaşırsınız: 

```console
$ cargo build
   Compiling tahmin_oyunu v0.1.0 (/home/rustdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
```

Bu satırlar derlemenin sadece *src/main.rs* dosyasındaki küçük değişiklikler gözetilerek gerçekleştirdildiğini gösterir. Bağımlılıkların değişmediğini ve projenin, önceden indirilip derlenen bağımlılıklarla kullanılmasının mümkün olduğunu anlayan Cargo, kodu sadece değişen kısmıyla yeniden oluşturur.   


#### `Cargo.lock` Dosyası ile Derlemeleri Tekrarlamak

Cargo, siz aksini belirtinceye kadar, kodunuzun her derlenişinde aynı bağımlılık sürümlerini kullanarak, aynı yapının yeniden oluşturulmasını sağlayan bir mekanizmaya sahiptir. Peki ama ileride önemli bir hatasını giderildiği ama kodumuzun bozulmasına sebep olan başka bir düzeltmeyi içeren `rand` sandığı, `0.8.4` sürümüyle yeniden yayınlanırsa ne olur?  

Bunun cevabı, `cargo build` komutunu ilk çalıştırdığınızda *tahmin_oyunu* dizininde oluşturulan *Cargo.lock* dosyasında bulunmaktadır. Bir projeyi ilk kez derlediğinizde kriterlere uyan tüm bağımlılık sürümleri Cargo tarafından belirlenerek *Cargo.lock* dosyasına yazılır. Daha sonraki bir zamanda projeyi yeniden derlemeniz gerektiğinde Cargo, *Cargo.lock* dosyasının halihazırda var olduğunu görecek ve tüm sürüm oluşturma işlemlerini yapmak yerine, orada belirtilmiş sürümleri kullanacaktır. Bu ise otomatik olarak tekrarlanabilir derlemelere sahip olmanızı sağlar. Başka bir ifadeyle, *Cargo.lock* dosyası sayesinde projeniz açık bir biçimde yeniden güncellenene kadar `0.8.3` sürümünde kalmaya devam eder.

#### Bir Sandığı Yeni Bir Sürüme Güncellemek

Bir sandığı güncellemek istediğinizde Cargo size, *Cargo.lock* dosyasını yok sayacak ve *Cargo.toml* dosyanızdaki kriterlere uygun son sürümleri bulmanızı sağlayacak `update` adında bir komut daha sağlar. Süreç başarıyla tamamlanırsa güncellenen bu sürümler *Cargo.lock* dosyasına yazılır.

Ancak güncelleme esnasında varsayılan olarak sadece `0.8.3`'ten büyük `0.9.0`'dan küçük olan sürümler aranacaktır. Eğer `rand` sandığı için `0.8.4` ve `0.9.0` olmak üzere iki yeni sürüm yayınlanmışsa `update` komutunu çalıştırdığınızda aşağıdaki gibi bir çıktı görünecektir:

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

Bu noktada *Cargo.lock* dosyanızda kullanmakta olduğunuz `rand` sandığı sürümünün `0.8.4`'e yükseltildiğini belirten değişikliğin yapıldığını fark edeceksiniz.

Eğer rand sandığının `0.9.0` veya `0.9.x` sürümlerinden birini kullanmak isterseniz, *Cargo.toml* dosyanızı aşağıdaki şekilde güncellemeniz gerekir:

```toml
[dependencies]
rand = "0.9.0"
```

Ardından `cargo build` komutunu çalıştırdığınızda, Cargo mevcut sandıkların kayıtlarını güncelleyerek `rand` kütüphanesi gereksinimlerini bildirdiğiniz yeni sürüme göre yeniden değerlendirecektir.

[Cargo ekosistemi](http://doc.crates.io/) hakkında söylenecek çok şey olmasına rağmen bunları, konuyu ayrıntılarıyla tartışacağımız 14. Bölüme saklayacağız. Şimdilik Cargo'nun, kütüphanelerin yeniden kullanım olanaklarını kolaylaştırdığını ve geliştiricilere, paketleri bir araya getirerek küçük projeler yazma olanağı sağladığını bilmemiz yeterlidir.

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

Önce projemizin kapsam alanına `use rand::Rng` şeklinde bir `use` satırı ekliyoruz. Rand kütüphanesinin `Rng` özelliği, rastgele sayı üreteçlerinin uyguladığı metotları tanımladığından, bu yöntemlerin kullanabilmesi için, kütüphanenin kapsama dahil edilmesi gerekir. Özellikler *(trait)* konusunu 10. Bölümde etraflıca inceleyeceğiz.

Hemen ardından ilk ekran çıktısını üreten satırdan sonra iki satır daha ekleyeceğiz. Bu satırlardan ilki olan `rand::thread_rng()` işlevinde, işletim sistemi tarafından başlatılan ve geçerli olan iş parçacığına özgü kullanılan rastgele sayı üreteci başlatılacak ve üretilecek olan sayı `gizli_sayi` adlı değişkende saklanacaktır. Bu sayının üretiminde ise `rand::Rng` olarak kapsama alanına dahil ettiğimiz `Rng` özelliğinde tanımlanmış `gen_range()` metodundan yararlanılacaktır. Kendisine verilen bir aralığa göre rasgele sayı üreten `gen_range()` metodunda kullanılan aralık ifadesi `başlangıç..bitiş` şeklinde olup, başlangıç olarak verilen alt sınır değeri kapsanmakta, bitiş olarak verilen üst sınır değeri ise hariç tutulmaktadır. Bu nedenle 1 ile 100 arasındaki sayılar arasından birini rastgele olarak talep edebilmemiz için metoda ileteceğimiz aralık değerlerini, aralığa dahil edilecek olan 1 ile aralığa dahil edilmeyecek olan üst sayı sınırını bildiren 101 olarak bildirmemiz gerekir. Eğer bu ifade biçimi size karışık geliyorsa, aynı işi yapan ve hem başlangıç hem de bitiş değerlerini aralığa dahil olarak gösterebileceğiniz `1..=100` şeklindeki gösterimi `gen_range()` metoduna aralık olarak iletebilirsiniz.

> Bir sandığın hangi özellik, metot ve işlevlerinin kullanılabileceğini her zaman bilemeyebilirsiniz.
> Sandıkların nasıl kullanılması gerektiğine dair talimatlar o sandığa ait belgelerde yer almaktadır.
> Cargo'nun bir başka güzel özelliği de, tüm bağımlılıklarınız tarafından sağlanan dökümantasyonu yerel 
> olarak oluşturup, tarayıcınızda uyumlu olarak çalıştıracak olan `cargo doc --open` komutunu sağlamasıdır.
> örneğin `rand` sandığındaki bulunan diğer işlevler hakkında bilgilenmek istiyorsanız, `cargo doc --open`
> komutunu çalıştırarak, sol kenar çubuğunda yer alan `rand` seçeneğine tıklamanız yeterlidir.     

Eklediğimiz ikinci satır ise `gizli_sayi` değişkenini yazdırmak için kullanılacaktır. Kodumuzun gelişme aşamasında test amaçlı kullanacağımız bu satır, programımızın nihai sürümünde yer almayacaktır. Başlatılır başlatılmaz gizli kalması gereken sayıyı açık eden program oyun değildir!

Programı birkaç defa çalıştırarak deneyin:

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

Program her çalıştırıldığında 1 ile 100 arasında farklı bir sayı gösteriyorsa tasarımımız başarılı demektir!

## Tahmin Sayısının Gizli Sayı ile Karşılaştırılması

Elimizde kullanıcıdan alınan bir tahmin sayısı ve tasadüfi olarak üretilen bir `gizli_sayı` olduğuna göre bunları karşılaştırabiliriz. Kodun bu bölümü Örnek 2-4'te gösterilmekte olup, daha sonra açıklayacağımız nedenlerden ötürü henüz derlenmemektedir. 

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

Bu koda eklediğimiz ilk yenilik, `std::cmp::Ordering;` adındaki bir türü standart kütüphaneden kod kapsamımıza aktaran yeni bir `use` deyiminin kullanılmış olmasıdır. Tıpkı `Result` türü gibi bir başka `enum` türü olan `Ordering` türü de, `less`, `Greater`, `Equal` şeklinde üç karşılaştırma varyantından oluşur ve bunlar, iki değeri karşılaştırırken ortaya çıkan üç olası sonucu temsil etmekte kullanılırlar.

Kodumuza eklediğimiz ikinci yenilik ise, bu `enum` türünü kullanmak amacıyla kodun en alt kısmına yerleştirdiğimiz beş tane yeni satırdan oluşan bir eşleme ifadesidir. Bu eşleme ifadesinde kullandığımız `cmp` metodu, birbiriyle karşılaştırılabilecek her şey için uygulanabilen bir işlevsellik olup, iki değerin karşılaştırılması amacıyla kullanılır. Karşılaştırılması istenen değerin referansını alarak çalışan bu metot, `tahmin` değişkeni içindeki değeri `gizli_sayı` değişkenindeki değer ile karşılaştıracak ve `use` anahtar kelimesiyle kod kapsamına aldığımız `Ordering` türünün varyantlarından uygun olan birini döndürecektir. Elde ettiğimiz dönüş değeriyle ne yapılacağına ise `tahmin` ve `gizli_sayi` değerlerini karşılaştıran `cmp` çağrısından döndürülecek olası sonuçlarla eşleştirdiğimiz ifadelerle karar verilecektir. 

Dilimize *eşleme* olarak çevirebileceğimiz [`match`](ch06-02-match.html) olası durumları ifade eden dallardan meydana gelir. Bu dallar, bir örüntü *(kalıp, şablon)* ve eşleme ifadesinin başlangıcında belirtilen değerin bu örüntüyle eşleşmesi halinde yürütülecek olan koddan ibarettir. Eşleştirilecek değeri alan Rust bunu sırasıyla her dalın örüntüsüyle karşılaştıracak ve eşleşen daldaki kodu işletecektir. Rust'ın `match` yapısı ve örüntüleri, kodunuzda karşılaşabileceğiniz çeşitli durumları ifade etmenize yarayan ve olası her durumun ele alındığından emin olmanızı sağlayan güçlü özelliklerdir. Bu özellikler sırasıyla 6. ve 18. bölümlerde ayrıntılı biçimde ele alınacaktır.

Burada kullanılan eşleme ifadesinin nasıl çalışacağını hayal etmeye çalışalım. Kullanıcının tahmin ettiği sayının 50, rasgele üretilen sayının da 38 olduğunu varsayalım. Kod 50 ile 38 sayılarını karşılaştırdığında, 50 sayısı 38'den büyük olduğundan `cmp` metodu `Ordering::Greater` döndürecek ve `match` ifadesi `Ordering::Greater` değerini alarak her dalın örüntüsünü teker teker kontrol etmeye başlayacaktır. İlk dalın `Ordering::Less` örüntüsü kontrol edildiğinde, bu değerin `Ordering::Greater` ile eşleşmediği görülecek ve bu daldaki kodlar yok sayılarak hemen bir sonraki dala geçilecektir. Geçilen bu dal incelendiğinde, daldaki `Ordering::Greater` örüntüsünün `match` ifademizin almış olduğu `Ordering::Greater` değeriyle aynı olduğu görülecek ve bu koldaki kodlar çalıştırılarak ekrana `Sayınız büyük!` mesajı yazdırılacaktır. Artık bir eşleme bulunmuş olduğundan `match` ifadesi kalan son dala bakmaya gerek duymayacak ve çalışmasını sonlandıracaktır.

Ancak Örnek 2-4'teki kodu derlemek istediğinizde henüz derlenmediğini göreceksiniz:

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

Çıktıda sorunun *tür uyumsuzluğundan* kaynaklandığı belirtiliyor. Rust güçlü bir statik tür sistemine ve tür çıkarım özelliğine sahip programlama dili olduğundan, tahmin değişkenini `let mut tahmin = String::new()` olarak bildirdiğimizde, değişkenin `String` türünde olacağını varsayar. Bu nedenle değişken türünün açıkça belirtilmesi gerekmez. Bununla birlikte programımızın rastgele ürettiği `gizli_sayi` ise sayısal bir değerdir. Rust'ta 1 ile 100 arasındaki sayıları gösterebilecek belli başlı sayısal türler de vardır. Bunlar, işaretli 32 bitlik sayılar için `i32`, işaretsiz 32 bitlik sayılar için `u32`, işaretli 64 bitlik sayılar için kullanılan `i64` gibi türleridir. Tür bilgisi kodun herhangi bir yerinde farklı değerlendirilebilecek şekilde girilmedikçe Rust, tamsayı türünü `i32` olarak varsayacağından, `gizli_sayi` değişkeni `i32` olarak atanacaktır. Dolayısıyla tür uyumsuzluğunun sebebi Rust'ın `String` türü ile  sayı türünü karşılaştıramayacak olmasıdır.

Bu sorunu çözebilmemiz için, kullanıcı girdisi olarak okunan `String` türünü gerçek bir sayı türüne dönüştürüp, sayısal değerli `gizli_sayi` değişkeniyle karşılaştırmamız gerekir. Bunu `main()` işlevine ekleyeğimiz tek satır kod ile gerçekleştirebiliriz:

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

Eklenen yeni satır:

```rust,ignore
let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı türü giriniz!");
```

Bu satır `tahmin` adında yeni bir değişken oluşturur. Hatırlarsanız programımızda kullanılan bir `tahmin` değişkeni zaten vardı. O halde bu satırda yeniden oluşturulan `tahmin` değişkenin anlamı nedir? Rust bir değişkeni, aynı adlı başka bir değişkenle değiştirmemize izin verir. Gölgeleme olarak adlandırılan bu özellik, bir değeri olduğu türden başka bir türe çevirmek istediğiniz durumlarda oldukça kullanışlıdır. Bu özellik örneğin `tahmin_str` ve `tahmin` gibi iki benzersiz değişken oluşturmak yerine `tahmin` değişken adını yeniden kullanmamıza izin verir. (Gölgeleme konusu 3. Bölümde ayrıntılarıyla ele alınmaktadır.)

Yeniden oluşturduğumuz `tahmin` değişkenini `tahmin.trim().parse()` ifadesine bağladığımızda, ifade içindeki `tahmin`, `String` türündeki kullanıcı girdisini içeren orjinal `tahmin` değişkenini gösterir. Bir `String` örneğine uygulanan `trim` metodu ise kendisine iletilen dizginin baş ve sonunda bulunan beyaz boşlukları temizler. Her ne kadar `u32` türü yalnızca sayısal karakterler içeriyor olsa da kullanıcının `read_line` işlemini yerine getirmek için enter tuşuna basması gereklidir. Kullanıcı enter tuşuna bastığındaysa dizgiye yeni bir satır eklenecektir. Örneğin, kullanıcı tahminin 5 olarak yazılıp enter tuşuna basıldığını düşünelim. Bu gerçekleştiğinde `tahmin` içindeki veri `5\n` olarak görünecek, enter tuşuna basılmasından kaynaklı `tahmin` dizgisine İngilizce karşılığı "newline" olan ve *yeni bir satırı* temsil eden `\n` karakteri eklenecektir. `trim` metodunun kullanılması, `\n` karakterinin temizlenerek girdinin sadece 5 olarak kalmasını sağlamış olur. 

Dizgilerle kullanılan [`parse`](https://github.com/rust-lang/book/blob/master/std/primitive.str.html#method.parse) metodu ise, karakter dizisini sayı türüne ayrıştırır. Bu metot çeşitli sayı türlerini ayrıştırabildiğinden, yapılmak istenenin Rust'a `let tahmin: u32 ` şeklinde açıkça bildirilmesi gerekir. `tahmin` değişkeninin hemen ardından gelen `(:)` iki nokta üst üste ise, bildirdiğimiz değişkene tür açıklaması ekleyeceğimizi gösterir. Rust'ta birkaç yerleşik sayısal tür bulunur ve burada kullandığımız `u32` türü, işaretsiz 32 bitlik bir tamsayıyı olduğundan, küçük bir pozitif sayı için gayet iyi bir tercihtir. (Diğer sayı türlerini 3. Bölümde inceleyeceğiz.) `tahmin` değişkenine eklenen `u32` şeklindeki tür açıklaması ve `tahmin` değişkeninin `gizli_sayi` ile karşılaştırılması, Rust tarafından `gizli_sayi` değişken türünün  `u32` olarak çıkarsanacağı anlamına gelmektedir. Artık karşılaştırma işlemi, aynı türden iki değer arasında gerçekleştirilecektir!

Dizgi içeriğinde `A👍%` şeklinde bir değerin bulunması halinde, bu değeri bir sayıya sorunsuzca dönüştürmenin herhangi bir yolu olmadığından, `parse` çağrısı kolaylıkla bir hata üretebilir. Bu nedenle `parse` metodu, başarısız olma ihtimaline karşı daha önce [*`Result` Türü ile Olası Hataları İşlemek*](#result-türü-ile-olası-hataları-i̇şlemek) başlığında incelediğimiz gibi ve `read_line` metoduna benzer şekilde bir `Result` türü döndürür. Döndürülen `Result` türünü ise `expect` metodunu kullanarak değerlendireceğiz. Eğer `parse` metoduyla dizgiden bir sayı elde edilemez ve `Result` türü `Err` varyantını döndürürse `expect` çağrısı programı çökertecek ve kendisine parametre olarak ilettiğimiz *Lütfen bir sayı türü giriniz!* mesajını gösterecektir. Fakat `parse` metodu başarılı olur ve bir sayı üretebilirse, `Result` türü `Ok` varyantını döndüreceğinden `expect` çağrısından da `Ok` varyantı içinde depolanan bu değer döndürülmüş olacaktır.   

Şimdi programımız yeniden çalıştıralım!

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

Kullanıcı girdisiyle alınan 76 sayısının önünde boşluklar olmasına rağmen kodun, tahmin değerini 76 olarak anlaması ne güzel! Lütfen programınızı "Sayınız küçük!", "Sayınız büyük!" ve "Bildiniz!" seçeneklerini üretecek şekilde birkaç defa çalıştırarak gözlemleyin.

Oyunun büyük bölümü doğru çalışıyormuş gibi görünüyor olsa da sadece bir tahmin hakkımızın olması dikkatinizi çekmiştir. Bu durumu oyuna bir döngü ekleyerek değiştirebiliriz! 

## Döngü ile Çok Sayıda Tahmine İzin Vermek

Bir anahtar kelime olan `loop` sonsuz bir döngü oluşturmakta kullanılır. Kullanıcıların tahmin olanaklarını arttırabilmemiz için kodumuzda `loop` döngüsünden yararlanacağız.

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

Göreceğiniz gibi 'tahmin giriş talebi'nden itibaren olan her şeyi döngü kapsamına taşıyarak, her satır için dört boşluk değerinde girinti oluşturduk. Programı çalıştırdığınızda kodun tam olarak istediğimiz şeyi yapmakla beraber, sonsuza kadar tahmin yapılmasını bekleyen yeni bir sorunla kullanıcı çıkışını engellediğini fark edeceksiniz!

Kullanıcılar *ctrl+d* klavye kısayolunu kullanarak programı her zaman sonlandırabilirler. Ancak bu doyumsuz canavardan kaçmanın başka bir yolu daha var. [Tahmin Sayısının Gizli Sayı ile Karşılaştırılması](#tahmin-sayısının-gizli-sayı-ile-karşılaştırılması) başlığında tartıştığımız `parse` konusundan hatırlayacağınız gibi, tahmin verisine sayısal olmayan bir verinin girilmesi programın çökerek sonlanmasını sağlıyordu. Programı çalıştırarak tahmin değerine sayısal olmayan herhangi bir veri ekleyebilirsiniz. Aşağıdaki örnekte tahmin değerine `çıkış` şeklinde bir karakter dizisi iletilmiştir: 

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

Tahmin değişkenine *çıkış* gibi sayısal olmayan herhangi bir ifadenin girilmesi programdan çıkılmasına yetiyor gibi görünse de bu mekanizma, "Tahmin sayısının doğru girilmesi halinde programın otomatik olarak sonlanması" talebimizi henüz karşılamıyor.

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

Kullanıcın doğru tahmini yaptığı ve "Bildiniz!" mesajının ekrana yazdırıldığı satırın ardına eklenen `break` ifadesi programın döngüden çıkmasını sağlar. Döngü `main` işlevinin son bölümü olduğundan döngüden çıkmak aynı zamanda programdan çıkmak anlamına da gelir. 

### Geçersiz Veri Girişlerini İşlemek

Oyunun davranışını daha da iyileştirebilmek amacıyla, sayısal olmayan bir değer alındığında programı çökertmek yerine, bu değerlerin yok sayılmasını ve kullanıcının doğru sayıyı bulana kadar tahmine devam etmesini sağlayalım. Bu iyileştirmeyi Örnek 2-5'te gösterildiği şekilde, `String` türündeki `tahmin` değişkenini, `u32` türüne dönüştüren satırda değişiklik yaparak gerçekleştirebiliriz.

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

`expect` çağrısını `match` ifadesiyle değiştirmek genellikle, programı çökerten bir hatadan düzgün şekilde işlenmiş bir hataya geçmek için kullanılan tekniktir. Ayrıştırma işlemini gerçekleştiren `parse` metodunun bir `Result` türü döndürdüğünü ve bu türün `OK` veya `Err` varyantlarına sahip bir `enum` türü olduğunu unutmayın. Tıpkı `cmp` metodunun `Ordering` türünden döndürdüğü sonuç değerlerini işlediğimiz gibi burada da bir `match` ifadesi kullandığımıza dikkat edin.

`parse` metoduyla dizgi, başarılı şekilde bir sayıya dönüştürebiliyorsa, elde edilen sayıyı içeren bir `Ok` değeri döndürülür. Bu `Ok` değeri ilk dalın örüntüsüyle eşleşecek, `match` ifadesi `parse` ile oluşturulan `sayi` değerini döndürerek `Ok` değerinini içine yerleştirecek ve bu sayı yeni oluşturduğumuz `tahmin` değişkeninde saklanacaktır.

Dizgi `parse` metodunda sayıya dönüştürülemiyorsa da, hata hakkında detaylı bilgi içeren `Err` değeri döndürülücektir. Bu değer `match` ifadesinin ilk dalı olan `Ok(sayi)` örüntüsüyle değil, ikinci dalın örüntüsü olan `Err(_)` kalıbıyla eşleşecektir. Bu kalıpta yer alan alt çizgi `_` ise, her şeyin kapsandığı bir değer olup, `Err` varyantındaki değerin ne olduğuna bakılmaksızın tüm `Err`  değerlerinin bu dal ile eşleştirileceğini söylemektedir. Program ikinci dalı çalıştırmakla, bu dalda bulunan ve döngünün bir sonraki yinelemesine devam ederek yeni bir tahmin verisi istemesini sağlayan `continue` ifadesi işletilecek, böylelikle `parse` metodunun karşılaşabileceği tüm olası hatalar göz ardı edilmiş olacaktır.    

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

Harika! Küçük bir ince ayar daha yaptıktan sonra oyunumuzu bitireceğiz. Fark edeceğiniz gibi program gizli numarayı hala ekrana yazdırıyor. Bu durum test aşamasında gayet iyi ve güzelken, oyunun bitmiş halinde `gizli_sayi`'nın açık seçik ortada olması tüm eğlenceyi bozuyor. Bunu durum `gizli_sayi` değişkenini ekrana yazdıran `println!` satırının silinmesiyle düzelir. Örnek 2-6 kodun tam ve hatasız çalışan halini göstermektedir.

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

Bu proje, `let`, `match`, *metotlar*, *ilişkili işlevler*, harici sandıkların kullanılması gibi birçok Rust kavramını size tanıtmanın uygulamalı bir yoluydu. Kitabın ilerleyen bölümlerinde bu kavramlar hakkında daha çok şey öğreneceksiniz. 
3. Bölümde değişkenler, veri türleri, işlevler gibi çoğu programlama dili tarafından kullanılan kavramları kapsanacak ve bunların Rust ile nasıl kullanıldığı gösterilecektir. 4. Bölümde ise Rust'ı diğer dillerden ayıran önemli bir özellik olan mülkiyet kavramı incelenecek, 5. Bölümde yapı ve metot söz dizimleri tartışılacak, 6. bölümdeyse `enum` türünün çalışması irdelenecektir.

[prelude]: https://doc.rust-lang.org/std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html
[string]: https://doc.rust-lang.org/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org//std/io/struct.Stdin.html#method.read_line
