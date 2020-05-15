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
    let haftanin_ilk_is_gunu = Gunler::Pazartesi;
    print!("Haftanın ilk iş günü {:?}'dir.", haftanin_ilk_is_gunu);
}
// Haftanın ilk iş günü Pazartesi'dir.
````

⭐️ Bir `enum` türünün varyantları:
  
  - Verisiz olarak yani *(Birim varyant)* şeklinde,
  - İsimsiz sıralı veriler halinde, yani çokuzlu varyantı şeklinde *(Çokuzlu varyant)* verilerden,
  - İsimlendirilmiş varyantlar, yani yapı alanlarına benzer biçimde *(Yapısal varyant)*

oluşabilir. 

```Rust
enum Denetim {
    Basarili,                               // Birim varyant
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
