use crate::{
    models::Submission,
    utils::{create_paragraph, empty_row},
};
use docx_rs::{
    AlignmentType, Paragraph, Run, Table, TableCell, TableCellBorderPosition, TableRow, WidthType,
    LineSpacing,
};

// Helper function to create a paragraph with bold heading and normal text content
fn create_paragraph_with_bold_heading(heading: &str, content: &str) -> Paragraph {
    let mut paragraph = Paragraph::new();
    paragraph = paragraph.add_run(Run::new().add_text(heading).bold().size(28).color("#2C3E50"));
    paragraph = paragraph.add_run(Run::new().add_text(content).size(28).color("#2C3E50"));
    paragraph
}

// Create an empty paragraph for spacing
fn create_spacing_paragraph() -> Paragraph {
    Paragraph::new().line_spacing(LineSpacing::new().after(240)) // 240 is approximately 12pt spacing
}

#[allow(dead_code)]
pub fn page2_content_with_table(submission: &Submission) -> (Vec<Paragraph>, Table) {
    // Create the base budget table
    let mut table_rows = vec![
        TableRow::new(vec![
            TableCell::new()
                .width(500, WidthType::Dxa)
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(""))),
            TableCell::new()
                .width(2000, WidthType::Dxa)
                .add_paragraph(create_paragraph("Item")),
            TableCell::new()
                .width(1000, WidthType::Dxa)
                .clear_border(TableCellBorderPosition::Right)
                .add_paragraph(create_paragraph("Year 1")),
            TableCell::new()
                .width(1000, WidthType::Dxa)
                .add_paragraph(create_paragraph("Year 2")),
            TableCell::new()
                .width(1000, WidthType::Dxa)
                .clear_border(TableCellBorderPosition::Left)
                .add_paragraph(create_paragraph("Year 3")),
            TableCell::new()
                .width(1500, WidthType::Dxa)
                .add_paragraph(create_paragraph("Total")),
            TableCell::new()
                .width(2000, WidthType::Dxa)
                .add_paragraph(create_paragraph("Justification")),
        ]),
    ];

    // Add budget items if available
    if let Some(budget_categories) = &submission.budget {
        for (index, category) in budget_categories.iter().enumerate() {
            // Add category header
            table_rows.push(
        TableRow::new(vec![
                    TableCell::new().add_paragraph(create_paragraph(&format!("{}", index + 1))),
                    TableCell::new().add_paragraph(create_paragraph(&category.category_type)),
                    TableCell::new().add_paragraph(Paragraph::new()),
                    TableCell::new().add_paragraph(Paragraph::new()),
                    TableCell::new().add_paragraph(Paragraph::new()),
                    TableCell::new().add_paragraph(Paragraph::new()),
                    TableCell::new().add_paragraph(Paragraph::new()),
                ]),
            );

            // Add items for this category
            for item in &category.items {
                let mut year_cells = Vec::new();
                for (i, &year_amount) in item.years.iter().enumerate().take(3) {
                    if i < item.years.len() {
                        year_cells.push(
                            TableCell::new().add_paragraph(create_paragraph(&year_amount.to_string())),
                        );
                    } else {
                        year_cells.push(TableCell::new().add_paragraph(create_paragraph("0")));
                    }
                }

                // Ensure we have 3 year cells
                while year_cells.len() < 3 {
                    year_cells.push(TableCell::new().add_paragraph(create_paragraph("0")));
                }

                table_rows.push(TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new()),
                    TableCell::new().add_paragraph(create_paragraph(&item.heading)),
                    year_cells[0].clone(),
                    year_cells[1].clone(),
                    year_cells[2].clone(),
                    TableCell::new().add_paragraph(create_paragraph(&item.total.to_string())),
                    TableCell::new().add_paragraph(create_paragraph(&item.justification)),
                ]));
            }

            // Add an empty row after each category for better readability
            table_rows.push(TableRow::new(empty_row(7)));
        }
    } else {
        // If no budget is provided, add default rows
        table_rows.push(TableRow::new(vec![
            TableCell::new().add_paragraph(create_paragraph("A")),
            TableCell::new().add_paragraph(create_paragraph("Recurring")),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new().add_paragraph(Paragraph::new()),
        ]));
        table_rows.push(TableRow::new(empty_row(7)));
        table_rows.push(TableRow::new(empty_row(7)));
        table_rows.push(TableRow::new(vec![
            TableCell::new().add_paragraph(create_paragraph("B")),
            TableCell::new().add_paragraph(create_paragraph("Non-Recurring")),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new().add_paragraph(Paragraph::new()),
            TableCell::new().add_paragraph(Paragraph::new()),
        ]));
        table_rows.push(TableRow::new(empty_row(7)));
    }

    let table = Table::new(table_rows).width(100, WidthType::Pct);

    let total_months = submission.project_duration.as_ref().map_or(0, |duration| {
        (duration.years * 12) + duration.months + (duration.days as f32 / 30.0).floor() as i32
    });

    // Extract values as owned strings to avoid temporary value issues
    let empty_string = String::new();
    let project_title = submission.project_title.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let track = submission.track.clone();
    let total_cost = calculate_total_budget(submission);
    let track_code = submission.track_code.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let trl_level = submission.trl_level.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let project_summary = submission.project_summary.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let project_keywords = submission.project_keywords.as_ref()
        .map(|keyword| keyword.join(", "))
        .unwrap_or_else(|| empty_string.clone());
    let project_origin = submission.project_origin.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let problem_definition = submission.problem_definition.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let objectives = format_objectives(submission);
    let international_status = submission.international_research_status.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let national_status = submission.national_research_status.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let project_importance = submission.project_importance.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let references = format_references(submission);
    let methodology = submission.methodology.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let work_organization = submission.work_organization.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let timeline = format_timeline(submission);
    let deliverables = format_deliverables(submission);
    let tiet_uq_facilities = submission.tiet_uq_facilities.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let industry_partner = submission.industry_partner.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let experts = format_experts(submission);
    let society_impact = submission.society_impact.as_ref().map_or_else(|| empty_string.clone(), |s| s.clone());
    let user_email = submission.user.clone();

    let mut paragraphs = Vec::new();
    
    // Section A header
    paragraphs.push(
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Section A").bold().size(48))
    );
    paragraphs.push(create_spacing_paragraph());
    
    // Section A content - add spacing between each point
    paragraphs.push(create_paragraph_with_bold_heading("1. Project Title: ", &project_title));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("2. Sub Area: ", &track));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("3. Total Cost: ", &total_cost));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("4. Duration in months: ", &total_months.to_string()));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("5. Name of the Investigator:", ""));
    paragraphs.push(create_paragraph_with_bold_heading("   • Designation:", ""));
    paragraphs.push(create_paragraph_with_bold_heading("   • E-Code: ", &track_code));
    paragraphs.push(create_paragraph_with_bold_heading("   • Contact:", ""));
    paragraphs.push(create_paragraph_with_bold_heading("   • Email: ", &user_email));
    paragraphs.push(create_paragraph_with_bold_heading("   • Department /School:", ""));
    paragraphs.push(create_paragraph_with_bold_heading("   • Area of Specialization:", ""));
    paragraphs.push(create_paragraph_with_bold_heading("   • Date of Joining the Institute:", ""));
    paragraphs.push(create_paragraph_with_bold_heading("   • TRL Level: ", &trl_level));
    paragraphs.push(create_spacing_paragraph());
    
    // Line break before Section B
    paragraphs.push(Paragraph::new());
    paragraphs.push(Paragraph::new());
    
    // Section B header
    paragraphs.push(
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Section B").bold().size(48))
    );
    paragraphs.push(create_spacing_paragraph());
    
    // Section B content - add spacing between each point
    paragraphs.push(create_paragraph_with_bold_heading("6. Project Title: ", &project_title));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("7. Project Summary (maximum 500 words): ", &project_summary));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("8. Keywords: ", &project_keywords));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("9. Introduction (under the following heads):", ""));
    paragraphs.push(create_paragraph_with_bold_heading("   • Origin of the proposal: ", &project_origin));
    paragraphs.push(create_paragraph_with_bold_heading("   • Definition of the problem: ", &problem_definition));
    paragraphs.push(create_paragraph_with_bold_heading("   • Objective: ", &objectives));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("10. Review and status of Research and Development in the subject:", ""));
    paragraphs.push(create_paragraph_with_bold_heading("   • International Status: ", &international_status));
    paragraphs.push(create_paragraph_with_bold_heading("   • National Status: ", &national_status));
    paragraphs.push(create_paragraph_with_bold_heading("   • Importance of the proposed project in the context of current status: ", &project_importance));
    paragraphs.push(create_paragraph_with_bold_heading("   • References: ", &references));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("11. Work plan:", ""));
    paragraphs.push(create_paragraph_with_bold_heading("   • Methodology: ", &methodology));
    paragraphs.push(create_paragraph_with_bold_heading("   • Organization of work elements: ", &work_organization));
    paragraphs.push(create_paragraph_with_bold_heading("   • Time schedule of activities giving milestones: ", &timeline));
    paragraphs.push(create_paragraph_with_bold_heading("   • Deliverables: ", &deliverables));
    paragraphs.push(create_spacing_paragraph());
    
    paragraphs.push(create_paragraph_with_bold_heading("12. Facilities available at TIET/UQ: ", &tiet_uq_facilities));
    paragraphs.push(create_paragraph_with_bold_heading("   • Industry Partner: ", &industry_partner));
    paragraphs.push(create_paragraph_with_bold_heading("   • Outside TIET/UQ Experts: ", &experts));
    paragraphs.push(create_paragraph_with_bold_heading("   • Society Impact: ", &society_impact));
    paragraphs.push(create_spacing_paragraph());
    
    // Add space before budget section
    paragraphs.push(Paragraph::new());
    paragraphs.push(create_paragraph_with_bold_heading(
        "13. Budget requirement with justification (Consumables, Equipment, Contingency)", ""
    ));
    paragraphs.push(create_spacing_paragraph());

    (paragraphs, table)
}

