## Durum paylaşımlı eş zamanlılık
İş parçaları arasında mesajlaşarak iletişim kurmak, eş zamanlılık yönetiminin güzel bir yolu olmakla birlikte tek seçeneği bu değildir. Hatırlanacağı gibi Go dili dökümanlarında yer alan sloganın bir parçası "Hafızanın paylaştırılarak iletişim kurulması" tavsiyesinde bulunuyordu. 
Peki ama, iş parçaları arasında bellek paylaşımıyla veri aktarmak nedir ve neden bunun yerine mesajlaşma yöntemiyle veri paylaşımı daha fazla tercih edilmektedir?

Herhangi bir programlama dilinde kanallar, bir bakıma tek mülkiyetli gibidirler; yani bir kanala herhangi bir değer aktarıldığında bu değer bir daha kullanılmaz. Oysa bellek paylaşımlı eş zamanlılık, birden fazla mülkiyetin olması demektir ve bu da haliyle, çok sayıda iş parçası aynı anda aynı bellek konumuna erişmesi anlamına gelmektedir. Fakat mülkiyetin çok sayıda paydaşının olması, bu farklı paydaşların her birinin ayrı ayrı yönetilmesini gerektirdiğinden, karmaşıklığı da beraberinde getirir. Rust’ın tür sistemi ve mülkiyet kuralları, bu durumla başa çıkabilmeyi oldukça kolaylaştırır. 
Paylaşılan hafıza yönetiminde en yaygın eşzamanlılık ilkelerinden biri olan **mutex**lere bakalım.

### Mutex'leri kullanarak iş parçası verilerine aynı anda erişmek
Mutex; tuttuğu verilere aynı anda sadece bir iş parçasının erişmesine izin veren, -İngilizce'deki "Mutual exclusion" sözcüklerinin- karşılıklı dışlama mekanizmasının kısaltılmış adıdır. Herhangi bir iş parçasının mutex içindeki verilere erişebilmesi için, erişim isteğinde bulunarak mutex kilidini talep etmesi gerekir. Talep edilen kilit, o an için verilere özel erişim izni olan ve iş parçalarını takip eden mutex mekanizmasına ait olan veri parçasıdır. Kısaca "mutex, içindeki verileri kilitleme mekanizması aracılığıyla korur" diyebiliriz.

* Mutex kullanımı, aşağıdaki kuralların hatırlanması gerekli olduğundan kullanılması oldukça zor olarak bilinir:
  * Verileri kullanmaya başlamadan önce kilidi alınmalıdır.
  * Mutex tarafından korunan verilerin kulanımı sona erdiğinde, kilidin diğer iş parçalarının kullanımına açılması gereklidir.

Mutex’lerin gerçek dünyada neye benzediğini kavrayabilmek için, kalabalık bir grup ve yalnızca bir mikrofon ile gerçekleştirilen bir panele katıldığınızı düşünün. Paneldeki her tartışmacı, konuşmaya başlamadan önce, mikrofonu kullanmak istediğini bildirecek veya işaret edecek, konuşmacılar sadece mikrofonu aldıklarında konuşabilecekler ve mikrofonu konuşmalarını bitirdiklerinde bir sonraki tartışmacıya verebileceklerdir. Konuşma sırasının bu şekilde ilerlediği bir toplantıda, konuşmacılardan birinin söylevini bitirdiğiinde mikrofonu elinde unutması veya başka bir tartışmacıya geri vermemesi, diğer konuşmacıların konuşmalarını engelleyeceğinden, mikrofon paylaşımının aksaması halinde tartışmanın planlandığı gibi ilerlemez.

Benzer şekilde, bazı durumlarda mutex mekanizmasının doğru biçimde idaresi oldukça zor olabileceğinden, pekçok programcı tercihlerini kanalları kullanmaktan yana yapar. Bununla birlikte, Rust’in **tür sistemi** ve **mülkiyet kuralları** sayesinde, yanlış kilitleme ve kilit açma işlemini gerçekleştirmeniz mümkün değildir.

### `Mutex<T>` Api'si
Mutex’lerin nasıl kullanılabileceğine dair örneklere aşağıdaki tek iş parçasına sahip kod ile başlayalım. 

