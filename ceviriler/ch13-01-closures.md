## Kapamalar: Ortam değişkenlerini yakalayabilen anonim işlevler
Rust'un kapamaları, bir değişkene kaydedebileceğiniz veya diğer işlevlere argüman olarak iletebileceğiniz isimsiz işlevlerdir. Kapamaları tek bir yerde oluşturabilir ve daha sonra farklı bir bağlamda değerlendirmek için yeniden çağırabilirsiniz. İşlevlerin aksine kapamalar, kullanacakları değerleri tanımlandıkları kapsamdan elde edebilirler. Kapamaların sahip olduğu bu özelliklerin, kodların yeniden kullanımına ve davranışlarının özelleştirilmesine nasıl izin verdiğini göstereceğiz.

### Kapamalar ile bir davranışı soyutlamak
Bir kapamayı daha sonra işletilmek üzere saklamanın yararlı olduğu bir örnek üzerinden ilerleyerek, kapama söz dizimi, tür çıkarımı ve özelliklerinden bahsedelim. 

Özel egzersiz planları üreten bir uygulama projesinde çalıştığımızı varsayalım. Egzersiz planlarını oluştururken kullanıcısının belirttiği; yaş, vücut kitle indeksi, egzersiz tercihleri, son egzersizler gibi birçok faktörü dikkate alan bu algoritmanın arka ucunu da Rust ile yazdığımızı düşünelim. Bu örnekte algoritmanın kullanılabilir olmasından ziyade birkaç saniye gibi kısa bir sürede çalışması beklendiğinden, algoritmayı bir kez ve sadece ihtiyacımız olduğunda çağararak kullanıcıyı boş yere bekletmek istemiyoruz. 

Bu varsayımsal algoritmayı örnek 13-1' de yer alan `simulated_expensive_calculation` işlevini çağırarak simüle edecek, ekrana `"yavaşça hesaplanıyor..."` yazdırdıktan sonra iki saniye beklecek, ardından işleve ilettiğimiz sayıyı geriye döndüreceğiz.

Dosya adı: src/main.rs
```Rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("yavaşça hesaplanıyor...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
````
Örnek 13-1: İki saniyelik ayakta durma egzersizinde kullanılan ve çalışması iki saniye süren kurgusal hesaplama işlevi

Bu programın bir sonraki önemli adımı ise, egzersiz uygulamasının bölümlerini içeren `main` işlevidir. Bu işlevdeyse kullanıcı bir egzersiz planı istediğinde uygulamanın çağıracağı kod yer alır. Çünkü uygulamanın ön ucuyla etkileşim, kapamaların kullanımıyla ilgili olmadığından, programımıza girdileri temsil eden değerleri kodlayacak ve ardından çıktıları yazdıracağız.

* İhtiyacımız olan girdiler şunlardır:
  * Kullanıcının talep ettiği egzersizin düşük ya da yüksek yoğunluklu olduğunu gösteren bir egzersiz yoğunluk numarası 
  * Farklı antreman planlarının üretilmesini sağlayan rastgele bir sayı
  
Çıktımız ise önerilen egzersiz planı olacaktır. Örnek 13-2 bu verilerin `main` işlevinde nasıl kullanıldığı gösterilir.

Dosya adı: src/main.rs
```Rust
fn main() { 
    let simulated_user_specified_value = 10; 
    let simulated_random_number = 7; 

    generate_workout(
        simulated_user_specified_value, 
        simulated_random_number 
    ); 
}
````
Örnek 13-2: Kullanıcı girişi ve rastgele sayı oluşturmayı simüle etmek için kodlanmış değerlerden oluşan `main` işlevi

Sadeliği koruyabilmek amacıyla `simulated_user_specified_value` değişkenini 10, `simulated_random_number` değişkenini 7 değerleriyle sabit biçimde kodladık. Oysa gerçek bir programda, yoğunluk numarasını uygulamanın ön ucundan alır ve 2. Bölüm' deki tahmin oyunu örneğinde yaptığımız gibi `rand` sandığını rastgele bir sayı üretmek için kullanmaya çalışırdık. Örneğimizdeki `main` işlevi simüle giriş değerlerini kullanarak `create_workout` işlevini çağırmaktadır.

