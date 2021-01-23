## Merhaba, Cargo

Cargo Rust'ın derleme sistemi ve paket yöneticisidir. Bu araç sizin için; kod oluşturmak, kodun bağımlı olduğu kütüphaneleri *(kodunuzun ihtiyaç duyduğu kitaplıklara bağımlılık adını veriyoruz)* indirmek ve bunları derlemek gibi pek çok görevi yerine getirdiğinden çoğu Rustacean bu aracı Rust projelerini yönetmek için kullanır.

Daha önce yazdığımıza benzeyen basit Rust programlarının herhangi bir kütüphaneye bağımlılığı bulunmadığından "Merhaba dünya!" gibi basit bir projeyi Cargo ile derlemeye çalışsaydık, Cargo'nun sadece kodu derlemekle ilgili olan bölümü kullanılmış olacaktı. Daha karmaşık ve farklı kütüphanelerle çalışmaya ihtiyaç duyan Rust programları yazmaya niyetlendiğinizde, projenin Cargo aracılığıyla oluşturulması bağımlılıkları yönetmenize epey yardımcı olacaktır.

Rust projelerinin büyük çoğunluğu Cargo aracını kullandığından, bu kitabın geri kalan bölümlerinde sizin de bu aracı kullandığınız varsayılacaktır. Eğer ["Kurulum"](ch01-01-installation.md) bölümünde açıklanan resmi yükleyicileri kullandıysanız Cargo aracı Rust ile birlikte gelmiş demektir. Rust'ı başka yollar ile kurduysanız sisteminizde Cargo'nun kurulu olup olmadığını terminalinize aşağıdaki kodları girerek kontrol edebilirsiniz.

```console
$ cargo --version
```

Terminalinizde çıktı olarak bir sürüm numarası görünüyorsa Cargo aracınız Rust kurulumuyla birlikte yüklenmiş demektir. Eğer `Command not found` şeklinde bir hata ile karşılaşıyorsanız Cargo'nun nasıl kurulacağına dair kullandığınız kurulum yönteminin belgelerine bakmalısınız.

### Cargo ile Proje Oluşturmak

Cargo aracılığıyla yeni bir proje oluşturarak bunun ilk projemiz olan "Merhaba dünya!" ile farklılıklarını incelemeye çalışalım. Hamgi işletim sizteminde olduğunuzdan bağımsız *projeler* dizini ya da kodlarınızı nereye kaydediyorsanız o dizine gelerek aşağıdaki komutları çalıştırım:

```console
$ cargo new merhaba_cargo
$ cd merhaba_cargo
```

İlk komut "merhaba_cargo" adlı yeni bir dizin oluşturur. Cargo bir proje için gerekli olan dosyaları aynı zamanda projemizin de adı olan bu dizinde oluşturur. 

Daha sonraki komutla da Cargo aracı tarafından yaratılan: Cargo.toml, main.rs adlı iki dosya ve src adlı bir dizinin listelendiği "merhaba_cargo" içeriği görüntülenir.

Bununla birlikte Cargo .gitignore dosyası ile eşliğinde yeni bir git deposunun da başlatılmasını sağlar. Eğer halihazırda mevcut bir deposu bulunuyorsa `cargo new` komutu yeni bir git deposu oluşturmaz. Eğer yine de bir git deposu oluşturmak istiyorsanız bunun için `cargo new --vcs=git` komutunu kullanmanız gerekir.

> Not: Git yaygın olarak kullanılan bir sürüm kontrol sistemidir. Cargo'yu --vcs bayrağı aracılığıyla farklı bir 
> sürüm kontrol sistemi kullanmak ya da sürüm kontrol sistemini kullanmamak üzere ayarlayabilirsiniz. Mevcut 
> seçenekleri görmek için `cargo new -help` komutunu çalıştırmanız yeterlidir.

Cargo.toml dosyasını metin düzenleyicinizde açtığınızda içeriği örnek 1-2'dekine benzer biçimde görünmelidir.

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[package]
name = "merhaba_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

<span class="caption"> Örnek 1-2: `cargo new` komutuyla oluşturulan Cargo.toml dosyası içeriği</span>

Bu dosya, Cargo'nun yapılandırma formatı olan [TOML](https://toml.io/en/)(Tom's Obvious, Minimal Language) biçimindedir.

İlk satırda bildirilen ve altındaki ifadeler tarafından oluşturulan [package] bölüm başlığı, paketin nasıl yapılandırıldığını gösterir. Paket içeriğine ayrıntı eklendikçe, farklı bölüm başlıkları da eklenecektir. 

Sonraki dört satır ise programınızın Cargo tarafından derlenebilmesi için gereken: İsim, sürüm, programın yazarı ve kullanılacak Rust sürümü gibi yapılandırma bilgilerinden oluşur. Cargo isim ve e-posta gibi bilgileri ortamınzdan edineceğinden bilgileriniz eksik veya yanlış ise bu dosyayı şimdiden düzenleyerek kaydetmeniz gerekir. `Sürüm` anahtarı konusuna ise Ek E bölümünde değineceğiz.

