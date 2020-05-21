## `Unwrap` ve `expect`

## `unwrap()` metodu
  - Bir `Option` türünün `Some` değeri varsa ya da bir `Result` türü `Ok` değeri içeriyorsa, içerdikleri bu değer bir sonraki adıma iletilir.
  - Bir `Option` türü None değerine ayarlanmışsa ya da bir `Result` türü `Err` değeri içeriyorsa panik üretilir. Eğer `Err` için bir hata mesajı tanımlanmışsa bu yazdırılır.
  
İşlevselliğiyse `match` yerine `unwrap()` metodu kullanan aşağıdaki kodlara benzer.

Öncelikle `unwrap()` metodu kullanmayan `Option` ve `match` örneğini inceleyelim:

```Rust
fn secimlik_bir_deger_ver() -> Option<&'static str> {
    //Seçimlik değer boş değilse
    if false {
        return Some("abc");
    }
    // diğer hallerde
    None
}

fn main() {
    let x;
    
    match secimlik_bir_deger_ver() {
        Some(deger) => x = deger,   // eğer Some("abc") ise, x’i "abc" olarak ayarla
        None        => panic!(),    // değer yoksa, hata mesajsız panik üret 
    }
    
    println!("Değer:{}", x);        
    // "abc" ; secimlik değerini `false` yerine `true` olarak değiştirin 
}
// ---------- Derleme zamanı hatası --------
// thread 'main' panicked at 'explicit panic', src/main.rs:15:24
````

Bir de `unwrap()` metodu kullanmayan `Result` ve `match` örneğini inceleyelim:

```Rust
fn secimlik_bir_deger_ver() -> Result<u64, String> {
    // Eğer hata oluşursa
    if true {
        return Err("Bir hata Mesajı".to_string());
    }
    // Oluşmazsa geçerli bir çıkış döndürelim
    Ok(255)
}

fn main() {
    let x;
    
    match secimlik_bir_deger_ver() {
        Ok(deger) => x = deger, // eğer Ok(255) ise x’i 255 olarak ayarla  
        Err(e)    => panic!(e), // Hata oluşmuşsa, panik mesajını yazdır 
    }
    
    println!("Değer:{}", x);        
    // 255 ; İşlevdeki `true` ifadesini `false` ile değiştirin
}
// ---------- Derleme zamanı hatası --------
// thread 'main' panicked at 'Bir hata Mesajı', src/main.rs:15:22
````
Yukarıdaki örneklerde bulunan `main()` işlevlerinde yer alan kodları `unwrap()` metodunundan yararlanarak iki satır ile yazabiliriz.

### 1. Öncelikle hatayı `Option` varyantı `None` için `unwrap()` metodu kullanarak yakalayalım
```Rust
fn secimlik_bir_deger_ver() -> Option<&'static str> {
    //Seçimlik değer boş değilse
    if false {
        return Some("abc");
    }
    // diğer hallerde
    None
}

fn main() {
    // Kodun nasıl kısaldığına dikkat edin!
    let x = secimlik_bir_deger_ver().unwrap();
    
    println!("Değer:{}", x);        
    // "abc" ; secimlik değerini `false` yerine `true` olarak değiştirin 
}
// ---------- Derleme zamanı hatası --------
// thread 'main' panicked at 'called `Option::unwrap()` 
// on a `None` value', src/main.rs:11:13
````

### 2. Aynı şekilde bu kez de `Result` varyantı `Err` için `unwrap()` metodunu kullanalım
```Rust
fn secimlik_bir_deger_ver() -> Result<u64, String> {
    // Eğer hata oluşursa
    if true {
        return Err("Bir hata Mesajı".to_string());
    }
    // Oluşmazsa geçerli bir çıkış döndürelim
    Ok(255)
}

fn main() {
    // Kodun nasıl kısaldığına dikkat edin!
    let x = secimlik_bir_deger_ver().unwrap();
    
    println!("Değer:{}", x);        
    // 255 ; İşlevdeki `true` ifadesini `false` ile değiştirin
}
// ---------- Derleme zamanı hatası --------
// thread 'main' panicked at 'called `Result::unwrap()` 
// on an `Err` value: "Bir hata Mesajı"', src/main.rs:13:13
````

⭐ Dikkat ederseniz, `unwrap()` metodunu kullandığınızda verilen hata mesajlarında paniğin gerçekleştiği satır numaraları tam olarak gösterilmemektedir.

## `expect()` metodu