Artık ihtiyaç duyduğumuz içeriği elde ettiğimize göre, algoritma üzerine yoğunlaşabiliriz. Örnek 13-3' te yer alan `create_workout` işlevi uygulamanın çalışma mantığını yansıtan en önemli kısmı olduğundan, örnek üzerinde çalıştığımız sürece gerçekleştireceğimiz bütün kod değişiklikleri yalnızca bu işlevi kapsayacaktır.

Dosya adı: src/main.rs
```Rust
fn generate_workout(intensity: u32, random_number: u32) { 
    if intensity < 25 { 
        println!( 
            "Bugün, {} şınav çek!", 
            simulated_expensive_calculation(intensity) 
        ); 
        println!( 
            "Sonrasında {} mekik çek!", 
            simulated_expensive_calculation(intensity) 
        ); 
    } else { 
        if random_number == 3 { 
            println!("Bugün mola ver! Sıvı tüketmeyi de ihmal etme!"); 
        } else { 
            println!( 
                "Bugün, {} dakika koş!", 
                simulated_expensive_calculation(intensity) 
            ); 
        } 
    }
}
````
Örnek 13-3: Girdi ve çağrılarına göre egzersiz planları yazdıran `simulated_expensive_calculation` işlevi

Örnek13-3'teki kod, yavaş hesaplama yapan işleve çok sayıda başvuruda bulunur. İlk `if` bloğu `simulated_expensive_calculation` işlevini iki defa çağırırken, `else` bloğunun içindeki birinci `if` bloğu ise başvuruda bulunmaz. Oysa bir sonraki `else` bloğunda `simulated_expensive_calculation` işlevine yeniden çağrıda bulunulur.  

Öncelikle `generate_workout` işlevinin beklenen davranışı, kullanıcının düşük yoğunluklu bir egzersizi mi *(25'ten az bir sayı ile gösterilir)* yoksa yüksek yoğunluklu bir egzersizi mi *(25 veya daha büyük bir sayı)* isteyip istemediğini kontrol etmektir.

Düşük yoğunluklu egzersiz planları, simüle ettiğimiz karmaşık algoritmaya dayanan bir dizi şınav ve mekik antremanını önerecektir. 

Kullanıcının yüksek yoğunluklu bir egzersiz planı istemesi halinde; işin içine biraz daha mantık girecek ve: Uygulamanın oluşturduğu rastgele sayı 3 olduğunda kullanıcıya mola vererek sıvı tüketmesi önerilecek, diğer hallerdeyse algoritmanın belirlediği süre kadar koşması önerilecektir.   

Bu kod, şimdilik işverenimizin istediği şekilde çalışmaktadır. Ancak bir süre sonra, şirketimizin veri bilimi ekibinin, `simulated_expensive_calculation` işlevini çağırma yönteminde bir takım değişiklikler yapılması gerektiğine karar verdiğini varsayalım. Bu güncelleme senaryosunda değişikliklerin basit tutulabilmesi için `simulated_expensive_calculation` işlevini kendisine başka bir çağrı eklemeden, sadece bir kez çağırmak ve halihazırda kendisine yapılmakta olan gereksiz çağrıları da kesip atmak istiyoruz. Nihayetinde bu işlev maliyetli bir işlev olduğundan, gerekmedikçe çağrıda bulunmamak, gerekiyorsa da sadece bir kez çağrıda bulunmak istiyoruz. 

### İşlevleri kullanarak yeniden düzenlemek
Egzersiz programını birçok şekilde yeniden yapılandırabiliriz. Öncelikle, örnek 13-4' te gösterildiği gibi, `simulated_expensive_calculation` işlevi için tekrarlanan çağrıyı bir değişkene çıkarmayı deneyeceğiz.

Dosya adı: src/main.rs
```Rust
fn generate_workout(intensity: u32, random_number: u32) { 
    let expensive_result = 
        simulated_expensive_calculation(intensity); 

    if intensity < 25 { 
        println!( 
            "Bugün, {} şınav çek!", 
            expensive_result 
        ); 
        println!( 
            "Sonrasında {} mekik çek!", 
            expensive_result 
        ); 
    } else { 
        if random_number == 3 { 
            println!(
                "Bugün bir mola ver! Sıvı tüketmeyi de ihmal etme!"); 
        } else { 
            println!(
                "Bugün, {} dakika koş!", 
                expensive_result 
            ); 
        } 
    } 
}
````
Örnek 13-4: `simulated_expensive_calculation` çağrılarını tek bir yere çıkarma ve sonucunu `expensive_result` değişkenine kaydetmek

