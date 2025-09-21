# BELAJAR RUST

Siapin otak kelean yak **ASU**

## Requirement

- sehat
- punya otak
- paham cara make komputer
- punya pengalaman bahasa pemograman lain
- dan yang paling penting **TIDAK GILA**

## Sejarah Rust

Rust pertama kali dibuat dengan proyek pribadi oleh karyawan mozila yang bernama **Graydon Hoare** di tahun 2006. Pada tahun 2009 mozila mulai mendukung proyek ini dan melakukn percobaan untuk membuat browser engine yang diberi nama **SERVO**, dan secara official diumumkan ke public sekitar 2010.

Rust dirilis secara public oleh mozila pada tahun 2015, dan sekarang rust banyak digunakan di perusahaan-perusahaan besar seperti Discord, Google, Meta, DropBox dll. Pada tahun 2022, Rust rust menjadi salahsatu bahasa pemograman yang didukung untuk pengembangan Linux Kernel, selain bahasa C dan Assambly.

## Kenapa Rust??

Rust memiliki memory safety, maksudnya rust memiliki sistem pemrosesan memory yang aman, tidak seperti C, C++, Java dll yang mana kita harus melakukan menejemen memory secara manual. Selain itu Rust juga merupakan bahasa pemograman yang high performace bahkan di beberpa hal kecepatan rust bisa hampir menyamai kecepatan bahasa C/C++.

Rust juga memiliki concurrency yang sangat safety dan sangat baik, ini sanat mendukung untuk pembuatan applikasi yang sangat high performace.

## Proses Pengembangan Applikasi Berbasis Rust

![image](images/runst.png)

## Installasi Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Untuk memperiksa apakah rust nya udah keinstall dengan bener kita bisa menjalankan perintah

```sh
rustup check
```

atau

```sh
rustc --version
```

dan untuk memngupdate versi rust nya, kita bisa menjalankan perintah

```sh
rustup update
```

## Membuat project rust

ketika kita menginstall rustup kita juga diberikan sebuah package manager yang bernama cargo. kita bisa memanfaatkan cargo untuk membuat project denga cara menjalankan command berikut.

```sh
cargo new nama_project
```

## File extension

Tiap tiap bahsa pemrogramman itu memiliki ektensi nya masing-masing, misalnya java ektensinya .java, C++ extensinya .cpp, golang ektensi nya .go, typescript extensinya .ts. Demikikian juga sama rust, extensi dari rust adalah .rs  
misalnya kita ingin membuat source code rust maka kita bisa membuat file dengan ektensi .rs.  
example: `main.rs`

## Hello Wolrd

```rs
fn main(){
    println!("Hello, world!");
}
```

Bahasa pemrogramman Rust itu sama seperti bahasa pemograman C++, Java dll yang mana membutuhkan main function. Main function merupakan gerbang utama atau kode utama yang akan dijalankan oleh Rust. Untuk membuat main function pada bahasa pemogramman Rust kita bisa menggunakan keyword `fn`.

## Print Function

Pada bahasa pemrogramman Rust jikalau kita ingin menampilkan sesuatu ke layar kita bisa menggunakan build in function misalnya seperti :

```rs
print!("Hello") // print ke console tampa melakukan enter
```

```rs
println!("Mas Ganteng") // menampilkan ke layar dengan melakukan enter
```

## Cargo

Ketika kita akan membangun Applikasi yang besar tentunya kita akan membutuhkan package manager untuk mengelola dependensi dari applikasi yang kita buat. Dengan menggunakan package manager kita bisa dengan mudah untuk mengelola dependensi, melakukan kompilasi, melakukan unit test dan sebagainya. Dengan adanya Dependensi manajemen ini kita sebagai developer dapat lebih produktif untuk mengembangkan applikasi

Sebelumnya kita telah menyinggung tentang **Cargo**. Pada bahasa pemogramman Rust kita bisa menggunakan Cargo sebagai Depensi manajemen kita.

## Membuat Distribusi File

Ketika kita sudah selesai develop sebuah applikasi tentunya kita akan membuat distribusi file untuk di upload ke server dan dijalankan di sisi server. Untuk melakukan hal tersebut pada bahasa pemogramman Rust kita bisa memanfaatkan perintah Cargo.

```sh
cargo build --release
```

Maka kita akan diberikan sebuah folder release didalam target. dan didalam folder release terdapat sebuah executable file yang kita bisa distribusikan ke server.  
 ![release](images/release.png)

## Unit Test

Dalam project Rust hanya boleh memiliki 1 main function. Hal tersebut tentunya bakal menjadi masalah kita kita baru belajar Rust karena tentunya kita akan membuat main function berkali kali untuk belajar dan bereksperimen.

Oleh karena itu alternatif yang bisa kita gunakan yaitu Unit test. Sama seperti bahasa pemogramman lain seperti Java, Golang, Typescript dan sebagainya, Rust juga memiliki Unit test yang bisa kita manfaatkan untuk bereksperimen atau belajar bahasa pemrogramman Rust.

Untuk membuat Unit test pada bahasa pemrogramman Rust caranya cukup sederhana kita bisa menggunakan arrtibute/annotation `#[test]` diatas kode fucntion.

```rs
#[test]
fn test_hello() {
    println!("Hallo Mas ganteng >_<")
}
```

