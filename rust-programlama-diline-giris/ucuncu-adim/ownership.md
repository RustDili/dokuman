# Sahiplik ve mülkiyet kavramı
Aşağıdaki örneklerde, `a`'nın değerini `b`'ye atamaya çalışıyoruz. Her iki kod bloğu da hemen hemen aynı kodlardan oluşuyor gibi görünse de ayrı veri türleriyle işlem yapılmaktadır. İlk kod bloğu hatasız derlenip çalıştırılabiliyorken, ikinci kod bloğu mülkiyet sorunu yüzünden hata üretip çalışmayacaktır.

Değerleri belleğin `stack` adlı bölgesinde depolanan değişkenlerle yapılan bir işlem:

```Rust
fn main() {
    let a = [1, 2, 3];
    let b = a;
    println!("a: {:?}, b: {:?}", a, b);
    // a: [1, 2, 3], b: [1, 2, 3]
}
````
Değerleri belleğin `heap` adlı bölgesinde depolanan değişkenlerle yapılan diğer işlem:

```Rust
fn main() {
    let a: Vec<u8> = vec![1,2,3];
    let b = a;
    println!("a: {:?}, b: {:?}", a, b);
}
// error[E0382]: borrow of moved value: `a`
````
## Mülkiyet nedir?
⭐️ Değişkenler bir değer ile bağlandıklarında, bağlandıkları değerin mülkiyetlerini de almış olurlar. Bununla birlikte bir veri parçasının aynı anda sadece bir sahibi olabilir. Bir değerin mülkiyetiyle bağlanmış bir değişken, kullanıldığı kapsamın dışına çıktığında onun için ayrılmış olan sistem kaynakları serbest bırakılarak sisteme iade edilir. Bu süreç otomatik olarak tür güvenliği sağlamaya yardımcı olur.

> [Mülkiyet](https://github.com/nikomatsakis/rust-tutorials-keynote/blob/master/Ownership%20and%20Borrowing.pdf)
Bir şeye sahip olma eylemi, durumu veya hakkı olarak dilimize çevrilebilir.

## Kopyalanan ve taşınan türler
