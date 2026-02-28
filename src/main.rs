

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
    let _payment_method: Payment =
        Payment::BankTranfer(String::from("BRI"), String::from("03432832"));
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
        }
        Level::HARD => {
            println!("User choose Hard level");
        }
        Level::MEDIUM => {
            println!("User choose Medium level");
        }
    }
}

impl Payment {
    fn pay_v2(&self, amount: u128) {
        match self {
            Payment::BankTranfer(bank, no_rek) => {
                println!(
                    "success fully paied ${} with bank {} {}",
                    amount, bank, no_rek
                );
            }
            Payment::CreditCard(credit, no_credit) => {
                println!(
                    "success fully paid ${} with credit {} {}",
                    amount, credit, no_credit
                );
            }
            Payment::Ewallet(number) => {
                println!("success fully paid ${} with ewallet {}", amount, number);
            }
        }
    }
}

#[test]
fn destructuring_enum_patterens() {
    let payment_method: Payment =
        Payment::BankTranfer(String::from("BRI"), String::from("009932434803248024"));
    payment_method.pay_v2(100000);
}

#[test]
fn test_else_matching() {
    let name: &str = "Abdillah Kim";
    match name {
        "Abdillah Kim" => {
            println!("1. my name is {}", name);
        }
        "Alliano" => {
            println!("2. my name is {}", name);
        }
        "Orang Kaya" => {
            println!("3. my name is {}", name);
        }
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
        }
        other_name => {
            println!("Anda Orang miskin {}", other_name)
        }
    }
}

#[test]
fn matching_range() {
    let age: u128 = 22;

    match age {
        1..=15 => {
            println!("Boy")
        }
        16..=23 => {
            println!("Adult")
        }
        17..=70 => {
            println!("Man")
        }
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
        }
        GeoLocation(0.0, lat) => {
            println!("lat {}", lat);
        }
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
        age: 22,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("first name: {} \nlast name: {}", first_name, last_name);
        }
    }
}

#[test]
fn match_expression() {
    let age: u32 = 100;
    let result = match age {
        0..=15 => "Boy",
        16..=23 => "Adult",
        24..=70 => "Man",
        _ => "Mayat",
    };

    println!("{}", result);
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    identity_number: IdentityNumber,
    age: Age,
    name: String,
}

#[test]
fn alias() {
    let custormer: Customer = Customer {
        age: 22,
        identity_number: String::from("1239123817238192"),
        name: String::from("Abdillah Kim"),
    };

    match custormer {
        Customer {
            age,
            identity_number,
            name,
        } => println!(
            "age: {}\nname: {}\nidentity_number: {}\n",
            age, identity_number, name
        ),
    }
}

mod model {
    pub struct User {
        pub first_name: String,
        pub last_name: String,
        pub age: u8,
        pub nick_name: String,
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
        nick_name: String::from("Alliano"),
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

mod bar;
mod foo;
mod foo_bar;

use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque}, ops::Range, sync::Arc};

use bar::say_hello as say_hello_second;
use foo::say_hello;

#[test]
fn feature() {
    say_hello("Alliano");
    say_hello_second("Abdillah");
}

#[test]
fn create_keyword() {
    foo_bar::foo_bar("Alliano");
}

#[test]
fn super_keyword() {
    foo_bar::foo_bar_v2_mdl::bar::hello("Abdillah Kim");
    foo_bar::foo_bar_v3_mdl::foo::hello("Alliano");
}

trait Animal {
    fn sound(&self, sound: &str) -> String;
    fn leg(&self) -> u8;
    fn can_eat(&self) -> bool {
        true
    }
}

struct Cow {
    name: String,
}
struct Cat {
    name: String,
}

impl Animal for Cow {
    fn sound(&self, sound: &str) -> String {
        format!("im {} my sound is {}", self.name, sound)
    }

    fn leg(&self) -> u8 {
        4
    }
}

impl Animal for Cat {
    fn sound(&self, sound: &str) -> String {
        format!("im {}, my sound is {}", self.name, sound)
    }

    fn leg(&self) -> u8 {
        4
    }
}