Untuk menjalankan kode unit test kita diatas kita bisa jalankan perintah `cargo test nama_function -- --exact`

```sh
cargo test test_hello -- --exact
```

Untuk kedepanya kita akan menggunakan unit test untuk belajar Rust.

## Variable

Variable adalah tempat menyimpan suatu data. Untuk membuat variable pada bahasa pemogramman Rust kita bisa menggunakan keyword `let` dan diikuti dengan nama variable. Pada bahasaa pemogramman Rust ketika variable tersebut telah di isi data maka variable tersebut tidak dapat diubah(variable bersifat immutable)

```rs
#[test]
fn test_variable() {
    let nama = "mas mas ganteng >_<"; // variable tidak dapat diubah(bersifat immutable)
    println!("Hallo {}", nama);
}
```

## Mutable

Pada chapter variable kita telah mengetahui bahwa variabel itu bersifat immutabel atau tidak bisa diubah lagi, namun jikalau kita ingin mengubah variable tersebut bersifat muttable itu bisa. Pada bahasa pemrogramman Rust untuk membuat variable yang bersifat mutable kita bisa menggunakan keyword `let mut`

```rs
#[test]
fn test_mutable() {
    let mut nama = "Mas kim";
    println!("nama saya {}", nama); // Output => nama saya Mas kim
    nama = "Mas Nardji"; // reassign nilai variable
    println!("Nama saya {}", nama); // Output => nama saya Mas Nardji
}
```

## Static Typing

Rust adalah bahasa yang menganut _Static Typing_ artinya setiap kita membuat variable atau sebuah function jenis datanya harus fix dan tidak dapat berubah lagi.

Pada chapter sebelumnya kita telah membuat variable dengan tipe data string atau text, ketika variable tersebut kita reassign dengan tipe data number.

```rs
#[test]
fn test_static_typing() {
    let mut nama = "mas kim";
    println!("nama saya {}", nama);
    nama = 10; // ini akan error
    println!("nama saya {}", nama);
}
```

## Variable Shadowing

Pada bahasa pemrogramman Rust kita bisa membuat variable dengan nama yang sama. Namun ketika kita membuat variabel dengan nama yang sama maka variabel sebelumnya dengan nama yang sama akan menjadi variable shadowing atau menjadi tertutup.  
Praktik ini tidak begitu disarankan karena kedepanya dapat membingungkan kita ketika membaca baris kode.

```rs
#[test]
fn test_shadowning() {
    let name: &str = "Mas Kim"; // ini akan menjadi variable shadowing(tertutup tidak dapat di akses oleh baris paling bawah setelah variabel baru dengan nama sama di deklarasikan)
    println!("nama saya {}", name);

    let nama = 22;
    println!("Umur saya {}", nama);
}
```

## Tipe Data

Pada bahasa pemrogramman Rust tipe data secara garis besar terbagi menjadi 2 subset yaitu:

- **Skalar:** Merepresentasikan singgle value(nilai tunggal) misalnya seperti integer, float, boolean, char
- **Compound:** Merepresentasikan beberapa nilai dalam 1 tipe misalnya seperti Array, atau Tuple

| Scalar Type | Deskripsi                                         |
| ----------- | ------------------------------------------------- |
| Integer     | Tipe data anggka dalam bilangan bulat             |
| Float       | Tipe data angka dalam bilangan desimal(pecahan)   |
| Boolean     | Tipe data yang hanya bernilai true atau false     |
| Char        | Tipe data karakter (huruf)                        |

| Compound Type | Deskripsi                                                           |
|--------------|---------------------------------------------------------------------|
| Array         | Kumpulan beberapa data(data collection) dengan tipe data yang sama  |
| Tuple         | Kumpulan beberap data(data collection) denga tipe data berbeda-beda |

## Explicit Type

Di pemogramman dengan bahasa Rust ketika kita memnbuat variable kita tidak perlu menyebutkan tipe datanya secara explicit, karena Rust dapat mendeteksi secara otomatis tipe daatanya berdasarkan value yang kita tugaskan/assign kedalam variable tersebut. Namun jikalau kita ingin menyebutkan tipe datanya kita bisa menambahkan tanda (:) setelah nama variable.

```rs
#[test]
fn test_tipe_data() {
    // secara explisit
    let umur = 22;

    // secara implisit
    let usia: i32 = 22;

    println!("Umur saya {}", umur);
    println!("Usia saya {}", usia);
}
```

## Integer

Berikut ini adalah tabel yang berisi jenis-jenis tipe bilangan bulat integer 

| Panjang | Signed | Range Signed                     | Unsigned | Range Unsigned 
|---------|--------|----------------------------------|----------|---------------------
| 8-bit   | i8     | -128 - 127                       | u8       | 0 - 255 
| 16-bit  | i16    | -32,768 - 32,767                 | u16      | 0 - 65,535 
| 32-bit  | i32    | -2,147,483,648 - 2,147,483,647   | u32      | 0 - 4,294,967,295 
| 64-bit  | i64    | -9,223,372,036,854,775,808 - 9,223,372,036,854,775,807 | u64 | 0 - 18,446,744,073,709,551,61 
| 128-bit | i128   | -170,141,183,460,469,231,731,687,303,715,884,105,728 - 170,141,183,460,469,231,731,687,303,715,884,105,727| u128 | 0 - 340,282,366,920,938,463,463,374,607,431,768,211,455

