# Enum
⭐️ Farklı varyantların nümeratik sırayla bir araya getirildiği ve olası değerlerlerinin tuttulduğu bir türdür:

```Rust
#[derive(Debug)]
enum Gunler {
    Pazartesi,
    Sali,
    Carsamba,
    Persembe,
    Cuma,
    Cumartesi,
    Pazar,
}
````

Örneğimizdeki `Gunler` bir `enum`dur ve süslü parantezler içinde yer alan Pazartesi, Sali, Carsamba, Persembe, Cuma, Cumartesi, Pazar günleri ise **bu enum’un varyantlarıdır.**

⭐️ Bir `enum`’un sahip olduğu varyantlara **`::`** gösterimi, yani `Gunler::Pazartesi` şeklindeki söz dizimiyle erişilir:  

```Rust
fn main() {
    let gun = Gunler::Sali;
    println!("Günlerden bir gun: {:?}", gun); 
}
// Günlerden bir gun: Sali
````

⭐️ Bir `enum` türünün varyantları:
  
  - Verisiz olarak yani birim varyant,
  - İsimsiz sıralı veriler halinde, yani çokuzlu varyantı şeklinde,
  - İsimlendirilmiş varyantlar, yani yapı alanlarına benzer biçimde

verilerden oluşabilir. 

```Rust
enum Denetim {
    Basarili,                               // Birim vryant
    Dikkat{kategori: i32, mesaj: String},   // Yapısal varyant
    Hata(String)                            // Çokuzlu varyant
}

fn denetim_mesaji_yazdir(dm: Denetim) {
    
    match dm {
        Denetim::Basarili => println!("Form sorunsuzca gönderildi"),
        Denetim::Dikkat{kategori, mesaj} => println!("Dikkat: {} - {}", kategori, mesaj),
        Denetim::Hata(msj) => println!("Hata oluştu: {}", msj)
    }
}
fn main() {
    let mut form_durumu = Denetim::Basarili;
    denetim_mesaji_yazdir(form_durumu);
    
    form_durumu = Denetim::Dikkat {kategori: 2, mesaj: String::from("O alan doldurulacak!")};
    denetim_mesaji_yazdir(form_durumu);
    
    form_durumu = Denetim::Hata(String::from("Bağlantı hatası!"));
    denetim_mesaji_yazdir(form_durumu);
}
````
