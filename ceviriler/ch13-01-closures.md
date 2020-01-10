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
        println!("calculating slowly..."); 
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
        println!("calculating slowly..."); 
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