#[test]
fn trait_test() {
    let cow: Cow = Cow {
        name: String::from("Cow"),
    };
    let cat: Cat = Cat {
        name: String::from("Cat"),
    };

    let cow_sound = cow.sound("Mooooooo");
    let cat_sound = cat.sound("Miauuuu");

    let cow_leg = cow.leg();
    let cat_leg = cat.leg();

    println!("{}", cow_sound);
    println!("{}", cow.can_eat());
    println!("{}", cat_sound);
    println!("{}", cat.can_eat());

    println!("cat have {} legs", cat_leg);
    println!("cow have {} legs", cow_leg);
}

fn animal_sound(animal: &impl Animal) {
    let result = animal.sound("MIAUUUUU");
    println!("{}", result);
}

#[test]
fn trait_as_param() {
    let cat: Cat = Cat {
        name: String::from("Cat"),
    };
    animal_sound(&cat);
}

struct Usr {
    first_name: String,
    last_name: String,
}

trait CanSayHello {
    fn hello(&self) -> String {
        format!("Hello")
    }

    fn hello_from(&self, name: &str) -> String;
}

trait CanSayGoogBeye {
    fn goog_beye(&self) -> String {
        format!("Hello")
    }

    fn good_beye_from(&self, name: &str) -> String;
}

impl CanSayHello for Usr {
    fn hello_from(&self, name: &str) -> String {
        format!("Hello from {}", name)
    }
}

impl CanSayGoogBeye for Usr {
    fn good_beye_from(&self, name: &str) -> String {
        format!("good beye from {}", name)
    }
}

fn hello_googbeye(value: &(impl CanSayGoogBeye + CanSayHello)) {
    println!("{}", value.hello_from("Kim"));
    println!("{}", value.good_beye_from("Alliano"));
}

#[test]
fn test_multiple_trait_impl_as_params() {
    let usr: Usr = Usr {
        first_name: String::from("Abdillah"),
        last_name: String::from("Kim"),
    };
    hello_googbeye(&usr);
}

struct Usr2 {
    first_name: String,
    last_name: String,
    age: u8,
}

trait GoAhead {
    fn go(&self) -> String;
}

impl GoAhead for Usr2 {
    fn go(&self) -> String {
        format!(
            "my name {} and my age is {}, i will never give up",
            self.first_name, self.age
        )
    }
}

fn trait_as_retrun_value(user: Usr2) -> impl GoAhead {
    Usr2 {
        first_name: user.first_name,
        last_name: user.last_name,
        age: user.age,
    }
}

#[test]
fn test_trait_as_return_value() {
    let user = trait_as_retrun_value(Usr2 {
        first_name: String::from("Abdillah"),
        last_name: String::from("Kim"),
        age: 22,
    });
    println!("{}", user.go());
}

struct Human {
    first_name: String,
    last_name: String,
    age: u32,
    hobbie: String,
}

trait CanDo {
    fn say_hello(&self, name: &str) -> String;
}

trait CanTalk {
    fn say_hello(&self) -> String;
}

impl CanDo for Human {
    fn say_hello(&self, name: &str) -> String {
        format!("hello {}", name)
    }
}

impl CanTalk for Human {
    fn say_hello(&self) -> String {
        format!("hello guys")
    }
}

#[test]
fn test_conflict_method() {
    let human: Human = Human {
        first_name: String::from("Abdillah"),
        last_name: String::from("Kim"),
        age: 22,
        hobbie: String::from("Watching"),
    };

    /*
     * Cara memanggil conflict method pada trait 
     */
    println!("{}", CanDo::say_hello(&human, "Abdillah"));

    println!("{}", CanTalk::say_hello(&human));
}



struct Company {
    name: String,
    no_regist: String,
    owner: String,
    sector: String,
    is_public: bool
}


trait Action {
    fn buy_back_stock(&self, amount: u64) -> String;

    fn do_right_issue(&self, amount: u64) -> String;
}

trait Expansion {
    fn sector(&self, name: String) -> String;
}

/*
 * trait corporate ini merupakan trait yang menggabungkan trait Action dan Expansion
 * jadi ketika kita mengimplementasikan trait Corporate pada struct Company maka kita juga harus mengimplementasikan trait Action dan Expansion
 */
