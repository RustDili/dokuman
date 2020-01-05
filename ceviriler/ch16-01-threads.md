## Eş zamanlı kod çalıştırmak için iş parçalarını kullanmak
Çalıştırılabilen bir programın kodu mevcut işletim sistemlerinin çoğunda tek süreçte başlatılır ve işletim sistemi aynı anda birden fazla işlemi yönetir. Elbette ki bir programda aynı anda çalışan bağımsız parçalar da bulunabilir ve bu bağımsız parçaları çalıştıran özelliklere **iş parçası** denilir.

Bir programın işleyişini birden fazla iş parçasına bölerek aynı anda çok sayıda iş yapılabileceğinden, o programın performansı da artar ancak bu durum karmaşıklığı da beraberinde getirir. İş parçaları eş zamanlı olarak çalışabildiğinden, kodun farklı iş parçalarına ayrılan bölümlerinin hangi sırayla çalıştırılacağına dair hiçbir garanti verilmez. Doğal olarak bu durum aşağıdaki gibi sorunlara yol açabilir.

 * İş parçalarının verilere veya kaynaklara tutarsız bir sırayla eriştiği yarış koşulları **-race conditions**
 * Başka bir iş parçasının sahip olduğu kaynağı kullanan iş parçalarının birbirlerinin işlerini bitirmelerini beklediği ve programın devam    etmesini engelleyen kilitlenme **-deadlock** 
  * Yalnızca belirli durumlarda meydana gelen ve güvenli biçimde düzeltilmesi zor olan hatalar.

Rust, iş parçası kullanmanın olumsuz etkilerini hafifletmeye çalışır. Ancak yine de çok sayıda iş parçasının olduğu sistemler için program yazmak, dikkatlice düşünmeyi ve tek bir iş parçasıylada çalışan programlardan farklı bir kod yapısı tasarlamayı gerektirir.

Programlama dilleri iş parçalarını birkaç farklı şekilde uygular. İşletim sistemlerinin yeni iş parçaları oluşturulabilmesi için bir API sağladığı ve programlama dilinin iş parçaları oluşturmak için bu API'leri çağırdığı bu modele **1:1** adı verilir. Bu model o programlama dili için işletim sisteminin bir adet iş parçası ayırdığı anlamına gelir. 
Yine birçok programlama dili kendi özel iş parçası uygulamalarını bünyesinde barındırır. Programlama dilinin kendi uygulamaları tarafından gerçekleştirilen bu iş parçalarına **yeşil iş parçaları** denir. Yeşil iş parçalarını kullanan diller bu parçaları farklı sayıda işletim sistemi iş parçası bağlamında yürütürler. Yeşil iş parçaları **M:N** şeklinde anılan bir modele sahiptirler. Ve anlamı da: **"M işletim sistemi iş parçası başına N adet yeşil iş parçası vardır ve bu iş parçası adetleri aynı sayıda olmak zorunda değildir."**

Her modelin kendine göre avantajları ve dezavantajları vardır ancak Rust için önemli olan kazanç çalışma zamanı desteğidir. Çalışma zamanı kavramı kafa karıştırıcı bir terimdir ve farklı bağlamlarda farklı anlamlara sahip olabilir. 
Burada anılan çalışma zamanı ile dil tarafından koda dahil edilen tüm ikili dosyalar kastedilmektedir. Bu kod dilin yapısına bağlı olarak büyük veya küçük boyutlarda olabilir, ancak tüm derlenmemiş program bir miktar çalışma zamanı koduna da sahip olacaktır. Bu nedenle, bir dilin "çalışma zamanı olmadığı" söylendiğinde, bu genellikle "çok kısa çalışma zamanı" anlamına gelmektedir. Haliyle kısa çalışma süreleri daha az özelliğe sahip olmakla birlikte daha küçük ikili dosyalara sahiptirler ve bu da dilin başka dillerle uyum içinde entegrasyonunu kolaylaştırır. Birçok dilde daha fazla özellik daha büyük çalışma zamanı demek olsa da Rust; olabildiğince özellikli, en kısa çalışma süresine sahip, ve performans konusunda C dilini aratmayacak şekilde çalışmakta ve bunların hiçbirinden ödün vermemektedir.

