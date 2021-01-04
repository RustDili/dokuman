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
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

<span class="caption"> Örnek 1-2: `cargo new` komutuyla oluşturulan Cargo.toml dosyası içeriği</span>

Bu dosya, Cargo'nun yapılandırma formatı olan [TOML](https://toml.io/en/)(Tom's Obvious, Minimal Language) biçimindedir.
<!-- Kaldım-->