trait Corporate: Action + Expansion {
    fn do_something(&self, example: String) -> String;
}


impl Expansion for Company {
    fn sector(&self, name: String) -> String {
        format!("{}", name)
    }
}

/*
 * ketika kita mengimplementasikan trait Corporate pada struct Company maka kita juga harus mengimplementasikan trait Action dan Expansion
 * karena trait Corporate itu merupakan trait yang menggabungkan trait Action dan Expansion
 */
impl Action for Company {
    fn buy_back_stock(&self, amount: u64) -> String {
        format!("company {} do buyback with amound {}", self.name, amount)
    }

    fn do_right_issue(&self, amount: u64) -> String {
        format!("company {} do right issue {}", self.name, amount)
    }
}

/*
 * ketika kita mengimplementasikan trait Corporate pada struct Company maka kita juga harus mengimplementasikan trait Action dan Expansion
 * karena trait Corporate itu merupakan trait yang menggabungkan trait Action dan Expansion
 */
impl Corporate for Company {
    fn do_something(&self, example: String) -> String {
        format!("{}", example)
    }
}


/*
 * Generic adalah feature dimana kita bisa membuat function , struct, enum, method, dan trait yang mana tipe datanya bisa kita tentukan ketika kita membuat instance nya.
 * feature ini sayangat berguna ketika kita membuat kode  yang generic atau general yang mana bisa digunakan untuk banyak tipe data
 * dengan menggunakan generic kita bisa menghindari duplikasi kode yang sama untuk tipe data yang berbeda 
 */


 /*
  * ketika kita membuat generic kita bisa menggunakan placeholder untuk tipe data nya, biasanya kita menggunakan huruf T untuk menandakan bahwa itu adalah placeholder untuk tipe data
  * kita juga bisa menggunakan huruf lain seperti U, V, dll untuk menandakan placeholder untuk tipe data yang berbeda
  * kita juga bisa menggunakan trait bound untuk membatasi tipe data yang bisa digunakan pada generic, misalnya kita hanya ingin tipe data yang bisa digunakan pada generic adalah tipe data yang implementasi trait tertentu
  * 
  * berikut ini adalah conoth penggunaan generit pada struct, kita membuat struct Point yang mana tipe data dari x dan y bisa kita tentukan ketika kita membuat instance nya    
  */
struct Point <T> {
    x: T,
    y: T,
}


#[test]
fn test_generic_struct() {
    let p1: Point<f32> = Point { x: 1.0, y: 2.0 };
    let p2: Point<i32> = Point { x: 1, y: 2 };
    println!("Point 1: ({}, {})", p1.x, p1.y);
    println!("Point 2: ({}, {})", p2.x, p2.y);
}

/*
 * berikut ini adalah contoh penggunaan generic pada enum, kita membuat enum Value yang mana tipe data dari value bisa kita tentukan ketika kita membuat instance nya
 */
enum Value<T> {
    SOME(T),
    NONE,
}

#[test]
fn test_generic_enum() {
    let value1: Value<i32> = Value::<i32>::SOME(32);
    let value2: Value<String> = Value::<String>::SOME(String::from("Hello"));
    match value1 {
        Value::SOME(val) => println!("Value 1: {}", val),
        Value::NONE => println!("Value 1: None"),
    }
    match value2 {
        Value::SOME(val) => println!("Value 2: {}", val),
        Value::NONE => println!("Value 2: None"),
    }
}



trait CanFight {
    fn power(&self) -> String;
}

struct SolarMan {
    name: String,
    demage: i16
}

impl CanFight for SolarMan {
    fn power(&self) -> String {
        format!("{} have power {}", self.name, self.demage)
    }
}

