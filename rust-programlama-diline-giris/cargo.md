# Paket yönetimi, sandıklar ve proje yapısı
### Cargo
Rust dilinin yerleşik [paket yöneticisi ve yapılandırma sistemi cargo](https://doc.rust-lang.org/cargo/)'dur ve rustup aracılığıyla yüklenen bu aracın temel kullanımında aşağıdaki komutlar oldukça işlevseldir.

- Yeni bir proje oluşturmak için: cargo new
- Projeyi yapılandırmak için: cargo build
- Projeyi çalıştırmak için: cargo run
- Proje bağımlılıklarını güncellemek için: cargo update
- Projeyi test etmek için: cargo test
- Projenin dökümantasyonu için: cargo doc
- Projenin yapılandırılmasından önce oluşabilecek hataları gözlemleyebilmek için: cargo check

Bu araç yardımıyla yeni bir Rust projesi `cargo new proje_adı --bin` komutuyla başlatılır. `--bin` eki isteğe bağlı olarak kullanılır ve her iki durumda da oluşturulan proje çalıştırılabilir bir dosyadır. Ancak projenin `--lib` parametresi ile başlatılması projenin bir kütüphane olarak oluşturulması için yeterlidir. 

### Crate
Rust programlama dilinde [crates.io](https://crates.io) aracılığıyla paylaşılan her paket `Crate` adını alır. Crate' nin Türkçe karşılığı "tahtadan yapılmış kasa" anlamına geldiğinden, dilimizde "sandık" olarak karşılık bulacaktır.

Aksi belirtilmediği sürece yeni bir proje başlatıldığında, proje adıyla çalıştırılabilir dosya oluşturulur. Bu davranış `rustc --crate-type` veya `cargo --bin` parametresi kullanılarak değiştirilebilmektedir. 

`cargo new proje_adi --bin` ya da varsayılan haliyle `cargo new proje_adi` komutlarıyla başlatılan çalıştırılabilir paket yapılandırmasının proje dizini aşağıdaki gibi görünecektir.

```Rust
├── Cargo.toml
└── src
    └── main.rs
````

`cargo new proje_adi --lib` komutuyla oluşturulan kütüphane paketinin dizin yapısı ise aşağıdakine benzeyecektir.

```Rust
├── Cargo.toml
└── src
    └── lib.rs
````

Oluşturulan sandıklar İstenildiğinde, cargo.toml dosyasında yer alan bağımlılıklarıyla birlikte, Rust ekosisteminde üretilen sandıkların bir arada tutulduğu [crates.io](https://crates.io) adlı Rust resmi sitesi üzerinden diğer kullanıcılar ile paylaşılabilir.

### Proje yapısı
Oluşturulan her projenin

- Kaynak kodları `src` dizininde tutulur.
- Varsayılan kütüphane klasörü `src/lib.rs`' dir.
- Varsayılan çalıştırılabilir dosyası `src/main.rs`' dir.
- Diğer çalıştırılabilir dosyalar `src/bin/*rs` şeklinde konumlandırılır.
- Birim testler gibi entegrasyon testleri, test dizininde bulundurulur. 
- Proje içindeki örnekler ise `examples` dizinine konur.
- Karşılaştırmalar `benches` adlı dizininde yer alır.

Böyle bir projenin dizin yapısı ise aşağıdaki şekle benzeyecektir.

```Rust
.
├── Cargo.lock
├── Cargo.toml
├── benches
│   └── large-input.rs
├── examples
│   └── simple.rs
├── src
│   ├── bin
│   │   └── another_executable.rs
│   ├── lib.rs
│   └── main.rs
└── tests
    └── some-integration-tests.rs
````
