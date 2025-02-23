use std::process::Command;

use docx_rs::{Paragraph, Run, TableCell};

pub fn create_paragraph(text: &str) -> Paragraph {
    Paragraph::new().add_run(Run::new().add_text(text).size(28).color("#2C3E50"))
}
pub fn empty_row(cols: usize) -> Vec<TableCell> {
    (0..cols)
        .map(|_| TableCell::new().add_paragraph(Paragraph::new()))
        .collect()
}

#[allow(dead_code)]
pub fn convert_docx_to_pdf(input: &str, output_dir: &str) -> std::io::Result<()> {
    let status = Command::new("libreoffice")
        .args([
            "--headless",
            "--convert-to",
            "pdf",
            "--outdir",
            output_dir,
            input,
        ])
        .status()?;

    if status.success() {
        println!("converted to pdf!");
    } else {
        eprintln!("conversion failed.");
    }

    Ok(())
}