Bu değişiklik, `simulated_expensive_calculation` çağrılarını birleştirerek işlevi gereksiz yere iki kez çağıran ilk `if` bloğunun sorununu çözecektir. Fakat ne yazık ki, bu defa da sonuç değerini hiç kullanmayan iç `if` bloğu da dahil, her durumda sonucu beklemek zorunda kalıyoruz. 

Oysa biz bu kodu, programımızın tek bir yerinde tanımlamak ve sadece sonuca gerçekten ihtiyaç duyduğumuz yerde işletmek istiyorduk. İşte bu durum tam da kapamaların kullanılmasını gerektiren bir durumdur.

### Bir kodu kapama kullanarak yeniden düzenlemek
Her `if` bloğu öncesinde `simulated_expensive_calculation` işlevini çağırmak yerine, örnek 13-5'te gösterildiği gibi bir kapama tanımlayabilir ve tanımlanan kapamayı işlev çağrısının sonucunu saklamak yerine bir değişkene depolayabiliriz. Aslında `simulated_expensive_calculation`' ın tüm gövdesini burada tanıtacağımız kapama içine de taşıyabiliriz.

Dosya adı: src/main.rs
```Rust
    let expensive_closure = |num| { 
        println!("yavaşça hesaplanıyor..."); 
        thread::sleep(Duration::from_secs(2)); 
        num 
    };
````
Örnek 13-5: Bir kapama tanımlayarak `expensive_closure` adlı değişkeninde saklamak

Kapama tanımı `expensive_closure` değişkenine atanabilmesi için atama operatöründen sonra gerçekleştirilir. Bir kapamanın tanımlanmasına içinde kapama parametrelerinin yer alacağı bir çift dikey boru `(|)` ile başlanır. Bu sözdizimi, Smalltalk ve Ruby'deki kapama tanımlarına benzediğinden dolayı seçilmiştir. Örneğimizdeki kapama, `num` adında yalnızca bir parametreye sahip olduğundan `|num|` biçiminde ifade edilir: Eğer kullanmamız gereken çok sayıda parametremiz olsaydı, bu parametreleri yine çift boru içine `| param1, param2 |` şeklinde virgüllerle ayırırarak kullanmamız gerekecekti.

Parametrelerin ardından, kapama gövdesini tutan kıvrımlı parantezleri yerleştiririz. Eğer kapama gövdesi tek bir ifadeden oluşuyorsa bu parantezleri kullanmak tercihinize bırakılır. `let` ifadesinin tamamlanabilmesi için kapamanın sonunda, yani kıvrımlı parantezin bitiminde, **`;`** noktalı virgülün kullanılması şarttır. İşlev gövdelerinde olduğu gibi kapama gövdelerindeki son değerler de döndürülen değer statüsünde olduklarından *(örneğimizde num)* noktalı virgül ile sonlandırılmazlar. 

`expensive_closure` adındaki bu `let` ifadesinin; isimsiz işlevin çağrılmasıyla oluşan sonuç değerini değil, **isimsiz işlev tanımını** içerdiğine dikkat edin. Kapatmaları: Bir noktada çağrılacak kodu tanımlamak, bu kodu saklamak ve programın ilerleyen safhalarında kendisine başvurabilmek için kullandığımızı unutmayın. Bu aşamada çağırmak istediğimiz kod artık `expensive_closure` içinde saklanmaya başlamıştır. 

Bu aşamada, tanımladığımız kapama programı çalıştırıldığında üretilen değeri kullanacak `if` bloğunun kodunu da değiştirebiliriz. Kapamaları, örnek 13-6’ da gösterilene benzer şekilde, tıpkı bir işlev çağırıyormuş gibi, kapatma tanımını tutan değişken adını verip, parantez içindede alacağı bağımsız değişkenleri belirtirek çağırabiliyoruz.

Dosya adı: src/main.rs
```Rust
fn generate_workout(intensity: u32, random_number: u32) { 
    let expensive_closure = |num| { 
        println!("yavaşça hesaplanıyor..."); 
        thread::sleep(Duration::from_secs(2)); 
        num 
    };


    if intensity < 25 { 
        println!( 
            "Bugün, {} şınav çek!", 
            expensive_closure(intensity) 
        ); 
        println!( 
            "Sonrasında {} mekik çek!", 
            expensive_closure(intensity)
        ); 
    } else { 
        if random_number == 3 { 
            println!(
                "Bugün bir mola ver! Sıvı tüketmeyi de ihmal etme!"); 
        } else { 
            println!(
                "Bugün, {} dakika koş!", 
                expensive_closure(intensity)
            ); 
        } 
    } 
}
````
Örnek 13-6: Tanımladığımız `expensive_closure` kapamasını çağırmak.

