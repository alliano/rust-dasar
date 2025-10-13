use std::ops::Range;

fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Mas ganteng")
}

#[test]
fn test_variable() {
    let nama = "Alliano";
    println!("Nama saya {}", nama);
}

#[test]
fn test_mutable() {
    let mut nama = "Mas kim";
    println!("nama saya {}", nama);

    nama = "Mas Nardji";

    println!("nama saya {}", nama);
}

#[test]
fn test_static_typing() {
    // let mut nama: &str = "Mas kim";
    // println!("nama saya {}", nama);
    // // nama = 10;
    // println!("nama saya {}", nama)
}

#[test]
fn test_shadowning() {
    let name: &str = "Mas Kim";
    println!("nama saya {}", name);

    let nama = 22;
    println!("Umur saya {}", nama);
}

#[test]
fn test_tipe_data() {
    let umur = 22;
    let usia: i32 = 22;

    println!("umur saya {}", umur);
    println!("usia saya {}", usia);
}

#[test]
fn test_number() {
    let angka = 10;
    let pecahan = 10.5;

    println!("angaka {}, pecahan {}", angka, pecahan);
}

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

    let bil: i32 = 1000000000;
    let bil2: i8 = bil as i8; // ini akan terjadi Error Integer Overflow

    println!("bilangan {}", bil2);
}

#[test]
fn test_operator_aritmatika() {
    let a = 10;
    let b = 12;
    let hasil = a * b; // contoh perkalian
    println!("hasil {}", hasil)
}

#[test]
fn test_comparation() {
    let hasil = 10 > 100;
    println!("hasil {}", hasil);
}

#[test]
fn test_char() {
    let a: char = 'p';
    let b: char = 's';
    println!("karakter a => {}", a);
    println!("karakter b => {}", b);
}

#[test]
fn test_tuple() {
    let data: (i32, f32, char) = (1000, 2.7, 'a');
    println!("{:?}", data)
}

#[test]
fn test_access_tuple() {
    let data: (char, i8, bool) = ('a', 10, false);

    let char1: char = data.0;
    let angka: i8 = data.1;
    let boolean: bool = data.2;

    println!("karakter {} angka {} boolean {}", char1, angka, boolean);
}

#[test]
fn test_desctucturing_tuple() {
    let data: (i32, char, bool) = (10, 's', true);
    let (angka, karakter, boolean) = data; // destructuring tuple
    println!("angak {} karakter {} boolean {}", angka, karakter, boolean)
}

#[test]
fn test_muttable_tuple() {
    let mut data: (i16, bool, char) = (10, true, 's');

    println!("angka {} boolean {} char {}", data.0, data.1, data.2);

    // reassigment data tuple
    data.0 = 20;
    data.1 = false;
    data.2 = 'a';

    println!("angaka {} boolean {} char {}", data.0, data.1, data.2);
}

#[allow(dead_code)]
fn unit() {
    println!("Halo mas ambarawa");
}

#[test]
fn test_unit() {
    let result: () = unit();
    println!("{:?}", result);
}

#[test]
fn test_array() {
    let array: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    println!("{:?}", array);
}

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

