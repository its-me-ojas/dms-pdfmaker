use axum::{
    self,
    body::Body,
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use docx_rs::{Docx, Paragraph};
use page1::page1_content;
use page2::{page2_content_signatures, page2_content_with_table};
use std::fs;

mod page1;
mod page2;
mod utils;

async fn generate_document() -> Response<Body> {
    let docx_path = "dms.docx";
    let pdf_path = "output/dms.pdf";

    // create the Word document
    let file = std::fs::File::create(docx_path).unwrap();
    let mut doc = Docx::new();

    // page 1 content
    for paragraph in page1_content() {
        doc = doc.add_paragraph(paragraph);
    }

    // page break
    doc = doc.add_paragraph(Paragraph::new().page_break_before(true));

    // page 2 content
    let (paragraphs, table) = page2_content_with_table();
    for paragraph in paragraphs {
        doc = doc.add_paragraph(paragraph);
    }
    doc = doc.add_table(table);
    for paragraph in page2_content_signatures() {
        doc = doc.add_paragraph(paragraph);
    }

    doc.build().pack(file).unwrap();

    // convert to PDF
    match utils::convert_docx_to_pdf(docx_path, "output") {
        Ok(_) => {
            // Read the generated PDF file
            match fs::read(pdf_path) {
                Ok(pdf_content) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/pdf")
                    .header(
                        header::CONTENT_DISPOSITION,
                        "attachment; filename=\"document.pdf\"",
                    )
                    .body(Body::from(pdf_content))
                    .unwrap(),
                Err(_) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to read PDF file"))
                    .unwrap(),
            }
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to generate PDF"))
            .unwrap(),
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/generate", get(generate_document));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server started...");
    axum::serve(listener, app).await.unwrap();
}
