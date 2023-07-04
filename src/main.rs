fn main() {
    
    let mut my_main_chain = String::from("Patika sen bir harikasin... <3"); //Ana Degisken Olusturuldu.

    let mut my_ref_chain = my_main_chain.clone(); // Ana Degiskenin Bir Kopyasi Olusturuldu.

    let my_mutant_chain = &mut my_main_chain; //Referans Eden Degisken de Olusturuldu.

    my_mutant_chain.push_str(" MUTANT");

//

    change_string(&mut my_ref_chain); // Kopya Degistirildi.

    println!("{}", my_ref_chain); // Olagan Cikti Alimi Icin Olusturuldu.

    println!("{}", my_mutant_chain); // Olagan Cikti Alimi Icin Olusturuldu.

//

    print_string(my_ref_chain);
    //print_string(my_main_chain); // Yardimci Degisken de Hata Almak Icin Ozel Olarak Olusturuldu.
    print_string(my_mutant_chain.to_string()); // .to_string Kullanilmazsa Hata Verir.
    
}

fn print_string(s: String) { 
    println!("{}", s); 
} 

fn change_string(s: &mut String) {
    s.push_str(" DEGISTIRILMIS !");
}