#[test]
fn array_dua_dimensi() {
    // array dua dimensi
    let array2d: [[i8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    println!("Array 2 dimensi");
    println!("{:?}", array2d)
}

const MAXIMUM_VALUE: i32 = 500;
#[test]
fn test_constant() {
    const MINIMUM_VALUE: i32 = 5;

    println!(
        "nilai minimum {}, nilai maksimum {}",
        MINIMUM_VALUE, MAXIMUM_VALUE
    );
}

fn fungsi_1() {
    // variabel umur akan disimpan di stack
    let umur: i32 = 22;

    /*
     * variabel nama ini akan disimpamn di heap karena tipe data String itu ukuranya tidak fixed
     * bisa saja kecil dan bisa saja besar tergantung banyaknya kareketer yang disimpan
     */
    let nama: String = String::from("Abdillah Kim");
    println!("Nama saya {}, umur saya {}", nama, umur);
}

fn fungsi_2() {
    let umur: i32 = 22;
    let nama: String = String::from("Ghroot");
    println!("Nama saya {}, umur saya {}", nama, umur);
}

#[test]
fn test_heap_stack() {
    /*
     * kedua fungsi ini akan disimpan di stack, namun tipe data String pada fungsi_1 dan fungsi_2
     * akan disimpan di heap
     */
    fungsi_1();
    fungsi_2();
}

#[test]
fn test_str() {
    let nama: &str = " Abdillah ";
    let new_name = nama.trim();
    println!("nama saya {}", new_name);
}

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

#[test]
fn orwnership() {
    let a: i32 = 10;

    {
        /*
         *b => berada pada scope yang berbeda dengan scope a, maka b tidak bisa di
         * akses di luar scope ini
         */
        let b: i32 = 20;
        println!("nilai b {}", b);
    }

    println!("nilai a {}", a);
    // print!("nilai b {}", b); // ini akan terjadi error karena variabel b sudah
}

#[test]
fn data_copy() {
    let a: i32 = 10;

    let b: i32 = a; // melakukan copy data dari a ke b

    println!("nilai a {}", a);
    println!("nilai b {}", b);
}

#[test]
fn test_ownership_movement() {
    let a: String = String::from("Mas Kim");
    let b: String = a; // melakukan move ownership dari a ke b
    println!("nilai b {}", b);
    // println!("nilai a {}", a); // ini akan terjadi error karena ownership a sudah di
}

#[test]
fn clone() {
    let a: String = String::from("Abdillah Kim");
    let b: String = a.clone(); // melakukan clone data dari a ke b
    println!("nilai a {}", a);
    println!("nilai b {}", b);
}

#[test]
fn if_expression() {
    let nilai: i32 = 80;
    let hasil: &str;

    if nilai >= 75 {
        hasil = "Lulus";
    } else {
        hasil = "Tidak Lulus";
    }

    println!("Hasil ujian anda {}", hasil);
}

#[test]
fn if_let_expression() {
    let nilai: i32 = 100;
    let result: &str = if nilai >= 75 {
        "Lulus"
    } else if nilai >= 50 {
        "Remedial"
    } else {
        "Tidak Lulus"
    };
    println!("Hasil ujian anda {}", result);
}

#[test]
fn loop_expression() {
    let mut counter: i32 = 0;

    loop {
        counter += 1;

        if counter == 10 {
            break;
        } else if counter % 2 != 0 {
            println!("nilai counter {}", counter);
            /*
             * continue disini akan mengulang ke awal loop
             * sehingga ketika counter bernilai genap
             * maka tidak akan di print
             */
            continue;
        }
    }
    println!("Hasil akhirnya adalah {}", counter);
}

#[test]
fn loop_return_value() {
    let mut counter: i32 = 0;
    let result: i32 = loop {
        if counter == 10 {
            /*
             * break disini akan mengembalikan nilai counter
             * ke dalam variabel result
             *
             * kita tidak perlu menuluskan keyword return
             * karena break sudah otomatis mengembalikan nilai
             * dari counter
             */
            break counter;
        } else {
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

#[test]
fn loop_lable() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
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

#[test]
fn while_expression() {
    let mut counter: i32 = 0;
    while counter < 10 {
        counter += 1;

        if counter % 2 != 0 {
            println!("nilai ganjil {}", counter);
            counter += 1;
        }
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = [
        "Mas Kim",
        "Mas Nardji",
        "Mas Ambarawa",
        "Mas Ganteng",
        "Mas Abdillah",
    ];
    let mut index: usize = 0;
    while index < array.len() {
        println!("nama ke-{} adalah {}", index, array[index]);
        index += 1;
    }
}

#[test]
fn for_expression() {
    let array: [&str; 5] = [
        "Mas Kim",
        "Mas Nardji",
        "Mas Ambarawa",
        "Mas Ganteng",
        "Mas Abdillah",
    ];

    for nama in array.iter() {
        println!("nama saya {}", nama);
    }
}

#[test]
fn rage_exclucive() {
    /*
     * Tipe data Range di Rust digunakan untuk merepresentasikan rentang nilai, biasanya digunakan pada perulangan.
     * Contoh: 0..5 adalah Range dari 0 sampai 4 (eksklusif 5).
     * Range<usize> berarti rentang nilai bertipe usize.
     * Range sering digunakan pada for loop untuk mengakses indeks array atau melakukan iterasi sejumlah tertentu.
     */
    let range: Range<usize> = 0..5;
    let names: [&str; 5] = [
        "Mas Kim",
        "Mas Nardji",
        "Mas Ambarawa",
        "Mas Ganteng",
        "Mas Abdillah",
    ];
    for index in range {
        println!("nama ke-{} adalah {}", index, names[index]);
    }
}

#[test]
fn range_inclusive() {
    /*
     * Tipe data RangeInclusive di Rust digunakan untuk merepresentasikan rentang nilai yang inklusif, biasanya digunakan pada perulangan.
     * Contoh: 0..=5 adalah Range dari 0 sampai 5 (inklusif 5).
     * RangeInclusive<usize> berarti rentang nilai bertipe usize.
     * RangeInclusive sering digunakan pada for loop untuk mengakses indeks array atau melakukan iterasi sejumlah tertentu.
     */
    let range: std::ops::RangeInclusive<usize> = 0..=5;
    let names: [&str; 6] = [
        "Mas Kim",
        "Mas Nardji",
        "Mas Ambarawa",
        "Mas Ganteng",
        "Mas Abdillah",
        "Mas God of War",
    ];
    for index in range {
        println!("nama ke-{} adalah {}", index, names[index]);
    }
}

fn fungsi_parameter(first_name: &str, last_name: &str) {
    println!("Nama lengkap saya {} {}", first_name, last_name);
}

#[test]
fn test_fungsi_parameter() {
    fungsi_parameter("Abdillah", "Kim");
}

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

fn print_text(text: &str, times: u8) {
    if times == 0 {
        return;
    } else {
        println!("{}", text);
    }
    print_text(text, times - 1);
}

#[test]
fn test_recursive_function() {
    print_text("Mas Kim", 5);
}

fn factorial_recursive(n: u8) -> u8 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let angka: u8 = 5;
    let result: u8 = factorial_recursive(angka);
    println!("Hasil faktorial dari {} adalah {}", angka, result);
}

fn print_number(number: i32) {
    println!("ini adalah angka {}", number);
}

fn print_str(nama: String) {
    println!("hallo nama saya {}", nama)
}

#[test]
fn function_ownership() {
    let number: i32 = 10;

    /*
     * i32 adalah tipe data yang fix, maka rust akan menyimpanan data tersebut pada stack
     * berhubung variable number bertipe i32. yang terjadi dibalik layar sebenarnya adalah, variabel number akan di copy ke parameter dari function print_number.
     * dan variabel tersebut karena
     */
    print_number(number);
    println!("selesai print data {}", number);

    /*
     * String adalah tipedata yang disimpan pada heep, maka yang terjadi dibalik layar adalah. Rust akan memindahkan
     * ownersip dari variabe nama ke parameter function print_str. setelah function tersebut selesai di eksekusi
     * maka data dari nama akan di hapus
     */
    let nama: String = String::from("Abdillah kim");
    print_str(nama);

    // println!("nama {}", nama); => ini akan error karena variable nama sudah dihapus dari proses/variabel nama sudah berpindah ownership nya(tidak dam scope ini lagi)
}

fn return_ownership(first_name: String, last_name: String) -> String {
    /*
     * Ownership dari retun value ini adalah variabel yang memanggil function ini
     */
    return format!("my full name is {} {}", first_name, last_name);
}

fn sum(arg1: i32, arg2: i32) -> i32 {
    /*
     * berhubung return value nya bertipe data fix(data yang disimpan pada stack)
     * maka return value nya merupakan hasil dari copy arg1 + arg2
     */
    return arg1 + arg2;
}

#[test]
fn return_value_ownership() {
    let first_name: String = String::from("Abdillah");
    let last_name: String = String::from("Kim");

    /*
     * variabel full_name akan menjadi owner dari return value dari function return_ownership
     */
    let full_name: String = return_ownership(first_name, last_name);
    println!("{}", full_name);

    // println!("{}",first_name); // first_name ini sudah tidak bisa diakses
    // println!("{}",last_name); // last_name juga tidak bisa diakses

    let number1: i32 = 100;
    let number2: i32 = 200;

    let result: i32 = sum(number1, number2);

    /*
     * variabel number1 dan number2 ownersip nya tidak akan berpindah. karena data tersebut
     * dicopy
     */
    println!(
        "hasil dari penjumlahan {} + {} adalah {}",
        number1, number2, result
    );
}

fn print_full_name(first_name: String, last_name: String) -> (String, String, String) {
    let full_name: String = format!("my full name is {} {}", first_name, last_name);
    return (first_name, last_name, full_name);
}

#[test]
fn test_tuple_owner() {
    let first_name: String = String::from("Abdillah");
    let last_name: String = String::from("Kim");
    /*
     * dengan begini kita bisa mengembaikan ownersip dari data first_name dan last_name
     * dengan teknik variabel shadowing menggunakan destructuring tuple
     */
    let (first_name, last_name, full_name) = print_full_name(first_name, last_name);

    println!("{}", full_name);
    println!("success fully {}", first_name);
    println!("success fully {}", last_name);
}

fn print_full_name_ref(first_name: &String, last_name: &String) -> String {
    return format!("my full name is {} {}", first_name, last_name);
}

#[test]
fn test_reference() {
    let first_name: String = String::from("Abdillah");
    let last_name: String = String::from("Kim");
    /*
     * dengan menggunakna &sebelum tipe data dari argumen nya. itu akan memberi
     * tahu rust bahwa owner dari data tersebut tidak dipindahkan melainkan hanya
     * memberi reference nya saja
     */
    let full_name: String = print_full_name_ref(&first_name, &last_name); // first_name dan last_name tidak berpindah ownership nya

    println!("{}", full_name);
    println!("success fully {}", first_name); // masih bissa di print
    println!("success fully {}", last_name); // masih bisa di print
}

fn add_str(name: &mut String) {
    // name.push_str("Kim") // ini akan error karena kita mencoba melakukan modifikasi data reference/boroowing
    println!("{}", name);
}

#[test]
fn terst_add_str() {
    let mut name: String = String::from("Abdillah");
    add_str(&mut name);
    name.push_str(" Kim"); // ini bisa kita lakukan karena variabel name merupakan owner dari scope ini(bukan borrowing/reference)
    println!("{}", name);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1: &[i32] = &array[..]; // membuat reverence dengan value reference dari variabel array
    let slice2: &[i32] = &array[0..5]; // membuat reference dari variabel array dari index 0 sampai 5
    let slice3: &[i32] = &array[8..]; // membuat reference dari variabel array dari index 8 sampai akhir

    println!("slice 1{:?}", slice1);
    println!("slice 2{:?}", slice2);
    println!("slice 3{:?}", slice3);
}

#[test]
fn string_slice() {
    let fullname: String = String::from("Guwathel");

    let str_ref1: &str = &fullname[..];
    let str_ref2: &str = &fullname[0..4];
    let str_ref3: &str = &fullname[3..];

    println!("str_ref1 {}", str_ref1);
    println!("str_ref2 {}", str_ref2);
    println!("str_ref3 {}", str_ref3);
}

#[test]
fn struct_test() {
    let person: Person = Person {
        first_name: String::from("Abdillah"),
        last_name: String::from("Kim"),
        age: 21,
        is_marige: false,
    };
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
    println!("{}", person.is_marige);
}

#[test]
fn struct_test_update_sintax() {
    let person: Person = Person {
        first_name: String::from("Abdillah"),
        last_name: String::from("Kim"),
        age: 21,
        is_marige: false,
    };

    // jika kita ingin membaut instance person lagi namun kita ingin isinya sama seprti instance person sebelumnya maka kita bisa
    // menggunakan update sintax struct(mirp kayak destructuring)
    let person_2: Person = Person { ..person };
    // namun jikalau kita melakukan hal tersebut perlu berhati2 karena untuk data yang disimpan di Heap makan akan berpindah ownership nya
    // jikalau gamau pindah ya harus di clone

    let person_3: Person = Person {
        first_name: person_2.first_name.clone(), // melakukan clone agar ownersip nya ga berubah
        last_name: person_2.first_name.clone(),
        ..person_2
    };

    println!("person 2 {}", person_2.first_name);
    println!("person 2 {}", person_2.last_name);
    println!("person 2 {}", person_2.age);
    println!("person 2 {}", person_2.is_marige);

    println!("person 3 {}", person_3.first_name);
    println!("person 3 {}", person_3.last_name);
    println!("person 3 {}", person_3.age);
    println!("person 3 {}", person_3.is_marige);
}

struct Person {
    first_name: String,
    last_name: String,
    age: u16,
    is_marige: bool,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("hello {} my name is {}", name, self.first_name);
    }
}
#[test]
fn method() {
    let person: Person = Person {
        first_name: String::from("Abdillah"),
        last_name: String::from("Kim"),
        age: 21,
        is_marige: false,
    };
    person.say_hello("Iszrail");
}

#[test]
fn tuple_struct() {
    let geopoint: GeoPoint = GeoPoint(1203.0231203, -0391.231324);
    println!("longnitude :{:?}", geopoint.0);
    println!("latitude: {:?}", geopoint.1);
}

struct GeoPoint(f64, f64);

struct Noting;

#[test]
fn test_noting() {
    let _noting: Noting = Noting;
}

struct GeoLocation(f64, f64);
impl GeoLocation {
    fn new(long: f64, lat: f64) -> GeoLocation {
        GeoLocation(long, lat)
    }
}

#[test]
fn associations_function() {
    let geolocation: GeoLocation = GeoLocation::new(0213.129321, -2392.1231243);
    println!("long: {}", geolocation.0);
    println!("lat: {}", geolocation.1);
}

enum Level {
    HARD,
    MEDIUM,
    EASY,
}

#[test]
fn enum_test() {
    let _level_1: Level = Level::EASY;
    let _level_2: Level = Level::MEDIUM;
    let _level_3: Level = Level::HARD;
}

enum Payment {
    Ewallet(String),
    BankTranfer(String, String),
    CreditCard(String, String),
}

impl Payment {
    fn pay(&self, amount: u64) {
        println!("success fully pay ${}", amount);
    }
}

#[test]
fn enum_data() {
    let _payment_method: Payment = Payment::BankTranfer(String::from("BRI"), String::from("03432832"));
    let _payment_method_2: Payment = Payment::CreditCard(String::from("ASD"), String::from("ASD"));
    let _payment_method_3: Payment = Payment::Ewallet(String::from("ASD"));
    _payment_method.pay(1000000);
}



#[test]
fn enum_matching() {
    let level: Level = Level::MEDIUM;

    match level {
        Level::EASY => {
            println!("User choose Easy level");
        },
        Level::HARD => {
            println!("User choose Hard level");
        },
        Level::MEDIUM => {
            println!("User choose Medium level");
        }
    }
}


impl Payment {
    fn pay_v2(&self, amount: u128) {
        match self {
            Payment::BankTranfer(bank, no_rek) => {
                println!("success fully paied ${} with bank {} {}", amount, bank, no_rek);
            },
            Payment::CreditCard(credit, no_credit) => {
                println!("success fully paid ${} with credit {} {}", amount, credit, no_credit);
            },
            Payment::Ewallet(number) => {
                println!("success fully paid ${} with ewallet {}", amount, number);
            }
        }
    }
}

#[test]
fn destructuring_enum_patterens() {
    let payment_method: Payment = Payment::BankTranfer(String::from("BRI"), String::from("009932434803248024"));
    payment_method.pay_v2(100000);
}


#[test]
fn test_else_matching() {
    let name: &str = "Abdillah Kim";
    match name {
        "Abdillah Kim" => {
            println!("1. my name is {}", name);
        },
        "Alliano" => {
            println!("2. my name is {}", name);
        },
        "Orang Kaya" => {
            println!("3. my name is {}", name);
        },
        other_name => {
            println!("4. other name {}", other_name);
        }
    }
}

#[test]
fn test_matching() {
    let name: &str = "Kim";
    match name {
        "Kim" | "Abdillah" => {
            println!("Hallo {} Mas mas kaya, Investor sukses", name);
        },
        other_name => {
            println!("Anda Orang miskin {}", other_name)
        }
    }
}

#[test]
fn matching_range() {
    let age:u128 = 22;

    match age {
        1..=15 => {
            println!("Boy")
        },
        16..=23 => {
            println!("Adult")
        },
        17..=70 => {
            println!("Man")
        },
        _other_age => {
            println!("Sepuh")
        }
    }
}

#[test]
fn tuple_struct_destructuring() {
    let geo_location: GeoLocation = GeoLocation::new(0.0, 1.00);
    match geo_location {
        GeoLocation(long, 0.0) => {
            println!("long {}", long);
        },
        GeoLocation(0.0, lat) => {
            println!("lat {}", lat);
        },
        GeoLocation(long, lat) => {
            println!("long {} lat {}", long, lat);
        }
    }
}

#[test]
fn descticturing_struct_matching() {
    let person: Person = Person {
        first_name: String::from("Abdillah"),
        last_name: String::from("Kim"),
        is_marige: false,
        age: 22
    };

    match person {
        Person {first_name, last_name, ..} => {
            println!("first name: {} \nlast name: {}", first_name, last_name);
        }
    }
}

#[test]
fn match_expression() {
    let age: u32 = 100;
    let result = match age {
        0..=15 => {
            "Boy"
        },
        16..=23 => {
            "Adult"
        },
        24..=70 => {
            "Man"
        },
        _ => {
            "Mayat"
        }
    };

    println!("{}", result);
}


type Age = u8;
type IdentityNumber = String;

struct Customer {
    identity_number: IdentityNumber,
    age: Age,
    name: String
}

#[test]
fn alias() {
    let custormer: Customer = Customer {
        age: 22,
        identity_number: String::from("1239123817238192"),
        name: String::from("Abdillah Kim")
    };

    match custormer {
        Customer {age, identity_number, name} => println!("age: {}\nname: {}\nidentity_number: {}\n", age, identity_number, name)
    }
}


mod model {
    pub struct User {
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub nick_name: String
    }

    impl User {
        pub fn say_hello(&self, name: &str) {
            println!("hello {} my name is {}", name, self.nick_name);
        }
    }
}

#[test]
fn module() {
    let user: model::User = model::User {
        first_name: String::from("Abdillah"),
        last_name: String::from("Kim"),
        age: 22,
        nick_name: String::from("Alliano")
    };

    user.say_hello("Jochcowi");
}




// mod foo {
//     pub fn say_hello(name: &str) {
//         println!("Hello {}", name);
//     }
// }

// mod bar {
//     pub fn say_hello(name: &str) {
//         println!("hello {}", name)
//     }
// }

mod foo;
mod bar;
mod foo_bar;

use foo::say_hello;
use bar::say_hello as say_hello_second;

#[test]
fn feature() {
    say_hello("Alliano");
    say_hello_second("Abdillah");
}

#[test]
fn create_keyword() {
    foo_bar::foo_bar("Alliano");
}