/**
 * disini kita memberi batasan pada generic T bahwa tipe data yang bisa digunakan pada generic ini adalah tipe data yang mengimplementasikan trait CanFight
 * dengan begini kita bisa memastikan bahwa ketika kita membuat instance dari struct Hero maka tipe data yang kita gunakan pada generic T adalah tipe data yang mengimplementasikan trait CanFight, sehingga kita bisa menggunakan method power() pada tipe data tersebut
 * jika kita mencoba membuat instance dari struct Hero dengan tipe data yang tidak mengimplementasikan trait CanFight maka akan terjadi error karena kita sudah memberi batasan pada generic T bahwa tipe data yang bisa digunakan pada generic ini adalah tipe data yang mengimplementasikan trait CanFight
 * jadi dengan menggunakan trait bound pada generic kita bisa memastikan bahwa tipe data yang kita gunakan pada generic adalah tipe data yang sesuai dengan kebutuhan kita, sehingga kita bisa menghindari error yang mungkin terjadi ketika kita menggunakan tipe data yang tidak sesuai dengan kebutuhan kita
 * 
 */
struct Hero<T: CanFight> {
    super_hero: T,
}

#[test]
fn test_generic_bound() {
    let solar: SolarMan = SolarMan {
        name: String::from("SolarMan"),
        demage: 100,
    };
    let hero: Hero<SolarMan> = Hero {
        super_hero: solar,
    };
    println!("{} {}", hero.super_hero.name, hero.super_hero.power());
}


/**
 * berikit ini adalah contoh penggunaan generic pada function, kita membuat function lg yang mana tipe data dari v1 dan v2 bisa kita tentukan ketika kita memanggil function tersebut
 * dengan menggunakan generic pada function kita bisa membuat function yang generic atau general yang mana bisa digunakan untuk banyak tipe data, sehingga kita bisa menghindari duplikasi kode yang sama untuk tipe data yang berbeda
 * pada function lg ini kita juga memberi batasan pada generic T bahwa tipe data yang bisa digunakan pada generic ini adalah tipe data yang mengimplementasikan trait PartialOrd, dengan begini kita bisa memastika bahwa tipe data yang kita gunakan pada generic T adalah tipe data yang bisa dibandingkan dengan operator >, sehingga kita bisa menggunakan operator > pada tipe data tersebut untuk membandingkan nilai dari v1 dan v2
 * 
 */
fn lg<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}

#[test]
fn test_generic_in_function() {
    let result: i32 = lg::<i32>(10, 20);
    println!("the largest value is {}", result);
}



/*
 * ketika membuat generic pada method, kita bisa menambahkan generic setelah kata kunci impl dan secara otomatis generic tersebut bisa digunakan di semua method
 * Atau jika kita ingin membuat generic pada method tertentu saja, kita bisa menambahkan generic setelah nama method maka secara otomatis generic tersebut hanya bisa digunakan pada method tersebut saja
 * berikut ini adalah contoh penggunaan generic pada method yang mana tipe data dari x dan y bisa kita tentukan ketika kita membuat instance dari struct Point, dan kita juga bisa menggunakan generic tersebut pada method get_x() dan get_y() untuk mengembalikan nilai dari x dan y dengan tipe data yang sesuai dengan tipe data yang kita tentukan pada saat membuat instance dari struct Point 
 */
impl <T> Point<T> {
    
    pub fn get_x(&self) -> &T {
        &self.x
    }

     pub fn get_y(&self) -> &T {
        &self.y
    }
}


#[test]
fn test_generic_method() {
    let point = Point::<i32>{
        x: 10,
        y: 20,
    };

    println!("x: {}, y: {}", point.get_x(), point.get_y());
}


/**
 * ini merupakan contoh penggunakan generic bound namun menggunakan where clause
 */
struct Enemy<T> where T: CanFight {
    pub enemy: T
   
}

#[test]
fn test_where_clause_generic() {
   let solar_man = SolarMan {
    demage: 100,
    name: String::from("Dragon")
   };

   // SolarMan merupakan implementasi dari CanFight, selain implementasi dari CanFight maka tidak bisa kita gunakan sebagai type generic pada struct Enemy
   let enemy = Enemy::<SolarMan>{
    enemy: solar_man
   };

   print!("{}", enemy.enemy.power())
}


/*
 * ketika generic type nya nga disebutkan maka secara otomatis rust akan menggunakan 
 * Person sebagai default generic type nya 
 */
struct MyWife<T = Person> {
    wife: T
}