Şimdi, pahalı hesaplama işlevi sadece tek bir yerde çağrılıyor ve bu kodu, sadece gerçekten sonuçlara ihtiyacımız olan yerde işletmiş oluyoruz. 

Bununla birlikte bu defa da, ilk `if` bloğunda kapamayı iki kez çağırarak örnek 13-3'teki sorunlardan biriyle yeniden karşılaşıyor ve bu pahalı kodu iki kez çağırmakla, kullanıcının uzun zaman alan iki işlem boyunca beklemesine neden oluyoruz. Bu sorunu ilk `if` bloğu kapsamında, kapamayı çağıran ve elde ettiği sonucu tutan yerel bir değişken tanımlayarak çözümleyebiliriz. Ancak kapamalar bize başka bir çözüm sağlar. Bu çözüm hakkında konuşmaya başlamadan önce, neden kapama tanımında ek açıklamalar bulunmadığından ve kapalarla ilgili bazı özelliklerden bahsedelim.

### Kapamalarda tür çıkarımı ve ek açıklamalar
Kapamalar, `fn` işlevlerinin gerektirdiği gibi parametre türlerinde veya dönüş değerlerinde açıklama eklenmesini gerektirmez. İşlevler kullanıcılara açık bir arayüzün parçaları olduklarından tür ek açıklamaları gereklidir. İşlevin ne tür değerler kullandığı veya hangi türden değerler döndürdüğünün, tüm kullanıcılar tarafından açıkça anlaşılabilmesi için, bu arayüzü katı bir şekilde tanımlamak önemlidir. Ancak kapamalar böyle açık bir arayüzde kullanılmak yerine; değişkenlerde depolanmakta, isimsiz olarak kullanılmakta ve kütüphanemizin diğer kullanıcılarına gösterilmeden değerlendirilebilmektedir.

Kapamalar genellikle kısa ve herhangi bir keyfi senaryodan ziyade, sadece dar bir bağlamda geçerlidir. Bu sınırlı bağlamda derleyici, değişken türlerinin çıkarsanmasına benzer şekilde, kapama parametre ve dönüş türlerini güvenli bir şekilde çıkarsayabilmektedir.

Programcıların bu isimsiz ve küçük işlevlerdeki türlere açıklama eklemesi, derleyicinin zaten sahip olduğu bilgilerle oldukça can sıkıcı ve büyük ölçüde gereksiz olacaktır.