### Float

Berikut ini adalah jenis-jenis tipe data bilangan floating poin atau pecahan 
| Panjang  | Float 
|----------|---------- 
| 32-bit   | f32 
| 64-bit   | f64

### Usize

Usize adalah tipe data integer yang panjang bit nya megikuti dari platform Sistem Operasi yang digunakan, misalnya jikalau kita menggunakan Linux dengan 64-bit maka unize nya akan menjadi 64-bit.

| Usize           | Keterangan    |
|-----------------|---------------|
| isize           | 32-bit/64-bit |
| usize(Unsigned) | 32-bit/64-bit |

## Defaut jenis Integer yang Digunakan

Ketika kita membuat variable dengan nilai number secara explisit(tidak menyebutkan jenis nya) maka Rust akan menggunakan defaut jenis number nya. jika bilangan bulat maka Rust akan menggunakan `i32` dan jikalau bilangan floating/pecahan maka akan menggunakan `f64`.

```rs
#[test]
fn test_number(){
    let angka = 10; // maka by defautl akan menggunakan jenis i32/32-bit
    let pecahan = 10.5; // maka by defautl akan menggunakan jenis f64/64-bit
    println!("angaka {}, pecahan {}", angka, pecahan);
}
```

## Number Conversion

Rust bisa melakukan konversi jenis tipe data number dari yang ukuranya kecil misalnya `i8` ke ukuran yang lebih besar misalnya `i16` begitupun sebaliknya. Namun perlu berhati-hati ketika melakukan konversi dari jenis yang besar misalnya `i16` ke jenis yang kecil misalnya `i8`, jikalau nilai dari `i16` itu lebih besar size nya dari pada jenis konversi nya misalnya `i8` maka akan terjadi error `Integer Overflow`.  
Untuk melakukan konversi kita bisa menggunaka keyword `as`;

```rs
#[test]
fn number_conversion() {
    let angka: i8 = 20;

    // melakukan konversi dari jenis i8 ke i16
    let angka2: i16 = angka as i16;

    println!("angaka {}", angka2);


    // melakukan konversi dari i16 ke i8
    let numer: i16 = 100;
    let number2: i8 = numer as i8;

    println!("number {}", number2);


    let bil: i32 = 500;
    let bil2: i8 = bil as i8; // ini akan terjadi Error Integer Overflow
}
```

## Operator Aritmatika

Berikut ini adalah tabel operator aritmatika 
| Operator | deskripsi 
|----------|------------------- 
| +        | penjumlahan 
| -        | pengurangan 
| *        | perkalian 
| /        | pembagian 
| %        | modulus/sisa bagi

```rs
#[test]
fn test_operator_aritmatika(){
    let a = 10;
    let b = 12;
    let hasil = a * b; // contoh perkalian
    println!("hasil {}", hasil)
}
```

## Operator komparasi

Berikut ini adalah operator komparasi yang dapat kita gunakan dalam bahasa pemrogramman Rust. 
| Operator | deskripsi 
|----------|----------- 
| >        | lebih dari 
| <        | kurang dari 
| =>       | lebih dari samadengan 
| <=       | kurang dari sama dengan 
| ==       | sama dengan

## Char

Char adalah tipe data karakter, untuk membuat tipe data char pada Rust kita bisa menggunakan tanda petik satu `''` dan diikuti dengan 1 karakter.

```rs
#[test]
fn test_char() {
    let a: char = 'p';
    let b: char = 's';
    println!("karakter a => {}", a);
    println!("karakter b => {}", b);
}
```

## Tuple

Tipe data tuple adalah tipe data kumpulan atau collection yang mana isinya lebih dari 1 tipe data biasanya. Jumlah data pada tuple sudah final atau fix tidak bisa bertambah atau berkurang. Misalnya kita membuat data dengan tipe data tuple dengan jumlah data 4 maka jumlah data tersebut tidak bisa dikurangi atau di tambah. Untuk membuat data dengan tipe data tuple kita bisa menggunakan simbol kurung `()` setelah itu kita isikan tipe data tipe data yang inigin kita gunakan didalam kurung.

```rs
#[test]
fn test_tuple() {
    let data: (i32, f32, char) = (1000, 2.7, 'a');
    println!("{:?}", data)
}
```

## Mengakses Tuple
Sebelumnya kita telah mempelajari bagaimana cara membuat data dengan tipe data tuple, namun bagaimana cara untuk mengakses tiap-tiap datanya??  
Untuk melakukan hal tersebut kita bisa melakukan degan cara menggunakan `.` setelah intu index dari tuple nya.

NOTE: Index tuple dimulai dari 0
```rs
#[test]
fn test_access_tuple() {
    let data: (char, i8, bool) = ('a', 10, false);

    let char1: char = data.0;
    let angka: i8 = data.1;
    let boolean: bool = data.2;

    println!("karakter {} angka {} boolean {}", char1, angka, boolean);
}
```

## Destucturing Tuple
Sebelumnya kita telah mengetahui cara mengakses data tuple. Ada satu cara lagi untuk mengakses data tuple tersebut, yaitu dengan cara destucturing.