#[test]
fn test_default_generic_type() {
    let my_wife = MyWife {
         wife: Person {
            first_name: String::from("gatau"),
            last_name: String::from("gatau juga"),
            age: 22,
            is_marige: false
         }
    };

    print!("my wife first name {} last name {} age {} ", my_wife.wife.first_name, my_wife.wife.last_name, my_wife.wife.age);
}


use core::{ops::Add, panic};

struct Manggo {
    pub quantity: i128
}

/*
 * Overloadable Operator
 * sebelumnya kita sudah mempelajari operator aritmatika seperti * +- / % dll untuk tipe data number
 * Apakah pada tipe data selain number menudukung operasi tersebut? jelas tentu tidak.
 * Namn Rust memiliki feature yang mana kita bisa mengimplementasikan operator dengan bentuk method, sehingga kita bisa menggunakna operator aritmatika.
 * Semua Overloadable Operator pada rust direpresentasikan dalam bentuk trait yang bisa kita implementasikan .
 * trait tersebut berada pada module core::ops
 */
impl Add for Manggo {
    type Output = Manggo;

    // membuat custom operasi add (+) untuk tipe data Manggo(struct)
    fn add(self, rhs: Self) -> Self::Output {
        Manggo {
            quantity: self.quantity + rhs.quantity
        }
    }
}

#[test]
fn test_custom_operator() {
    let manggo1 = Manggo {quantity: 10};
    let manggo2 = Manggo {quantity: 10};
    let manggo3 = Manggo {quantity: 10};

    let result = manggo1 + manggo2 + manggo3;
    // let mut result = manggo1.add(manggo2);
    // result = result.add(manggo3);
    println!("manggo quantity {}", result.quantity)
}


/*
 * Null aatau Undifind
 * 
 * jika kit asebelumnya pernah belajar bahasa pemrogramman seperti Java, PHP, Javascript
 * Pasti kita mengenali istilah Null atau Undifined Value atau nilai kosong dari suatu variable
 * Pada Bahasa pemrogramman Rust tidak memiliki hal tersebut, di Rust ketika kita membuat vairabel kita wajib memberikan value nya
 * haltersebut dikarnakan agar ketika kita mengakses variabel tersebut maka tidak terjadi eror. 
 * Lantas gimana kalo kita ingin membuat vairabel dan value nya tidak wajin kita isi?
 * Maka kita bisa menggunakan Option Enum
 * 
 * Optional Value
 * 
 * Rust memiliki Option Enum yang merupakan representasi dari optional value(nilau atau value yang tidak wajib diisi)
 * Simple nya Option menyediakan 2 opsi None unutk opsi nilai kosong dan some(T) unutuk value tidak kosong
 * Keuntungan menggunakan Option adalah kita bisa menggunakan pattern matching ketika melakukan pengeceka pada Enum Option tersebut
 * 
 * 
 */


 fn double(value: Option<i32>) -> Option<i32> {
     match value {
         None => None,
         Some(i) => Some(i * 2)
     }
 }


 #[test]
 fn test_option_value() {
     let result = double(Some(32));
     print!("{:?}", result);

     let result2 = double(None);
     print!("{:?}", result2)
 }


 /*
  * Compare operator 
  *
  */

  impl PartialEq for Manggo {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
  }

  impl PartialOrd for Manggo {
      fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
          self.quantity.partial_cmp(&other.quantity)
      }
  }

#[test]
fn test_compare_operation() {
    let manggo1 = Manggo {quantity: 10};
    let manggo2 = Manggo {quantity: 20};

    println!("Manggo 1 == Manggo 2 {}", manggo1 == manggo2);
    println!("Manggo 1 < Manggo 2 {}", manggo1 < manggo2);
    println!("Manggo 1 > Manggo 2 {}", manggo1 > manggo2);
}


#[test]
fn test_string_manipulation() {
    let name = String::from("Abdillah");

    println!("{}", name.to_ascii_uppercase());
    println!("{}", name.to_lowercase());
    println!("{}", name.replace("Abdillah", "Kim"));
    println!("{}", name.len());
    println!("{}", name.contains("Kim"));
    println!("{}", name.starts_with("Kim"));
    println!("{}", name.ends_with("Kim"));
    println!("{}", name.trim());
}

