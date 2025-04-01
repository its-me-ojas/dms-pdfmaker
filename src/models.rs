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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectDuration {
    pub days: i32,
    pub months: i32,
    pub years: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoPI {
    pub email: String,
    pub name: String,
    pub status: String,
    #[serde(default)]
    pub accessLevel: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BudgetItem {
    pub heading: String,
    pub id: String,
    pub isRequired: bool,
    pub justification: String,
    pub total: i32,
    pub years: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BudgetCategory {
    pub items: Vec<BudgetItem>,
    #[serde(rename = "type")]
    pub category_type: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Submission {
    #[serde(rename = "_id")]
    pub id: Option<serde_json::Value>,
    pub status: String,
    pub track: String,
    #[serde(rename = "trackCode", default)]
    pub track_code: Option<String>,
    #[serde(rename = "unique_id")]
    pub unique_id: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: serde_json::Value, // Could be DateTime<Utc> or a custom format
    pub user: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: Option<String>,
    #[serde(rename = "maxFilled", default)]
    pub max_filled: Option<i32>,
    #[serde(rename = "additional_information", default)]
    pub additional_information: Option<String>,
    #[serde(rename = "industry_partner", default)]
    pub industry_partner: Option<String>,
    #[serde(rename = "international_research_status", default)]
    pub international_research_status: Option<String>,
    #[serde(rename = "methodology", default)]
    pub methodology: Option<String>,
    #[serde(rename = "national_research_status", default)]
    pub national_research_status: Option<String>,
    #[serde(rename = "outside_tiet_uq_experts", default)]
    pub outside_tiet_uq_experts: Option<Vec<String>>,
    #[serde(rename = "outside_tiet_uq_experts_new", default)]
    pub outside_tiet_uq_experts_new: Option<String>,
    #[serde(rename = "problem_definition", default)]
    pub problem_definition: Option<String>,
    #[serde(rename = "project_deliverables", default)]
    pub project_deliverables: Option<Vec<String>>,
    #[serde(rename = "project_deliverables_new", default)]
    pub project_deliverables_new: Option<String>,
    #[serde(rename = "project_duration", default)]
    pub project_duration: Option<ProjectDuration>,
    #[serde(rename = "project_importance", default)]
    pub project_importance: Option<String>,
    #[serde(rename = "project_keywords", default)]
    pub project_keywords: Option<Vec<String>>,
    #[serde(rename = "project_objective", default)]
    pub project_objective: Option<Vec<String>>,
    #[serde(rename = "project_objective_new", default)]
    pub project_objective_new: Option<String>,
    #[serde(rename = "project_origin", default)]
    pub project_origin: Option<String>,
    #[serde(rename = "project_summary", default)]
    pub project_summary: Option<String>,
    #[serde(rename = "project_timeline", default)]
    pub project_timeline: Option<Vec<String>>,
    #[serde(rename = "project_timeline_new", default)]
    pub project_timeline_new: Option<String>,
    #[serde(rename = "project_title", default)]
    pub project_title: Option<String>,
    #[serde(rename = "proposal_ppt", default)]
    pub proposal_ppt: Option<Vec<String>>,
    #[serde(rename = "references", default)]
    pub references: Option<Vec<String>>,
    #[serde(rename = "references_new", default)]
    pub references_new: Option<String>,
    #[serde(rename = "society_impact", default)]
    pub society_impact: Option<String>,
    #[serde(rename = "supporting_documents", default)]
    pub supporting_documents: Option<Vec<String>>,
    #[serde(rename = "tiet_uq_facilities", default)]
    pub tiet_uq_facilities: Option<String>,
    #[serde(rename = "timeline_diagram", default)]
    pub timeline_diagram: Option<Vec<String>>,
    #[serde(rename = "trl_level", default)]
    pub trl_level: Option<String>,
    #[serde(rename = "work_organization", default)]
    pub work_organization: Option<String>,
    #[serde(rename = "budget", default)]
    pub budget: Option<Vec<BudgetCategory>>,
    #[serde(rename = "coPI", default)]
    pub co_pi: Option<Vec<CoPI>>,
    #[serde(rename = "discardedAt", default)]
    pub discarded_at: Option<serde_json::Value>,
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
    project_duration: Option<ProjectDuration>,
    project_keywords: Option<Vec<String>>,
    project_objective: Option<Vec<String>>,
    project_objective_new: Option<String>,
    project_summary: Option<String>,
    project_title: Option<String>,
    supporting_documents: Option<Vec<String>>,
    co_pi: Option<Vec<CoPI>>,
    discarded_at: Option<String>,
}

#[allow(dead_code)]
impl SubmissionResponse {
    pub fn from_submissions(submissions: Vec<Submission>) -> Vec<Self> {
        submissions
            .into_iter()
            .map(|s| Self {
                id: s.id.map_or_else(|| "".to_string(), |v| v.to_string()),
                status: s.status,
                track: s.track,
                track_code: s.track_code,
                unique_id: s.unique_id,
                updated_at: s.updated_at.to_string(),
                user: s.user,
                created_at: s.created_at,
                max_filled: s.max_filled,
                additional_information: s.additional_information,
                project_duration: s.project_duration,
                project_keywords: s.project_keywords,
                project_objective: s.project_objective,
                project_objective_new: s.project_objective_new,
                project_summary: s.project_summary,
                project_title: s.project_title,
                supporting_documents: s.supporting_documents,
                co_pi: s.co_pi,
                discarded_at: s.discarded_at.map(|d| d.to_string()),
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
