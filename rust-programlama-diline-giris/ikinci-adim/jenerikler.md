# Jenerikler

Bir veri türü oluşturulurken yahut bir işlev tanımlanırken bunların farklı türde argümanlarla da çalışması istenir. Rust' ta **jenerikler**, veri türlerini tek noktada toplayarak kodun başka türler için tekrar yazılmasını önlerler. Farklı veri türleri için  genelleştirilmiş olan algoritma, her veri türü için yeniden üretilmemiş olacağından, kod tasarımı sadeleşmiş geliştirme hızı da arttırılmış olur. 

Genelleştirme kavramında özel bir veri türü bildirmek yerine türün yerine geçebilen `(x: u8)` yerine `(x: T )` gibi genel bir belirteç kullanılır. Ancak bu yapılmadan önce bu genel türün derleyici tarafından anlaşılabilmesi için `<T>` şeklinde tanımlanarak bildirilmesi gereklidir.

### Jenerik işlevler
Aynı işlevin farklı türlerle kullanılabiliyor olması kodun gereksizce uzamasını önleyerek daha esnek olmasını sağlar.
```Rust
fn her_ture_uygun<T>(x: T) { 
    // x T türünde bir parametredir. T ise jenerik türdür yani farklı türleri için genelleştirilmiştir.  
} 

fn ayni_turden_iki_tane<T>(x: T, y: T) { 
    // Her ikisi de aynı türden parametre bekler 
} 

fn farkli_turden_iki_tane<T, U>(x: T, y: U) {  
    // Farklı türde parametreler.
}
````
Bir verinin hangi tür olduğunu öğrenebilmek için `std::any` kütüphanesinden yararlanabiliriz.
```Rust
fn her_ture_uygun<T>(_: T) { 
    // x T türündedir. T ise jenerik türdür yani farklı türleri için genelleştirilmiştir.  
    println!("Bu veri {} türündedir", std::any::type_name::<T>());
} 

fn main() {
    
    let bir_tur = 6;
    //let bir_tur = 65u8;
    // let bir_tur = String::from("Merhaba");
    her_ture_uygun(bir_tur);
}
````

### Jenerik yapılar
