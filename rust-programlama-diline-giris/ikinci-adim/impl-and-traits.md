# Uygulamalar ve Özellikler
💡 **C** benzeri yapılar bölümünde bu tür yapıların nesne yönelimli dillerdeki sınıflara benzediğini, ancak metotlarının olmadığından bahsetmiştik. Rust' ta yapı ve `enum` türlerinin metotlarını tanımlamak için `impl` yani uygulamalar kullanılır.

💡 Özellikler `trait` ise nesne yönelimli dillerde bulunan **arayüzler**e benzerler ve bir türün sağlaması gereken işlevleri tanımlamak için kullanılırlar. Tek bir tür için çok sayıda uygulama tanımlanabildiği gibi bu özellikler varsayılan metot uygulamalarını da içerebilir. 

⭐️️ Ancak bu türler uygulanırken varsayılan metotlar `override` ile bastırılarak geçersiz kılınabilir.

## Özellik kullanmayan bir uygulama örneği
💡 Kullanıcı tarafından Rust’ın `i8`, `f64` gibi hemen hemen her yerleşik türü için yeni özellikler tanımlanıp uygulanabilir. Benzer şekilde kullanıcı tarafından oluşturulan yeni türler için de yerleşik özellikler uygulanabilir. Ancak kullanıcılar yerleşik özellikleri zaten yerleşik durumdaki türlere yeniden uygulayamazlar:

```Rust
struct Oyuncu {
    ilk_adi : String,
    son_adi : String,
}

impl Oyuncu {
    fn tam_adi(&self) -> String {
        return format!("{} {}", self.ilk_adi, self.son_adi);
    }
}

fn main() {
    let bas_rol = Oyuncu {ilk_adi: String::from("Reha"),
                            son_adi: "Özcan".to_string() };
    
    println!("Baş rol oyuncusu: {}", bas_rol.tam_adi());
    // Baş rol oyuncusu: Reha Özcan
}
````

⭐️ Her uygulamanın kendi türü ile aynı sandığın içinde yer almasına dikkat edin.