Değişkenlerde olduğu gibi, açıklığı ve netliği gerekli olandan daha ayrıntılı olma pahasına artırmak istiyorsak, kapamalara da tür ek açıklamaları ekleyebiliriz. Örnek 13-5'te tanımladığımız kapamanın, kullanacağı türler için ekleyeceğimiz açıklamaları örnek 13-7' deki kodda gösterilmektedir.

Dosya adı: src/main.rs
```Rust
    let expensive_closure = |num: u32| -> u32 { 
        println!("yavaşça hesaplanıyor..."); 
        thread::sleep(Duration::from_secs(2)); 
        num 
    };
````
Örnek 13-7: Kapama parametre ve dönüş değerlerine isteğe bağlı tür ek açıklamalarını eklemek.

Tür ek açıklamaları eklenildiğinde, kapama sözdizimi işlev sözdizimine benzemeye başlıyor. Aşağıda, parametresine 1 ekleyen bir işlev tanımı ile aynı davranışa sahip bir kapama sözdiziminin dikey karşılaştırması yer almaktadır. İlgili parçaları hizalamak için bazı alanlar ekledik. Bu örnekleme; boru kullanımı ve isteğe bağlı sözdizimi haricinde, kapama sözdizimi ile işlev söz diziminin birbirlerine nasıl benzediğini göstermektedir:

```Rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
````

Örneğin ilk satırında bir işlev tanımı, ikinci satırında giriş ve dönüş türleri açıklanan bir kapama tanımı yer almaktadır. Üçüncü satırda, tür açıklamaları kapama tanımından kaldırılırken, dördüncü satırda  isteğe bağlı olan parantezleri kaldırılmıştır. Hatırlayacağınız gibi bir kapama tanımlanırken, kapatma gövdesi yalnızca bir ifadeden oluştuğunda süslü parantezler kullanılmamaktaydı. Yukarıda sunduğumuz kapama ifadelerinin her biri, çağrıldığında aynı davranışı üretecek geçerli tanımlamalardır.

Kapama tanımlarında, her bir parametre ve dönüş değeri için somut bir tür hesaplanır. Örnek 13-8, yalnızca parametre olarak aldığı değeri döndüren kısa bir kapama tanımını göstermektedir. Bu kapama, sadece bu örneği sunabilmek için tasarlandığında gerçek kullanımda pek yararlı değildir. Kapamayı ik kez çağırdığımızda, dizgiyi bir argüman olarak geçirmeyi başarırız, ancak u32 türü kullanan ikinci denememiz bir hata ile sonuçlanacaktır. Tanıma herhangi bir tür ek açıklaması eklenmediğine dikkat ediniz.

Dosya adı: src/main.rs
```Rust
let example_closure = |x| x; 
let s = example_closure(String::from("hello")); 
let n = example_closure(5);
````
Örnek 13-8: Girdi ve çıktı değerlerinin, iki farklı tür üzerinden çıkarsanması beklenen bir kapama örneği

Derleyici bize şu hatayı verir:
```Binary
error[E0308]: mismatched types
 --> src/main.rs
  |
  | let n = example_closure(5);
  |                         ^ expected struct `std::string::String`, found
  integer
  |
  = note: expected type `std::string::String`
             found type `{integer}`
````
İlk seferinde, yani `example_closure` kapaması dizgi değeri ile çağrıldığında, derleyici `x` parametresi ve dönüş türünü dizgi olarak algılar. Algılanan bu türler daha sonra `example_closure` içindeki kapamaya kilitlenir ve aynı kapama ile farklı bir tür kullanmaya çalışıldığında bir tür hatası ile karşılaşılır.

### Jenerik parametreler ve `Fn` özelliklerini kullanarak kapamaları hafızaya almak
Egzersiz planı üreten uygulamamıza geri dönecek olursak; örnek 13-6' daki kodumuz, halen pahalı hesaplama yapan kapama işlevimizi gerekenden daha çok çağırmaktaydı. Bu sorunu çözmenin bir yolu; maliyetli kapama sonucunu daha sonra yeniden kullanabilmek için  bir değişkene kaydetmek ve bu değişkeni, kapamayı tekrar çağırmak yerine, bu sonuca ihtiyacımız olan yerlerde kullanmaktır. Ancak, bu yöntem de çok sayıda kod tekrarı yapılmasını gerektirir. 

