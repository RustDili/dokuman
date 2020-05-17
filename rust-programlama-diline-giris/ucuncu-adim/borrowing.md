# Borçlanma
Gerçek hayattaki uygulamalarda çoğu zaman, bir değerle bağlanmış değişkenleri işlevlere iletmek veya bunları diğer değişkenlerin bağlanması amacıyla o değişkene atamak zorunda kalıyoruz. Bir değerin mülkiyetini üstlenmeden işlem yapmak gerektiğinde, o değerin sahibi olan orijinal değişken bağlamına bir başvuru yapılır.

## Borçlanma Nedir?
> [Borçlanma](https://github.com/nikomatsakis/rust-tutorials-keynote/blob/master/Ownership%20and%20Borrowing.pdf) Bu kavram dilimizde: *Bir şeyi iade etme garantisi verilerek ödünç almak* anlamına gelir.

## Paylaşılan ve Değişken borçlanmalar
⭐️ Rust'ta iki tür borçlanmadan bahsedebiliriz:

1. **Paylaşılan Borçlanma** `(&T)`

   * Bir veri parçası **içeriği değiştirilmemek kaydıyla, bir ya da çok kullanıcı tarafından ödünç** alınabilir. 
   
2. **Değişken Borçlanma** `(&mut T)`

   * Bir veri parçası, **tek bir kullanıcı tarafından ödünç alınabilir ve değiştirilebilir**, ancak bu durumda o verilere başkaları erişemez ve kullanamaz.
   
## Borçlanma Kuralları