struct Category {
    id: i32,
    name: String
}

use std::fmt::Debug;

/**
 * Formating
 * Sebelunya kita sering kali menggunakna println!
 * println! merupakan sebuah macro bukan function
 * ketika menggunakan macro println! kita sering kali menambahkan paramerter tambahan untuk
 * menampilkan sebuh data.
 * Secara default data tidak dapat ditampilkan dengan println!/macro.
 * data yang bisa ditampilkan ahanyalah dat yang sudah implementasi std::fmt::*
 */
impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_category_debug() {
    let category = Category { id: 1, name: String::from("Economic") };
    println!("{:?}", category);
}

/*
 * Closeure
 * Closure adalah function yang tidak memiliki nama, biasanya disimpan pada variabel atau digunakan di parameter
 * Kita bisa membuat Clusure dan menggunakan Closure ketika mebutuhkanya. Untuk membuat Closure 
 * kita bisa menggunakan tipe data fn(paramType)->returnType 
 * Dan unutk menmanggil Closure, kita bisa panggil menggunakan nama variabel atau parameternya secara langsung.
 */

#[test]
fn test_closure() {
    let sum: fn(i32, i32) -> i32 = |param1, param2| param1 + param2;
    let result = sum(10, 20);
    println!("the result is {}", result);
}


fn print_with_filter(name: String, filter: fn(&str) -> String) {
    let result = filter(&name);
    println!("{}", result);
}

#[test]
fn test_closure_as_param() {
    let name = String::from("Abdillah");
    print_with_filter(name, |name| ->  String {
        name.to_uppercase()
    });
}

/*
 * Collection
 * Sebelumnya kita sudah membahas tipe data Array, namun size of array tidak bisa berkembang atau sudah FIX ukuranya
 * Rust memiliki tipe data Collection yang mana kita bisa gunakan untuk menyimpan data yang jumlahnya tidak pasti atau bisa berkembang.
 * Collection ini disipan di sisi Heep berbeda denga array yang disipan pada stack.
 * 
 */

 /*
  * Tipe Data COllection
  * Secara garis besar tipe data collectio terbagi menjadi 3:
  * 1. Sequence: tipe data collection yang memiliki Index
  * 2. Map: tipe data colleciton berupa key value
  * 3. Sets: tipe data collection yang unix dan tidak memiliki index
  */


  /*
   * Sequence
   * Sequence adalah tipe data collection yang memiliki index, jadi kita bisa mengakses data pada sequence dengan menggunakan index nya
   * contoh tipe data sequence adalah Vector, LinkedList, VecDeque, dll
   */


/*
 * Vector
 * Vector merupakan Sequence yang urutanya sesuai dengan yang kita inginkan
 * Menambah data ke vector dilakukan dibagian belakang
 * Cocok untuk implementasi Stack(Tumpukan) atau Last in First Out (LIFO)
 */

 #[test]
 fn test_sequence_vector() {
     let mut names: Vec<String> = Vec::<String>::new();
     names.push(String::from("Abdillah"));
     names.push(String::from("Kim"));
     names.push(String::from("Alliano"));

     for nama in &names {
         println!("Vec name: {}", nama);
     }
 }

