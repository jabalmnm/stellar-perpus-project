#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan data peminjaman buku
#[contracttype]
#[derive(Clone, Debug)]
pub struct BookLoan {
    pub id: u64,
    pub borrower_name: String,
    pub book_title: String,
}

// Storage key untuk data peminjaman (maksimal 9 karakter untuk symbol_short)
const LOAN_DATA: Symbol = symbol_short!("LOANS");

#[contract]
pub struct LibraryContract;

#[contractimpl]
impl LibraryContract {
    // [READ] - Fungsi untuk melihat semua daftar pinjaman buku
    pub fn get_loans(env: Env) -> Vec<BookLoan> {
        env.storage().instance().get(&LOAN_DATA).unwrap_or(Vec::new(&env))
    }

    // [CREATE] - Fungsi untuk mencatat peminjaman buku baru
    pub fn borrow_book(env: Env, borrower_name: String, book_title: String) -> String {
        let mut loans: Vec<BookLoan> = env.storage().instance().get(&LOAN_DATA).unwrap_or(Vec::new(&env));
        
        let new_loan = BookLoan {
            id: env.prng().gen::<u64>(),
            borrower_name: borrower_name,
            book_title: book_title,
        };
        
        loans.push_back(new_loan);
        env.storage().instance().set(&LOAN_DATA, &loans);
        
        String::from_str(&env, "Peminjaman buku berhasil dicatat")
    }

    // [UPDATE] - Fungsi untuk mengubah data peminjaman (misal: ganti judul buku)
    pub fn update_loan(env: Env, id: u64, new_book_title: String) -> String {
        let mut loans: Vec<BookLoan> = env.storage().instance().get(&LOAN_DATA).unwrap_or(Vec::new(&env));

        for i in 0..loans.len() {
            let mut loan = loans.get(i).unwrap();
            
            // Jika ID cocok, ubah judul bukunya
            if loan.id == id {
                loan.book_title = new_book_title;
                loans.set(i, loan); // Simpan kembali ke index tersebut
                
                env.storage().instance().set(&LOAN_DATA, &loans);
                return String::from_str(&env, "Data peminjaman berhasil diperbarui");
            }
        }

        String::from_str(&env, "Data pinjaman tidak ditemukan")
    }

    // [DELETE] - Fungsi untuk mengembalikan buku (menghapus data pinjaman)
    pub fn return_book(env: Env, id: u64) -> String {
        let mut loans: Vec<BookLoan> = env.storage().instance().get(&LOAN_DATA).unwrap_or(Vec::new(&env));

        for i in 0..loans.len() {
            if loans.get(i).unwrap().id == id {
                loans.remove(i);

                env.storage().instance().set(&LOAN_DATA, &loans);
                return String::from_str(&env, "Buku berhasil dikembalikan (Data dihapus)");
            }
        }

        String::from_str(&env, "Data pinjaman tidak ditemukan")
    }
}

mod test;