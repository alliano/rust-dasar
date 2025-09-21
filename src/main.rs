
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
fn test_operator_aritmatika(){
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
    let array2d: [[i8; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    println!("Array 2 dimensi");
    println!("{:?}", array2d)
}


const MAXIMUM_VALUE: i32 = 500;
#[test]
fn test_constant() {
    const MINIMUM_VALUE: i32 = 5;

    println!("nilai minimum {}, nilai maksimum {}", MINIMUM_VALUE, MAXIMUM_VALUE);
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

#[test]
fn data_copy() {
    let a: i32 = 10;

    let b: i32 = a; // melakukan copy data dari a ke b

    println!("nilai a {}", a);
    println!("nilai b {}", b);
}

#[test ]
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

#[test]
fn loop_expression(){
    let mut counter: i32 = 0;

    loop {
        counter += 1;

        if counter == 10 {
            break;
        }else if counter % 2 != 0 {
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


#[test]
fn loop_lable(){
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

#[test]
fn array_iteration(){
    let array: [&str; 5] = ["Mas Kim", "Mas Nardji", "Mas Ambarawa", "Mas Ganteng", "Mas Abdillah"];
    let mut index: usize = 0;
    while index < array.len() {
        println!("nama ke-{} adalah {}", index, array[index]);
        index += 1;
    }    
}