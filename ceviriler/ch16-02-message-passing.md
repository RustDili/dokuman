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
