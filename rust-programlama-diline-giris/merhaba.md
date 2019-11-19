### Merhaba Dünya!
Diğer dillerde olduğu gibi Rust dilinde de bir programın kodlamaya başlarken `main()` işlevinden başlanır. Bu  işlevin içindeki ifadeler programın derlenmiş hali çağrıldığı anda yürütülür.

```Rust
fn main() { 
  println!("Merhaba Dünya!"); 
}
````
Örneğimizde `println!;` "Merhaba Dünya!" mesajını ekrana bastıran bir makrodur.

Eğer bu kodu merhaba_dunya.rs olarak kaydettiyseniz terminalinizde rustc merhaba_dunya.rs komutu ile derleyebilir, ./merhaba_dunya komutu ile de çalıştırabilirsiniz.

### Rust Playground
Rust kodlarınızı internet üzerinde [çevrim içi](https://play.rust-lang.org/) çalıştırabileceğiniz bir alandır.

![Playground](https://github.com/rust-lang-tr/dokuman/blob/master/resimler/Rust-Playground.png)

### Println! Kullanımı
Bir makro olarak kullanılan println! için farklı kullanım örneklerine göz atalım.

```Rust
fn main() { 
  println!("{}, {}!", "Merhaba", "Dünya"); // Merhaba, Dünya! 

  println!("{0}, {1}!", "Merhaba", "Plüton"); // Merhaba, Plüton! 

  println!("{selamla}, {isim}!", selamla="Selam", isim="Erdem"); // Selam, Erdem!

  println!("{:?}", [1,2,3]); // [1, 2, 3] 

  println!("{:#?}", [1,2,3]); /* [ 1, 2, 3 ] */ // 

  let x = format!("{}, {}!", "Merhaba", "Mars"); 

  println!("{}", x); // Merhaba, Mars! 
}
````

### macro_rules!
Makrolar, isimlerinin bir ünlem `!` ile bitmesi dışında işlevlere benzerler. Ancak makrolar bir işlev çağrısı oluşturmak yerine, programın geri kalanıyla derlenen kaynak koduna genişletilir. C ve diğer dillerdeki makrolardan farklı olarak, Rust makroları dizgi ön işleme yerine soyut söz dizimi ağaçlarına genişletilir, bu nedenle beklenmedik hatalar ile karşılaşmazsınız.

Rust, meta-programlama yapabileceğimiz güçlü bir makro sistemi sağlar. Bir makro tanımlamak için macro_rules! Deyimini kullanmanız yeterlidir.
Aşağıdaki örnekte ‘merhaba_de’ adında basit bir makro tanımlanmaktadır.

```Rust
  macro_rules! merhaba_de {
   // ‘()’ Boş parantez makronun argüman almadığını gösterir.
  () => {
    // Makro bu bloğun içinde genişler.
    println!(“Merhaba!”);
  };
}

fn main() {
  // Alttaki çağrı println!(“Merhaba!”); ifadesini çalıştırır. 
  merhaba_de!()
}
````

- Kod tekrarını azaltırlar.
- Belirli bir amaç için özel söz dizimi tanımlanmasına izin verirler.
- Değişken arayüzlerin tasarlanmasına olanak sağlarlar.
