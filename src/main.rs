use std::collections::HashMap;

#[derive(Debug)]
struct Banka {
    hesaplar: HashMap<u32, Hesap>,
}

impl Banka {
    fn yeni() -> Self {
        Self {
            hesaplar: HashMap::new(),
        }
    }
    
    fn hesap_ekle(&mut self, hesap: Hesap) {
        self.hesaplar.insert(hesap.id, hesap);
    }
    
    fn para_transfer_et(&mut self, gonderen_id: u32, alici_id: u32, miktar: f64) -> Result<(), String> {
        if let Some(gonderen) = self.hesaplar.get_mut(&gonderen_id) {
            if gonderen.bakiye < miktar {
                return Err("Yetersiz bakiye".to_string());
            }
            if let Some(alici) = self.hesaplar.get_mut(&alici_id) {
                gonderen.bakiye -= miktar;
                alici.bakiye += miktar;
                return Ok(());
            }
            return Err("Alıcı hesap bulunamadı".to_string());
        }
        Err("Gönderen hesap bulunamadı".to_string())
    }
}

#[derive(Debug)]
struct Hesap {
    id: u32,
    kullanici_id: u32,
    bakiye: f64,
}

impl Hesap {
    fn yeni(id: u32, kullanici_id: u32, bakiye: f64) -> Self {
        Self { id, kullanici_id, bakiye }
    }
}

fn main() {
    let mut banka = Banka::yeni();
    let hesap1 = Hesap::yeni(1, 101, 1000.0);
    let hesap2 = Hesap::yeni(2, 102, 500.0);
    
    banka.hesap_ekle(hesap1);
    banka.hesap_ekle(hesap2);
    
    match banka.para_transfer_et(1, 2, 200.0) {
        Ok(_) => println!("Transfer başarılı"),
        Err(e) => println!("Transfer başarısız: {}", e),
    }
    
    println!("{:?}", banka);
}
