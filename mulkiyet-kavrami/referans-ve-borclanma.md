### Referans ve Borçlanma
Hafızanın heap bölümünde depolanan değişken kaynaklarının, atama yoluyla başka bir değişkenin mülkiyetine aktarılması neticesinde önceki değişkene artık erişilemenesi olumsuz bir özellikmiş gibi görünüyor olabilir. Daha da sıkıcı olan şey ise işlevlere parametre yoluyla geçirilen değişkenlerin de programın ilerleyen bölümlerinde bir daha kullanılamıyor olmasıdır. Bir değişkenin mülkiyetine dokunmadan sadece verisine erişmek istendiğinde Rust’ ın borçlanma mekanizmasından yararlanılır.

Değişkenin referansı; o değişkenin sahip olduğu verinin kendisi yerine bellekteki adresini gösterdiğinden, nesnenin kendisi yerine referansının kullanılması **borçlanma** olarak adlandırılır.
Bir nesnenin referansına başvurulduğu sürece o nesnenin yaşamı sona ermez. Rust programları derlenmeden hemen önce **Borrow Checker** adlı dahili bir mekanizma tarafından denetlenerek nesne referanslarının geçerliliğine bakılır. Borç kontrolcüsü tarafından geçerli referansa sahip olmayan ve yaşam alanı sonlanmış bir nesne tespit edildiğinde o nesne düşürülür.

```rust
fn main() {
    // Box içinde bir integer oluşturuyoruz.g
    let boxed_int = Box::new(5);
    let ref_int = &boxed_int; // '&' ile referans oluşturuyoruz.

    println!("değişken: {}, referansı: {}", boxed_int, ref_int);
}
```

Bir değişkenin başka bir değişkene atanması ya da parametre yoluyla bir işleve geçirilmesi sırasında kullanılan **&** sembolü, o değişkenin kendisinin kullanılması yerine referansının değerlendirilmesine neden olacağından yukarıdaki örnekte yer alan `boxed_int` ve `ref_int` değişkenlerinin her ikisi de kullanılabilir durumdadırlar. 
Aşağıdaki örnekte bir vektörün referans yoluyla işlev parametresine geçirilmesi incelenmektedir.

```rust
fn foo(v1: &Vec<i32>) {                       // <--- '&Vec' Parametre bir referanstır
    println!("referans vektör: {:?}", v1);
}

fn main() {
    let v1 = vec![1, 2, 3];                   // Vektörlerin ham verileri hesap üzerinde depolanır.
    foo(&v1); // foo işlevine referans olarak v1'i gönderildi.
    println!("vektörün kendisi: {:?}", v1);   // Vektör halen kullanılabilir haldedir.
}
```

Bir kaynağın referansından önce silinmesi, referansın artık null bir alanı göstermesine neden olacağından, bu durumun yaşanmasına Rust’ın statik çalışan **Borrow Checker** mekanizması tarafından izin verilmez.

```rust
fn foo(v1: Vec<i32>) {    // <- İşlev normal bir vektör değeri beklediğinden
    println!("fonksiyondaki vektör: {:?}", v1);
}

fn main() {
    let v1 = vec![1, 2, 3];
    let referans_vec = &v1;

    foo(v1);              // <-'ödünç alınmış bir değişken taşınamaz' hatası üretecektir!
}  
```

Ancak referansın düşürülmesiyle borçlanma da ortadan kalkacağından aşağıdaki program beklendiği gibi çalışacaktır.

```rust
fn foo(v1: Vec<i32>) { // <- İşlev normal bir vektör değeri beklediğinden
    println!("İşlevdeki vektör: {:?}", v1);
}
fn main() {
    let v1 = vec![1, 2, 3];

    {           // <- Anonim bir blok açıyoruz
        let referans_vec = &v1;
    }           // <- Bir değişkenin ömrü sadece bulunduğu blok içinde anlamlı
    // olduğundan kapanan parantez ile verilen referans düşürülmüş olur.

    foo(v1);    // <- Artık nesnenin bir referansı olmadığından işlev çalışır.
}
```

Referansın isimsiz bir işlev içinde yaratılıp işlev sonlandığında kendiliğinden düşürülmesi ve `foo()` işlevinin çağrıldığı kod uzayında `v1` vektörüne ait bir referans kalmaması nedeniyle vektörün mülkiyeti sorunsuz şekilde işleve aktarılacaktır.

Rust, değiştirilemez olarak bildirilmiş değişkelere, değişebilir referanslar verilmesine müsaade etmez. Ancak, değişebilir olarak tanımlanmış değişkenlerin `&T` yerine `&mut T` şeklinde değişebilir referanslarını kabul eder.

```rust
fn main() {
    let mut x = 5;

    {
        let y = &mut x; // değişebilir değişkenin değişebilir referansı.
        *y += 1;        // Referansın gösterdiği kaynak arttırılır.
    }

    println!("{}", x);  // Değişken değeri 6 olduğundan '6' çıktısı verecektir.
}
```

Örnekte olduğu gibi `let` anahtar sözcüğüyle tanımlanmış `immutable` değişkene, `mutable` bir değişken referansı geçirilirken, atanan değişkenin `mutable` olmasına ihtiyaç yoktur. Rust derleyicisinin tür çıkarsama mekanizması, eşitliğin sağ tarafına bakarak çıkarımını gerçekleştirdiğinden bunu gereksiz kılar. 
Ancak `mutable` olarak borç alınmış bir değişken referansının `let y = &x;` şeklinde değişmez olarak bildirilmesi; `*y += 1` "değişmez olarak borçlanılmış değişkene bir değer atanıyor" hatasının alınmasıyla sonuçlanacaktır.
Borç alınan değişkenin `let mut x = 5;` şekilde tanımlanmamış olması da `immutable` değişkenin `mutable` olarak referansı alınıyor hatası ile sonuçlanır.

`Mutable` olsun veya olmasın bir değişkene ait veriler değişken ödünç alındığında dondurulur. Bu dondurulmuş veriler, bütün referanslar kapsam dışına çıkana kadar orjinal nesne üzerinde değişmez olarak korunur. Bu işleme **freezing** adı verilir.

```Rust
fn main() {
    let mut sayi = 7i32;

    {
        // 'sayi' ödünç verildiğinde referans kapsamı da başlıyor.
        let ref_sayi = &sayi;

        sayi = 50;      // <- Bu satır, sayı değişkeni referans alındığı için 
                        // hata üretecektir.
    }                   // <- Referans kapsamından çıkıyor.

    sayi = 3;           // <- Referansı kalmayan mutable değişken değiştirilebilecektir.
}
```
