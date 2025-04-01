use crate::models::Submission;
use docx_rs::{AlignmentType, Paragraph, Pic, Run};
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
pub fn page1_content(submission: &Submission) -> Vec<Paragraph> {
    let mut file = match File::open("./public/thapar_logo.png") {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open logo file: {}", e);
            // Return a placeholder if logo can't be loaded
            return create_placeholder_content(submission);
        }
    };
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        println!("Failed to read logo file: {}", e);
        return create_placeholder_content(submission);
    }
    
    let image = Pic::new(&buffer);
    
    let co_pi_names = match &submission.co_pi {
        Some(co_pis) if !co_pis.is_empty() => {
            let names: Vec<String> = co_pis.iter().map(|co_pi| co_pi.name.clone()).collect();
            names.join(", ")
        },
        _ => "".to_string()
    };
    
    vec![
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text(
                    submission
                        .project_title
                        .as_ref()
                        .unwrap_or(&"<Title>".to_string()),
                )
                .bold()
                .size(48),
        ),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("SEED MONEY GRANT PROPOSAL")
                .bold()
                .size(48),
        ),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_image(image)),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text(&submission.user).bold().size(48)),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Principal Investigator").bold().size(48)),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text(&co_pi_names).bold().size(36)),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Co-Principal Investigator(s)").bold().size(36)),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("Dean, Research and Development")
                .bold()
                .size(48),
        ),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("THAPAR INSTITUTE OF ENGINEERING & TECHNOLOGY")
                .bold()
                .size(36),
        ),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("BHADSON ROAD").bold().size(36)),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("PATIALA-147004").bold().size(36)),
    ]
}

fn create_placeholder_content(submission: &Submission) -> Vec<Paragraph> {
    // A simplified version without the logo
    vec![
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text(
                    submission
                        .project_title
                        .as_ref()
                        .unwrap_or(&"<Title>".to_string()),
                )
                .bold()
                .size(48),
        ),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("SEED MONEY GRANT PROPOSAL")
                .bold()
                .size(48),
        ),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text(&submission.user).bold().size(48)),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Principal Investigator").bold().size(48)),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("Dean, Research and Development")
                .bold()
                .size(48),
        ),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("THAPAR INSTITUTE OF ENGINEERING & TECHNOLOGY")
                .bold()
                .size(36),
        ),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("BHADSON ROAD").bold().size(36)),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("PATIALA-147004").bold().size(36)),
    ]
}
