use axum::{
    self,
    body::Body,
    http::{header, StatusCode},
    response::Response,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDuration {
    pub days: i32,
    pub months: i32,
    pub years: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoPI {
    email: String,
    name: String,
    status: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Submission {
    #[serde(rename = "_id")]
    pub id: String,
    pub status: String,
    pub track: String,
    #[serde(rename = "trackCode", default)]
    pub track_code: Option<String>,
    #[serde(rename = "unique_id")]
    pub unique_id: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub user: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "maxFilled", default)]
    pub max_filled: Option<i32>,
    #[serde(rename = "additional-information", default)]
    pub additional_information: Option<String>,
    #[serde(rename = "date-phd-award", default)]
    pub date_phd_award: Option<String>,
    #[serde(rename = "funding-department", default)]
    pub funding_department: Option<String>,
    #[serde(rename = "project-duration", default)]
    pub project_duration: Option<ProjectDuration>,
    #[serde(rename = "project-keywords", default)]
    pub project_keywords: Option<Vec<String>>,
    #[serde(rename = "project-objective", default)]
    pub project_objective: Option<Vec<String>>,
    #[serde(rename = "project-objective_new", default)]
    pub project_objective_new: Option<String>,
    #[serde(rename = "project-summary", default)]
    pub project_summary: Option<String>,
    #[serde(rename = "project-title", default)]
    pub project_title: Option<String>,
    #[serde(rename = "proposal-supporting-files", default)]
    pub proposal_supporting_files: Option<Vec<serde_json::Value>>,
    #[serde(rename = "total-cost", default)]
    pub total_cost: Option<String>,
    #[serde(rename = "coPI", default)]
    pub co_pi: Option<Vec<CoPI>>,
    #[serde(rename = "discardedAt", default)]
    pub discarded_at: Option<DateTime<Utc>>,
}
#[derive(Serialize)]
pub struct SubmissionResponse {
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

#[allow(dead_code)]
impl SubmissionResponse {
    pub fn from_submissions(submissions: Vec<Submission>) -> Vec<Self> {
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

    pub fn to_json_response(submissions: Vec<Submission>) -> Response<Body> {
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
