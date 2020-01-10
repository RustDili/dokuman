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

Öncelikle `generate_workout` işlevinin beklenen davranışı, kullanıcının düşük yoğunluklu bir egzersizi mi (25'ten az bir sayı ile gösterilir) yoksa yüksek yoğunluklu bir egzersizi mi (25 veya daha büyük bir sayı) isteyip istemediğini kontrol etmektir.

Düşük yoğunluklu egzersiz planları, simüle ettiğimiz karmaşık algoritmaya dayanan bir dizi şınav ve mekik antremanını önerecektir. 

Kullanıcının yüksek yoğunluklu bir egzersiz planı istemesi halinde; işin içine biraz daha mantık girecek ve: Uygulamanın oluşturduğu rastgele sayı 3 olduğunda kullanıcıya mola vererek sıvı tüketmesi önerilecek, diğer hallerdeyse algoritmanın belirlediği süre kadar koşması önerilecektir.   

Bu kod, şimdilik işverenimizin istediği şekilde çalışmaktadır. Ancak bir süre sonra, şirketimizin veri bilimi ekibinin, `simulated_expensive_calculation` işlevini çağırma yönteminde bir takım değişiklikler yapılması gerektiğine karar verdiğini varsayalım. Bu güncelleme senaryosunda değişikliklerin basit tutulabilmesi için `simulated_expensive_calculation` işlevini kendisine başka bir çağrı eklemeden, sadece bir kez çağırmak ve halihazırda kendisine yapılmakta olan gereksiz çağrıları da kesip atmak istiyoruz. Nihayetinde bu işlev maliyetli bir işlev olduğundan, gerekmedikçe çağrıda bulunmamak, gerekiyorsa da sadece bir kez çağrıda bulunmak istiyoruz. 

### İşlevleri kullanarak yeniden düzenlemek
