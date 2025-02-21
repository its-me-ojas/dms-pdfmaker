use std::fs::File;
use std::io::Read;

use docx_rs::{AlignmentType, Paragraph, Pic, Run};

pub fn page1_content() -> Vec<Paragraph> {
    let mut file = File::open("./public/thapar_logo.png").expect("Failed to open logo file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read logo file");
    let image = Pic::new(&buffer);
    vec![
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("<Title>").bold().size(48)),
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
            .add_run(Run::new().add_text("<Name>").bold().size(48)),
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