``` rs
#[test]
fn test_desctucturing_tuple() {
    let data: (i32, char, bool) = (10, 's', true);
    let (angka, karakter, boolean) = data; // destructuring tuple
    println!("angak {} karakter {} boolean {}", angka, karakter, boolean)
}
```
## Muttable Tuple
Sebelumnya ketika kita membuat tipe data tuple seperti yang kita lakukan diatas, tipe data tuple tersebut bersifat mutable artinya tiap-tiap datanya tidak dapat diubah lagi atau sudah fix. Jikalau kita ingin tipe data tuple tersebut bersifat mutable atau isi data nya dapat diubah kembali kita bisa menggunakan keyword mut setelah keyword let.

```rs
#[test]
fn test_muttable_tuple() {
    let mut data: (i16, bool, char) = (10, true, 's');

    println!("angka {} boolean {} char {}", data.0, data.1, data.2);

    // reassigment data tuple, data tuple tidak imutable
    data.0 = 20;
    data.1 = false;
    data.2 = 'a';

    println!("angaka {} boolean {} char {}", data.0, data.1, data.2);
}
```

## Unit
Unit pada bahasa pemrogramman Rust adalah tipe data tupple kosong `()` artinya tuple tampa nilai apapun. Unit ini biasanya digunakan untuk function yang tidak memiliki return value
``` rs
// fungsi yang tidak memiliki return value
fn unit() {
    println!("Halo mas ambarawa");
}

#[test]
fn test_unit() {
    // ketika kita memanggil fungsi unit() maka fungsi unit() akan mengembalikan tuple kosong
    let result: () = unit();
    println!("{:?}", result);
}
```

## Array
Tipe data array termasuk tipe data collection, tipe data array merupakan tipe data yang berisi kumpulan data; mirip seperti tipe data tuple namun yang membedakan tipe data array dengan tuple adalah: pada tipe data tuple kita bisa menggunakan banyak tipe data untuk element-element nya jikalau pada array kita hanya boleh menggunakan satu tipe data saja.  
  
Untuk membuat tipe data array kita bisa menggunakan simbole `[]` dan didalam kurung siku memiliki 2 parameter:
- parameter pertama tipe data
- parameter kedua panjang element

``` rs
#[test]
fn test_array() {
    /**
     * [char, 5] => char merupakan 
     * */
    let array: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    println!("{:?}", array);
}
```

Untuk mengakses elemen array kita bisa mengaksesnya dengan menyebutkan index nya:
``` rs
#[test]
fn test_akses_index() {
    let array: [char; 5] = ['1', '2', '3', '4', '5'];
    let elemen1: char = array[0]; // mengakses elemen array dengan menggunakan INDEX dari array
    println!("elemen array 1 {}", elemen1);
}
```

## Mutable Array
Sebelumnya kita telah mempelajari mengenai tipe data array. By default saat kita membuat variabel dengan tipe data array maka tiap-tiap elemen nya tidak bisa dirubah kembali(bersifat immutable) jikalau kita ingin bisa mengubah elemen array nya maka kita harus mengubah variable tersebut menjadi `muttable`.

``` rs
#[test]
fn test_muttable_array() {
    // membuat array dengan tipe data number u8 dan bersifat mutable
    let mut array: [u8; 5] = [1, 2, 3, 4, 5];
    println!("array baru => {:?}", array);

    // mengubah nilai dari tiap tipa elemen array
    array[0] = 7;
    array[1] = 8;
    array[2] = 9;
    array[3] = 10;
    array[4] = 11;

    println!("array setelah dibuah ==>>{:?}", array);
}
```
Jika kita ingin mengetahui panjang atau banyaknya elemen array kita bisa menggunakan fungsi `len()` milik array.


``` rs

#[test]
fn test_muttable_array() {
    // membuat array dengan tipe data number u8 dan bersifat mutable
    let mut array: [u8; 5] = [1, 2, 3, 4, 5];
    // mengubah nilai dari tiap tipa elemen array
    array[0] = 7;
    array[1] = 8;
    array[2] = 9;
    array[3] = 10;
    array[4] = 11;
    /**
     * ketika kita memanggil fungsi len() maka len() akan mengembalikan angka
     * sesuai banyaknya elemen array dengan tipe data usize
     * */
    let length: usize = array.len();
    println!("panjang array {}", length);
}
```

## Array dua Dimensi
Array dua dimensi adalah tipe data array yang mana elemen nya merupakan array(array didalam array). hal tersebut biasanya sering sekali kita lakukan ketika kita mengembangkan sebuah applikasi.
``` rs
#[test]
fn array_dua_dimensi() {
    // array dua dimensi
    let array2d: [[i8; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    println!("Array 2 dimensi");
    println!("{:?}", array2d)
}
```

## Constant
Constant adalah sebuah variabel yang bersifat immutabel(tidak dapat diubah nilainya) yang dibuat dengan keyword `const`. `const` dan `let` merupakan keyword untuk membuat variabel namun yang membedakan keyword `const` dan `let` yaitu: 
- **const**
  - `const` tidak dapat diubah menjadi muttabel
  - nilai dari variable `cosnt` harus langsung dideklarasikan ketika variabel dibuat