Yeşil iş parçası olarak bilinen *M:N* modeli, iş parçalarını yönetebilmek için daha uzun bir çalışma zamanı gerektirir. Bu nedenle Rust’ın standart kütüphanesi yalnızca **1:1** iş parçası uygulama olanağı sunar. Bununla birlikte Rust düşük seviyeli bir dil olduğundan **M:N** modeli olarak anılan yeşil iş parçalarını sunan sandıklardan da yararlanır. Bu sandıklar hangi iş parçasının ne zaman çalıştığı gibi iş parçaları üzerinde daha fazla kontrol ve denetim sağlarlar.

Artık Rust’ta bulunan iş parçaları tanımlandığına göre standart kütüphane tarafından sağlanan iş parçasıyla ilgili API’nin kullanımını inceleyebiliriz. 

### `spawn` ile yeni bir iş parçası oluşturmak
Yeni bir iş parçası olarak çalıştırılmak istenen kod bir kapama işlevi şeklinde `thread::spawn()` işlevine iletilir. Aşağıdaki örnekte ana iş parçasından alınan bir metin ve ek iş parçasından alınan başka bir metin yazdırılır.

Dosya: src/main.rs
```Rust
use std::thread;
use std::time::Duration; 

fn main() { 
    thread::spawn(|| { 
        for i in 1..10 { 
            println!("Selam, ben yeni iş parçasından elde edilen numara {}", i); 
            thread::sleep(Duration::from_millis(1)); 
        } 
    }); 

    for i in 1..5 { 
        println!("Merhaba, ben ana iş parçasından elde edilen {} numara!", i); 
        thread::sleep(Duration::from_millis(1)); 
    } 
}
````
Örnek 16.1- Ana iş parçası çalışırken yeni bir iş parçasını eş zamanlı çalıştırmak

Bu örneğin çıktısında da görüleceği gibi, ana iş parçasının çalışması sona erdiğinde işi bitsin veya bitmesin, yeni iş parçasının çalışması durdurulur. Program her çalıştırıldığında aşağıdakine benzer çıktı üretecektir.
```binary
Merhaba, ben ana iş parçasından elde edilen 1 numara!
Selam, ben yeni iş parçasından elde edilen numara 1
Merhaba, ben ana iş parçasından elde edilen 2 numara!
Selam, ben yeni iş parçasından elde edilen numara 2
Merhaba, ben ana iş parçasından elde edilen 3 numara!
Selam, ben yeni iş parçasından elde edilen numara 3
Merhaba, ben ana iş parçasından elde edilen 4 numara!
Selam, ben yeni iş parçasından elde edilen numara 4
````
Örnekte yer alan `thread::sleep()` işlevi o anda çalışan iş parçasının yürütmesini kısa bir süreliğine durdurarak farklı bir iş parçasının çalışmasına izin verir. İş parçaları olasılıkla sırayla işleyecektir ancak bu sıralı çalışma süreci, işletim sisteminin iş parçalarını programlama şekline bağlı olduğundan garanti edilemez. Bu programda da ilk olarak yeni iş parçasının ekrana bir numara bastırması beklenirken, ilk olarak ana iş parçası ekrana yazdırılmakta; hatta yeni iş parçasına 9’a kadar yazması söylenmesine rağmen ana iş parçası kapanana kadar sadece 5’e kadar yazabildiği görünmektedir. 

Kod çalıştırıldığında sadece ana iş parçasının çıktısı görülüyorsa ana iş parçası üzerindeki `sleep()` değerini biraz arttırarak yeni iş parçasının nasıl çalıştığı gözlemlenmelidir. Ana iş parçası yeteri kadar süre bekletildiğinde yeni iş parçasının 9’a kadar olan döngü adımlarının tamamlayacağı görülecektir.