fn calculate_total_budget(submission: &Submission) -> String {
    let mut total = 0;
    
    if let Some(budget_categories) = &submission.budget {
        for category in budget_categories {
            for item in &category.items {
                total += item.total;
            }
        }
    }
    
    total.to_string()
}

fn format_objectives(submission: &Submission) -> String {
    if let Some(objectives) = &submission.project_objective {
        if !objectives.is_empty() {
            return objectives.join("; ");
        }
    }
    
    submission.project_objective_new.as_ref().map_or_else(|| String::new(), |s| s.clone())
}

fn format_references(submission: &Submission) -> String {
    if let Some(references) = &submission.references {
        if !references.is_empty() {
            return references.join("; ");
        }
    }
    
    submission.references_new.as_ref().map_or_else(|| String::new(), |s| s.clone())
}

fn format_timeline(submission: &Submission) -> String {
    if let Some(timeline) = &submission.project_timeline {
        if !timeline.is_empty() {
            return timeline.join("; ");
        }
    }
    
    submission.project_timeline_new.as_ref().map_or_else(|| String::new(), |s| s.clone())
}

fn format_deliverables(submission: &Submission) -> String {
    if let Some(deliverables) = &submission.project_deliverables {
        if !deliverables.is_empty() {
            return deliverables.join("; ");
        }
    }
    
    submission.project_deliverables_new.as_ref().map_or_else(|| String::new(), |s| s.clone())
}

fn format_experts(submission: &Submission) -> String {
    if let Some(experts) = &submission.outside_tiet_uq_experts {
        if !experts.is_empty() {
            return experts.join("; ");
        }
    }
    
    submission.outside_tiet_uq_experts_new.as_ref().map_or_else(|| String::new(), |s| s.clone())
}

#[allow(dead_code)]
pub fn page2_content_signatures() -> Vec<Paragraph> {
    vec![
        Paragraph::new(),
        create_paragraph_with_bold_heading("14. Any other information which the investigator may like to give in support of his proposal", ""),
        create_spacing_paragraph(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Left)
            .add_run(Run::new().add_text("Signature of the Applicant").size(24)),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        // Paragraph::new(),
        // Paragraph::new(),
        // Paragraph::new(),
        // Paragraph::new(),
        // Paragraph::new(),
        // Paragraph::new()
        //     .align(AlignmentType::Right)
        //     .add_run(Run::new().add_text("Head of the Department").bold().size(26)),

    ]
}
