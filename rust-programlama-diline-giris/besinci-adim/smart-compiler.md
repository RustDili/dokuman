## Akıllı derleyici
Rust programlama dilinin yükünü zekice tasarlanmış derleyicisi çeker. Bu bölümde Rust derleyicisine odaklanacağız.

## Neden derleyici?
Rust programlarındaki hataları önlemek söz konusu olduğunda en önemli işleri Rust’ın derleyicisi yapar. Öyle ki, çalıştırılmaya hazır bir programı derleme aşamasında analiz ederken, kodun bellek yönetim kurallarına veya yaşam süresi ek açıklamalarına doğru biçimde uyup uymadığını denetleyerek, uyumsuzlukla karşılaştığında en ilgili uyarıyı yayınlar. Örneğin:

```Rust
#[allow(unused_variables)]
💡  // kullanılmayan değişken tespitinde 
    // hata uyarılarını bastıran öznitelik

fn main() {
    let a: Vec<u16> = vec![1,2,3];
    let b = a;
    
    print!("{:?} ", a);
}

// ---------- Derleme zamanı hatası --------
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:9:21
  |
6 |     let a: Vec<u16> = vec![1,2,3];
  |         - move occurs because `a` has type `std::vec::Vec<u16>`, which does not implement the `Copy` trait
7 |     let b = a;
  |             - value moved here
8 |     
9 |     print!("{:?} ", a);
  |                     ^ value borrowed here after move

error: aborting due to previous error
For more information about this error, try `rustc --explain E0382`
````
⭐ Yukarıdaki örnekte `#[allow(unused_variables)]` özniteliğini kullanarak `let b = a;` ifadesindeki hatayı bastırmıştık. Bunun yerine 
// `let _b = a;` şeklinde bir kullanım da mümkündür. Ayrıca döndürülecek tüm dönüş değerlerini yok saymak için `let _ =` şeklinde bir kullanım mümkündür.
