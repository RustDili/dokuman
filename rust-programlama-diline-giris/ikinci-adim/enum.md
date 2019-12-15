# Enum
Tek bir türden oluşan `enum`, o türün belirli bir zamanda olası değerlerini tutan değişkenlerden ibarettir.
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
Örneğimizdeki `Gunler` bir enumdur ve süslü parantezler içinde yer alan Pazartesi, Sali, Carsamba, Persembe, Cuma, Cumartesi, Pazar günleri ise **bu enum’un varyantlarıdır.**
Enum’ a ait varyantlara `Gunler::Pazartesi` gösterimiyle erişilir.  
```Rust
fn main() {
    let gun = Gunler::Sali;
    println!("Günlerden bir gun: {:?}", gun);  // Günlerden bir gun: Sali
}
````
Enum varyantları; verisiz, çokuzlularda olduğu gibi isimlendirilmemiş verilerle veya yapılarda olduğu gibi adlandırılmış verilerden oluşabilir. 
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