- **let**
  - `let` bisa diubah menjadi muttabel dengan keyword `mut`
  - nilai variabel yang dibuat menggunakan `let` tidak harus langsung di deklarasikan ketika variabel dibuat.
  
Untuk membuat variabel dengan menggunakan keyword `const` kita harus menyebutkan tipe datanya secara explisit dan nama variabel harus menggunakan huruf kapital, jikalau nama variabelnya lebih dari 1 kata maka pemisah dari nama variabel tersebut menggunakan (_), atau simpelnya menggunakan format `snake case`

``` rs 
const MAXIMUM_VALUE: i32 = 500;
#[test]
fn test_constant() {
    const MINIMUM_VALUE: i32 = 5;
    println!("nilai minimum {}, nilai maksimum {}", MINIMUM_VALUE, MAXIMUM_VALUE)
}
```

## Gerbage Collection
*Gerbage Collection* adalah sebuah feature yang banyak digunakan oleh banyak bahasa pemrogramman misalnya seperti Go, Java untuk melakukan management memory. Cara kerja *Gerbage Collection* yaitu dengan cara memantau data yang sudah tidak diguanakan lagi di memory dan menghapus data tersebut secara otomatis.  
  
Namun ada juga bahasa pemrogramman yang tidak memiliki *gerbage collection* misalnya seperti C atau C++. Bahasa pemrogramman yang tidak memiliki *Gerbage collection* harus melakukan menejemen memory secara manual, jadi kita sebagai programmer kita harus mengalokasikan atau menghapus data di memory secara manual.  
  
Bahsa pemrogramman Rust memiliki pendekatan yang berbeda untuk menejemen memory. Rust tidak memiliki *gerbage collection* dan Rust tidak melakukan menejemen memory secara manual.

## Stack dan Heap
Rust menggunakan pendekatan *Stack* dan *Heap* unutk melakukan menejemen memory.  
**Stack:** adalah tempat penyimpanan data dalam struktur data stack(*first in last out*) atau tumpukan. Data yang disimpan dalam Stack ukuranya sudah fixed atau sudah pasti.

Contoh:
``` rs
#[test]
fn test(){
    // data umur ini akan disimpan pada Stack karena ukuranya fixed, yaitu i32
    let umur: i32 = 22;
    println!("Umur saya {}", umur);
}
```

**Heap:** Adalah tempat penyimpanan data yang ukuranya belum fixed atau belum pasti. Ketika terapat data yang berlum fixed. Rust akan melakukan Request kepada *Heap* dan  *Heap* akan meperintah Memory Allocator untuk mencari area kosong didalam memory dan menyimpan data tersebut didalam area tersebut; Setelah Memory Allocatory berhasil mentimpan data maka Kita akan diberikan Pointer(petunjuk/address) dimana data tersebut disimpan didalam memory, Dan setelah itu pointer tersebut disimpan di *Stack* karena pointer tersebut bersifat fixed.  

Contoh:

``` rs
fn fungsi_1() {
    // variabel umur akan disimpan di stack
    let umur: i32 = 22;

    /*
     * variabel nama ini akan disimpamn di heap karena tipe data String itu ukuranya tidak fixed
     * bisa saja kecil dan bisa saja besar tergantung banyaknya kareketer yang disimpan
     */
    let nama: String = String::from("Mas Kim");
    println!("Nama saya {}, umur saya {}", nama, umur);
}

fn fungsi_2() {
    let umur: i32 = 22;
    let nama: String = String::from("Mas Kim");
    println!("Nama saya {}, umur saya {}", nama, umur);
}


#[test]
fn test_heap_stack() {
    /*
     * kedua funcsi ini akan disimpan di stack
     */
    fungsi_1();
    fungsi_2();
}
```

Dibawah ini adalah diagram bagaimana kode diatas di proses oleh heap dan stack pada bahasa pemrogramman rust.  

![heap stack](/images/heap.png)


## &str dan String
Pada bahasa pemrogramman rust terapat 2 tipe data string:
- **&str(string slice):**
tipe data ini berisfat fix tidak bisa bertambah atau mengurang ukuranya, maka dari itu rust akan menyimpaan data dengan tipe &str pada stack

- **String:**
Sedangkan tipe String ini bersifat growable, artinya size nya bisa bertamabah atu berkurang, maka rust akan mentimpan data dengan tipe String pada memory Heep.


## Immutable &str
Karena ukuran dari &`str` itu fix size maka operasi yang kita dapat gunakan untuk tipe data `&str` adalah operasi immutable, maksudnya isi dari tipe data `&str` tidak dapat diubah.


```rust
#[test]
fn strnig() {
    /*
     * &str memiliki banyak operasi yang bisa digunakan, salah satunya adalah trim()
     * ketika kita melakukan trim() data pada variabel nama tidak akan berubah
     * karena &str itu bersifat immutable (tidak bisa diubah)
     * sehingga kita perlu menampung hasil dari trim() ke dalam variabel baru
     */
   let nama: &str = " God of War ";
   let new_name: &str = nama.trim();

    println!("nama 1 {}", nama);
    println!("nama 2 {}", new_name);
}
```


## Stirng type
Stirng pada rust merupakan tipe data text UTF-8, dan bisa berkembang ukuranya, namun ketika kita membuat string dengan immutable maka size nya akan fix namun data tersebut teap akan disimpan pada heap.

