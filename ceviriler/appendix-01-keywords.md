## Ek A: Anahtar Kelimeler
Aşağıdaki listede, Rust dili tarafından geçerli veya gelecekteki kullanım için ayrılmış anahtar kelimeler bulunmaktadır. Bu yüzden "[Ham Tanımlayıcılar][ham-tanımlayıcılar]<!-- ignore -->" bölümünde tartışacağımız ham tanımlayıcılar hariç olmak kaydıyla) listede yer alan kelimeler; işlev isimlerini, değişkenleri, parametreleri, yapı alanlarını, modülleri, sandıkları, sabitleri, makroları, statik değerleri, nitelikleri, türleri, özellikleri veya yaşam sürelerini  tanımlamak için kullanılamazlar.

[ham-tanımlayıcılar]: #ham-tanımlayıcılar

### Halihazırda kullanılmakta olan anahtar kelimeler
* Aşağıdaki anahtar kelimeler, açıklamalarında bildirilen işleri gerçekleştirmekte kullanılırlar.
  * `as` - ilkel tür dönüşümlerini gerçekleştirme, bir öğe içeren belirli özelliği belirsizleştirme veya `use` ve `extern crate` ifadelerindeki öğeleri yeniden adlandırırken.
  * `async` - eş zamansız işlemlerde geçerli iş parçasını engellemeden çalışmak gerektiğinde
  * `await` - bir `Future`' ın sonucu hazır olana kadar yürütmeyi askıya almak gerektiğinde
  * `break` - bir döngüden hemen çıkmak gerektiğinde
  * `const` - sabit öğeleri veya sabit ham işaretçileri bildirirken
  * `continue`- bir sonraki döngü yinelemesine geçilmek istenirken
  * `crate` - harici bir sandığı veya makro içindeki bir sandığı temsil etmekte
  * `dyn` - bir özellik nesnesine dinamik başvuru yaparken
  * `else` - kontrol akışı yapıları olan `if` ve `if let` için olası bir durumu temsil ederken
  * `enum` - numaralandırılmış çoklu seçim yani cebirsel veri türü tanımlamakta
  * `extern` - harici bir sandık, değişken veya işlev bağlarken
  * `false` - mantıksal yanlış değerini ifade eden Boolean değişmezi gerektiğinde
  * `fn` - bir işlev veya işlev işaretçisini tanımlarken
  * `for` - bir yineleyici üzerindeki öğeler arasında geçiş yapmak, bir özelliği uygulamak veya daha yüksek seviyede yaşam süresi bildirmekte
  * `if` - bir koşullu ifadenin sonucuna göre yönlenirken
  * `impl` - doğal veya sürekli işlevsellik uygulama gerektiğinde
  * `in` - `for` döngüsü söz dizimini kullanırken
  * `let` - bir değişkeni bağlarken
  * `loop` - bir koşulsuz döngü gerektiğinde
  * `match` - herhangi bir değeri en uygun kalıpla eşleştirirken
  * `mod` - bir modül tanımlarken ya da bildirirken
  * `move` - Kapama işlevlerindeki tüm parametrelerin mülkiyetini almakta
  * `mut` - referans, ham işaretçiler veya değişken bağlamalarında değişkenlik gerektiğinde 
  * `pub` - yapı alanları, impl blokları veya modüllerde genel görünürlük gerektiğinde
  * `ref` - referans ile bağlamak gerektiğinde
  * `return` - bir işlevden dönülmesi gerektiğinde
  * `Self` - tanımlanan veya uygulanan bir türün kendisine başvurulurken (başvuru esnasında kullanılan takma ad)
  * `self` - halihazırdaki modül veya bir metodun alıcısına başvurulurken
  * `static` - programın sonlanana kadar veya yaşam süresi boyunca geçerli bir genel değişken gerektiğinde 
  * `struct` - bir yapıyı tanımlarken
  * `super` - geçerli modülün üst öğesine başvurulurken
  * `trait` - türe ait bir özellik tanımlanırken
  * `true` - mantıksal doğru değerini ifade eden Boolean değişmezi gerektiğinde
  * `type` - mevcut bir tür için lakap tanımlanırken
  * `union` - yalnızca [Birlik](https://doc.rust-lang.org/reference/items/unions.html) tanımı esnasında anahtar kelime olarak
  * `unsafe` - bellek güvenliği tür sistemi tarafından doğrulanamayan kod veya arabirimler: işlev, özellik, uygulama vb. tanımlanırken
  * `use` - işaret ettiğini kapsama dahil ederken
  * `where` - bir türün kısıtlama adımlarını bildirimekte
  * `while` - bir koşullu ifade sonucuna bağlı olan döngü gerektiğinde

### İleride olası bir kullanım amacıyla Rust için ayrılmış anahtar kelimeler
* Aşağıdaki anahtar kelimeler Rust tarafından gelecekteki olası bir kullanım için ayrılmış olup, halihazırda herhangi bir işleve sahip değildirler.
  * `abstract`
  * `become`
  * `box`
  * `do`
  * `final`
  * `macro`
  * `override`
  * `priv`
  * `try`
  * `typeof`
  * `unsized`
  * `virtual`
  * `yield`
  
### Ham tanımlayıcılar
Ham tanımlayıcılar normalde kullanılmasına izin verilmeyen anahtar kelimelerin kullanılmasını sağlayan söz dizimleridir. Bir anahtar kelimenin önüne `r#` ön eki getirilerek kullanılır.
  
Örneğin `match` örüntü eşlemede kullanılan bir anahtar kelimedir. Bu kelimeyi isim olarak kullanmaya çalışan bir işlevi: 

Dosya adı: src/main.rs
```Rust
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
````
derlemeye çalışırsanız, aşağıdaki hatayı alırsınız:

```Binary
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
````
Bu hata `match` anahtar kelimesinin tanımlayıcı olarak kullanılamayacağını gösterir. Bu anahtar kelimeyi işlev adı olarak kullanılabilmeniz için, ham tanımlayıcı söz diziminden yararlanmanız gerekir:

Dosya adı:
```Rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
   assert!(r#match("foo", "foobar"));
}
````

Bu kod hatasız derlenecektir. Ek olarak, işlev adındaki bitişik `r#` ön ekinin yanı sıra, `main()` işlevi içinden yapılan çağrıda da `r#` nin kullanılmış olduğuna dikkat edin.

Ham tanımlayıcılar, istediğiniz bir anahtar kelimenin Rust'ın kullanımı için ayrılmış olmasına rağmen yeniden kullanılmasına olanak sağlarlar. Ek olarak bu tanımlayıcılar, sandığınızın kullandığından farklı bir Rust sürümünde yazılmış kütüphanelerin de kullanılmasına izin verir. Örneğin, 2015 sürümünde anahtar kelime olmayan, ancak 2018 sürümünde anahtar kelime olarak ayrılmış bir kelimenin işlev adı olarak kullanıldığı bir program kullanmanız gerektiğini düşünün. Bu programı Rust'ın güncel bir sürümüyle kulanabilmeniz için ham tanımlayıcı söz dizimini kullanmanız gerekir. Sürümler hakkında daha fazla bilgi için [Ek E][appendix-e]'ye göz atınız.

[appendix-e]: appendix-05-editions.html 
