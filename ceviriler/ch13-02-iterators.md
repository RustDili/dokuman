## Yineleyiciler ile bir dizi öğeyi işlemek
Bir koleksiyonun tüm elemanları tükenene kadar her bir elemanı üzerinde sırasıyla belirli işlemleri gerçekleştirmekten yineleyiciler sorumludurlar. Yineleyici kullandığınızda bütün bu işlemlerin her birini tekrar tekrar gerçekleştirmek zorunda kalmazsınız.

Rust'ta yineleyiciler tembel olduklarından, kendilerini tüketen yöntemler çağırılana kadar programlarınızı etkilemezler. Örneğin aşağıdaki kod; `Vec<T>` üzerinde tanımlanan `iter` yöntemini çağırarak `v1` vektöründeki öğeler üzerinde bir yineleyici oluşturur. Bu kod tek başına anlamlı bir şey gerçekleştirmez.

Dosya adı: src/main.rs
```Rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
````
[Örnek 13-13:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=52bd47f729da130719a194242eb2dbb2) Bir yineleyici oluşturmak

Bir yineleyici oluşturduktan sonra artık bunu çeşitli şekillerde kullanabiliriz. Bölüm 3'ün sonlarında yer alan örnek 3-5'te, her eleman üzerinde bir takım işlemleri gerçekleştirmek amacıyla `for` döngüsünün tüketimine verilen `iter` tanımlamasıyla yineleyici kullanmış ancak  yineleyicilere şu ana kadar derinlemesine odaklanamamıştık.

Aşağıdaki örnekte, yineleyicinin oluşturulması ile `for` döngüsünde kullanımı birbirinden bağımsız olarak sunulmaktadır. Kendi başına `v1_iter` değişkeninde saklanan yineleyicinin tanımlandığı satırda hiç bir işlem gerçekleşmezken, bu değişkene adapte edilmiş yineleyiciyi kullanan bir `for` döngüsü ile çağrıldığında, kendisine geçirilmiş olan her öğenin değerini ekrana yazdıran bir döngü yineleyicisi olarak kullanışlı hale gelir.

Dosya adı: src/main.rs
```Rust
fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    
    let iter = v1.iter();
    
    for i in iter {
        println!("Okunan: {}", i);
    }
}
````
[Örnek 13-14:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=329f8b44fc0e94dccaca707c0ca98574) Bir yineleyici `for` döngüsünde kullanmak

Standart kitaplıklarında yineleyici bulunmayan dillerde bu tarz bir işlev, olasılıkla dizinin ilk elemanıyla değerinin arttırıldığı bir sayaç değişkeniyle başlatılacak, dizinin sonuna erişilene kadar her eleman için değişken birer birer güncellenerek işletilecektir.

Oysa yineleyiciler bütün bu karmaşık sayım sürecindeki mantığı sizin için üstlenebilir, muhtemel kod tekrarlarını azaltarak potansiyel karışıklıkların üstesinden gelebilirler. Yineleyiciler sadece vektörler gibi indekslenebilir veri yapılarıyle değil, aynı mantığın uygulandığı pekçok farklı koleksiyon türüyle kullanılılırken de fazlasıyla esnektirler. Haydi yineleyicilerin bunu nasıl yaptığını birlikte inceleyelim.

### `Iterator` Özelliği ve `next` Metodu
Tüm yineleyiciler standart kitaplıkta tanımlanan `Iterator` adlı bir özelliği uygular. Özelliğin tanımı şuna benzer:

```Rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // Şu an için varsayılan uygulamaları gösterilmeyen metodlar
}
````

Bu tanımda `type Item` ve `Self::Item` gibi bu özelliklerle ilişkilendirilmiş türü bildiren yeni söz dizimleri kullanıldığına dikkat edin. İlişkili türlerden bölüm 19’da ayrıntılı olarak bahsedeceğimizden şimdilik bilmeniz gereken tek şey; bu kodun yineleyici özelliğini *(`Itarator trait`)* uygulayabilmek için bir öğe türü *(`Item type`)* tanımlanması gerektiği ve bu öğe türünün `next` metodunun dönüş türünde kullanıldığını belirtmesidir. Başka bir deyişle öğe türü yineleyiciden döndürülen tür olacaktır.

Yineleyici özelliği uygulayıcılara sadece bir metodu tanımlamak için ihtiyaç duyar. Tanımlanan `next` metodu yineleme devam ettiği sürece öğeleri `Some` ile sarmalayarak birer birer döndürürken, yineleme sona erdiğinde `None` döndürecektir.

Yineleyicideki `next` metodunu doğrudan kendimiz de çağırabiliriz. Örnek 13-15, `v1` vektöründen oluşturulan yineleyici üzerinde tekrarlanan çağrılardan `next` metoduna hangi değerlerin döndürüldüğünü göstermektedir.

Dosya adı: src/lib.rs
```Rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
````
[Örnek 13-15:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e482568e0feb193f65ef94da694624ee) Yineleyicideki `next` metodunu çağırmak

Öncelikle `v1_iter` değişmezinin `mut` anahtar sözcüğüyle değişebilir hale dönüştürülmesi gerektiğine dikkat edin. Bir yineleyicide `next` metodunun çağrılması, yineleyicinin dizide bulunduğu yeri izlemek için kullandığı iç konumu değiştirir. Başka bir deyişle, metodu çağıran kod yineleyiciyi tüketir veya kullanır. Her `next` metodu çağrısı yineleyicide bir öğenin tüketilmesine neden olur. Oysa `v1_iter` değişkeni `for` döngüsü ile kullanıldığında, değişkenin mülkiyeti döngüye aktarıldığından, durumu perde arkasında değişebilir olarak değiştirilir ve böylelikle `v1_iter` değişmezinin değişebilir olarak dönüştürülmesine gerek duyulmaz.

Ayrıca `next` metodu çağrılarından aldığımız değerlerin, vektördeki değerlerin değişmez referansları olduğunu ve `Iter` metodunun değişmez referanslar üzerinde bir yineleyici ürettiğini unutmayın. Eğer `v1` vektörünün mülkiyetini alarak, sahip olunan değerleri döndürecek bir yineleyici oluşturmak istiyorsanız, iter yerine `into_iter` metodunu çağırabilirsiniz. Benzer şekilde, değişebilir referanslar üzerinde yineleme yapmanız gerektiğinde, `iter` kullanmak yerine, `iter_mut` metodunu kullanabilirsiniz.

### Yineleyici Tüketen Metodlar

Standart kitaplık tarafından sağlanan `Iterator` özelliğinin varsayılan uygulaması, halihazırda bir dizi metoda sahip olduğundan, bu metodlar hakkındaki bilgilere `Iterator` özelliğinin API belgelerini inceleyerek ulaşabilirsiniz. Bu metodlardan bazıları tanımlarında bulunan `next` metodunu çağırdığından `Iterator` özelliğini uygularken bu metodu da uygulamanız gerekmektedir.

Bu metodunu çağıran işlevler ise çağrıları sırasında yineleyiciyi kullandıklarından onlara *tüketici adaptörleri* adı verilir. Yineleyicinin mülkiyetini alarak her öğe için `next()` metod çağırısını yineleyen ve bu esnada yineleyiciyi tüketen `sum` metodu buna iyi bir örnektir. Bu metod yineleme süresince her elemanı toplama ekleyer ve yineleme sona erdiğinde bu toplamı döndürür. Örnek 13-16 `sum` metodu kullanımını örnekleyen bir test içerir.

