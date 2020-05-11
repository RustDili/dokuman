# Kontrol akışı
Bir koşulun doğru olup olmadığına bağlı olarak bazı kodların çalıştırılmasına karar vermek ya da bir koşul doğru olduğu sürece bazı kodları tekrar tekrar çalıştırmak çoğu programlama dilinin olmazsa olmazıdır. 

## If - else if - else
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
If koşulu bir programın akışını kontrol etmeyi sağlayan en yaygın yapılardandır. Yukarıda en basit haliyle yer alan kod aşağıdaki gibi de yazılabilirdi:

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
Bir ifade olarak kullanıldığında dönüş değerlerinin her blokta `text, text` veya `bool, bool` gibi aynı türden olması beklenir. Yukarıdaki örnekler aşağıdaki gibi iyileştirilebilirler:

```Rust
fn main() {
    let beden = 7;
    
    let beden_aciklamasi = if beden < 5 {
        "Küçük boy" // ⭐️ sonlandırma için ; gerekli değildir.
    } else if beden < 10 {
        "Orta boy"
    } else {
        "Büyük boy"
    };
    
    println!("Müşterinin tercihi: {}", beden_aciklamasi);
}
````

İfade dönüş değerinin değişkende depolanması: 

```Rust
fn main() {
    let beden = 7;
    
    let depo_kontrol = if beden < 15 { true } else { false };
    println!("Elimizde var mı?: {}", depo_kontrol);
}
````

⭐️  If-else if-else akışında eğer dönüş türü bir ifadeyse, her kontrol bloğunda döndürülen deönüş türü aynı olmak zorundadır.

## match
Kontrol akışını desen eşleştirmesi yoluyla yapar.

```Rust
fn main() {
    let beden = 20;
    
    let beden_olcusu = match beden {
        10 => "S",                  // 10 Kontrol edilir
        17 | 18 => "M",             // 17 ve 18 kontrol edilir
        19..= 21 => "L",            // 19’dan 21’e (19,20,21) Kontrol edilir
        22 => "XL",
        _ => "Standart dışı ölçü",
    };
    println!("Müşterinin tercihi: {}", beden_olcusu);
}
````
Eşleştirmede kullanılan alt çizgi **`_`** bir değeri yok saymak için kullanılır. Her şeyle eşleşen ancak bir değişkene bağlanmayan bu operatör, desen ile eşleştirme yapılırken tıpkı switch düzeneğinde olduğu gibi bir çeşit varsayılan durum oluşturmak amacıyla son karşılaştırma ifadesinde kullanılır.

```Rust
fn main() {
    let paket_turu = false;
    
    let secim = match paket_turu {
        true => "Tam paket seçildi",
        false => "Kısıtlı pakette kalındı"
    };
    println!("Müşterinin tercihi: {}", secim);
}
````

Bu karşılaştırma işlemi boolean değer kontrolüne dayandığından, sadece olası iki durum karşılaştırılıyor. Bu nedenle **`_`** oparatörünün temsil ettiği varsayılan durum rahatlıkla atlanabilir.

```Rust
fn main() {
    let alinin_notu: u8 = 30;
    let mertin_notu: u8 = 25;
    
    let degerlendirme = match (alinin_notu, mertin_notu) {
        (50, 50) => "Her ikisi de başarılı",
        (50, _) => "Ali oldukça başarılı",
        (_, 50) => "Mert oldukça başarılı",
        (x, y) if x > 25 && y > 25 => "İkisi de fena değil",
        (_, _) => "Daha sıkı çalışmaları gerek!"
    };
    println!("Değerlendirme sonucu: {}", degerlendirme);
}
````
## while
Bildirilen koşul sağlandığı sürece döngünün devam etmesini sağlayan anahtar kelimedir. Döngüye girilmeden önce koşul çalıştırılır ve eğer koşul doğru olarak değerlendirilirse döngü içinde yer alan ifadeler işletilir, aksi durumda döngüden çıkılır.

```Rust
fn main() {
    let mut deger = 1;
    
    while deger <= 10 {
        println!("Anlık değer: {}", deger);
        deger += 1;
    }
}
````
Diğer programlama dillerinden alışkın olduğumuz **`++`** arttırma ve **`--`** eksiltme operatörleri Rust'ta bulunmaz. Yerine  arttırma ya da eksiltme işlemleri için bileşik atama oparatörleri kullanılır. 
Aşağıda ise `break` ve `continue` anahtar kelimelerinin kullanılışı örneklenmektedir.

