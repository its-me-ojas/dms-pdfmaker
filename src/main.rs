use axum::{
    self,
    body::Body,
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use docx_rs::{Docx, Paragraph};
use models::*;
use page1::page1_content;
use page2::{page2_content_signatures, page2_content_with_table};
use reqwest;
use std::fs;

mod models;
mod page1;
mod page2;
mod utils;

async fn admin_login() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let login_request = LoginRequest {
        email: "dsai@thapar.edu".to_string(),
        password: "Test@4756".to_string(),
    };

    let response = client
        .post("https://api-dms.ccstiet.com/auth/admin-login")
        .json(&login_request)
        .send()
        .await?;

    let login_response: LoginResponse = response.json().await?;
    Ok(login_response.token)
}

async fn fetch_submissions(token: &str) -> Result<Vec<Submission>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api-dms.ccstiet.com/submissions/fetch-admin")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    println!("Response Status: {}", response.status());

    let body = response.text().await?;
    // println!("Response body: {}", body);

    // Try parsing into Vec<Submission>
    match serde_json::from_str::<Vec<Submission>>(&body) {
        Ok(submissions) => {
            println!("Successfully parsed {} submissions", submissions.len());
            Ok(submissions)
        }
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            println!("Response body: {}", body);
            Err(Box::new(e))
        }
    }
}

async fn get_submissions() -> Response<Body> {
    // First get the token
    let token = match admin_login().await {
        Ok(token) => token,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to login"))
                .unwrap();
        }
    };

    // println!("token: {}", token);

    // Then fetch submissions
    match fetch_submissions(&token).await {
        Ok(submissions) => SubmissionResponse::to_json_response(submissions),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to fetch submissions"))
            .unwrap(),
    }
}

async fn generate_document_with_data() -> Response<Body> {
    // First get the token
    let token = match admin_login().await {
        Ok(token) => token,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to login"))
                .unwrap();
        }
    };

    // Then fetch submissions
    let submissions = match fetch_submissions(&token).await {
        Ok(submissions) => submissions,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to fetch submissions"))
                .unwrap();
        }
    };

    // filter for submitted submissions
    let submitted_submissions: Vec<&Submission> = submissions
        .iter()
        .filter(|s| s.status.to_lowercase() == "submitted")
        .collect();

    if submitted_submissions.is_empty() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("No submitted applications found"))
            .unwrap();
    }

    let submission = &submitted_submissions[0];
    let submission_id = submission.unique_id.clone();

    let docx_path = format!("dms_{}.docx", submission_id);
    let pdf_path = format!("output/dms_{}.pdf", submission_id);

    // create the Word document
    let file = std::fs::File::create(&docx_path).unwrap();
    let mut doc = Docx::new();

    // page 1 content
    for paragraph in page1_content(submission) {
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
    match utils::convert_docx_to_pdf(&docx_path, "output") {
        Ok(_) => match fs::read(&pdf_path) {
            Ok(pdf_content) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/pdf")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"dms_{}.pdf\"", submission_id),
                )
                .body(Body::from(pdf_content))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to read PDF file"))
                .unwrap(),
        },
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
        .route("/generate", get(generate_document_with_data))
        .route("/fetch-submissions", get(get_submissions));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server started...");
    axum::serve(listener, app).await.unwrap();
}