Dosya adı: src/lib.rs
```rust
#[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
````
[Örnek 13-16:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7ca2caae18e715846a7b3ed6fdbe42aa) Yineleyicideki tüm öğelerin toplamını alan `sum` metodunu çağırmak

`sum` metodu, kendisine yapılan çağrı ile `v1_iter` değişmezindeki yineleyicinin mülkiyetini üzerine alacağından bu değişkenin kullanılmasına izin verilmez.

### Diğer Yineleyicileri Üreten Metodlar

`Iterator` özelliğinde tanımlanan ve *yineleyici adaptörleri* adı verilen diğer yöntemler ise yineleyicileri farklı yineleyicilerle değiştirmenize olanak sağlarlar. Bu adaptörlere karmaşık eylemleri okunabilir şekilde gerçekleştirmek çok sayıda çağrı zincirleyebilirsiniz. Ancak tüm yineleyiciler tembel olduklarından, yineleyici adaptörlerine yapılan çağrılardan sonuç alabilmek için *tüketici adaptörleri*nden birini çağırmanız gerekir.

Örnek 13-17'de yeni bir yineleyici üretmek üzere her öğenin çağrıldığı bir kapama işlevine sahip yineleyici adaptörü olan `map` metodunun bir örneği gösterilir. Buradaki kapama işlevi vektördeki her öğe değerinin 1 arttırıldığı yeni bir yineleyici oluşturacaktır. Ancak bu kod bir uyarı vermektedir.

Dosya adı: src/main.rs

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
````
Örnek 13-17: Bir yineleyici adaptörü olan `map` metodunu yeni bir yineleyici oluşturmak üzere çağırmak

aldığımız uyarının çıktısı aşağıdaki gibidir:

```Binary

$ cargo run
   Compiling iterators v0.1.0 (file:///projects/iterators)
warning: unused `std::iter::Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: iterators are lazy and do nothing unless consumed

    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/iterators`

````
Örnek 13-17'de yer alan kod hiçbir şey yapmadığı gibi bildirdiğimiz kapama işlevi de hiçbir zaman çağrılmaz. Yineleyici adaptörleri tembel olduklarından, derleyicinin uyarısı bize yinelecinin tüketilmesi gerektiği uyarısını yapıyor.

Bu durumu düzeltmek ve yineleyiciyi kullanabilmek için Bölüm 12, Örnek 12-1'de yer alan ve `env::args` ile kullandığımız `collect` metodundan yararlanacağız. Bu metod yineleyiciyi tüketerek elde ettiği değerleri bir koleksiyon veri türüne depolar.

Örnek 13-18'de `map` metoduna yapılan çağrı vasıtasıyla yineleyici üzerinde yapılan yinelemeden döndürülen sonuçları bir vektörde topluyoruz. Sonuçların depolandığı bu vektör, orijinal vektördeki her öğenin değerine 1 eklenmiş sayılardan oluşmaktadır.

Dosya adı: src/main.rs

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
````
[Örnek 13-18:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=79ccf9899d5e99ad545a5016e2ab09f2) Yeni bir yineleyici oluşturmak üzere `map` yöntemini ve üretilen bu yineleyiciyi vektör oluştururken tüketen `collect` yöntemini çağırmak

`map` metodu bir kapama işlevi aldığından, her bir öğe için uygulamak istediğimiz herhangi bir işlemi belirtebiliriz. Bu örnek, `Iterator` özelliğinin sağladığı yineleme davranışını yeniden kullanırken, kapamaların bazı davranışları özelleştirmenize nasıl izin verdiğini gösteren harika bir örnektir.

### Ortamlarını Yakalayan Kapamalar Kullanmak

Artık yineleyicileri kullanıma sunduğumuza göre, bir yineleyici adaptörünü olan `filter` metodu kullanarak ortamlarını yakalayan kapamaların yaygın bir kullanımını gösterebiliriz. Bir yineleyicideki `filter` metodu, yineleyiciden aldığı her öğe karşılığında Boolean döndüren bir kapama işlevini kullanır. Kapama `true` döndürdüğünde, değer `filter` tarafından üretilen yineleyiciye dahil edilecek, `false` döndürdüğündeyse yineleyiciye dahil edilmeyecektir.

Örnek 13-19'da `Shoe` adlı yapı örneklerinden oluşan koleksiyon üzerinde yineleme yapmak üzere `shoe_size` değişkenini ortamından elde eden bir kapama işleviyle `filter` metodunu birlikte kullanıyor ve sadece belirtilen boyuttaki ayakkabıları döndürüyoruz.

Dosya adı: src/lib.rs

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("Spor ayakkabı"),
            },
            Shoe {
                size: 13,
                style: String::from("Sandalet"),
            },
            Shoe {
                size: 10,
                style: String::from("Bot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("Spor ayakkabı")
                },
                Shoe {
                    size: 10,
                    style: String::from("Bot")
                },
            ]
        );
    }
}