Neyse ki bizim için başka bir çözüm yolu mevcut. Kapamayı tutacak bir yapı tanımlayıp, kapama çağrıldığında sonuç değerini o yapı üzerinden oluşturabiliriz. Tanımlayacağımız bu yapı kapamayı yalnızca sonuç değerine ihtiyaç duyduğumuzda işleterek sonucunu önbelleğe alacak, böylelikle kodun geri kalanı sonucu kaydetmek ve yeniden kullanmak zorunda kalmamış olacaktır. Bu kalıbı daha önceki tecrübelerinizden, **tembel değerlendirmeler** veya **ezberleme** olarak biliyor olabilirsiniz.

Kapama tutabilen yapıların tanımlanabilmesi için, kapama işlevinin giririş ve dönüş türlerinin bildirilmesi gereklidir. Çünkü yapılar oluşturulurken sahip oldukları alanlar isimlendirildiklerinde türlerinin de bildirilmesi bir zorunluluktur. Her kapama örneğinin kendine özgü isimsiz türü olacağından, iki kapama aynı imzaya sahip olsalar bile, farklı türlerde oldukları kabul edilmektedir. Kapamaları kullanan **yapı**, **enum** veya **işlev parametreleri**ni tanımlarken Bölüm 10'da tartıştığımız **jenerikler ve özellik sınırlarını** da kullanabiliyoruz.

