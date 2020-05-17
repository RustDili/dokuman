# Borçlanma
Gerçek hayattaki uygulamalarda çoğu zaman, bir değerle bağlanmış değişkenleri işlevlere iletmek veya bunları diğer değişkenlerin bağlanması amacıyla o değişkene atamak zorunda kalıyoruz. Bir değerin mülkiyetini üstlenmeden işlem yapmak gerektiğinde, o değerin sahibi olan orijinal değişken bağlamına bir başvuru yapılır.

## Borçlanma Nedir?
> [Borçlanma](https://github.com/nikomatsakis/rust-tutorials-keynote/blob/master/Ownership%20and%20Borrowing.pdf) Bu kavram dilimizde: *Bir şeyi iade etme garantisi verilerek ödünç almak* anlamına gelir.

## Paylaşılan ve Değişken borçlanmalar
⭐️ Rust'ta iki tür borçlanmadan bahsedebiliriz:

1. **Paylaşılan Borçlanma** `(&T)`

   * Bir veri parçası **içeriği değiştirilmemek kaydıyla, bir ya da çok kullanıcı tarafından ödünç** alınabilir. 
   
2. **Değişken Borçlanma** `(&mut T)`

   * Bir veri parçası, **tek bir kullanıcı tarafından ödünç alınabilir ve değiştirilebilir**, ancak bu durumda o verilere başkaları erişemez ve kullanamaz.
   
## Borçlanma Kuralları
Borçlanmanın oldukça önemli kuralları bulunmaktadır:

1. Bir veri parçası belli bir anda; ya paylaşılan ya da değişken borçlanma şeklinde ödünç alınabilir. Ancak **ikisi de aynı anda olamaz.**

2. Borçlanma **hem kopyalanan hem de taşınan türleri** kapsar.

2. **Canlılık** kavramı **↴**

```Rust
fn main() {
    let mut a = vec![1,2,3];
    let b = &mut a; // 'a' nın değişken borçlanması yani `&mut` burada başlar
                    // :
                    // :
    // Bazı kodlar  // :
    // Bazı kodlar  // :
}                   // &mut değişken borçlanma burada sona erer.
````

```Rust
fn main() {
    let mut a = vec![1,2,3];
    let b = &mut a;         // 'a' nın değişken borçlanması &mut burada başlar
    // Bazı kodlar          // :
    
    println!("{:?}", a);    // paylaşılan borç olarak verilen 'a'ya erişmeye
                            // çalışmak derleyicinin hata vermesine neden olur.
}                           // &mut değişken borçlanma burada sona erer.
````

```Rust
fn main() {
    let mut a = vec![1,2,3];
    
    {
        let b = &mut a;     // 'a' nın değişken borçlanması &mut burada başlar
        // Bazı kodlar      // :
    }                       // &mut değişken borçlanma burada sona erer.
    
    println!("{:?}", a);    // burada 'a' nın paylaşılan borçlanmasına izin verilir
}
````

Yukarıdaki bahsi geçen **paylaşılan ve değişken borçlanmaların** nasıl kullanılacağını aşağıdaki örneklerde inceleyelim.

### Paylaşılan Borçlanma Örnekleri



### Değişken Borçlanma Örnekleri