### `join` Handle kullanarak tüm iş parçalarının sonlanmasını beklemek 
İş parçalarının çalışma sıralarının garanti edilememesi, önceki kodda `main()` işlevinin erken sonlanması nedeniyle yeni iş parçasının işletiminin kesilmesine neden olur. Bu yüzden yeni iş parçasının sonlanıp sonlanmayacağı da garanti edilmez. 
Bu sorunla başa çıkmanın bir yolu yukarıda anlatıldığı gibi `main()` işlevinin bekleme süresini, yeni iş parçasının çalışma süresini bekleyecek şekilde ayarlamaktır.

Bir başka çözüm ise `thread::spawn()` dönüş değerinin bir değişkende saklanmasıdır. `Thread::spawn()`’dan elde edilen dönüş türüne `JoinHandle` adı verilir bu değer `join` metodunu çağırdığımızda, iş parçasının sonlanmasını bekleyen bir değerdir. Aşağıdaki program bir önceki örnekte oluşturulan yeni iş parçasının, `JoinHandle` dönüş değeri kullanılarak yeniden tasarlanmış halidir. Örnekte `main()` sonlanmadan önce yeni iş parçasının işini bitirmesi ve sonlanması beklenmiştir.

Dosya: src/main.rs
```Rust

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Selam, ben yeni iş parçasından elde edilen numara {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("Merhaba, ben ana iş parçasından elde edilen {} numara!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}
````
Örnek 16.2- `thread::spawn` işlevinden kaydedilen `joinHandle`’ın, iş parçasının görevini tamamlatmak için kullanılması 

Handle’daki birleştirme çağrısı, tanıtıcı tarafından temsil edilen iş parçası sona erene kadar çalışmakta olan iş parçasını engeller. Bir iş parçasının engellenmesi o iş parçasının iş yapması veya sonlanmasının engellenmesi anlamına gelmektedir. Dolayısıyla üstteki programda ana iş parçasının döngüsü biter bitmez `join` çağrısı yapıldığından programın çıktısı aşağıdakine benzeyecektir. 

```Binary
Merhaba, ben ana iş parçasından elde edilen 1 numara!
Selam, ben yeni iş parçasından elde edilen numara 1
Merhaba, ben ana iş parçasından elde edilen 2 numara!
Selam, ben yeni iş parçasından elde edilen numara 2
Merhaba, ben ana iş parçasından elde edilen 3 numara!
Selam, ben yeni iş parçasından elde edilen numara 3
Merhaba, ben ana iş parçasından elde edilen 4 numara!
Selam, ben yeni iş parçasından elde edilen numara 4
Selam, ben yeni iş parçasından elde edilen numara 5
Selam, ben yeni iş parçasından elde edilen numara 6
Selam, ben yeni iş parçasından elde edilen numara 7
Selam, ben yeni iş parçasından elde edilen numara 8
Selam, ben yeni iş parçasından elde edilen numara 9
````
Programın işletildiğinde iki iş parçası dönüşümlü olarak çalışarak ana iş parçasının son adımına kadar ilerlerler. Bu noktada ana iş parçası `handle.join()` çağrısı nedeniyle yeni iş parçasının işini bitirmesini bekler ve program yeni iş parçası işini bitirine kadar sonlanmaz.

Yukarıdaki örneğin `handle.join()` işlevi, `main()` içindeki `for` döngüsünün üstüne alındığında birleştirme işlemi anlamsızlaşır.