/*
 * VecDeque
 * VecDeque sebenarnya mirip seperti vector, nammun vecDeque memiliki kelebihan yaitu kita bisa menambah data di bagian depan dan belakang, sedangkan vector hanya bisa menambah data di bagian belakang saja
 * cocok untuk implementasi Queue atau First in First Out (FIFO)
 */

 #[test]
 fn test_vector_deque() {
     let mut names: VecDeque<String> = VecDeque::<String>::new();
     names.push_back(String::from("Abdillah"));
     names.push_back(String::from("Kim"));
     names.push_front(String::from("Alliano"));

     /*
      * disini saya menggunakna &name sebagai reference. perlu diingat data collection itu disimpan di sisi heap
      * jadi kalo nga make reference maka ownership nya akan dipindahkan 
      */
     for name in &names {
         println!("VecDeque name: {}", name);
     }
 }

 /*
  * LinkedList
  * LinkedList merupakan sequence yang mana data nya disimpan dalam node yang saling terhubung satu sama lain, jadi kita bisa menambah data di bagian depan dan belakang, sedangkan vector hanya bisa menambah data di bagian belakang saja
  * cocok untuk implementasi Queue atau First in First Out (FIFO)
  * Struktur data LinkedList sangat evisien unutk penambahan dan pengurangan data, oleh karena itu sangat cocok ketika kita membutuhkan 
  * sequence yang tidak terprediksi ukuranya.
  * Namin perlu diperhatikan bahwa LinkedList ini memiliki performa yang tidak secepat Vector atau VecDeque karena LinkedList tidak memiliki 
  * feature mengakses data melalui Index
  */

  #[test]
  fn test_linked_list() {
      let mut names: LinkedList<String> = LinkedList::<String>::new();
      names.push_back(String::from("Abdillah"));
      names.push_back(String::from("Kim"));
      names.push_front(String::from("Alliano"));

      for name in &names {
          println!("LinkedList name: {}", name);
      }

      // Ini akan error karena mencoba mengakses melalui Index, sedangkan LinkedList tidak memiliki feature mengakses data melalui Index
      // println!("name : {}", names[0]);
  }

  /*
   * Map
   * Map merupakan jenis Collection yang berbasis Key value, Berbeda dengan Sequence yang index nya menggunakan number secara otomatis, pada Map kita bisa menggunakan tipe data apapun sebagai key nya, dan value nya juga bisa menggunakan tipe data apapun
   * pada Map kita bebas menentukan tipe key atau index nya, namun ketika key nya duplikat maka yang akan terjadi adalah data dengan key sebelumnya akan di replace dengan data yang baru, jadi kita tidak bisa memiliki key yang duplikat pada Map
   * contoh tipe data Map adalah HashMap, BTreeMap, dll
   */

   /**
    * HashMap & ThreeMap
    * Rust memiliki 2 iplementasi Map yaitu HashMap dan BTreeMap, perbedaan yang mencolok pada keduanya adalah
    * BTreeMap key akan diurutkan sedangkan HashMap keynya tidak diurutkan oleh karena itu operasi unutk memasukan data
    * pada HashMap cendrung lenih cepat dibanding BTreeMap, namun untuk operasi pencarian data pada BTreeMap cendrung lebih cepat dibanding HashMap karena pada BTreeMap key nya sudah diurutkan sedangkan pada HashMap key nya tidak diurutkan sehingga ketika kita mencari data pada HashMap maka kita harus melakukan pencarian secara linear sedangkan pada BTreeMap kita bisa melakukan pencarian secara binary search karena key nya sudah diurutkan
    * 
    */
  #[test]
  fn test_hash_map() {
      let mut names: HashMap<String, String> = HashMap::<String, String>::new();
      names.insert(String::from("first_name"), String::from("Abdillah"));
      names.insert(String::from("last_name"), String::from("Kim"));

      println!("first name: {}", &names.get("first_name").unwrap());
      println!("last name: {}", &names.get("last_name").unwrap());
  }


  #[test]
  fn test_b_tree_map() {
      let mut names: BTreeMap<String, String> = BTreeMap::<String, String>::new();
        names.insert(String::from("first_name"), String::from("Abdillah"));
        names.insert(String::from("last_name"), String::from("Kim"));

        for entry in &names {
            println!("{}: {}", entry.0, entry.1);
        }
  }


  /*
   * Set
   * Set merupakan tipe data Collection yang mana data didalama Set tidak boleh duplikat, jika kita memmasukan data set yang sudah ada(duplikat)
   * Maka secara otomatis data tersebut tidak akan diterima atau tidak akan dimasukan ke dalam Set, jadi kita tidak perlu khawatir tentang data yang duplikat pada Set
   * contoh tipe data Set adalah HashSet, BTreeSet, dll
   * 
   * Set ini tidak seperti data Sequence, data set tidak bisa diakses melalui index
   */

