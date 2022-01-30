## Yöntem Söz Dizimi

*Yöntemler (methods)*, fonksiyonlara benzerdir: `fn` anahtar sözcüğü ve  fonksiyon adıyla bildirilirler,
parametrelere, bir dönüş değerine sahip olabilirler
ve başka bir yerden çağrıldıklarında çalışan bazı kodlar içerirler.
Bununla beraber yöntemler (methods), bir yapı (numaralandırma veya bir özellik nesnesi
-sırasıyla Bölüm 6 ve 17'de ele aldığımız-)
bağlamında tanımlanmaları bakımından fonksiyonlardan farklıdır
ve ilk parametreleri her zaman `self`'dir (yöntemin bağlı olduğu yapı).

## Yöntem (method) tanımlama

Parametre olarak `dikdortgen`  kabul eden `alan` fonksiyonunu değiştirelim
ve bunun yerine `dikdortgen` yapısı bağlamında tanımlanmış bir `alan` üye fonksiyonu  tanımlayalım (Bölüm 5-13'te gösterildiği gibi).

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Bölüm  5-13: `dikdortgen` yapısı bağlamında `alan` fonksiyonu tanımlama</span>

Fonksiyonu dikdortgen bağlamında tanımlamak için bir `impl` bloğu başlatırız.
Bu `impl` bloğu içindeki her şey `dikdortgen` türüyle ilişkilendirilecektir.
Sonra `alan` fonksiyonunu `impl` bloğu içinde yürütürüz. ve
ilk parametreyi (bu durumda  yalnızca bir tane) `self` olarak fonksiyon imzasında ve gövdede değiştiririz.
`alan` fonksiyonunu çağırdığımız ve parametre olarak `dikdortgen` ilettiğimiz `main`de, bunun yerine `dikdortgen` örneğinde `alan` fonksiyonunu kullanmak için yöntem (method) sözdizimini kullanabiliriz.
Yöntem sözdizimi bir örneğin ardından gelir: yöntem adından sonra bir nokta (.)
parantezler ve eğer varsa parametreler.

`alan` imzasında `dikdortgen: dikdortgen` yerine `&self` kullanıyoruz.
`&self` aslında `self: &Self`in kısaltmasıdır. Bir `impl`
bloğu içinde, `Self` tipi, `impl` bloğunun bağlı olduğu tip için bir kısaltmadır.
Yöntemlerin ilk parametreleri için `Self` türünde `self` adlı bir parametresi olmalıdır,
bu nedenle Rust bunu ilk parametre noktasında yalnızca `self` adıyla kısaltmanıza izin verir.
Bu yöntemin, tıpkı `dikdortgen: &dikdortgen`de yaptığımız gibi,
`Self` örneğini ödünç aldığını belirtmek için `self` kısaltmasının önünde `&`
kullanmamız gerektiğini unutmayın.
Yöntemler, burada yaptığımız gibi `self`in sahipliğini alabilir, değişmez bir şekilde
`self`i ödünç alabilir veya diğer herhangi bir parametrede olduğu gibi `self`i
değişken bir şekilde ödünç alabilir.

Fonksiyon  versiyonunda `&Dikdortgen`i seçmemizle aynı nedenden dolayı burada `&self`i seçtik: Sahiplik almak istemiyoruz 
ve sadece yapıdaki verileri okumak istiyoruz, yapıya yazmak  değil.
Yöntemin yaptıklarının bir parçası olarak, yöntemi çağırdığımız örneği değiştirmek isteseydik, ilk parametre olarak
`&mut self` kullanırdık.
İlk parametre olarak sadece `self`i kullanarak örneğin sahipliğini alan bir yönteme sahip olmak nadirdir;
bu teknik genellikle, yöntem `self`i başka bir şeye dönüştürdüğünde ve arayan kişinin dönüşümden sonra orijinal örneği kullanmasını engellemek istediğinizde kullanılır.

Yöntem (method) sözdizimini kullanmanın ve her yöntemin imzasında `self`
türünü tekrarlamak zorunda kalmamanın yanı sıra, fonksiyonlar yerine yöntemleri kullanmanın temel faydası
kodun organizasyonu içindir. Kodumuzun gelecekteki kullanıcılarını; sağladığımız kütüphanede
(library) çeşitli yerlerinde `dikdortgen` fonksiyonlarını veya diğer şeyleri aramaları yerine,
bir türün örneğiyle yapabileceğimiz her şeyi tek bir `impl` bloğuna koyduk.

Bir yönteme (method) yapının alanlarından (field) biriyle aynı adı vermeyi seçebileceğimizi unutmayın.
Örneğin, `Dikdortgen`e bağlı  `width` olarak adlandırılan bir yöntem tanımlayabiliriz:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

Örneğin `width` alanındaki değer 0'dan büyükse doğru (true), değer 0 ise yanlış (false)
döndürmesi için `width` yöntemini  seçiyoruz:
aynı adı taşıyan bir yöntem (method) içindeki bir alanı herhangi bir amaç
için kullanabiliriz. `Main`de, `dik1.width`i parantez içinde yazdığımızda Rust, `width` yöntemini
kastettiğimizi biliyor. Parantez kullanmadığımızda Rust, `width` alanını kastettiğimizi bilir.

Her zaman olmasa da çoğu zaman, alanla (field) aynı ada sahip yöntemler, 
yalnızca alandaki değeri döndürecek ve başka hiçbir şey yapmayacak şekilde tanımlanır.
Bunun gibi yöntemlere *getiriciler* (getters) denir ve Rust,
diğer bazı dillerin yaptığı gibi, bunları yapı (struct) alanları için otomatik olarak uygulamaz.
Getiriciler, özel alanları  (field) genel (public) hale getirebildiğiniz ve böylece türün genel API'sinin bir parçası olarak bu alana salt okunur erişimi etkinleştirebildiğiniz
için kullanışlıdır. Public (genel) ve private'ın (erişime kapalı) ne olduğunu
ve bir alan veya yöntemin nasıl public veya private olarak tanımlanacağını Bölüm 7'de tartışacağız.

> ### `->` Operatörü yok mu?
>
> C ve C++'da, yöntemleri çağırmak için iki farklı operatör kullanılır: doğrudan nesne üzerinde 
> bir yöntemi çağırıyorsanız `.` kullanırsınız ve yöntemi nesneye işaret eden bir işaretçiden (pointer) çağırıyorsanız `->` kullanırsınız
> ve önce işaretçinin referansını kaldırmanız gerekir.
> Başka bir deyişle, `nesne` bir işaretçiyse, `nesne->bir_şey()`,  `(*nesne).bir_şey()` ile eşdeğerdir.
> Rust'ın `->` operatörüyle eşdeğeri yoktur; bunun yerine, Rust'ın *otomatik referans verme ve referans kaldırma*
> adlı bir özelliği vardır. Yöntem çağrıları, Rust'ta 
> bu davranışa sahip birkaç yerden biridir.
> Şu şekilde çalışır: `object.bir_şey()` ile bir yöntemi çağırdığınızda, Rust otomatik olarak 
> `&`, `&mut` veya `*` ekler, bu nedenle `nesne` yöntemin imzasıyla eşleşir. Başka bir deyişle, aşağıdakiler aynıdır:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Nokta {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Nokta {
> #    fn mesafe(&self, diger: &Nokta) -> f64 {
> #        let x_squared = f64::powi(diger.x - self.x, 2);
> #        let y_squared = f64::powi(diger.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Nokta{ x: 0.0, y: 0.0 };
> # let p2 = Nokta{ x: 5.0, y: 6.5 };
> p1.mesafe(&p2);
> (&p1).mesafe(&p2);
> ```
>
> The first one looks much cleaner. This automatic referencing behavior works
> because methods have a clear receiver—the type of `self`. Given the receiver
> and name of a method, Rust can figure out definitively whether the method is
> reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact
> that Rust makes borrowing implicit for method receivers is a big part of
> making ownership ergonomic in practice.

###  Daha Fazla Parametreli Yöntemler

Let’s practice using methods by implementing a second method on the `Rectangle`
struct. This time, we want an instance of `Rectangle` to take another instance
of `Rectangle` and return `true` if the second `Rectangle` can fit completely
within `self`; otherwise it should return `false`. That is, we want to be able
to write the program shown in Listing 5-14, once we’ve defined the `can_hold`
method.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Listing 5-14: Using the as-yet-unwritten `can_hold`
method</span>

And the expected output would look like the following, because both dimensions
of `rect2` are smaller than the dimensions of `rect1` but `rect3` is wider than
`rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

We know we want to define a method, so it will be within the `impl Rectangle`
block. The method name will be `can_hold`, and it will take an immutable borrow
of another `Rectangle` as a parameter. We can tell what the type of the
parameter will be by looking at the code that calls the method:
`rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to
`rect2`, an instance of `Rectangle`. This makes sense because we only need to
read `rect2` (rather than write, which would mean we’d need a mutable borrow),
and we want `main` to retain ownership of `rect2` so we can use it again after
calling the `can_hold` method. The return value of `can_hold` will be a
Boolean, and the implementation will check whether the width and height of
`self` are both greater than the width and height of the other `Rectangle`,
respectively. Let’s add the new `can_hold` method to the `impl` block from
Listing 5-13, shown in Listing 5-15.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Listing 5-15: Implementing the `can_hold` method on
`Rectangle` that takes another `Rectangle` instance as a parameter</span>

When we run this code with the `main` function in Listing 5-14, we’ll get our
desired output. Methods can take multiple parameters that we add to the
signature after the `self` parameter, and those parameters work just like
parameters in functions.

### İlişkili Fonksiyonlar

Bir `impl` bloğu içinde tanımlanan tüm işlevler, `impl`'den sonra bildirilen türle ilişkili oldukları için
*ilişkili fonksiyonlar* olarak adlandırılır. Çalışmak için bir örnek tipine ihtiyaç duymadıklarından,
ilk parametresi `self` olmayan (ve dolayısıyla yöntem olmayan)
ilişkili işlevleri tanımlayabiliriz. Bunun gibi bir işlevi
zaten kullandık, `String` tipinde tanımlanan `String::from` fonksiyonu.

Yöntem olmayan ilişkili fonksiyonlar, genellikle yapının yeni bir örneğini döndürecek yapıcılar için kullanılır.
Örneğin, bir `boyut` parametresine sahip olacak ve bunu hem genişlik hem de yükseklik olarak kullanacak
ilişkili bir fonksiyon sağlayabiliriz, böylece aynı değeri iki kez belirtmek
zorunda kalmak yerine `Rectangle` oluşturmayı kolaylaştırabiliriz:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

Bu ilişkili fonksiyonu çağırmak için, yapı adıyla `::` sözdizimini kullanırız;
örnek: `let sq = Rectangle::square(3);`.
Bu fonksiyon, yapının ad alanındadır: `::` sözdizimi hem ilişkili fonksiyonlar
hem de modüller tarafından oluşturulan ad alanları için kullanılır.
Modülleri Bölüm 7'de inceleyeceğiz.

### Çoklu `impl` Blokları

Her yapının birden çok `impl` bloğuna sahip olmasına izin verilir.
Örneğin; bölüm 5-15, bölüm 5-16'da gösterilen ve her yöntemin kendi
`impl` bloğunda yer aldığı koda eşdeğerdir.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Listing 5-16: Rewriting Listing 5-15 using multiple `impl`
blocks</span>

There’s no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax. We’ll see a case in which multiple `impl` blocks are
useful in Chapter 10, where we discuss generic types and traits.

## Summary

Structs let you create custom types that are meaningful for your domain. By
using structs, you can keep associated pieces of data connected to each other
and name each piece to make your code clear. In `impl` blocks, you can define
functions that are associated with your type, and methods are a kind of
associated function that let you specify the behavior that instances of your
structs have.

But structs aren’t the only way you can create custom types: let’s turn to
Rust’s enum feature to add another tool to your toolbox.
