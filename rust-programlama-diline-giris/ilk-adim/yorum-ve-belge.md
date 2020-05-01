# Yorum satırları ve kodun belgelenmesi
### Yorum satırları
Yorum satırları programların olmazsa olmazlarıdır. Rust birkaç farklı yorum satırını destekler.

```Rust
// Bu bir satır içi yorumdur
/* Bu ise sonlandırma imi bildirilene kadar tüm bloğu kapsar */
/* Rust /* iç içe */ yapılan blok yorumlarını da destekler.
````

💡 Her ne kadar blok yorumlarını destekleniyorsa da, mümkün olduğunca satır içi yorum kullanılması tavsiye edilmektedir.

### Belgelendirilen yorum satırları
Döküman yorumları `markdown` gösterimlerini destekler. Bu yorum satırlarına sahip programlar `cargo doc` komutu kullanılarak `HTML` biçiminde belgelendirilir. 

```Rust
/// Satır içi yorumlar: Bir alttaki öğeyi belgeler
/** Blok Yorumu: Bir alttaki öğeyi belgeler */
//! Satır içi yorumlar: Alt parçadaki öğeyi kapsam boyunca belgeler
/*! Blok Yorumu: Alt parçadaki öğeyi kapsam boyunca belgeler !*/
````

Aşağıdaki örneklerde görüleceği gibi her iki yorum seti de aynı modülü belgelemek için kullanılıyor. 

```Rust
/// Modül testlerini içerir 
mod test { 
  // ... 
} 
````
Dikkat ederseniz üstteki yorum seti modülün önüne eklenirken, alttaki yorum setinin modül içinde bulunmaktadır.

```Rust
mod test { 
  //! Burası da Modül testlerini içerir 
  // ... 
}
````

Sandık ve modül düzeyinde belgeleme yapmak için sadece `//!` kullanmak yeterlidir. Eğer ilk örnekteki gibi mod blokları belgelenecekse   `/// yorum satırı` mod bloğunun dışında kullanılır.

### Belge nitelikleri
Kodların belgelenmesinde döküman niteliklerinden de yararlanılır. Aşağıdaki örnekte bulunan her yorum ilgili verinin niteliklerine eşdeğerdir.

```Rust
/// Foo 
#[doc="Foo"] 

//! Foo 
#![doc="Foo"]
````

Daha fazla bilgi için [Rust belgelerini](https://doc.rust-lang.org/1.30.0/book/first-edition/documentation.html) inceleyebilirsiniz.
