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
Boş vektörün içinde saklanacak tür derleyici tarafından bilinemeyeceğinden, oluşturulurken tür açıklaması eklenir. 

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
