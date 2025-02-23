use axum::{
    self,
    body::Body,
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use docx_rs::{Docx, Paragraph};
use page1::page1_content;
use page2::{page2_content_signatures, page2_content_with_table};
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;

mod page1;
mod page2;
mod utils;

#[derive(Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectDuration {
    days: i32,
    months: i32,
    years: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct CoPI {
    email: String,
    name: String,
    status: String,
}

#[derive(Deserialize, Debug)]
struct Submission {
    #[serde(rename = "_id")]
    id: String,
    status: String,
    track: String,
    #[serde(rename = "trackCode", default)]
    track_code: Option<String>,
    #[serde(rename = "unique_id")]
    unique_id: String,
    #[serde(rename = "updatedAt")]
    updated_at: DateTime<Utc>,
    user: String,
    #[serde(rename = "createdAt", default)]
    created_at: Option<DateTime<Utc>>,
    #[serde(rename = "maxFilled", default)]
    max_filled: Option<i32>,
    #[serde(rename = "additional-information", default)]
    additional_information: Option<String>,
    #[serde(rename = "date-phd-award", default)]
    date_phd_award: Option<String>,
    #[serde(rename = "funding-department", default)]
    funding_department: Option<String>,
    #[serde(rename = "project-duration", default)]
    project_duration: Option<ProjectDuration>,
    #[serde(rename = "project-keywords", default)]
    project_keywords: Option<Vec<String>>,
    #[serde(rename = "project-objective", default)]
    project_objective: Option<Vec<String>>,
    #[serde(rename = "project-objective_new", default)]
    project_objective_new: Option<String>,
    #[serde(rename = "project-summary", default)]
    project_summary: Option<String>,
    #[serde(rename = "project-title", default)]
    project_title: Option<String>,
    #[serde(rename = "proposal-supporting-files", default)]
    proposal_supporting_files: Option<Vec<serde_json::Value>>, // Changed to Value to handle both strings and empty objects
    #[serde(rename = "total-cost", default)]
    total_cost: Option<String>,
    #[serde(rename = "coPI", default)]
    co_pi: Option<Vec<CoPI>>,
    #[serde(rename = "discardedAt", default)]
    discarded_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
struct SubmissionResponse {
    id: String,
    status: String,
    track: String,
    track_code: Option<String>,
    unique_id: String,
    updated_at: String,
    user: String,
    created_at: Option<String>,
    max_filled: Option<i32>,
    additional_information: Option<String>,
    date_phd_award: Option<String>,
    funding_department: Option<String>,
    project_duration: Option<ProjectDuration>,
    project_keywords: Option<Vec<String>>,
    project_objective: Option<Vec<String>>,
    project_objective_new: Option<String>,
    project_summary: Option<String>,
    project_title: Option<String>,
    proposal_supporting_files: Option<Vec<serde_json::Value>>,
    total_cost: Option<String>,
    co_pi: Option<Vec<CoPI>>,
    discarded_at: Option<String>,
}

impl SubmissionResponse {
    fn from_submissions(submissions: Vec<Submission>) -> Vec<Self> {
        submissions
            .into_iter()
            .map(|s| Self {
                id: s.id,
                status: s.status,
                track: s.track,
                track_code: s.track_code,
                unique_id: s.unique_id,
                updated_at: s.updated_at.to_rfc3339(),
                user: s.user,
                created_at: s.created_at.map(|d| d.to_rfc3339()),
                max_filled: s.max_filled,
                additional_information: s.additional_information,
                date_phd_award: s.date_phd_award,
                funding_department: s.funding_department,
                project_duration: s.project_duration,
                project_keywords: s.project_keywords,
                project_objective: s.project_objective,
                project_objective_new: s.project_objective_new,
                project_summary: s.project_summary,
                project_title: s.project_title,
                proposal_supporting_files: s.proposal_supporting_files,
                total_cost: s.total_cost,
                co_pi: s.co_pi,
                discarded_at: s.discarded_at.map(|d| d.to_rfc3339()),
            })
            .collect()
    }

    fn to_json_response(submissions: Vec<Submission>) -> Response<Body> {
        let response_submissions = Self::from_submissions(submissions);

        match serde_json::to_string(&response_submissions) {
            Ok(json) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to serialize submissions"))
                .unwrap(),
        }
    }
}

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
    // for submission in &submissions {
    //     println!("{:#?}", submission);
    // }
    // let response_data = SubmissionResponse::from_submissions(submissions);

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
        Ok(_) => match fs::read(pdf_path) {
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