Dosya: src/main.rs
```Rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Selam, ben yeni iş parçasından elde edilen numara {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    handle.join().unwrap();
    
    for i in 1..5 {
        println!("Merhaba, ben ana iş parçasından elde edilen {} numara!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
````
Örnek 16.2.1- `Join` işleminin yerinin değişmesi ve sonuçlarının gözlenmesi

Bu durumda ana iş parçası, yeni iş parçasının işini bitirmesini bekler, yeni iş parçasının işi biter bitmez kendi döngüsünü çalıştırmaya başlar ancak handle kaynağında işini bitirmek için bekleyen bir iş parçası olmadığından çıktı artık birleştirilmez.

```Binary
Selam, ben yeni iş parçasından elde edilen numara 1
Selam, ben yeni iş parçasından elde edilen numara 2
Selam, ben yeni iş parçasından elde edilen numara 3
Selam, ben yeni iş parçasından elde edilen numara 4
Selam, ben yeni iş parçasından elde edilen numara 5
Selam, ben yeni iş parçasından elde edilen numara 6
Selam, ben yeni iş parçasından elde edilen numara 7
Selam, ben yeni iş parçasından elde edilen numara 8
Selam, ben yeni iş parçasından elde edilen numara 9
Merhaba, ben ana iş parçasından elde edilen 1 numara!
Merhaba, ben ana iş parçasından elde edilen 2 numara!
Merhaba, ben ana iş parçasından elde edilen 3 numara!
Merhaba, ben ana iş parçasından elde edilen 4 numara!
````
Unutulmamalıdır ki birleştirmenin çağrıldığı yer iş parçalarının aynı anda çalışıp çalışmamasını etkileyebilir. Bu nedenle `handle.Join()` işlev çağrısının yapıldığı yerin dikkatlice tasarlanmasını önemlidir.

### İş parçası ve `move` Kullanımı 
Bir kapama işlevi olan `move` genellikle `thread::spawn` ile birlikte kullanılır. Çünkü bu işlev bir iş parçasında bulunan verilerin başka bir iş parçasına taşınarak kullanılabilmesini sağlar. 
Parametre listesinden önce kullanılan `move`; kullanıldığı kapama işlevini, etkileştiği ortam değişkenlerinin mülkiyetini almaya zorlar. Bu teknik, özellikle değerlerin mülkiyet haklarını iş parçalarına aktarırken veya yeni iş parçaları oluştururken kullanışlıdır. 

Örnek 16-1'de, `thread::spawn()`' a geçirilen kapama işlevi hiçbir argüman almadığından, ana iş parçasına ait hiçbir veri, yeni iş parçacısında kullanılamamaktadır. Bir programda tasarlanan ek iş parçalarının ana iş parçasında bulunan verileri kullanabilmeleri için, ait oldukları kapama işlevinin ana iş parçasındaki verilere sahip olması gerekmektedir.
Aşağıdaki girişim ana iş parçasında oluşturulan bir vektörü yeni iş parçası üzerinde çalıştırmayı amaçlamaktadır. Ancak tahmin edileceği gibi bu yöntem bu şekliyle çalışmaz.

Dosya: src/main.rs
```Rust

use std::thread; 

fn main() { 
    let v = vec![1, 2, 3]; 
    let handle = thread::spawn(|| { 
        println!("İşte sana bir vektör: {:?}", v); 
    }); 

    handle.join().unwrap(); 
}
````
Örnek 16.3- Ana iş parçacığı tarafından oluşturulan bir vektörü başka bir iş parçasında kullanma girişimi

`handle` değişkeniyle temsil edilen kapama işlevi `v` bağlamını kullandığından,  `v` vektörünü yakalayarak kapama ortamının parçası haline getirir. Ve işlev `thread::spawn()` tarafından yeni bir iş parçasında çalıştırıldığından, yeni iş parçası içinden `v` verilerine mantıken ulaşılabiliyor olması beklenmektedir. Ancak örnek derlendiğinde aşağıdaki hata ile karşılaşılır:
```Binary
error[E0373]: closure may outlive the current function, but it borrows `v`,
which is owned by the current function
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("İşte sana bir vektör: {:?}", v);
  |                                           - `v` is borrowed here
  |
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
````
Kapama işlevindeki `println!()` makrosu, `v` bağlamındaki vektörü yazdırabilmesi için `v` referansına ihtiyaç duyduğundan, `v` bağlamını ödünç almaya çalışmakta, bu durum ise programın hata üretmesine neden olmaktadır. Üretilen hata bildiriminde derleyici ihtiyaç duyulan verinin nasıl ödünç alınacağını özetlemektedir. Geçerli olmayan `v`’ ye referans olabilecek daha iyi bir senaryo aşağıda önerilmektedir.