```Rust
fn main() {
    let mut degisen = 0;
    
    while degisen < 8 {
        if degisen == 2 {
            println!("Bu değeri atla: {}", degisen);
            degisen += 1;
            continue;
        } else if degisen == 6 {
            println!("Bu değere ulaştığında döngüden çık: {}", degisen);
            break;
        }
        
        println!("Anlık değer: {}", degisen);
        degisen += 1;
    }
}
````
#### Outer break
```Rust
fn main() {
    let mut birinci = 1;
    
    'outer_while: while birinci < 6 {                               // outer_while etiketi ile ayarlanır
        let mut ikinci = 1;
        
        'inner_while: while ikinci < 6 {
            println!("Anlık değerler: [{}][{}]", birinci, ikinci);
            
            if birinci == 2 && ikinci == 2 { break 'outer_while; }  // outer_while sonlandırılır
                ikinci += 1;
        }
            birinci += 1;
    }
}
````
## loop
Rust'ta desteklenen en basit döngü türü olup `break` anahtar sözcüğü ile kesilmediği ya da programdan çıkılmadığı sürece sonsuza dek çalıştırılır.

```Rust
fn main() {
    let mut degisen = 0;
    
    loop {
        if degisen == 0 {
            println!("Atlanan değer. {}", degisen);
            degisen += 1;
            continue;
        } else if degisen == 4 {
            println!("Bu değere ulaşıldığında çıkılıyor: {}", degisen);
            break;
        }
        
        println!("Anlık değer: {}", degisen);
        degisen += 1;
    }
}
````

#### Outer break
```Rust
fn main() {
    let mut birinci = 1;
    'outer_loop: loop {
        
        let mut ikinci = 1;
        'inner_loop: loop {
            println!("Anlık değer: [{}][{}]", birinci, ikinci);
            
            if birinci == 2 && ikinci == 2 {
                break 'outer_loop;
            } else if ikinci == 5 {
                break;
            }
            ikinci += 1;
        }
        birinci += 1;
    }
}
````

## for
Bu anaktar sözcük **`for-in`** döngülerinde kullanılır ve **`in`** anahtar sözcüğü ile işaret edilen aralık tüketilene kadar yinelenir.
```Rust
fn main() {
    for don in 0..10 {
        println!("Anlık değer: {}", don);
    }
}
````
Örnekte yer alan `for sayac in 0..10` ifadesi, diğer programlama dillerinde kullanılan for `a = 0; a < 10; a++` ifadesine benzer. Döngü beklendiği gibi `0` ile başlar ve aralığın son değerine ulaştığında durur. Döngünün `break` ve `continue` anahtar sözcükleriyle yönlendirilmeleri aşağıda örneklenmiştir.

```Rust
fn main() {
    for don in 0..6 {
        if don == 2 {
            println!("Atlanan değer: {}", don);
            continue;
        } else if don == 4 {
            println!("Bu değere ulaşıldığında döngüden çıkılıyor: {}", don);
            break;
        }
        println!("Anlık değer: {}", don);
    }
}
````

Diziler ve vektörleri for döngüsü ile işlemek oldukça kolaydır.
```Rust
fn main() {
    let ekip: [&str; 4] = ["Zafer", "Meral", "Hüseyin", "Selda"];
    
    for e in 0..ekip.len() {
        println!("Ekip üye numarası: {:?}", [e]);
    }
    
    // Ekip üyelerini görelim
    for kisi in ekip.iter() {
        println!("Ekip üyesi: {:?}", kisi);
    }
}
````

#### Outer break
```Rust
fn main() {
    'outer_for: for birinci in 1..6 {
        'inner_for: for ikinci in 1..6 {
            println!("Anlık değer: [{}][{}]", birinci, ikinci);
            
            if birinci == 2 && ikinci == 2 { break 'outer_for; }
        }
    }
}
````
