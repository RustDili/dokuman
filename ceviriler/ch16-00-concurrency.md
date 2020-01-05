# 16. Korkusuz Eş zamanlılık 
Eş zamanlı programlamayı verimli ve güvenli şekilde kullanabilmek Rust dilinin önemli hedeflerinden biridir. 
Bir programın farklı bölümlerinin bağımsız olarak yürütüldüğü eş zamanlı programlama ve bir programın farklı bölümlerinin aynı anda yürütüldüğü paralel programlama, gerek çok sayıda bilgisayarın, gerekse çok sayıda işlemcinin bir arada kullanılabildiği günümüzde giderek önem kazanmaktadır. 
Ancak bu önem programlama sürecini zorlu ve hataya oldukça açık bir duruma getirdiğinden bu sorunların üstesinden gelinmesi zorunludur.

Bu sorunla ilgilenen Rust ekibi ilk zamanlarda hafıza güvenliğini sağlamanın ve eş zamanlılık sorunlarını önlemenin farklı yöntemlerle çözülmesi gereken iki ayrı zorluk olduğunu düşünmekteydi. Ancak ekip zamanla, sahiplik kavramı ve tür sistemlerinin bellek güvenliği ve eş zamanlılık sorunlarını yönetmeye yardımcı olacak güçlü bir araç seti olduğunu keşfetti. Mülkiyet ve tür denetimi kavramları işin içine girdiğinde pekçok eş zamanlılık hatası, çalışma zamanı hatası yerine derleme zamanı hatalarına dönüşerek problemin çalışma zamanında değil, derleme zamanında giderilmesi kolaylaştı. 

Gelinen noktada programcının eş zamanlılık hatasını giderebilmek için hatanın oluştuğu koşulları defalarca oluşturarak zaman kaybetmesi yerine iş artık derleyiciye bırakılmıştır. Ve derleyici yanlış yazılan kodu derlemeyi reddederek sorunu açıklayan detaylı bir hata raporu sunar. Sonuçta programın üretime gönderildikten sonra ortaya çıkaracağı potansiyel sorunlarla boğuşmak yerine, hataların derleme zamanında ve halen üzerinde çalışılırken ortaya çıkarılıp düzeltilmesi sağlanmış olur. 

Bu bölüm programcının boşa zaman kaybetmekten korkmadığı bir geliştirme sürecine odaklanabileceğinden dolayı **Korkusuz eş zamanlılık** adı altında anılmaktadır. Ve "Korkusuz eş zamanlılık"; hassas hatalardan arındırılarak yeni hatalara geçit vermeyen haliyle kolay kod yazmaya ve düzenlemeye olanak tanır.

Bu kitap eş zamanlılık ve paralellizm hakkında değildir ve haliyle konunun daha rahat anlaşılabilmesi için sorunların çoğu eş zamanlılık üzerinden anlatılmaktadır. Bu nedenle bütün bir bölüm boyunca eş zamanlılık olarak ifade edilen her deyimin yerine paralellizmin de ifade edildiği varsayılmalıdır.

Diğer diller eş zamanlılık problemlerinin üstesinden gelebilmek amacıyla dogmatik çözümler sunar. Örneğin Erlang dili eş zamanlı süreçler arasında mesaj iletmek için zarif bir işleve sahipken iş parçalarını arasındaki durum bilgisine açıkça erişilebilecek bir yol bulundurmaz. Bunun bir nedeni, üst düzey dillerin gerekli soyutlamaları gerçekleştirilebilmek için bazı kontrollerden vazgeçilmesi gerekmesidir. Bu nedenle bir üst düzey dil için yalnızca olası çözümler alt kümesini desteklemek makul bir stratejidir. 
Bununla birlikte daha düşük seviyeli dillerin, herhangi bir durumda en iyi performansı sağlaması ve donanım üzerinde daha az soyutlama gerçekleştirmesi beklenir. Bu nedenle Rust, her ne şekilde olursa olsun karşılaşılan sorunları modellemek için duruma ve gereksinimlere uygun çeşitli araçlar sunar. 

* Bu bölümde ele alınacak konular aşağıdaki özetlenmektedir.
  * Aynı anda birden fazla kod parçasını çalıştırmak için iş parçası oluşturmak
  * Eşzamanlı mesaj iletimi ve iş parçaları arasında mesajları taşıyan kanallar
  * Birden fazla iş parçasının bir veri parçasına erişebildiği ortak durum eş zamanlılığı
  
Rust’in eşzamanlılık güvencesini genişleten Sync ve Send özellikleri, standart kütüphane tarafından sağlanan türlerin yanı sıra kullanıcı tanımlı türler için de garanti edilir.