fn main() {}
````

[Örnek 13-19:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c1eb9d413086a693939c175a54a3fec2) `shoe_size` değerini ortamından yakalayan bir kapama ile `filter` metodunu birlikte kullanmak

`shoes_in_my_size` işlevi, parametre olarak bir ayakkabı vektörü ve bir ayakkabı numarasının mülkiyetini alarak sadece belirtilen ölçüdeki ayakkabıları içeren bir yeni vektör döndürür.

`shoes_in_my_size` işlevinin gövdesinde vektörün mülkiyetini alacak bir yineleyici oluşturmak üzere `into_iter` metodunu çağırıyor, sonra bu yineleyiciyi, kapamanın sadece `true` döndürdüğü öğelerden oluşan yeni bir yineleyiciye uyarlayamak amacıyla `filter` metodunu kullanıyoruz.

Ortamdan `shoe_size` parametresini yakalayan kapama, bu değeri her bir ayakkabının numarasıyla karşılaştırarak yalnızca belirtilen ölçüdeki ayakkabıları tutar. Son olarak, `collect` çağrısı, uyarlanmış yineleyici tarafından döndürülen değerleri işlev tarafından döndürülen bir vektöre depolar.

Örneğimizdeki test, `shoes_in_my_size` işlevini çağırdığımızda, yalnızca belirttiğimiz ölçülere uygun ayakkabıların döndürüldüğünü göstermektedir.

### `Iterator` Özelliği ile Kendi Yineleyicilerimizi Oluşturmak

Bir vektör üzerinde `iter`, `into_iter` veya `iter_mut` metodlarını çağırarak bir yineleyici oluşturabileceğinizi gösterdik. Tıpkı bu vektör için oluşturduğumuz yineleyici gibi, standart kütüphanedeki eşleme haritaları veya diğer koleksiyon türleri için de yineleyiciler hazırlayabilir, `Iterator` özelliğini kendi türlerinize uygulayarak dilediğiniz işlemleri gefçekleştiren yineleyiciler oluşturabilirsiniz. Daha önce de belirtildiği gibi, tanımlamanız gereken tek metod `next` metodu olduğundan, bu metodu tanımladığınızda, `Iterator` özelliği tarafından sağlanan varsayılan uygulamalara sahip metodların tümünü kullanabilirsiniz!

Bu bilgiyi pekiştirmek adına, sadece 1'den 5'e kadar sayacak bir yineleyici oluşturalım. Öncelikle bunun için, bazı değerleri tutan bir yapı oluşturacak, ardından bu yapıya `Iterator` özelliğini uygulayacak ve bu uygulamadaki değerler vasıtasıyla bu yapıyı bir yineleyici haline getireceğiz.


Örnek 13-20, `Counter` yapısının tanımını ve bu yapının örneklerini oluşturmak amacıyla ilişkilendirilmiş `new()` işlevini göstermektedir:

Dosya adı: src/lib.rs

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

fn main() {}
````

[Örnek 13-20:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1b9ac102ba21df313b03e48ca1fd2b6b) Örnek 13-20: `Counter` yapısı ve `count` alanı başlangıç değeri 0 olan yapı örnekleri başlatan `new` işlevini tanımlamak

`Counter` yapısının `count` adlı bir alanı vardır. Bu alan, 1'den 5'e kadar olan yineleme sürecinde nerede olduğumuzu takip edecek `u32` türünden bir değer tuttuğundan ve `count` uygulamasının değerini yöneteceğinden özeldir. `new` işlevi ise her yeni örnek başlatıldığında, başlatılan bu örnekleri `count` alanı sayesinde daima 0 değeriyle ilklendirmeye çalışır.