Untuk menambahkan atau menguragi dari data dengan tipe Stirng kita harus memastikan bahwa data tersebut bersifat mutable, dengan demikian data tersebut pada heep dapat berkembang atau berkurang.

`String` juga memilik banyak method yang dapat kita gunakan untuk detailnya bisa kunjungi disini https://doc.rust-lang.org/beta/std/string/struct.String.html

``` rust
#[test]
fn string_type() {
    /*
     * String itu bersifat mutable (bisa diubah)
     * sehingga ketika kita melakukan trim() data pada variabel nama akan berubah
     * dan tidak perlu menampungnya ke dalam variabel baru
     */
    let mut nama: String = String::from(" God of War ");
    nama = nama.trim().to_string(); // melakukan trim() dan mengubahnya ke tipe data String

    println!("game saya {}", nama);

    nama.push_str(" Ragnarok"); // menambahkan string ke variabel nama
    println!("game saya {}", nama);
}
```

## Ownership
Ownership adalah konsep inti dalam Rust yang mengatur bagaimana memori dialokasikan, digunakan, dan dibebaskan. Setiap nilai di Rust memiliki "pemilik"—yaitu variabel yang bertanggung jawab atas nilai tersebut. Ketika pemilik keluar dari scope, Rust secara otomatis membebaskan memori yang digunakan oleh nilai tersebut.

Prinsip utama ownership:

>Setiap nilaihanya punya satu pemilik pada satu waktu.
Ketika pemilik keluar dari scope, nilai akan di-drop (memori dibebaskan).
Ownership bisa dipindahkan (move), tapi tidak bisa diduplikasi secara bebas kecuali tipe tersebut mendukung copy.
Pada contoh kode di atas:

>Variabel a memiliki nilai 10 dan tetap hidup selama fungsi berjalan.
Variabel b hanya hidup di dalam blok { ... }. Setelah blok selesai, b keluar dari scope dan memori untuk b dibebaskan.
Jika mencoba mengakses b di luar scope-nya, akan terjadi error karena ownership b sudah berakhir.
Konsep ini membantu Rust mencegah bug seperti dangling pointer dan memory leak tanpa garbage collector.


## Data Copy
```rust
#[test]
fn orwnership() {
    let a:i32 = 10;
    
    {
        /*
         *b => berada pada scope yang berbeda dengan scope a, maka b tidak bisa di 
         * akses di luar scope ini
         */
        let b: i32 =  20;
        println!("nilai b {}", b);

    }

    println!("nilai a {}", a);
    // print!("nilai b {}", b); // ini akan terjadi error karena variabel b sudah
}
```


## Data Copy
Data copy merujuk pada proses menyalin nilai dari satu variabel ke variabel lain. Ada dua konsep utama terkait penyalinan data di Rust: copy dan move.

Penjelasan Data Copy di Rust:
- Copy terjadi ketika tipe data memiliki trait Copy. Artinya, saat Anda menyalin variabel, kedua variabel akan memiliki salinan data yang sama, dan keduanya tetap valid.

- Tipe data sederhana seperti integer (`i32`, `u8`), boolean (`bool`), dan tipe yang berukuran tetap biasanya memiliki trait Copy.

Contoh:
``` rs
let a = 5;
let b = a; // a dan b sama-sama bernilai 5, dan keduanya valid
```


- Move terjadi pada tipe data yang tidak memiliki trait `Copy`, seperti `String` atau `Vec<T>`. Saat Anda menyalin variabel, kepemilikan data berpindah ke variabel baru, dan variabel lama menjadi tidak valid.

Contoh:
``` rs
let s1 = String::from("hello");
let s2 = s1; // s1 tidak lagi valid, s2 memiliki data "hello"
```

```rust
#[test]
fn data_copy() {
    let a: i32 = 10;

    let b: i32 = a; // melakukan copy data dari a ke b

    println!("nilai a {}", a);
    println!("nilai b {}", b);
}

```


## Ownership movement
Ownership adalah konsep inti dalam Rust yang mengatur bagaimana memori dikelola secara aman tanpa garbage collector. Setiap nilai di Rust memiliki "owner" (pemilik), biasanya berupa variabel. Ketika sebuah nilai dipindahkan (move) ke variabel lain, kepemilikan berpindah, dan variabel lama tidak bisa lagi digunakan untuk mengakses nilai tersebut.

contoh:  

```rs
#[test ]
fn test_ownership_movement() {
    let a: String = String::from("Abdillah kim");
    let b: String = a; // melakukan move ownership dari a ke b
    println!("nilai b {}", b);
    // println!("nilai a {}", a); // ini akan terjadi error karena ownership a sudah di
}
```
Penjelasan langkah demi langkah:
- `let a: String = String::from("Abdilah kim");`
Variabel a menjadi pemilik dari string "Abdillah kim".
- `let b: String = a;`
Ownership dari nilai string dipindahkan ke b. Setelah ini, a tidak lagi valid untuk digunakan.
- Jika Anda mencoba mengakses a setelah ownership berpindah (println!("nilai a {}", a);), Rust akan memberikan error pada waktu kompilasi.


