use yew::prelude::*;

const EXAMPLE_IMG: &'static str = "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";

#[derive(PartialEq, Clone)]
struct Book {
    title: String,
    series: Option<BookSeriesInfo>,
    image_base64: Option<String>,
    torrent_link: String,
    author: Vec<String>,
    isbn: u64,
    narrator_name: Option<String>,
    custom_info: String,
}

#[derive(PartialEq, Clone)]
struct BookSeriesInfo {
    name: String,
    index: String,
}

fn generate_mock_books() -> Vec<Book> {
    vec![
        Book {
            title: "The Way of Kings".to_string(),
            series: Some(BookSeriesInfo {
                name: "The Stormlight Archive".to_string(),
                index: "1".to_string(),
            }),
            image_base64: Some(EXAMPLE_IMG.to_string()),
            torrent_link: "magnet:?xt=urn:btih:1234567890abcdef&dn=The+Way+of+Kings".to_string(),
            author: vec!["Brandon Sanderson".to_string(), "Katie Bechings".to_string()],
            isbn: 9780765326355,
            narrator_name: Some("Kate Reading".to_string()),
            custom_info: "2010, 45 hours 55 minutes".to_string(),
        },
        Book {
            title: "Words of Radiance".to_string(),
            series: Some(BookSeriesInfo {
                name: "The Stormlight Archive".to_string(),
                index: "2".to_string(),
            }),
            image_base64: None,
            torrent_link: "magnet:?xt=urn:btih:abcdef1234567890&dn=Words+of+Radiance".to_string(),
            author: vec!["Brandon Sanderson".to_string()],
            isbn: 9780765365286,
            narrator_name: Some("Michael Kramer".to_string()),
            custom_info: "2014, 48 hours 22 minutes".to_string(),
        },
        Book {
            title: "The Name of the Wind".to_string(),
            series: Some(BookSeriesInfo {
                name: "The Kingkiller Chronicle".to_string(),
                index: "1".to_string(),
            }),
            image_base64: None,
            torrent_link: "magnet:?xt=urn:btih:7890abcdef123456&dn=The+Name+of+the+Wind"
                .to_string(),
            author: vec!["Patrick Rothfuss".to_string()],
            isbn: 9780756404741,
            narrator_name: Some("Nick Podehl".to_string()),
            custom_info: "2007, 27 hours 58 minutes".to_string(),
        },
        Book {
            title: "A Game of Thrones".to_string(),
            series: Some(BookSeriesInfo {
                name: "A Song of Ice and Fire".to_string(),
                index: "1".to_string(),
            }),
            image_base64: None,
            torrent_link: "magnet:?xt=urn:btih:4567890abcdef123&dn=A+Game+of+Thrones".to_string(),
            author: vec!["George R.R. Martin".to_string()],
            isbn: 9780553103540,
            narrator_name: Some("Roy Dotrice".to_string()),
            custom_info: "1996, 33 hours 47 minutes".to_string(),
        },
        Book {
            title: "Project Hail Mary".to_string(),
            series: None,
            image_base64: None,
            torrent_link: "magnet:?xt=urn:btih:def1234567890abc&dn=Project+Hail+Mary".to_string(),
            author: vec!["Andy Weir".to_string()],
            isbn: 9780593135204,
            narrator_name: Some("Ray Porter".to_string()),
            custom_info: "2021, 16 hours 10 minutes".to_string(),
        },
        Book {
            title: "Dune".to_string(),
            series: Some(BookSeriesInfo {
                name: "Dune Chronicles".to_string(),
                index: "1".to_string(),
            }),
            image_base64: None,
            torrent_link: "magnet:?xt=urn:btih:234567890abcdef1&dn=Dune".to_string(),
            author: vec!["Frank Herbert".to_string()],
            isbn: 9780441013593,
            narrator_name: Some("Scott Brick".to_string()),
            custom_info: "1965, 21 hours 2 minutes".to_string(),
        },
        Book {
            title: "The Hobbit".to_string(),
            series: Some(BookSeriesInfo {
                name: "The Lord of the Rings".to_string(),
                index: "0.5".to_string(),
            }),
            image_base64: None,
            torrent_link: "magnet:?xt=urn:btih:34567890abcdef12&dn=The+Hobbit".to_string(),
            author: vec!["J.R.R. Tolkien".to_string()],
            isbn: 9780547928227,
            narrator_name: Some("Rob Inglis".to_string()),
            custom_info: "1937, 11 hours 5 minutes".to_string(),
        },
        Book {
            title: "Mistborn: The Final Empire".to_string(),
            series: Some(BookSeriesInfo {
                name: "Mistborn".to_string(),
                index: "1".to_string(),
            }),
            image_base64: None,
            torrent_link: "magnet:?xt=urn:btih:567890abcdef1234&dn=Mistborn+The+Final+Empire"
                .to_string(),
            author: vec!["Brandon Sanderson".to_string()],
            isbn: 9780765311788,
            narrator_name: Some("Michael Kramer".to_string()),
            custom_info: "2006, 24 hours 39 minutes".to_string(),
        },
        Book {
            title: "The Martian".to_string(),
            series: None,
            image_base64: None,
            torrent_link: "magnet:?xt=urn:btih:67890abcdef12345&dn=The+Martian".to_string(),
            author: vec!["Andy Weir".to_string()],
            isbn: 9780804139021,
            narrator_name: Some("R.C. Bray".to_string()),
            custom_info: "2014, 10 hours 53 minutes".to_string(),
        },
        Book {
            title: "Red Rising".to_string(),
            series: Some(BookSeriesInfo {
                name: "Red Rising Saga".to_string(),
                index: "1".to_string(),
            }),
            image_base64: None,
            torrent_link: "magnet:?xt=urn:btih:7890abcdef123456&dn=Red+Rising".to_string(),
            author: vec!["Pierce Brown".to_string()],
            isbn: 9780345539786,
            narrator_name: Some("Tim Gerard Reynolds".to_string()),
            custom_info: "2014, 16 hours 12 minutes".to_string(),
        },
    ]
}


#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div></div>
    }
}


#[function_component(QueuedBooks)]
pub fn queued_books() -> Html {
    let queued_books = generate_mock_books();

    let book_row = |book: &Book| -> Html {
        html! {
             <div class="book-sample-short">
                 if let Some(img_data) = &book.image_base64 {
                     <img width=100px height=100px src={format!("data:image/png;base64,{}", img_data)} />
                 }

                 <div class="book-info-holder">
                     <div class="title-author">
                         <span id="title">{&book.title}</span>
                         <div id="author">
                            //<span class="material-symbols-outlined"> { "ink_pen" } </span>
                            <span>{book.author.join(", ")}</span>
                         </div>
                     </div>
                     <div class="meta-info">
                         if let Some(series) = &book.series {
                             <span id="series">{format!("{} #{}", series.name, series.index)}</span>
                         }
                         if let Some(narrator_name) = &book.narrator_name {
                            <div>
                                <span id="series">{ format!("Read by {}", narrator_name) }</span>
                            </div>
                         }
                     </div>
                 </div>
             </div>
         }
    };

    html! {
        <div class="queuedbooks">
            <h1> { "Queued Books" } </h1>
            { queued_books.iter().map(book_row).collect::<Html>() }
        </div>
    }
}
