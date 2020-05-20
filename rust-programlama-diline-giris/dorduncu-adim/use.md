## `Use` anahtar sözcüğü
Şimdi `use` anahtar sözcüğünün temel kullanımlarını görelim.

## 01. Öğe ithalinin tam yolunu belirtmek
Temelde `use` anahtar sözcüğü, bir öğeye erişimin tam yolunu yeni bir isim ile ilişkilendirmek için kullanılır. Böylelikle kullanıcı o öğeye her erişiminde elemanın tam yolunu belirtmek zorunda kalmamış olur. İlk örneğimiz `use` anahtar sözcüğünü kullanmayan öğe erişimini gösterir:

```Rust
// `use` anahtar sözcüğü kullanılmadan
mod soylem { 
  pub mod selamla { 
    pub fn merhaba() { 
      println!("Merhaba dünya!");
    }
  }
}

fn main() { 
  soylem::selamla::merhaba(); // Öğye erişimde tam yol kullanımı
}
````

Aynı örneği `use` anahtar sözcüğü kullanarak tekrarladığımızda ithal edilen modül için takma ad oluşturmuş oluruz:

```Rust
// Modül için takma ad oluşturma
use soylem::selamla;
fn main() { 
  selamla::merhaba();
}
````

Aynı şekilde `use` anahtar sözcüğünü kullanarak modüle ait öğeleri de takma isim belirterek kullanıma sokabiliriz.

```Rust
// Modül öğeleri için takma ad oluşturma
use soylem::selamla::merhaba;
fn main() { 
  merhaba();
}
````

Yapılan isimlendirmeleri `as` anahtar kelimesi yoluyla özelleştirebiliriz.

```Rust
// `as` anahtar kelimesiyle isimleri özelleştirme
use soylem::selamla::merhaba as selam;
fn main() { 
  selam();
}
```

## 02. Öğeleri kapsama almak