## Clone
Konsep `clone` di Rust
- `clone()` adalah metode yang digunakan untuk membuat salinan data dari sebuah variabel.
- Berbeda dengan copy sederhana, clone() melakukan deep copy, yaitu menyalin seluruh data yang dimiliki, bukan hanya referensinya.
```rust
#[test]
fn clone() {
    let a: String = String::from("Abdillah Kim");
    let b: String = a.clone(); // melakukan clone data dari a ke b
    println!("nilai a {}", a);
    println!("nilai b {}", b);
}
```
**Penjelaskan code**
```rs
let a: String = String::from("Abdillah Kim");
let b: String = a.clone(); // melakukan clone data dari a ke b
```
- `a` adalah sebuah `String` yang berisi `"Abdillah Kim"`.
- `b` dibuat dengan `a.clone()`, sehingga `b` memiliki salinan data yang sama persis dengan `a`, tetapi berada di memori yang berbeda.


## if expresion(branching)
Branching adalah konsep dalam pemrograman di mana alur eksekusi program dapat bercabang berdasarkan kondisi tertentu. Pada contoh kode Rust di atas, branching dilakukan menggunakan ekspresi `if`.

**Penjelasan code:**

```rs
#[test]
fn if_expression(){
    let nilai: i32 = 80;
    let hasil: &str;

    if nilai >= 75 {
        hasil = "Lulus";
    } else {
        hasil = "Tidak Lulus";
    }

    println!("Hasil ujian anda {}", hasil);
}
```

- `if nilai >= 75`: Mengecek apakah nilai lebih besar atau sama dengan 75.
- Jika kondisi benar, variabel hasil diisi dengan "Lulus".
- Jika kondisi salah, hasil diisi dengan "Tidak Lulus".

Branching digunakan untuk:

- Mengambil keputusan berdasarkan data.
- Menjalankan kode yang berbeda sesuai kondisi.

```rust
#[test]
fn if_let_expression(){
    let nilai: i32 = 100;
    let result: &str = if nilai >= 75 {
        "Lulus"
    }else if nilai >= 50 {
        "Remedial"
    } else {
        "Tidak Lulus"
    };
    println!("Hasil ujian anda {}", result);
}
```

## Loop expression
**Konsep Dasar:**
- `loop` adalah salah satu cara membuat perulangan di Rust yang akan berjalan terus-menerus sampai dihentikan secara eksplisit (biasanya dengan `break`).
- Pada contoh ini, `loop` digunakan untuk menaikkan nilai `counter` dari 0 hingga 10.

**contoh kode:**
``` rust
#[test]
fn loop_expression(){
    let mut counter: i32 = 0;

    loop {
        counter += 1;

        if counter == 10 {
            break;
        }else if counter % 2 != 0 {
            println!("nilai counter {}", counter);
            /**
             * continue disini akan mengulang ke awal loop
             * sehingga ketika counter bernilai genap
             * maka tidak akan di print
             */
            continue;
        }
    }
    println!("Hasil akhirnya adalah {}", counter);
}
```
**Langkah per langkah:**
- Inisialisasi
>Variabel counter di-set ke 0.

- `loop {...}`
>Memulai perulangan tanpa batas.
- `counter += 1;`
> Setiap iterasi, nilai counter bertambah 1.

- `if counter == 10 { break; }`
> Jika counter sudah 10, keluar dari loop.

- `else if counter % 2 != 0 {...}`
> Jika counter adalah bilangan ganjil, tampilkan nilainya dan lanjutkan ke iterasi berikutnya (continue).

- `println!("Hasil akhirnya adalah {}", counter);`
> Setelah loop selesai, cetak nilai akhir counter.

## Loop return value

Dalam Rust, sebuah loop dapat mengembalikan nilai menggunakan perintah break diikuti dengan ekspresi. Nilai yang diberikan pada break akan menjadi hasil dari loop tersebut. Ini berbeda dengan banyak bahasa lain, di mana loop biasanya tidak langsung mengembalikan nilai.  

**contoh kode:**
``` rust
#[test]
fn loop_return_value(){
    let mut counter: i32 = 0;
    let result: i32 = loop {
        if counter == 10{
            /*
             * break disini akan mengembalikan nilai counter
             * ke dalam variabel result
             * 
             * kita tidak perlu menuluskan keyword return
             * karena break sudah otomatis mengembalikan nilai
             * dari counter
             */
            break counter;
        }else {
            counter += 1;
            println!("menambah nilai counter {}", counter);

            /*
             * continue disini akan mengulang ke awal loop
             */
            continue;
        }
    };

    println!("Hasil akhirnya adalah {}", result);
}
```

- **loop**: Membuat loop tanpa kondisi akhir otomatis (infinite loop).
- **break counter**: Ketika kondisi terpenuhi (`counter == 10`), loop berhenti dan nilai counter dikembalikan sebagai hasil loop.
- **continue:** Melanjutkan ke iterasi berikutnya tanpa menjalankan kode setelahnya dalam loop.

## Label loop
Pada case tertentu kita sering kali membuat nested loop(looping didalam looping), dan pada kondisi tertentu jikalau kita ingin menghentikan loop terluar maka tidak bisa dilakukan.  
  
Dengan adanya feature `loop lable` kita bisa melakukan hal tersebut dengan menamai loop terluarnya.


