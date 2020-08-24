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

Öncelikle `v1_iter` değişmezinin `mut` anahtar sözcüğüyle değişebilir hale dönüştürülmesi gerektiğine dikkat edin. Bir yineleyicide `next()` metodunun çağrılması, yineleyicinin dizide bulunduğu yeri izlemek için kullandığı iç konumu değiştirir. Başka bir deyişle, metodu çağıran kod yineleyiciyi tüketir veya kullanır. Her `next()` metodu çağrısı yineleyicide bir öğenin tüketilmesine neden olur. Oysa `v1_iter` değişkeni `for` döngüsü ile kullanıldığında, değişkenin mülkiyeti döngüye aktarıldığından, durumu perde arkasında değişebilir olarak değiştirilir ve böylelikle `v1_iter` değişmezinin değişebilir olarak dönüştürülmesine gerek duyulmaz.

Ayrıca `next` metodu çağrılarından aldığımız değerlerin vektördeki değerlerin değişmez referansları olduğunu ve `Iter` metodunun değişmez referanslar üzerinde bir yineleyici ürettiğini unutmayın. Eğer `v1` vektörünün mülkiyetini alarak, sahip olunan değerleri döndürecek bir yineleyici oluşturmak istiyorsanız, iter yerine `into_iter` metodunu çağırabilirsiniz. Benzer şekilde, değişebilir referanslar üzerinde yineleme yapmanız gerektiğinde, `iter` kullanmak yerine, `iter_mut` metodunu kullanabilirsiniz.

### Yineleyici Tüketen Metodlar
