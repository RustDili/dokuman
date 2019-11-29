### Hafızanın Bölümleri: Stack ve Heap
Rust' un mülkiyet kavramını ele alış şekline girmeden önce, hafızanın `stack` ve `heap` olarak adlandırılan bölümlerinin ne olduğuna ve türlere ait değerlerin nerede depolandığına hızlıca göz atalım.

Aynı belleğin parçaları olmalarına rağmen `stack` ve `heap` farklı veri yapılarıyla temsil edilirler. Hafızada verilerin girildiği sırayla depolandığı ancak ters sırada kaldırıldığı (**lifo:**  last in first out) ve çok hızlı çalışan bölümüne **stack** adı verilirken, okuma ve yazma için daha fazla hesaplama gerektiren ve bir ağaç yapısını andıran bölümüne de **heap** adı verilir.

Hafızanın bu bölümlerinde neler olduğu ise üzerinde çalışılan verilerle ilgilidir. Rust’ ta tam sayılar, kayan noktalı sayılar, işaretçi tipleri gibi boyutları sabit ya da derleme zamanında bilinen türler, belleğin `stack` bölümünde depolanır. Derleme zamanında boyutları dinamik olarak değişebilen ya da hiç bilinmeyen türler ise silinirlerken özel bir temizlik çalışması gerektirdiğinden `heap` bölümüne konulur.
O nedenle önceki örnekte bulunan string nesnesinin kendisi; kapasite, uzunluk ve ara bellek işaretçi değerleri, değişmez ve sabit boyutta olduğundan `stack` üzerinde, sahip olduğu ham verileri için kendisine ayrılan ara bellek ise `heap` üzerinde depolanır. 

**C** gibi programlama dillerinin çoğunda hafıza işlemleri bu şekilde yürütülürken; **Rust**, heap bölümünde veri depolamaktan kaçınır ve mülkiyet kavramı konusunda ayrıntılarıyla değineceğimiz `Box` gibi belirli işaretçi türlerini kullanıma sokar.