Dosya: src/main.rs
```Rust
use std::thread; 

fn main() { 
    let v = vec![1, 2, 3]; 
    let handle = thread::spawn(|| { 
        println!("İşte sana bir vektör: {:?}", v); 
    }); 

    Drop(v); // Hayır, olamaz!
    handle.join().unwrap(); 
}
````
Örnek 16.4- `drop()` işlevi yardımıyla ana iş parçasındam referans yakalamaya çalışan bir kapama örneği 

Eğer derleyici bu kodun çalıştırılmasına izin vermiş olsaydı, yeni iş parçasının neredeyse hiç çalıştırılmadan arka plana atılması olasıydı. Çünkü yeni iş parçası halihazırda `v` referansına sahipken, ana iş parçası üzerinde `drop()` işlevinin kullanılması `v` ’nin hemen düşmesine neden olur. Ardından yeni iş parçası yürütülmeye başlandığında `v` artık geçersiz olduğundan buna yapılan başvurular da geçersiz olmaktadır.

Hatayı giderebilmek için hata iletisinde bulunan tavsiyelerden yararlanabiliriz.

```Binary
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
  ````
`move` anahatar kelimesinin kapama işlevinin önünde kullanılması; derleyiciyi değerleri ödünç almak yerine, kapama işlevini kullanılan değerlere sahip olmaya zorlayacaktır. Yani `move` anahtar kelimesinin kullanılması inisiyatifi derleyiciye bırakmak yerine, kapama işlevine ihtiyaç duyduğu değerlerin mülkiyetini aktararak programın çalışmasını sağlamaktadır.

Dosya: src/main.rs
```Rust
use std::thread; 

fn main() { 
    let v = vec![1, 2, 3]; 
    let handle = thread::spawn(move || { 
        println!("İşte sana bir vektör: {:?}", v); 
    }); 

    handle.join().unwrap(); 
}

````
Örnek 16.5- `move` anahtar kelimesi kullanarak kapama işlevini kullandığı değerlerin sahipliğini almaya zorlamak   

Peki ama örnek 16-4' te, `move` yönetemini kullanmış olsaydık ana iş parçası yani main işlevi içinde kullanmış olduğumuz `drop()` drop işlevine ne olurdu yani `move`' un kullanılması sorunları çözebilir miydi? Maalesef hayır; örnek 16-4'ün yapmaya çalıştığı şeye izin verilmeyeceğinden bu defa farklı bir hata ile karşılaşırdık. O örnekteki kapama işlevinde `move` kullanmış olsaydık `v`' yi kapama işlevine taşımış olur ana iş parçasına değerleri düşürme çağrısı yapamazdık. Bu senaryoda aşağıdaki gibi bir hata ile karşılaşmış olurduk.   
```Binary
error[E0382]: use of moved value: `v`
  --> src/main.rs:10:10
   |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
10 |     drop(v); // oh no!
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
   not implement the `Copy` trait
````
Rust' ın mülkiyet kuralları bizi tekrar kurtardı! Daha önce de örnek 16-3'te tasarlanan iş parçası `v` vektörünün mülkiyetini almak yerine, değerlerini ödünç almaya çalıştığından; Rust derleyicisi riske girmeyi seçmek yerine, `v`' nin mülkiyetinin iş parçasına taşınması gerektiğini bildiren bir hata döndürmüştü. Bu bildirim aynı zamanda `v` vektörünün mülkiyetinin yeni iş parçasına aktarılmasıyla ana iş parçasının artık `v` vektörünü kullanmayacağının da garantisidir. 
Aynı şekilde örnek 16-4'teki yeni iş parçasında `move` kullanmak; mülkiyeti yeni iş parçasına aktarılan `v` vektörünün ana iş parçasındaki `drop` işlevi üzerinden kullanma girişimi sayılacağından mülkiyet kurallarının ihlal edilmesi anlamına gelir. Buna izin verilmez.

İş parçası ve iş parçası API'sı hakkında edindiğimiz temel bilgiler ışığında, iş parçalarıyla neler yapabileceğimize bakalım.
