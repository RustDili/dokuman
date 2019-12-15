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
