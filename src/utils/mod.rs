use std::process::Command;
use std::fs;
use std::path::Path;

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
pub fn convert_docx_to_pdf(input: &str, output_dir: &str, output_filename: Option<&str>) -> std::io::Result<String> {
    // Create the output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    // Run LibreOffice to convert the DOCX to PDF
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

    if !status.success() {
        eprintln!("Conversion failed.");
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "LibreOffice conversion failed"));
    }

    // Get the base filename without extension
    let input_path = Path::new(input);
    let input_stem = input_path.file_stem().unwrap().to_str().unwrap();
    
    // Calculate the default output path from LibreOffice
    let default_output = format!("{}/{}.pdf", output_dir, input_stem);
    
    // If custom output filename is provided, rename the file
    if let Some(output_name) = output_filename {
        let custom_output = format!("{}/{}", output_dir, output_name);
        fs::rename(&default_output, &custom_output)?;
        println!("Converted to PDF and renamed to: {}", custom_output);
        Ok(custom_output)
    } else {
        println!("Converted to PDF: {}", default_output);
        Ok(default_output)
    }
}
