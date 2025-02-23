use crate::models::Submission;
use docx_rs::{AlignmentType, Paragraph, Pic, Run};
use std::fs::File;
use std::io::Read;

pub fn page1_content(submission: &Submission) -> Vec<Paragraph> {
    let mut file = File::open("./public/thapar_logo.png").expect("Failed to open logo file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read logo file");
    let image = Pic::new(&buffer);
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
            .add_run(Run::new().add_text("<Designation>").bold().size(48)),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("<Department>").bold().size(48)),
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
