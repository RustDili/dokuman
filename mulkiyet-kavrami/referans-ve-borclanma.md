### Referans ve Borçlanma
Verileri hafızanın heap bölümünde depolanan bir değişkene ait kaynakların, atandığı başka bir değişkenin mülkiyetine aktarılıyor olması ve bu nedenle düşürülen bir önceki değişken ve kaynaklarına bir daha erişilemiyor olması olumsuz bir özellikmiş gibi görünüyor olabilir. Bir işleve parametre yoluyla geçirilen değişkenlerin programın ilerleyen bölümlerinde bir daha kullanılamıyor olması ise oldukça can sıkıcıdır. Değişkenin mülkiyeti devredilmeden sadece verisine erişilmek istenen bu gibi durumlarda Rust’ ın borçlanma mekanizmasından yararlanılır.

Bir değişkenin referansı; o değişkenin sahip olduğu verinin kendisi yerine bellekteki adresini gösterdiğinden, nesnenin kendisi yerine referansının kullanılarak işlenmesine borçlanma adını veriyoruz.
Bir nesnenin referansı işlendiği sürece düşürülmemesi; program derlenmeden önce Rust’ ın dahili **Borrow Checker** adlı mekanizması tarafından referansların geçerliliği kontrol edilerek sağlanır.

```rust
fn main() {
    // Box içinde bir integer oluşturuyoruz.g
    let boxed_int = Box::new(5);
    let ref_int = &boxed_int; // '&' ile referans oluşturuyoruz.

    println!("değişken: {}, referansı: {}", boxed_int, ref_int);
}
```

Bir değişkenin başka bir değişkene atanması ya da parametre yoluyla bir işleve geçirilmesi esnasında kullanılan & sembolü o değişkenin sadece referans verilmesine neden olacağından yukarıdaki örnekte yer alan boxed_int ve ref_int değişkenlerinin her ikisi de kullanılabilir durumdadırlar. Aşağıdaki örnekte bir vektörün referans yoluyla işlev parametresine geçirilmesi irdelenmektedir.

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

Bir kaynağın referansından önce silinmesi referansın artık null bir alanı göstermesine neden olacağından böyle bir duruma Rust’ın statik olarak çalışan Borrow Checker mekanizması tarafından izin verilmez.

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

Ancak referansın düşürülmesi veya yıkılmasıyla borçlanma da ortadan kalkmış olacağından aşağıdaki işlev beklendiği gibi çalışacaktır.

```rust
fn foo(v1: Vec<i32>) { // <- İşlev normal bir vektör değeri beklediğinden
    println!("İşlevdeki vektör: {:?}", v1);
}
fn main() {
    let v1 = vec![1, 2, 3];

    {           // <- Anonim bir blok açıyoruz
        let referans_vec = &v1;
    }           // <- Bir değişkenin ömrü sadece bulunduğu blok içinde anlamlı
    // olduğundan kapanan parantez ile verilen referans yıkılmış olur.

    foo(v1);    // <- Artık nesnenin bir referansı olmadığından işlev çalışır.
}
```

Referansın isimsiz bir işlev içinde yaratılıp işlev sonlandığında kendiliğinden yıkılması ve foo() işlevinin çağrıldığı kod uzayında v1 vektörüne verilen bir referansın bulunmaması nedeniyle vektörün mülkiyeti sorunsuz şekilde işleve aktarılacaktır.

Rust değiştirilemez olarak bağlanmış bir değişkenin değişebilir bir referansının oluşturulmasına izin vermez. Bununla birlikte değişebilir olarak tanımlanmış değişkenlerin &T yerine &mut T şeklinde değişebilir olarak bildirilmiş referanslarını da kabul eder.

```rust
fn main() {
    let mut x = 5;

    {
        let y = &mut x;   // değişebilir değişkenin değişebilir referansı.
        *y += 1;  // Referansın gösterdiği kaynak arttırılır.
    }

    println!("{}", x);    // Değişken değeri 6 olduğundan '6' çıktısı verecektir.
}
```

Örnekte olduğu gibi let anahtarıyla tanımlanmış bir değişkene değişebilir bir değişkenin referansı geçirilirken **let y = &mut x;** atanan değişkenin de mutable olarak bildirilmesi gerekmez. Rust derleyicisinin tür çıkarsama mekanizması eşitliğin sağ tarafına bakarak çıkarımını gerçekleştirdiğinden bunu gereksiz kılar. Ancak mutable olarak borç alınmış bir değişkenin referansının **let y = &x;** şeklinde değişmez olarak bildirilmesi *y += 1 değişmez olarak borçlanılmış değişkene bir değer atanıyor hatası döndürecektir. Borç alınan değişken x’in let mut x = 5; şekilde tanımlanmamış olması da immutable değişkenin mutable olarak referansı alınıyor hatası ile sonuçlanır.

Mutable olsun olmasın bir değişkeninin verileri, değişken ödünç alındığında dondurulur. Donmuş veriler bütün referanslar kapsam dışına çıkana kadar orjinal nesne üzerinde değişmez olarak korunur.  Bu işleme **freezing** adı verilir.

```Rust
fn main() {
    let mut sayi = 7i32;

    {
        // 'sayi' ödünç verildiğinde referans kapsamı da başlıyor.
        let ref_sayi = &sayi;

        sayi = 50;    // <- Bu satır, sayı değişkeni referans alındığı için 
  // hata üretecektir.
    } // <- Referans kapsamından çıkıyor.

    sayi = 3;         // <- Referansı kalmayan mutable değişken değiştirilebilecektir.
}
```
