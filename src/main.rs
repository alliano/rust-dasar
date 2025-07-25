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