Dosya: src/main.rs
```Rust
use std::sync::Mutex; 
fn main() { 
    let m = Mutex::new(5); 
    
    { 
        let mut num = m.lock().unwrap(); 
        *num = 6; 
    } 

    println!("m = {:?}", m); 
}
 ````
 Örnek 16.12- Tek iş parçacıklı basit bir örnek üzerinden `Mutex<T>` Api'sini keşfetmek.
 
Rust türlerinin çoğunda olduğu gibi, yeni bir `Mutex<T>` oluşturabilmek için türün `new()` işlevinden yararlanılır. Kilidi elde ederek muteks içindeki verilere ulaşmak içinse `lock()` işlevi kullanılır. Bu işlevin çağrılmasıyla, kilit çağıran tarafın yönetimine geçene kadar yürürlükte olan iş parçasını hiçbir şey yapamayacak şekilde engellenir. 

Ancak bu devir teslim sırasında, kilidi elinde tutan iş parçası panik ile sonlanacak bir hata ile karşılaşmış olsaydı, erişilmek istenen verinin, çağrı yapılan iş parçasına ait olması nedeniyle, yapılan çağrı da başarısız olacak ve kilit hiç kimse tarafından elde edilemeyecekti.

Kilit elde edildikten sonra, `num` adlı dönüş değerini, içindeki verilere referans olan bir değişken olarak değerlendirebiliriz. `Mutex<i32>` türündeki `m` değişkenini `i32` türü olarak kullanmadan önce tür sisteminin sağladığı kilidi elde etmemiz gerekir. Tür sistemi mutex kilidi elde edilmeden `i32` değerinin kullanılmasına izin vermez.

Sizin de tahmin edeceğiniz gibi `Mutex<T>` aslında akıllı bir işaretçidir. Daha kesin ifadeyle, kilitleme ile birlikte yapılan `unwrap()` çağrısı, `LockResult` ile sarmalanmış `MutexGuard` adında akıllı bir işaretçi döndürür. `MutexGuard` akıllı işaretçisi, hem kapsam içindeki verilerimize işaret eden `Deref` uygulamasına, hem de kapsamın sonuna gelindiğinde kilidi otomatik olarak iade eden serbest bırakma uygulaması olan `Drop` yöntemine sahiptir. Sonuçta hem mutex'in başka iş parçaları tarafından kullanılması önlenmiş, hem de  mekanizmanın otomatik açılmasıyla kilidin kapalı halde unutulması engellenmiş olur. 

Kapsam içinde değiştirilen mutex değerinin elde edilip yazdırılması ise ancak kilidin serbest bırakılmasından sonra gerçekleşir.

### İş parçaları arasında `Mutex<T>` paylaşımı
Bir değeri, `Mutex<T>` kullanarak çok sayıda iş parçası arasında paylaşmayı deneyelim. Deneyimizde her biri, 0’ dan 10’ a kadar ilerleyen sayacımızın değerini 1 arttıran, 10 adet iş parçamız olsun. Bu örnek ve devamındaki birkaç örnekte karşılaşılacak olan derleme hatalarından edinilecek bilgiler sayesinde `Mutex<T>` konusunu anlamada Rust’ın ne kadar yardımcı olduğunu gözlemleyeceğiz.

Dosya: src/main.rs
```Rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
````
Örnek 16.13- `Mutex<T>` içinde korunan bir sayacın, 10 ayrı iş parçası tarafından birer birer arttırılması deneyi

Örnek 16.12’de olduğu gibi `i32` türünden oluşturduğumuz sayacın değerini `Mutex<T>` içinde tutabileceğimiz `counter` değişkeniyle tanımlıyoruz. Ardından 0..10 arası sayı aralığını yineleyen bir döngüde `thread::spawn` kullanarak 10 adet iş parçası üretiyor ve tüm iş parçalarını aynı şekilde kullanıp sonlandırıyoruz. Her bir iş parçası çalışırken `Mutex<T>`'nin `lock` metodunu çağırarak kilit mekanizmasını elde ediyor ve mutex içindeki sayacı bir arttırıyoruz. Sürecin sonunda her bir iş parçasının kapama işlevi tamamlandığında, `num` değişkeni de kapsam dışınana çıkacağından, sahip olduğu kilidi de serbest bırakacak, böylece başka bir iş parçasının bu kilidi elde ederek çalışması sağlanmış olacaktır.

Programda çalışan bütün iş parçalarını ana iş parçası üzerinde bir araya getiriyor ve tüm iş parçalarının çalışmalarını bitirdiğinden emin olmak için her bir tutamağı `join` işleviyle birleştiriyoruz. Bu aşamada ana iş parçası olan `main()` işlevi, mutex kilidini elde edecek ve programın sonucunu yazdırmaya çalışacaktır.

Başında da bu örneğin derlenmeyeceğininden bahsetmiştik. Haydi şimdi neden derlenmediğini anlamaya çalışalım.

```Binary
error[E0382]: capture of moved value: `counter`
  --> src/main.rs:10:27
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
10 |             let mut num = counter.lock().unwrap();
   |                           ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:21:29
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
````
Hata mesajında `counter` değerinin kapama işlevine aktarıldığı ve kilit çağrısı yapıldığında yakalandığı belirtiliyor. Bu beklediğimiz türden bir açıklama olmasına rağmen sorunun giderilmesi için yeterli değil.  

Bu sorunu programı for in döngüsü ile yaratılan 10 adet iş parçası ile tasarlamak yerine, döngü kulanmaksızın sadece iki ayrı iş parçasından oluşan daha basit bir tasarımla çözümlemeyi deneyelim.

Dosya: src/main.rs
```Rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
````
Örnek 16.13 için yeniden düzenleme