Daha sonra, Örnek 13-21'de gösterildiği gibi, bu yineleyici kullanıldığında üstleneceği görevleri bildiren `next` metodunun gövdesini tanımlayarak `Counter` türüne `Iterator` özelliğini uygulayacağız:

Dosya adı: src/lib.rs

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
````

[Örnek 13-21:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=bd3d6be3e4520010b3681a7811ff166f) `Counter` yapısına `Iterator` özelliğini uygulamak

Yineleyicimiz için ilişkili öğe türünü `u32` olarak belirleyip `type Item = u32;` şeklinde ayarladığımızdan yineleyiciden `u32` türünde değerler döndürülecektir. Bu noktada *İlişkili Türler* konusunu Bölüm 19'da ele alacağımızı hatırlatarak konuya devam ediyoruz.

Yineleyicimizin mevcut duruma 1 eklemesini istediğimizden, 1'i döndürebilmesi için `count` u 0 olarak başlattık. `count` değeri 5'ten küçük olduğu sürece `next` metodu `count` değerini artırarak `Some` içine sarılmış geçerli değeri döndürecek, `count` değeri 5 olduğundaysa, yineleyicimiz `count` değerini artırmayı sona erdirerek `None` döndürmeye başlayacaktır.

#### `Counter` Yineleyicisinin `next` Metodunu Kullanmak

`Iterator` özelliğini uyguladığımıza göre artık elimizde bir yineleyicimiz var demektir. Tıpkı Örnek 13-15'te yaptığımız bir *vektörden oluşturulan yineleyici*de olduğu gibi, aşağıda yer alan Örnek 13-22 de, `Counter` yapısının yineleme işlevini `next` metodunu doğrudan çağırarak kullanabileceğimizi gösteren bir test bölümü içerir.

Dosya adı: src/lib.rs

```rust
#[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
````
[Örnek 13-22:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3441a6dcb227612dad4eb17d77cbaaef) `next` metodu uygulamasının işlevselliğini test etmek

Bu test, `counter` değişkeninde yeni bir `Counter` örneği oluşturur. Ardından `next` metodunu defalarca çağırıp bu yineleyicinin sahip olmasını istediğimiz davranışı uyguladığımızı doğrulayam 1'den 5'e kadar olan değerleri döndürür.

#### Diğer `Iterator` Özellik Metodlarını Kullanmak

Artık `next` metodunu tanımlayarak `Iterator` özelliğini uyguladığımıza ve hepsinin `next` metodunun işlevselliğini kullandıklarını bildiğimize göre, bundan böyle standart kitaplıkta tanımlanan tüm `Iterator` özellik metodlarının varsayılan uygulamalarını kullanabiliriz. 

Örnek 13-23'teki testte de gösterildiği gibi, bir `Counter` örneği tarafından üretilen değerleri almak istediğimizi, bunları ilk değeri atladıktan sonra başka bir `Counter` örneği tarafından üretilen değerlerle eşleştirdiğimizi, her çifti birbiriyle çarptığımızı ve elde edilen değerlerin sadece 3'e bölünebilenlerini alarak birbiriyle topladığımız bir örneği düşünün:   

<span class="filename">Dosya adı: src/lib.rs</span>

```rust
#[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
````
[Örnek 13-23:](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=90da8313eb87b9027217336831dcb19a) `Counter` yineleyicisinde farklı `Iterator` özellik metodlarını kullanmak

`zip` metodu, girdi yineleyicilerinden herhangi birisinin `None` döndürmesi halinde `None` varyantını döndüreceğinden, `zip` ifadesinin yalnızca dört çift oluşturabildiğini ve teorik olarak beşinci çift `(5, None)` olacağından, hiçbir zaman üretilmeyeceğini aklınızdan çıkarmayın.

Standart kitaplık `next` metodunu çağıran diğer yöntemler için varsayılan uygulamaları sağladığından, `next` metodunun nasıl çalıştığını belirledikten sonra metod çağrılarının tamamını kullanmamız mümkündür.