Son satırda projenizin bağımlılıklarını listelemeye yarayan [dependencies] bölümü yer alır. Rust'ta kodların paketler halinde tutulduğu yapılara `crate` yani sandık adı verilir. Bu proje için harici bir sandığa ihtiyaç duymayacak fakat 2. Bölümde gerçekleştireceğimiz ilk projede bağımlılıklar bölümünü kullanacağız.

Şimdi *src/main.rs* dosyasını açalım ve inceleyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
fn main() {
    println!("Merhaba, dünya!");
}
```

Cargo sizin için tıpkı Örnek 1-1'de olduğu gibi ekranınıza "Merhaba, dünya!" metnini bastıran bir program oluşturdu. Önceki projemiz ile Cargo tarafından üretilen bu proje arasındaki farklar ise, Cargonun projeyi *src* adlı dizine yerleştirmesi ve üst dizinde ise bir *Cargo.toml* dosyası yaratması olarak özetlenebilir.

Cargo kaynak dosyalarının *src* dizininde bulundurulmasını bekler. Projenin ana dizin içeriği, sadece README dosyaları, lisans bilgileri, yapılandırma bilgileri ve kodunuzu ilgilendiren diğer şeyler içindir. Dolayısıyla Cargo, her şeyin kendi yerinde depolanmasını sağlayarak projelerinizin düzenlenmesine yardımcı olur.

Eğer bir projeyi ilk "Merhaba, dünya!" örneğindeki gibi Cargo kullanmadan başlattıysanız, bu projeyi Cargo kullanarak olşuturulmuş haline döndürebilirsiniz. Bunun için proje kodunu *src* dizinine taşımanız ve projenin ana dizinde *Cargo.toml* dosyası oluşturmanız yeterlidir.

### Bir Cargo Projesini Derleyip Çalıştırmak

Şimdi "Merhaba, dünya!" programını Cargo kullanarak derleyip çalıştırdığımızda oluşan farklılıkları gözlemleyelim. Terminalinizde *merhaba_cargo* dizinine gelerek aşağıdaki komut yardımıyla projenizi oluşturun: 

```console
$ cargo build
    Compiling merhaba_cargo v0.1.0 (/home/rustdili/projeler/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.54s
```

Bu komut geçerli dizininiz yerine *target/debug/merhaba_cargo* (veya Windows ortamında *target\debug\merhaba_cargo.exe*) 
konumunda çalıştırılabilir bir dosya oluşturu Bu dosyayı şu komutla çalıştırabilirsiniz:

```console
$ ./target/debug/merhaba_cargo # veya Windows ortamında .\target\debug\merhaba_cargo.exe
Hello, world!
```

Her şey yolunda giderse terminalinizde "Hello, world!" yazısı görünecektir. Cargo uygulamasının ilk kez çalıştırılması projenizin ana dizininde *Cargo.lock* adında yeni bir dosya oluşturulmasına neden olur. Bu dosya projenizdeki bağımlılıkların tam sürümlerini takip eder. Halihazırda bu proje harici bir kasaya bağımlı olmadığından bu dosya epey boş görünecektir. Bu dosya içeriği Cargo tarafından otomatik olarak yönetildiğinden bunu elle değiştirmeniz gerekmez.

Aslında *cargo build* komutu ile derleyip *./target/debug/merhaba_cargo* komutu ile çalıştırdığımız bu projeyi, `cargo run` komutu ile hem derleyip hem çalıştırabiliriz. Vereceğimiz tek bir `cargo run` komutu ile proje derlenecek ve ardından oluşturulan çalıştırılabilir dosya hemen işletilecektir.

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/merhaba_cargo`
Hello, world!
```

Bu kez Cargo'nun `merhaba_cargo` programını derlediğini bildiren çıktıyı göremediğimize dikkat edin. Bunun sebebi, Cargo'nun kaynak kodun değişmediğini bilmesidir. Eğer kaynak kodunu değiştirmiş olsaydınız program Cargo tarafından yeniden derlenerek çalıştırılacak ve terminalde aşağıdaki çıktı görünecekti:

```console
$ cargo run
   Compiling merhaba_cargo v0.1.0 (/home/dogan/projeler/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/merhaba_cargo`
Merhaba Cargo!
```

Yine Cargo tarafından sağlanan ve kodunuzun çalıştırılabilir olup olmadığını denetleyen, ancak çalıştırılabilir bir dosya oluşturmayan `cargo check` adında bir komut daha vardır:

```console
$ cargo check
    Checking merhaba_cargo v0.1.0 (/home/dogan/projeler/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
```
<!-- Kaldım-->
