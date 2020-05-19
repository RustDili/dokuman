## Sandıklar
💭 Dilimize **sandık** olarak çevirebileceğimiz derlenebilen bu program parçaları diğer dillerdeki paket kavramıyla benzeşirler. Eğer bir sandık alt dosya modüllerine sahipse bu dosyalar sandık dosyasıyla birleştirilerek tek parça olarak derlenirler.

💭 Bir sandık çalıştırılabilir ikili dosya olarak veya bir kütüphane olarak üretilebilir. Sandığın çalıştırılabilir versiyonunun kök dizini, yani giriş noktası: `src/main.rs` iken, kütüphane hali için: `src/lib.rs` şeklinde dizinlenir. 

## 01. Çalıştırılabilir sandık olarak *lib.rs*
💡 Çalıştırılabilir ikili sandıklar yazılırken temel işlevleri `src/lib.rs` dosyasına taşıyabilir, taşınan bu temel işlevleri de programın giriş noktası olan `src/main.rs` üzerinden bir kütüphane olarak kullanabiliriz. Bu yaklaşım çalıştırılabilir sandıklar için oldukça yaygındır.

```Rust
// # terminal yardımıyla çalıştığınız dizinde bu komutları uyguladığımız düşünelim 
cargo new selamla
touch selamla/src/lib.rs

// # Oluşturulan klasörün içeriği şu şekilde olacaktır. 
selamla
 ├── Cargo.toml
 └── src
    ├── lib.rs
    └── main.rs

// # Aşağıdaki dosyaları şu şekilde değiştirdiğimizi düşünelim

// 01. selamla/src/lib.rs
pub fn merhaba() {
    println!("Merhaba mars!");
}

// 02. selamla/src/main.rs
extern crate selamla;

fn main() {
    selamla::merhaba();
}
// Merhaba mars!
````

> 💯 Daha önce de belirttiim gibi karmaşıklığı azaltabilmek için burada verdiğim örnekleri olabildiğince basit tutmaya çalışıyorum yararlanıyoruz. Ancak kodun test edilebilir olabilmesi için `selamla/src/lib.rs` dosyasındaki kodları şu şekilde yazmamamız gerekiyor:

```Rust
// selamla/src/lib.rs
pub fn merhaba() -> String {
  //! Burası `Merhaba dünya!` dizgesini döndürür 
  ("Merhaba dünya!").to_string()
}

// 01. `merhaba()` işlevi için test 
#[test]               // İşlevin bir test işlevi olduğunu belirten öznitelik
fn test_merhaba() {
  assert_eq!(merhaba(), "Merhaba dünya!");
}

// 02. `merhaba()`işlev testi için Idiomatic yol 
#[cfg(test)]          // Sadece test aşamasında derlenir
mod tests {           // Her bir test birimi için ayrı ayrı 
  use super::merhaba; // `merhaba()` işlevini kök olarak ithal eder 
  
    #[test]
    fn test_merhaba() {
        assert_eq!(merhaba(), "Merhaba dünya!");
    }
}
```