`Fn` özellikleriyse standart kütüphane tarafından sağlanmaktadır ve tüm kapama işlevleri; `Fn`, `FnMut` veya `FnOnce` özelliklerinden en az birini uygular. Bu özelliklerin arasındaki farkları [Kapanışlar ile Ortamı Yakalamak](https://github.com/rust-lang/book/blob/master/src/ch13-01-closures.md#capturing-the-environment-with-closures) bölümünde tartışacağız; bu örnek için, `Fn` özelliğini kullanmamızda sakınca yok.

Parametrelerin türlerini temsil etmek ve kapamaların bu özellik sınırıyla eşleşmesi gereken değerleri döndürmek için `Fn` özelliğine bağlı türler ekliyoruz. Bu durumda, kapamamızın `u32` türünde bir parametresi olduğundan ve bir `u32` türü döndüreceğinden belirttiğimiz özellik sınırı `Fn (u32) -> u32` olacaktır.

Örnek 13-9, bir kapama ve opsiyonel sonuç değeri tutan `Cacher` yapısının tanımını gösterir.

Dosya adı: src/main.rs
```Rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
````
Örnek 13-9: Bir `calculation` ve opsiyonel sonuç değerinden oluşan kapamayı tutan `Cacher` adlı yapının tanımlanması

`Cacher` yapısı, `T` türünde jenerik bir hesaplama alanına sahiptir. `T` üzerindeki özellik sınırları, bunun `Fn` özelliğini kullanmakta olan bir kapatma olduğunu belirtir. Yapının `calculation` adlı hesaplama alanında saklamak istediğimiz tüm kapamaların `u32` türünden bir parametresi *(Fn'den sonra parantez içinde belirtilir)* bulunmalı ve bu kapamadan bir `u32` türünde *(-> işaretinden sonra belirtilir)* değer döndürülmelidir.

```Binary
Not: İşlevler Fn özelliklerinin üçünü de uygulayabilir. Yapmak istediğimiz şey, ortamdan bir değer yakalamayı gerektirmiyorsa ve Fn özelliğini uygulayan bir şeye ihtiyacımız varsa kapatma yerine işlev kullanmayı tercih edebiliriz.
````

Yapının `value` adındaki alanı `Option<u32>` türündedir. Kapama işletilmeden önce bu alan `None` varyantını göstermektedir. Eğer `Cacher` yapısını kullanan bir program kapamanın sonucunu isterse, yapı içerisindeki kapamam işletilecek, bu defa da oluşan sonuç değeri `value` alanının `Some` varyantı içinde saklanacaktır. Eğer kapatmanın sonucu bu program tarafından yeniden talep edilirse, sonuç zaten depolanmış olduğundan kapama tekrar işletilmeyecek, bu yapının `Some` varyantında tutulan değer döndürülecektir.

Az önce tanımladığımız `value` alanının mantığı örnek 13-10' da gösterilmektedir.

Dosya adı: src/main.rs
```Rust
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
````
Örnek 13-10: `Cacher` yapısının önbellek mantığı

Bu yapıyı çağıracak olan kodun alanlardaki değerleri doğrudan değiştirmesini tercih etmek yerine, sadece yapı alanlarının değerleriyle ilgilenmesini istediğimizden alanlar dışarıdan erişime kapatarak özelleştiriyoruz. 

`Cacher::new` işlevi, `Cacher` yapısıyla aynı özelliğe bağlı olarak tanımladığımız jenerik `T` parametresi alır. Daha sonra kapama işlevini henüz gerçekleştirmemiş olduğundan `calculation` alanında belirtilen kapama değerini ve `value` alanındaki `None` değerinden oluşan bir `Cacher` örneği döndürür.

Böylelikle kodu çağıran taraf, kapama işleminden elde edilen sonuca ihtiyaç duyduğunda, kapamayı doğrudan çağırmak yerine `value` yöntemini çağırmış olacaktır. Bu yöntem, `Some` varyantında `self.value` cinsinden sonuç değerine sahip olup olmadığımızı kontrol eder. Bu değere sahipsek kapama bir daha işletilmez ve `Some` içinde depolanmakta olan değer döndürülür.

Ancak `self.value` değeri `None` olarak görünüyorsa, kod `self.calculation`'da depolanan kapamayı çağıracak, sonucu ileride kullanılmak üzere `self.value`'ye kaydedecek ve oluşan değeri çağıran tarafa döndürecektir.

Aşağıdaki örnek, `Cacher` yapısını örnek 13-6'da bulunan `create_workout` işlevinde nasıl kullanabileceğimizi göstermektedir.

Dosya adı: src/main.rs
```Rust
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("yavaşça hesaplanıyor...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Bugün, {} şınav çek!",
            expensive_result.value(intensity)
        );
        println!(
            "Sonrasında {} mekik çek!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Bugün bir mola ver! Sıvı tüketmeyi de ihmal etme!");
        } else {
            println!(
                "Bugün, {} dakika koş!",
                expensive_result.value(intensity)
            );
        }
    }
}
````
[Örnek 13-11:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b9feaef7d951bd0c68a15ba3368d3faf) Önbellek mantığını soyutlamak için generate_workout işlevinde Cacher yapısını kullanmak.

Böylelikle kapamayı doğrudan bir değişkene kaydetmek yerine, bu kapamayı tutması için yeni bir `Cacher` örneğini kaydediyoruz. Bu noktadan itibaren sonuca ihtiyacımız olan her yerde, `Cacher` yapısının bir örneğini oluşturup, `value` metodunu çağırırarak tembelce hesaplanan sonuca ulaşırız. Ayrıca pahalı hesaplama sonucunu döndüren `expensive_result` işlevi en fazla bir kez çağırılacağından `value` yöntemini çağırmak tercihimize kalmıştır.

Bu programı örnek 13-2' deki `main` işleviyle çalıştırmayı deneyin. Tüm `if` ve `else` bloklarında, `yavaşça hesaplanıyor...` çıktısının sadece bir kez ve gerektiğinde göründüğünü test edebilmeniz için `simulated_user_specified_value` ve `simulated_random_number` değişken değerlerini dilediğiniz kadar değiştirebilirsiniz. `Cacher` ön bellek yapısı, pahalı hesaplamayı ihtiyacımız kadar çağırarak `create_workout` iş mantığına rahatlıkla odaklanabilir.

### Cacher Uygulamasının Kısıtlamaları
