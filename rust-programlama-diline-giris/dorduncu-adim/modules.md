## Modüller
## 01. Tek bir dosyada
İlişkili kodlar ve veriler bir tek modülde gruplanır ve aynı dosya içinde saklanır:

```Rust
mod selamla {
    // ⭐️ Varsayılan olarak, bir modül içindeki her şey özeldir
    // Bu yüzden işlevin herkesin dışarıdan erişimine açık olması gerekir
    pub fn merhaba() {
        println!("Merhaba dünya!");
    }
}

fn main() {
    selamla::merhaba();
}
// Merhaba dünya!
````

Modüller iç içe geçmiş olarak bildirilebilirler:

```Rust
mod soylemler {
    pub mod selamlar {
        pub fn merhaba () {
            println!("Merhaba dünya!");
        }
    }
}

fn main() {
    soylemler::selamlar::merhaba();
}
// Merhaba dünya!
````

Özel işlevler aynı modül veya bir alt modülden çağırılabilirler. Aşağıdaki örnek aynı modülün özel işlevini çağırmaktadır:

```Rust
mod soylemler {
    pub fn selam() {
        merhaba(); // ya da `self::merhaba();`
    }
    fn merhaba() {
        println!("Merhaba dünya!");
    }
}

fn main() {
    soylemler::selam();
}

// Merhaba dünya!
````

Aşağıdaki örnekte ise üst modülün özel işlevi çağrılmaktadır:

```Rust
mod soylemler {
    fn ozel_bir_islev() {
        println!("Merhaba dünya!");
    }
    
    pub mod selamla {
        pub fn merhaba() {
            // açıklamasına aşağıdan ulaşabilirsiniz 
            super::ozel_bir_islev();
        }
    }
}

fn main() {
    soylemler::selamla::merhaba();
}
// Merhaba dünya!
````

💡 `self` anahtar sözcüğü aynı modülü belirtmek için kullanılırken, `super` anahtar sözcüğü üst modülü belirtmek için kullanılır. Ayrıca, `super` anahtar sözcüğü bir modülün içinden kök işlevlerine erişmek için kullanılabilir.

```Rust
mod selamlama {
    pub fn merhaba() {
        super::merhaba();
    }
}

fn merhaba() {
    println!("Merhaba dünya");
}

fn main() {
    selamlama::merhaba();
}
// Merhaba dünya!
````

🔎 Test kodlarını yazarken bütün test kodlarını ayrı bir test modülünün içine almanız, bu kodlar yalnızca test aşamasında derleneceklerinden iyi bir yaklaşım olur.

```Rust
// Test kodlarında dönüş türü kullanılmalıdır
// yani bir prosedür deüil bir işlev yazılmalıdır 
fn selam() -> String {
    "Merhaba dünya!".to_string()
}

#[cfg(test)] // sadece test aşamasında derlenecektir
mod tests {
    use super::selam; // Kök selam işlevini içe aktarma
    
    #[test]
    fn test_selam() {
        assert_eq!("Merhaba dünya!", selam());
    }
}

/*
running 1 test
test tests::test_selam ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
*/
````

## 02. Aynı dizin, ancak farklı bir dosyada
