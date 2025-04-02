use crate::models::Submission;
use docx_rs::{AlignmentType, Paragraph, Pic, Run};
use std::fs::File;
use std::io::Read;
use chrono::Local;

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
    let current_date = Local::now();
    let formatted_date = format!("{} {}", current_date.format("%B"), current_date.format("%Y"));
    
    // Maximum number of Co-PIs we'll display individually
    const MAX_COPI_DISPLAY: usize = 5;
    
    // Extract Co-PI emails
    let mut co_pi_emails = Vec::new();
    if let Some(co_pis) = &submission.co_pi {
        for co_pi in co_pis.iter().take(MAX_COPI_DISPLAY) {
            co_pi_emails.push(co_pi.email.clone());
        }
    }
    
    // Check if there are any co-PIs
    let has_co_pis = !co_pi_emails.is_empty();
    
    let mut paragraphs = vec![
        // Project Title at the top
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text(
                    submission
                        .project_title
                        .as_ref()
                        .unwrap_or(&"<Title>".to_string()),
                )
                .bold()
                .size(48)
        ),
        Paragraph::new(),
        Paragraph::new(),
        // COE-DSAI title 
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("COE-DSAI SEED GRANT PROPOSAL")
                .bold()
                .size(48)
                .color("#800020") // Burgundy color to match the image
        ),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        // PI Details subtitle
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("PI Details")
                .bold()
                .size(48)
        ),
        Paragraph::new(),
        // Principal Investigator label
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("Principal Investigator")
                .size(36)
        ),
        Paragraph::new(),
        // User email (PI email)
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text(&submission.user)
                .bold()
                .size(40)
        ),
        Paragraph::new(),
    ];
    
    // Add Co-PI label and emails if there are co-PIs
    if has_co_pis {
        // Add Co-PI label
        paragraphs.push(
            Paragraph::new().align(AlignmentType::Center).add_run(
                Run::new()
                    .add_text("Co-Principal Investigator(s)")
                    .size(32)
            )
        );
        paragraphs.push(Paragraph::new());
        
        // Add Co-PI emails
        for email in &co_pi_emails {
            paragraphs.push(
                Paragraph::new()
                    .align(AlignmentType::Center)
                    .add_run(Run::new().add_text(email).bold().size(36))
            );
        }
    }
    
    // Add fixed number of blank paragraphs to maintain layout
    // If we have Co-PIs, we'll add fewer blank paragraphs to compensate
    let blank_paragraphs_needed = if has_co_pis {
        // 2 for the Co-PI label + number of Co-PI emails
        8 - (2 + co_pi_emails.len()).min(7)
    } else {
        8 // Maximum number of blank paragraphs
    };
    
    for _ in 0..blank_paragraphs_needed {
        paragraphs.push(Paragraph::new());
    }
    
    // Add the rest of the content at the bottom of the page
    paragraphs.extend(vec![
        // Thapar Logo
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_image(image)),
        Paragraph::new(),
        // Centre of Excellence text
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Centre of Excellence in Data Science and").bold().size(36).color("#0066CC")), // Blue color to match the image
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Artificial Intelligence").bold().size(36).color("#0066CC")),
        Paragraph::new(),
        // Institution name
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("Thapar Institute of Engineering and Technology")
                .bold()
                .size(36)
        ),
        Paragraph::new(),
        // Address
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Patiala 147004").bold().size(36)),
        Paragraph::new(),
        // Date
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text(&formatted_date).size(36)),
    ]);
    
    paragraphs
}

fn create_placeholder_content(submission: &Submission) -> Vec<Paragraph> {
    // A simplified version without the logo
    let current_date = Local::now();
    let formatted_date = format!("{} {}", current_date.format("%B"), current_date.format("%Y"));
    
    // Maximum number of Co-PIs we'll display individually
    const MAX_COPI_DISPLAY: usize = 5;
    
    // Extract Co-PI emails
    let mut co_pi_emails = Vec::new();
    if let Some(co_pis) = &submission.co_pi {
        for co_pi in co_pis.iter().take(MAX_COPI_DISPLAY) {
            co_pi_emails.push(co_pi.email.clone());
        }
    }
    
    // Check if there are any co-PIs
    let has_co_pis = !co_pi_emails.is_empty();
    
    let mut paragraphs = vec![
        // Project Title at the top
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text(
                    submission
                        .project_title
                        .as_ref()
                        .unwrap_or(&"<Title>".to_string()),
                )
                .bold()
                .size(48)
        ),
        Paragraph::new(),
        Paragraph::new(),
        // COE-DSAI title
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("COE-DSAI SEED GRANT PROPOSAL")
                .bold()
                .size(48)
                .color("#800020") // Burgundy color to match the image
        ),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        // PI Details subtitle
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("PI Details")
                .bold()
                .size(48)
        ),
        Paragraph::new(),
        // Principal Investigator label
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("Principal Investigator")
                .size(36)
        ),
        Paragraph::new(),
        // User email (PI email)
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text(&submission.user)
                .bold()
                .size(40)
        ),
        Paragraph::new(),
    ];
    
    // Add Co-PI label and emails if there are co-PIs
    if has_co_pis {
        // Add Co-PI label
        paragraphs.push(
            Paragraph::new().align(AlignmentType::Center).add_run(
                Run::new()
                    .add_text("Co-Principal Investigator(s)")
                    .size(32)
            )
        );
        paragraphs.push(Paragraph::new());
        
        // Add Co-PI emails
        for email in &co_pi_emails {
            paragraphs.push(
                Paragraph::new()
                    .align(AlignmentType::Center)
                    .add_run(Run::new().add_text(email).bold().size(36))
            );
        }
    }
    
    // Add fixed number of blank paragraphs to maintain layout
    // If we have Co-PIs, we'll add fewer blank paragraphs to compensate
    let blank_paragraphs_needed = if has_co_pis {
        // 2 for the Co-PI label + number of Co-PI emails
        8 - (2 + co_pi_emails.len()).min(7)
    } else {
        8 // Maximum number of blank paragraphs
    };
    
    for _ in 0..blank_paragraphs_needed {
        paragraphs.push(Paragraph::new());
    }
    
    // Add the rest of the content at the bottom of the page
    paragraphs.extend(vec![
        // Centre of Excellence text
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Centre of Excellence in Data Science and").bold().size(36).color("#0066CC")), // Blue color to match the image
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Artificial Intelligence").bold().size(36).color("#0066CC")),
        Paragraph::new(),
        // Institution name
        Paragraph::new().align(AlignmentType::Center).add_run(
            Run::new()
                .add_text("Thapar Institute of Engineering and Technology")
                .bold()
                .size(36)
        ),
        Paragraph::new(),
        // Address
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Patiala 147004").bold().size(36)),
        Paragraph::new(),
        // Date
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text(&formatted_date).size(36)),
    ]);
    
    paragraphs
}