Birbirinin kopyası iki iş parçasından oluşan ve ikinci iş parçasında kullanılan değişken isimleri `handle2` ve `num2` olarak değiştirilen programımızı çalıştırdığımızdaysa aşağıdaki hataları alırız.

```Binary
error[E0382]: capture of moved value: `counter`
  --> src/main.rs:16:24
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
16 |         let mut num2 = counter.lock().unwrap();
   |                        ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:26:29
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
26 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
````

Aha! İlk hata mesajında `counter` değişkeninin, ilgili iş parçasının kapama işlevine taşındığı belirtilir. Bu taşınma kilitlemeye çalıştığımız `counter` değişkenini yakalamamızı ve sonucunu ikinci iş parçasında sakladığımız ` num2` değişkeninde depolamamızı engeliyor. Aslında burada derleyici bize, `counter` değişkeni mülkiyetinin birden fazla iş parçasına aktarılamayacağını söylüyor. Önceki örneğimizde iş parçaların bir döngü yardımıyla oluşturulması ve her adımda hatanın oluştuğu farklı iş parçasına işaret edilmesi gerektiğinden dolayı bu sorunu sağlıklı şekilde gözlemlemek oldukça zordu. Artık mülkiyet paylaşımından kaynaklanan bu sorunu 15. bölümde bahsedilen çoklu mülkiyet yöntemini kullanarak çözümleyebiliriz.

### Çoklu iş parçaları ve çoklu mülkiyet
Referans sayım değeri oluşturabilmek için `Rc<T>` akıllı işaretçisi kullanmayı ve mülkiyet haklarını birer değer ile ilişkilendirerek saymayı 15. Bölümde göstermiştik. Bu hatayı giderebilmek için `Rc<T>` akıllı işaretçisini kullanmayı deneyebilir ve sonucun ne olacağını görebiliriz. Aşağıdaki örnekte de görüleceği gibi `Mutex<T>` türünü `Rc<T>` içinde sarmalayıp mülkiyetini iş parçasına aktarmadan önce de klonlarız. Artık karşılaşabileceğimiz hataların neler olduğunu bildiğimizden `for` döngüsüne yeniden başvurabilir, move anahtar kelimesiyle kapama işlevinden yararlanabiliriz.

Dosya: src/main.rs
```Rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
````
Örnek 16.14- `Rc<T>` kullanılarak çok sayıda iş parçasının `Mutex<T>` ile çalışmasını sağlamak

Programı bir kez daha derledik ve bir kez daha derleyicinin farklı şeyler öğrettiği yeni hatalarla karşılaştık.