```rust
#[test]
fn loop_lable(){
    let mut number = 1;
    // memberi nama loop terluar dengan nama outer
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                // menghentikan loop dengan label/nama outer
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;

            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}
```

## While
Konsep Dasar While Loop
while loop digunakan untuk menjalankan blok kode selama kondisi tertentu bernilai `true`.  
Sintaks:
- `while <kondisi> { /* blok kode */ }`
> Kondisi dicek sebelum setiap iterasi. Jika kondisi `false`, loop berhenti.
``` rust
#[test]
fn while_expression(){
    let mut counter: i32 = 0;
    while counter < 10 {
        counter += 1;

        if counter % 2 != 0 {
            println!("nilai ganjil {}", counter);
            counter += 1;
        }
    }
}
```
**Langkah-langkah Eksekusi:**
- Inisialisasi: `counter` mulai dari 0.
- Kondisi: Loop berjalan selama `counter < 10`.
- Iterasi:
 >- `counter` ditambah 1 setiap kali loop berjalan.
 >- Jika `counter` ganjil (`counter % 2 != 0`), cetak ke layar dan tambah lagi 1.
Ini membuat `counter` bisa bertambah 2 sekaligus jika ganjil.


## Array iteration
ketika kita menggunakan array kita sering kali menampilkan elemen array berdasarkan index nya. untuk melakukan hal tersebut kita bisa memanfaatkan `while`.
``` rust
#[test]
fn array_iteration(){
    let array: [&str; 5] = ["Mas Kim", "Mas Nardji", "Mas Ambarawa", "Mas Ganteng", "Mas Abdillah"];
    let mut index: usize = 0;
    while index < array.len() {
        println!("nama ke-{} adalah {}", index, array[index]);
        index += 1;
    }
}
```
## For loop
Sebelumnya kita telah mempelajar bagaimana cara mengambil elemen array menggunakan `while loop`. Sebenarnya ada cara yang lebih mudah dan praktis daripada menggunakan `while loop` yaitu dengan menggunakan `for loop`.  
```rust
#[test]
fn for_expression(){
    let array: [&str; 5] = ["Mas Kim", "Mas Nardji", "Mas Ambarawa", "Mas Ganteng", "Mas Abdillah"];

    for nama in array.iter() {
        println!("nama saya {}", nama);
    }
}
```
## Range Data Type 
Pada bahasa pemrogramman Rust, tipe data **Range** digunakan untuk merepresentasikan rentang nilai, biasanya dalam iterasi atau perulangan.

### Penjelasan Range
- **Sintaks:** `start..end`
- **Artinya:** Rentang dari `start` hingga sebelum `end` (eksklusif `end`).
- **Contoh:** `0..5` berarti 0, 1, 2, 3, 4.


```rust
#[test]
fn range(){
    /*
    * Tipe data Range di Rust digunakan untuk merepresentasikan rentang nilai, biasanya digunakan pada perulangan.
    * Contoh: 0..5 adalah Range dari 0 sampai 4 (eksklusif 5).
    * Range<usize> berarti rentang nilai bertipe usize.
    * Range sering digunakan pada for loop untuk mengakses indeks array atau melakukan iterasi sejumlah tertentu.
     */
    let range: Range<usize> = 0..5;
    let names: [&str; 5] = ["Mas Kim", "Mas Nardji", "Mas Ambarawa", "Mas Ganteng", "Mas Abdillah"];
    
    // ini artinya akan menampilkan elemen array mulai dari index 0 sampai sebelum 5(index ke 4)
    for index in range {
        println!("nama ke-{} adalah {}", index, names[index]);
    }
}
```

## Range inclusive

### Penjelasan RangeInclusive di Rust

Tipe data `RangeInclusive` pada Rust digunakan untuk merepresentasikan rentang nilai yang inklusif, yaitu nilai awal dan nilai akhir akan ikut diiterasi. Sintaks penulisan `0..=5` berarti rentang dari 0 sampai 5, termasuk angka 5. Tipe ini berguna ketika ingin melakukan perulangan yang mencakup batas atas, misalnya mengakses seluruh elemen array berdasarkan indeks dari 0 hingga panjang array - 1.

Contoh penggunaan:
```rust
#[test]
fn range_inclusive(){
    let range: std::ops::RangeInclusive<usize> = 0..=5;
    let names: [&str; 6] = ["Mas Kim", "Mas Nardji", "Mas Ambarawa", "Mas Ganteng", "Mas Abdillah", "Mas God of War"];
    for index in range {
        println!("nama ke-{} adalah {}", index, names[index]);
    }
}
```

## Function Parameter
```rust
fn fungsi_parameter(first_name: &str, last_name: &str) {
    println!("Nama lengkap saya {} {}", first_name, last_name);
}

#[test]
fn test_fungsi_parameter() {
    fungsi_parameter("Abdillah", "Kim");
}
```

## Funcition With Return Value
``` rust
fn factorial(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result: i32 = 1;

    for i in 1..=n {
        result *= i;
    }
    result
}

#[test]
fn fungsi_return_value() {
    let angka: i32 = 5;
    let result: i32 = factorial(angka);
    println!("Hasil faktorial dari {} adalah {}", angka, result);
}
```