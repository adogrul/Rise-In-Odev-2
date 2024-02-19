enum Publication { Book(Book), Magazine(Magazine),}

struct Book {title: String, author: String, page_count: u32,}

struct Magazine {title: String, issue: u32, topic: String,}

fn Yayin_Yazdir(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(ref book) => {
                println!("Kitap: {} <-> Yazar: {} <-> {} Sayfa",book.title, book.author, book.page_count);
            }
            Publication::Magazine(ref magazine) => {
                println!("Dergi: {} <-> Konu: {} <-> Başlık: {}",magazine.title, magazine.issue, magazine.topic);
            }
        }
    }
}

fn main() {
    let book1 = Book {
        title: "Kayra ve Kinyas".to_string(),
        author: "Hakan Günday".to_string(),
        page_count: 300,
    };

    let magazine1 = Magazine {
        title: "OT Dergisi".to_string(),
        issue: 15,
        topic: "Edebiyat".to_string(),
    };

    let publications = vec![Publication::Book(book1), Publication::Magazine(magazine1)];

    Yayin_Yazdir(publications);
}