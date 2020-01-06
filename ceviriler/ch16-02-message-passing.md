## Mesajlaşma yöntemiyle iş parçaları arasında veri aktarmak   
Güvenli eşzamanlılık sağlama konusunda gittikçe popülerleşen bir diğer yaklaşım ise, iş parçaları veya aktörlerinin birbirleriyle veri içeren mesajlar göndererek iletişim kurduğu mesajlaşma yöntemidir. Bu konuda [Go dili dökümanlarında](https://golang.org/doc/effective_go.html) şöyle bir ifade bulunmaktadır: "Belleği paylaşarak iletişim kurmak yerine, iletişim kurarak belleğin paylaşılması gereklidir."

Rust'un eş zamanlılığı gerçekleştirmek için kullandığı önemli araçlardan biri de, mesaj alıp vermek için kullandığı kanallardır. Kanallar, Rust’un standart kütüphanesinin uyguladığı bir programlama konseptidir. Kanalları anlamanın en basit yolu, tıpkı gerçek hayatta olduğu gibi onları; dere ya da nehirler gibi akmakta olan bir su kanalı olarak düşünmektir. Haliyle o akıntıya bir lastik ördek veya tekne gibi sürüklenebilecek bir şey bırakıldığında, o şey yolunun sonuna kadar akış yönünde hareket edecektir. 
Programlamada bahsedilen kanallar ise; biri alıcı, diğeri verici olmak üzere iki ayrı parçadan oluşurlar. Kanalın verici parçasını; lastik ördeğin nehre bırakıldığı yokuş olan kısım, alıcı parçasını ise; lastik ördeğin akıntıda sürüklenerek denize ulaştığı yani nehrin bittiği kısım olarak düşünmek gerekir. Yani sözün kısası yazılan kodun bir kısmı gönderilmek istenen veriler ve yöntemlerini içerirken, diğer kısmı yollanan bu mesajların gelip gelmediğini kontrol eder. Verici veya alıcı bölümlerin herhangi bir tanesi görevini tamamladığında o kanal da veri akışına kapanacaktır.

Bu bölümde bir değer üreterek, bu değeri bir kanala gönderen iş parçasını ve kanala verilen bu değeri alıp işleyerek çıkışa yazdıracak başka bir iş parçasını gösteren programlar örneklenecektir. Bu örneklerde, kanalın özelliklerini daha iyi anlatabilmek amacıyla, iş parçacıkları arasında basit değerler gönderilecek ve alınacaktır. Haliyle örneklenen teknikler anlaşıldıkça, kanalları kullanan sohbet sistemleri veya hesaplamalarını çok sayıda iş parçasının gerçekleştirdiği karmaşık programları tasarlamak kolaylaşacaktır. 

Aşağıdaki örnekte hiçbir şey yapmayan bir kanal oluşurulmaya çalışılmıştır. Ancak bu kanal üzerinden ne tür değerlerin gönderileceği derleyici tarafından bilinmiyor olduğundan program derlenemeyecektir.

Dosya: src/main.rs
```Rust
use std::sync::mpsc; 

fn main() { 
    let (tx, rx) = mpsc::channel(); 
}
````
Örnek 16.6- Bir kanal oluşturmak ve bu kanalı `tx` ve `rx` adlı iki parçaya atamak 

Yukarıdaki örnekte çoklu üretici, tekli tüketici anlamına gelen `mpsc::channel()` işlevini kullanarak yeni bir kanal oluştulmaktadır. Işleve adının baş harfleri `mpsc`' yi yani "çok sayıda üretici, tek bir tane tüketici" kavramını veren özellik, Rust standart kütüphanesinin kanalları uygulama şekli olup; Bir kanalda değer üretebilen birçok gönderme ucu olabileceğini, ancak bu değerleri alıp işleyebilecek sadece bir tane alıcı taraf bulunabileceğini ifade etmektedir. 

Bu durum; aynı anda büyük bir nehre doğru akan çok sayıda dere ve çayın olduğu, bu dere ve çayların herhangi birinin akıntısına bırakılan her şeyin nihayetinde bu nehre ulaşacağını düşünmekle eş değerdedir. Şimdilik buradaki örnekler tek bir üretici kısımla başlayacak ve daha sonra örneklere çok sayıda üreteci kısım eklenecektir. Geleneksel bir yaklaşımla belirlenen `tx` ve `rx` kısaltmaları, sırasıyla alıcı ve verici bölümlerini temsil amacıyla kullanılmakta, kısaltılmış bu isimler ayrıştırılan bir çokuzlu yardımıyla `let` ifadesinde bağlanmaktadır. Değişkenlerden bu şekilde yararlanmak, `mpsc::channel` tarafından döndürülen çokuzlunun parçalarını ayıklamak için uygun bir yaklaşımdır. 

Aşağıdaki örnek, gönderim ucunun yeni iş parçasına bağlanmasıyla aktarılan bir dizgiye odaklanmaktadır. Örnekteki yeni iş parçası, `tx.send()` işlevi üzerinden ana iş parçası ile iletişime geçecektir. Bu süreç tıpkı akan nehir suyuna bir lastik ördek bırakmak ya da bir iş parçacığından diğerine bir sohbet mesajı göndermek gibidir. 

Dosya: src/main.rs
```Rust
use std::thread; 
use std::sync::mpsc; 

fn main() { 
    let (tx, rx) = mpsc::channel(); 

    thread::spawn(move || { 
        let val = String::from("Merhaba"); 
        tx.send(val).unwrap(); 
    }); 
}
````
Örnek 16.7- Yeni iş parçasına `tx` ucu aracılığıyla "Merhaba" mesajını yollamak

Yukarıdaki örnekte de, yeni bir iş parçası oluşturabilmek için `thread::spawn()` işlevinden ve `tx`'i kapama işlevine gönderebilmek için `move` komutunundan yararlanıyoruz. Ancak bu yeni iş parçasının kanalı kullanarak mesaj yollayabilmesi için kanalın verici ucuna sahip olması gereklidir. 

Bu verici uç doğası gereği iletilecek verileri alarak bunları gönderebilecek özelliklere sahiptir. Dilimizdeki göndermenin kaeşılığı olan `send` adındaki bu özellik kullanıldığında `Result<T, E>` türünde biir değer döndürür. Bu `Result<T, E>` türü bir enum yapısı olup varyantlarından biriyle; eğer alıcı uç ile bağlantı kurulamıyor veya gönderilecek veriler için alıcı tarafında yeteri kadar yer bulunmuyorsa gönderme işlemi sonucunda bir hata döndürülür. 

Aşağıdaki örnekte `main` işlevinde bulunan kanalın alıcı ucundan elde edilen değeri işleyeceğiz. Bu süreç; suya bırakılan lastik ördeği, nehrin sonuna ulaştığında sudan almak, ya da bir sohbet programında arkadaşınızdan "Merhaba" mesajı almaya benzer.

Dosya: src/main.rs
```Rust
use std::thread; 
use std::sync::mpsc; 

fn main() { 
    let (tx, rx) = mpsc::channel(); 
    thread::spawn(move || { 
        let val = String::from("Merhaba"); 
        tx.send(val).unwrap(); 
    }); 

    let received = rx.recv().unwrap(); 
    println!("Alınan Mesaj: {}", received); 
}
````
Örnek 16.8- Ana iş parçası üzerinden "Merhaba" mesajını almak ve yazdırmak

Bir kanalın alıcı ucunda `recv()` ve `try_recv()` adlarında olmak üzere oldukça kullanışlı iki metod bulunur. Yukarıdaki örnekte, kanaldan bir değer gönderilinceye kadar ana iş parçasının yürütülmesini engelleyerek bekletecek olan ve alıcı sözcüğünün kısaltılmış hali olan `recv()` metodunu kullanıyoruz. Bu işlev; kullanılmakta olan kanala bir değer gönderilmişse `Result<T, E>` şeklinde bir sonuç döndürür. Kanalın gönderen ucu kapandığında, `recv()` işlevi daha fazla değer gelmeyeceğini bildiren bir hata döndürür.

Diğer yöntem olan `try_rcv()` işleviyse, ana iş parçasını çalışmasını engellemez. Bunun yerine kanalda kullanılabilir bir mesaj bulunuyorsa `Ok` varyantından, bulunmuyorsa `Err` varyantından oluşan bir `Result<T, E>` türündeki değeri, oldukça hızlı bir biçimde geriye döndürür. Bu yöntem, kullanıldığı iş parçasının **diğer iş parçalarından mesajlar gelene kadar yapacakları başka işleri varsa** kullanışlıdır. Bir döngü yardımıyla `try_recv()` işlevinin çağırılarak kontrol edildiği ve alınan bir mesaj varsa bunun işlenerek sürece devam edildiği, herhangi bir mesaj alınmadıysa iş parçasının kendi işleriyle ilgilendiği bir örnek hayal edin. 

Verdiğimiz örnekte ana iş parçasının, diğer iş parçasından gelecek mesajları beklemekten başka yapacak bir işi olmadığından ve bu nedenle engellenmesinde bir mahsur bulunmadığından, tekniğin kolayca anlaşılabilmesi amacıyla `recv()` yöntemi kullanılmaktadır.

16.8 numaralı örneği çalıştırdığımızda, yeni iş parçasından elde edilen mesajın ana iş parçası tarafından "Alınan Mesaj: Merhaba" olarak bastırılan haliyle karşılaşacağız: **Mükemmel!**

### Kanallar ve mülkiyet aktarımı
Mülkiyet kuralları, güvenli ve eş zamanlı kod yazmanıza yardımcı oldukları için, mesaj göndermede hayati rol oynarlar. Rust programlarında mülkiyet kavramı üzerinde düşünmek eş zamanlı programlamada karşılaşılabilecek hataları önlemek açısından da oldukça yararlıdır. 
Yeni iş parçasında oluşturulan bir değerin, kanala gönderildikten sonra yazdırılmaya çalışılması kanallar ve mülkiyet ilişkisinin nasıl gerçekleştiğini ve olası sorunların üstesinden nasıl gelinebileceğini gösteren deneysel bir örnek olabilir.

Derleyicinin böyle bir kod tasarımının çalıştırılmasına neden izin vermediğini kavrayabilmek için aşağıda yer alan programı çalıştırmayı deneyin.

Dosya: src/main.rs
```Rust
use std::thread; 
use std::sync::mpsc; 

fn main() { 
    let (tx, rx) = mpsc::channel(); 

    thread::spawn(move || { 
        let val = String::from("Merhaba"); 
        tx.send(val).unwrap(); 
        println!("val değeri {}", val); 
    }); 

    let received = rx.recv().unwrap(); 
    println!("Alınan: {}", received); 
}
````
Örnek 16.9- Bir değeri kanala yolladıktan sonra kullanmaya çalışmak

Yukarıdaki örnekte `tx.send()` işlevi aracılığıyla kanala gönderilmiş olan bir değer yeniden kullanılmak isteniyor. Oysa bu değerin gönderildiği iş parçası tarafından değiştirilip değiştirilmediği bilinmeden kullanılmaya kalkışılması ve elbette ki derleyicinin buna izin vermesi oldukça kötü bir fikirdir. Bu durumda programın çalıştırılmasına izin verilmesi, olasılıkla diğer iş parçacığının gerçekleştirebilecekleri değişiklikler nedeniyle tutarsız süreçlere veya var olmayan veriler yüzünden hatalara yahut hiç beklenmeyen sonuçlara neden olabilir. Haliyle yukarıdaki kodu derlemeye kalkışmak, aşağıdaki gibi bir hata ile sonuçlanacaktır.

```Binary
error[E0382]: use of moved value: `val`
  --> src/main.rs:10:31
   |
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val değeri {}", val);
   |                               ^^^ value used here after move
   |
   = note: move occurs because `val` has type `std::string::String`, which does
not implement the `Copy` trait
````

Gönderme işlevi `tx.send()` kendisine bir parametre geçirildiğinde o parametreyi mülkiyetine almakta, ancak alınan mülkiyet `move` aracılığıyla alıcı tarafına aktarılmış olduğundan kodumuz derlenmez. Çünkü Rust’ın mülkiyet sistemi her şeyin yolunda olup olmadığını kontrol ettiğinden, gönderilmiş bir değeri daha sonra yanlışlıkla tekrar kullanmamızı engeller. İşte bu nedenle yukarıdaki program derleme zamanı hatasına neden olur.

### Çoklu veri gönderirken alıcıyı gözlemlemek