```Binary
error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send` is not satisfied in `[closure@src/main.rs:11:36:
15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
  --> src/main.rs:11:22
   |
11 |         let handle = thread::spawn(move || {
   |                      ^^^^^^^^^^^^^ `std::rc::Rc<std::sync::Mutex<i32>>`
cannot be sent between threads safely
   |
   = help: within `[closure@src/main.rs:11:36: 15:10
counter:std::rc::Rc<std::sync::Mutex<i32>>]`, the trait `std::marker::Send` is
not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
   = note: required because it appears within the type
`[closure@src/main.rs:11:36: 15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
   = note: required by `std::thread::spawn`
````
Bu endişe verici hata mesajında odaklanılması gereken önemli bölümlerden ilki "`std::rc::Rc<std::sync::Mutex<i32>>`'nin, iş parçaları arasında güvenle gönderilemeyeceği" uyarısıdır. Bu oldukça temiz görünen uyarı, bir sonraki bölümde incelenecek olan `Send` özelliğinin bağlı olmadığı anlamına gelmektedir. Sonraki bölümde ele alınacak olan `Send` özelliğiyse; iş parçalarından yararlanan türlerin, eş zamanlı süreçlerde kullanılması için tasarlanmış bir özelliktir.

Ve ne yazık ki `Rc<T>`, iş parçaları arasında paylaşımda bulunmak için güvenli değildir. Çünkü referans sayımı `Rc<T>` ile yönetildiğinde `clone` ile türetilen her çağrı sayıma eklenecek, düşürülen her kopya ise referans sayımından çıkarılacak ve sayımdaki değişikliklerin, başka bir imş parçası tarafından kesilmememesi için eş zamanlılık ilkelleri de kullanılamayacaktır. Bu durum yanlış referans sayımıyla oluşabilecek bellek sızıntılarına, değerlerin vaktinden önce düşürülmesi gibi hassas sorunlara yol açabileceğinden referans sayımını güvenle yapabilecek `Rc<T>` benzeyen başka bir türe ihtiyacımız vardır.

### `Arc<T>` ile atomik referans sayımı
Neyse ki Rust bize `Rc<T>` benzeri, ancak eş zamanlı işlemlerde kullanılması güvenli olan `Arc<T>` adında bir tür daha sunar ve bu tür özü gereği atomik olduğundan referans sayımını da atomik olarak yapar. Atomikler bu bölümde ele alınmayan bir tür eş zamanlılık ilkesidir. Bunlarla ilgili daha fazla bilgi için standart kütüphane belgelerinde `std::sync::atomic` başlığına bakmak yararlı olabilir. Ancak bu noktada; atomiklerin ilkel türler gibi çalıştığının ve iş parçaları arasındaki paylaşımları güvenle sağladıklarının bilinmesi yeterlidir.

Bu noktada neden ilkel türlerin de atomik olmadığını veya standart kütüphane türlerinin Arc<T> türünü niçin varsayılan olarak kullanamadığını merak ediyor olabilirsiniz. Bunun açıklaması; "iş parçası güvenliğinin maliyetli olması ve gerçekten ihtiyaç duyulduğunda kullanılması gerektiği" olarak yapılabilir. Eğer sadece tek bir iş parçasını ilgilendiren değerler ile işlem yapıyorsanız, atomiklerin sağladığı garantileri uygulamak zorunda kalmayacağınız için kodunuz daha hızlı çalışacaktır.

Örneğimize dönecek olursak `Arc<T>` ile `Rc<T>` aynı API’ye sahip olduğundan, programımızı sadece gerekli bölümlerini değiştirerek yeniden düzenleyebiliriz. Sonunda kodumuz derlenecek ve beklendiği gibi çalışacaktır.

Dosya: src/main.rs
```Rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
````
Örnek 16.15- Mülkiyetin farklı iş parçalarıyla paylaşılabilmesi için `Mutex<T>` yi saran `Arc<T>` türünün kullanılması.

Sonunda 0’dan 10’a kadar sayarak birer iş parçası oluşturan programımızı çalıştırmayı başardık. Bu başarı çok etkileyici görünmese de `Mutex<T>` ve iş parçası güvenliği hakkında oldukça fazla şey öğrenmemize neden oldu. Programın tasarımından, daha karmaşık kodlar üretmek için yararlanabilir, bu stratejiden faydalanarak kodlarınızdaki hesaplamaları ayrı bölümlere taşıyabilir, her bölümü ayrı iş parçaları arasında paylaştırabilir, ve ardından her iş parçasının kesin sonuçlarını güncellemek için `Mutex<T>` kullanabilirsiniz.

### `RefCell<T>` / `Rc<T>` ile `Mutex<T>` / `Arc<T>` arasındaki benzerlikler
Bu bölümün örneklerinde kullanılan `counter` adlı bağlamın bir değişmez olduğunu fark etmişsinizdir. Bunun nedeni; `Mutex<T>` iç yapısının, tıpkı `Cell` ailesi gibi değişkenliğe izin vermesidir ve bu değişkenlik sayesinde `counter` değişmezi içindeki değere, değişken referansı alabildik. Benzer biçimde daha önce bölüm 15.'te `Rc<T>` içinde bulunan verileri değiştirmek için `RefCell` kullanmıştık; tıpkı önceki örnekte `Arc<T>` içindeki verileri değiştirmek için `Mutex<T>` türünü kullandığımız gibi. 

Hatırlanması gereken bir başka detay ise, `Mutex<T>` kullanımı sırasında Rust'un programcıyı mantık hatalarından koruyamayacağıdır. Bölüm 15’te, `Rc<T>` kullanılmasının, iki `Rc<T>` değerinin birbirine referans verdiğini ve bunun bellek sızıntısına neden olduğunu da unutmayın. Benzer şekilde, `Mutex<T>` de kilitlenme riskiyle birlikte gelir. Bu durum, bir işlemin iki kaynağı kilitlemesi gerektiğinde ve iki iş parçasından birinin kilitlerden birini alması ve birbirlerini sonsuza dek beklemelerine neden olması durumunda ortaya çıkar. 
Kilitlenme ile ilgileniyorsanız, kilitlenme olan bir Rust programı oluşturmayı deneyin; daha sonra herhangi bir dilde mutekseler için kilitlenme azaltma stratejileri araştırın ve bunları Rust'ta uygulamayı deneyin. `Mutex<T>` ve `MutexGuard` için standart kütüphane API belgeleri yararlı bilgiler sunar. 

Sonraki başlıkta `Send` ve `sync` özellikleri ve bunların türlerle nasıl kullanılabileceği konularından bahsedeceğiz.
