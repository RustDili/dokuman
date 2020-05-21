## Özel hata türleri
Rust, kullanıcıların kendi türlerini oluşturmasına izin verir. Oluşturulan bu türler ise "Özel Hata Türleri" olarak tanımlanır.

## `Error` özelliği
Bilindiği gibi Rust'ta özellikler, **bir türün sağlaması gereken işlevler**i tanımlarlar. Ancak Rust'ın **standart kütüphanesi** kendi türlerimizle de kullanılabilecek yeniden kullanılmaya uygun bazı özellikleri varsayılan olarak sunduğundan ortak işlevler için çoğunlukla yeni özellikler tanımlamamız gerekmez. Özel hata türleri oluştururken faydalanacağımız [`std::error::Error trait`](https://doc.rust-lang.org/std/error/trait.Error.html) kütüphanesi herhangi bir `Err` türünü başka bir türe dönüştürmemize yardım eder.

```Rust
use std::fmt::{Debug, Display};

fn main() {
    pub trait Error: Debug + Display {
        fn kaynak(&self) -> Option<&(Error + 'static)> { ... }
    }
}
````

> [Özellikler ve kalıtım](https://github.com/rust-lang-tr/dokuman/blob/master/rust-programlama-diline-giris/ikinci-adim/impl-and-traits.md#%C3%B6zellikler-ve-kal%C4%B1t%C4%B1m) bölümünde tartıştığımız gibi, bir özellik başka özelliklerden miras yoluyla edinilebilir. `Trait Error: Debug + Display` ifadesi, bu özelliğin `fmt::Debug` ve `fmt::Display` özelliklerinden devralınacağı anlamını taşır.

```Rust
// Rust standart kütüphanesi core fmt modülü/ std::fmt içindeki özellikler
pub trait Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}

pub trait Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}
````

- `Display`
  - Bu hata mesajı son kullanıcılara/ nasıl kullanıcı odaklı bir çıktı olarak gösterilebilir?
  - Genellikle `println!("{}")` veya `eprintln!("{}")` yardımıyla yazdırılarak.

- `Debug`
  - hata ayıklama sürecinde `Err` mesajları nasıl kullanıcı odaklı birer çıktı olarak gösterilebilir?
  - Genellikle `println!("{:?}")` veya `eprintln!("{:?}")` yardımıyla yazdırılarak.
  - Ya da temiz çıktı `println!("{:#?}")` veya `eprintln!("{:#?}")` yardımıyla yazdırılarak.

- `source()`
  - Bu hatanın eğer varsa alt düzey kaynağı.
  - Optional.

Öncelikle, oldukça basit bir özel hata türüne `std::error::Error` özelliğinin nasıl uygulanacağını inceleyelim:

```Rust
use std::fmt;
// Özel hata türü; mevcut sandıkta tanımlanan herhangi bir tip olabilir
// 💡 Burada, örneği basitleştirmek için basit bir "birim yapı" kullanıyoruz

struct Hataci;

// AppError için std::fmt::Display özelliğini uygulama
impl fmt::Display for Hataci {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Kullanıcı odaklı çıktı
        write!(f, "Bir hata oluştu. Lütfen tekrar deneyin!") 
    }
}

// AppError için std::fmt::Debug özelliğini uygulama
impl fmt::Debug for Hataci {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Kullanıcı odaklı çıktı
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

// `Err` türünde bir AppError hatası üretmek için örnek işlev
fn hata_uret() -> Result<(), Hataci> {
    Err(Hataci)
}

fn main() {
    match hata_uret() {
        Err(e)  => eprintln!("{}", e), // Bir hata oluştu. Lütfen tekrar deneyin!
        _ => println!("Hatasız işlem"),
    }
    
    eprintln!("{:?}", hata_uret());
}

// ---------- Derleme zamanı hatası --------
// Bir hata oluştu. Lütfen tekrar deneyin!
// Err({ file: src/main.rs, line: 19 })
````

