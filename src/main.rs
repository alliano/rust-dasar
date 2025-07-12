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