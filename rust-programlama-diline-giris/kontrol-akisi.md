# Kontrol akışı
Bir koşulun doğru olup olmadığına bağlı olarak bazı kodların çalıştırılmasına karar vermek veya bir koşul doğru olduğu sürece bazı kodları tekrar tekrar çalıştırmak çoğu programlama dilinde temel yapı taşlarıdır. 

#### i. If - else if - else
```Rust
fn main() {
    let olcu = 7; 

    if olcu < 5 { 
        println!("Küçük boy"); 
    } else if olcu < 10 { 
        println!("Orta boy"); 
    } else { 
        println!("Büyük boy"); 
    }
}
````
If koşulu bir programın akışını kontrol etmeyi sağlayan en yaygın yapılardandır. Yukarıda en basit haliyle yer alan kod aşağıdaki gibi de yazılabilirdi.

```Rust
fn main() {
    let beden = 7; 
    let beden_aciklamasi; 

    if beden < 5 { 
        beden_aciklamasi = "Küçük boy"; 
    } else if beden < 10 { 
        beden_aciklamasi = "Orta boy"; 
    } else { 
        beden_aciklamasi = "Büyük boy"; 
    } 

    println!("Müşterinin tercihi: {}", beden_aciklamasi);
}
````
Bir ifade olarak kullanıldığında dönüş değerleri türünün her blokta `text, text``veya `bool, bool` gibi aynı türden olması beklenir. Yukarıdaki örnekler aşağıdaki gibi de optimize edilebilirlerdi.

```Rust
fn main() {
    let beden = 7;
    
    let beden_aciklamasi = if beden < 5 {
        "Küçük boy" // sonlandırma için ; gerekli değildir.
    } else if beden < 10 {
        "Orta boy"
    } else {
        "Büyük boy"
    };
    
    println!("Müşterinin tercihi: {}", beden_aciklamasi);

    let depo_kontrol = if beden < 15 { true } else { false };
    println!("Elimizde var mı?: {}", depo_kontrol);
}
````

#### ii. match