/*
 * HashSet & BTreeSet
 * Rust memiliki 2 implementasi Set yaitu HashSet dan BTreeSet, perbedaan
 * HashSet tidak menjamin mengenai urutan data karena tujuan HashSet adalah memastikan tidak ada data duplikat secara cepat.
 * Sedangkan BTreeSet memastikan tidak ada data yang duplikat dan juga mengurutkan data didalam Set nya oleh karena itu Performanya lebih lama daripada HashSet 
 * Karena perlu mengurutkan data set ketika kita menambhakan atau menghapus data pada BTreeSet
 */

 #[test]
 fn test_hash_set() {
     let mut names: HashSet<String> = HashSet::<String>::new();
     names.insert(String::from("Abdillah"));
     names.insert(String::from("Kiim"));
     names.insert(String::from("Abdillah"));

     for name in &names {
         println!("Name : {}", name);
     }
 }

 #[test]
 fn test_b_tree_set() {
     let mut names: BTreeSet<String> = BTreeSet::<String>::new();
     names.insert(String::from("Abdillah"));
     names.insert(String::from("Kiim"));
     names.insert(String::from("Abdillah"));

     for name in &names {
         println!("Name : {}", name);
     }
    }


    /*
     * Iterator
     * Rust memiliki Modul yang bernama Iterator yang digunakan sebagai mekanisme untuk melakukan operasi
     * urutan dari data. Jadi semua data yang berisifat multiple misalnya seperti Array, Slice, dan Collection itu memiliki
     * Feature interator. 
     * Dengan menggunakan Iterator, Secara otomatis kita dapat melakukan iterasi(perulangan/looping) terhadap data.
     * 
     */

    #[test]
    fn test_iterator() {
        let numbers: [i32; 5] = [1, 2, 4, 5, 8];
        let mut interator = numbers.iter();

        while let Some(number) = interator.next() {
            println!("number: {}", number);
        }

        for number in interator {
            println!("number-x: {}", number);
        }
    }

    #[test]
    fn test_iterator_method() {
        let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
        println!("vector: {:?}", vector);
        let sum = vector.iter().sum::<i32>();
        println!("sum: {}", sum);
        let count = vector.iter().count();
        println!("count: {}", count);
        let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
        println!("doubled: {:?}", doubled);
        let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect::<Vec<&i32>>();
        println!("odd: {:?}", odd);
    }


    /*
     * Error Handling
     * Error Handling merupakan hal yang sudah biasa dalam pengenmbangan applikasi, hampir semua bahasa pemrogramman memiliki Error Handling termasuk Rust
     * Rust membagi Error Handling menjadi 2 juenis yaitu recoverable(dapat dipulihkan) dan unreciverable(tidak dapat dipulihkan)
     * Rust tidak memiliki tipe data Exception seperti bahasa pemrogramman lain seperti Java, PHP, Javascript
     * Pada bahsa pemrogramman Rust unutk error Handling nya menggunakan pendekatan lain
     */

    /*
     * Unrecoverable Error
     * Jika kita mendapatkan jenis error yang tidak adapat dipulihkan maka kita bisa menggunakna jenis error Unrecoverable Error, jenis error ini biasanya terjadi ketika kita melakukan operasi yang tidak valid seperti mengakses index yang diluar batas array, atau melakukan operasi yang tidak valid pada tipe data tertentu
     * Rust menggunakan macro panic! untuk melakukan hal tersbeut.
     */

    fn connect_database(host: Option<String>){
        match host {
            None => {
                panic!("host is not provided");
            }
            Some(host) => {
                println!("connecting to database with host {}", host);
            }
        }
    }

    #[test]
    fn test_panic() {
        connect_database(None);
        connect_database(Some(String::from("localhost:5432S")));
    }

    fn conection_cache(host: Option<String>) -> Result<String, String> {
        match host {
            None => Err("cache not connected".to_string()),
            Some(host) => Ok(format!("connecting to cache with host {}", host))
        }
    }


    #[test]
    fn test_recoverable() {
        let catching = conection_cache(Some(String::from("localhost:6379")));
        match catching {
            Ok(message) => println!("Success: {}", message),
            Err(error) => println!("Error: {}", error),
        }
    }