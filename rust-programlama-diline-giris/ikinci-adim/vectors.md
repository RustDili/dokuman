# Vektörler
Diziler aynı türden oluşan verileri liste halinde bir arada tutan sabit uzunluktaki değişmezlerdir. Onların bu değişmezliği `mut` anahtar sözcüğüyle tanımlanıyor olsalar bile boyutlarının değiştirlmesine yetmez. Dilimler ise dizinin belli bir bölümüne referans verdiklerinden boyutları dinamik olarak değişebilen müstakil veri yapıları değildirler. Rust'ta, aynı türden öğelere sahip ve boyutları değiştirilebilen listeler oluşturulabilmesini sağlayan veri yapılarına vektör denir.

`Vec<T>` söz dizimiyle ifade edilirler. Söz diziminde yer alan `T` depolanacak veri türünü temsil eder. Örneğin `i32` türünde bir vektör basitçe `Vec<i32>` olarak ifade edilir. 
Vektörler boyutları dinamik olarak değişebilen veri türleri olduklarından depoladıkları öğelere ait veriler, `heap` üzerinde kendisi için  özel olarak ayrılmış dinamik bir alanda tutulurlar.

### Boş bir vektör oluşturmak
Boş bir vektörü oluşturmak için türün `new()` işlevini ya da `vec!` makrosunu kullanırız.
```Rust
  let v: Vec<i32> = Vec::new()  //1. new() işlevi yardımıyla 
  let mv: Vec<i32> = vec![];    //2. vec! makrosu kullanarak
````

Boş vektörün içinde saklanacak tür derleyici tarafından bilinemeyeceğinden, oluşturulurken tür açıklaması eklenir. Öğeleri `mut` anahtar kelimesiyle değişebilir olarak tanımlanan vektör öğeleri kapsam içinde biliniyorsa tür bildirimi atlanabilir.
```Rust
let mut v = Vec::new();
v.push(1);
v.push(2);
println!("v: {:?}", v);
````

### Veri türü kullanarak vektör oluşturmak
Aksi belirtilmedikçe Rust, sayısal türler için `i32` türü kullanıldığını varsayacağından, aşağıdaki ifadeyle 32 bit işaretli tam sayı değerlerinden oluşan bir vektör tanımlanır.   
```Rust
let v1 = vec![1, 2, 3];     // (-/+) değerler -2, -1, 0, 1, 2, 3
println!("v1: {:?}", v1);
````

Vektörün türü istenildiğinde depoladığı herhangi bir elemanın türüne ayarlanabilir. Bu durumda vektörün bir arada tuttuğu tüm öğeler ayarlanan türden olurlar.   

```Rust
let v2 = vec![1, 2u32, 3];		//Vektörün veri türü 2. öğenin türü ile aynı olur 
println!("v2: {:?}", v2);
````

Tür bilgisi vektör ilklendirilirken de bildirilebilir.
```Rust
 let v3: Vec<char> = vec!['a', 'b', 'c']; 
 println!("v3: {:?}", v3);
 ````
 
 İstenildiğinde tüm eleman değerleri ve tür bilgisi tek seferde bildirilebilir.
 ```Rust
 let v4 = vec![0; 10];      // 0 değerli 10 adet eleman
 println!("v4: {:?}", v4);
 ````
 
 Bir vektör yineleyici kullanılarak ilklendirilebilir. Bu çoğu durumda kullanışlı bir özelliktir.
 ```Rust
 let v5: Vec<i32> = (-5..5).collect();
 println!("Depolanan değerler: {:?}", v5);// [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4];
 ````
Vektörler index işlemlerini desteklerler. Değişebilir yapıda tanımlanan öğelere aritmetik işlem uygulamak mümkündür.
```Rust
fn main () {
 
   let mut v = vec![1, 2, 3];
   v[1] = v[1] + 5;
   
   println!("v: {:?}", v);  // v: [1, 7, 3] 
   
}
````

### Vektör öğelerine erişmek
Bir vektörün öğelerine index numarası ya da `.get()` işlevi yardımıyla erişilir. 
```Rust
fn main () {
    let v = vec![1, 2, 3, 4, 5];
    
    let ikinci: &i32 = &v[1];
    println!("Vektörün ikinci Öğesi: {}", ikinci);  // 2
        
    let ucuncu = v[2];
    println!("Vektörün üçüncü Öğesi: {}", ucuncu);  // 3
    
    match v.get(2) {
        Some(ucuncu) => println!("Üçüncü öğenin değeri: {}' dir.", ucuncu),
        None => println!("Üçüncü öğe bulunmuyor."),
    }
}
````

Dizilerde olduğu gibi beş öğeden oluşan bir vektörün altıncı elemanına index yoluyla erişmeye çalışmak, programın panik üreterek çökmesine neden olur. Ancak aralığın dışında kalan bir öğeye `.get()` işlevi kullanarak erişmeye çalışmak daha kullanıcı dostu olan `None` döndürülmesini sağlar.     
```Rust
// Olmayan öğeye index yoluyla erişmek 
println!("Vektörün sondan bir fazlası: {:?}", v[5]);  // Panic!

// Olmayan öğeye .get() işlevi ile erişmek 
let bu_oge_yok = v.get(100);
println!("Bu öğe var mı?: {:?}", bu_oge_yok);         // None
````
### .push() işlevi
Bir vektöre yeni öğe eklemek için `.pop()` işlevinden yararlanılır. Öğeler bu işlev kullanıldığında vektörün sonuna eklenirler.

```Rust
fn main () {
   let mut v = vec!['C','a', 'l', 'ı'];
   println!("v: {:?}", v);
   
   v.push('ş');
   println!("v: {:?}", v);
}
````

### .pop() işlevi
Aynı şekilde bir vektörün son elemanı `.pop()` işlevi kullanılarak çıkartılır. 
```Rust
fn main () {
 
   let mut v = vec!['C','a', 'l', 'ı', 'ş'];
   println!("v: {:?}", v);
   
   v.pop();
   v.pop();
   println!("v: {:?}", v);
   
}
````

Bir vektöre öğe eklemek ya da vektörden eleman çıkartabilmek için o vektörün öğelerinin `mut` anahtar sözcüğüyle değişebilir olarak tanımlanmış olması gerekmektedir. 
```Rust
fn main () {
 
   let v = vec![1, 2, 3];
   
   v.push(4);
   println!("v: {:?}", v);
   // cannot borrow `v` as mutable, as it is not declared as mutable
}
````
### Vektörün yaşam süresi 
Bir vektör yaşam süresi tanımlandığı kapsam boyuncadır. Kapsam dışına gelindiğinde vektörün yaşamı sona erer ve onun için ayrılan hafıza kaynakları boşaltılarak sisteme iade edilir. 

```Rust
fn main () {
 
   {
        let  v = vec![1, 2, 3];
        println!("v: {:?}", v); // v: [1, 2, 3]
        // işlemler
        
   } //<- v bu noktada kapsam dışına çıkar ve kaynakları serbest bırakılır 
 
}
````
