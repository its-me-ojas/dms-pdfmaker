use axum::{
    self,
    body::Body,
    extract::Json,
    http::{header, StatusCode},
    response::Response,
    routing::{get, post},
    Router,
};
use docx_rs::{Docx, Paragraph};
use models::*;
use page1::page1_content;
use page2::{page2_content_signatures, page2_content_with_table};
use reqwest;
use std::fs;
use std::sync::Arc;
use chrono;
use tower_http::cors::{Any, CorsLayer}; // Import CorsLayer


mod models;
mod page1;
mod page2;
mod utils;



// New function for the POST endpoint that accepts a JSON submission
async fn generate_document_from_json(Json(submission): Json<Submission>) -> Response<Body> {
    generate_pdf(&submission)
}

// Helper function to generate PDF from a submission
fn generate_pdf(submission: &Submission) -> Response<Body> {
    let submission_id = &submission.unique_id;
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S");
    
    let docx_path = format!("temp_{}.docx", timestamp);
    let pdf_filename = format!("proposal_{}.pdf", submission_id);
    let pdf_path = format!("output/{}", pdf_filename);

    // Ensure output directory exists
    if let Err(e) = fs::create_dir_all("output") {
        println!("Error creating output directory: {}", e);
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to create output directory"))
            .unwrap();
    }

    // create the Word document
    let file = match std::fs::File::create(&docx_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating DOCX file: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to create DOCX file"))
                .unwrap();
        }
    };
    
    let mut doc = Docx::new();

    // page 1 content
    for paragraph in page1_content(submission) {
        doc = doc.add_paragraph(paragraph);
    }

    // page break
    doc = doc.add_paragraph(Paragraph::new().page_break_before(true));

    // page 2 content
    let (paragraphs, table) = page2_content_with_table(submission);
    for paragraph in paragraphs {
        doc = doc.add_paragraph(paragraph);
    }
    doc = doc.add_table(table);
    for paragraph in page2_content_signatures() {
        doc = doc.add_paragraph(paragraph);
    }

    if let Err(e) = doc.build().pack(file) {
        println!("Error packing DOCX: {}", e);
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to create DOCX document"))
            .unwrap();
    }

    // convert to PDF
    let response = match utils::convert_docx_to_pdf(&docx_path, "output", Some(&pdf_filename)) {
        Ok(_) => match fs::read(&pdf_path) {
            Ok(pdf_content) => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/pdf")
                    .header(
                        header::CONTENT_DISPOSITION,
                        format!("attachment; filename=\"{}\"", pdf_filename),
                    )
                    .body(Body::from(pdf_content))
                    .unwrap();

                // clean up files after creating the response
                if let Err(e) = fs::remove_file(&docx_path) {
                    println!("Error removing DOCX file: {}", e);
                }

                response
            }
            Err(e) => {
                println!("Error reading PDF file: {}", e);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to read PDF file"))
                    .unwrap()
            }
        },
        Err(e) => {
            println!("Error converting to PDF: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to generate PDF"))
                .unwrap()
        }
    };

    response
}

async fn root() -> &'static str {
    "You have reached DMS Pdf Maker!"
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow requests from any origin
        .allow_methods(Any) // Allow any HTTP method
        .allow_headers(Any); // Allow any HTTP header

    let app = Router::new()
        .route("/", get(root))
        .route("/submissions/download", post(generate_document_from_json)) // New POST endpoint
        .layer(cors); // Add the CORS layer to the router

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("server started on port 8080...");
    axum::serve(listener, app).await.unwrap();
}
