use std::process::Command;
use std::fs;
use std::path::Path;

use docx_rs::{Paragraph, Run, TableCell, RunFonts};

pub fn create_paragraph(text: &str) -> Paragraph {
    Paragraph::new().add_run(
        Run::new()
            .add_text(text)
            .size(28)
            .fonts(RunFonts::new().ascii("Calibri"))  // Using Calibri for table content
            .color("#333333")  // Dark gray for better readability
    )
}
pub fn empty_row(cols: usize) -> Vec<TableCell> {
    (0..cols)
        .map(|_| TableCell::new().add_paragraph(Paragraph::new()))
        .collect()
}

#[allow(dead_code)]
pub fn convert_docx_to_pdf(input: &str, output_dir: &str, output_filename: Option<&str>) -> std::io::Result<String> {
    // Print debug information
    println!("Starting PDF conversion");
    println!("Input file: {}", input);
    println!("Output directory: {}", output_dir);
    println!("Output filename: {:?}", output_filename);
    
    // Create the output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;
    println!("Created output directory: {}", output_dir);

    // First check if the input file exists
    if !Path::new(input).exists() {
        println!("Error: Input file does not exist: {}", input);
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Input file not found"));
    }
    
    // Run LibreOffice to convert the DOCX to PDF with more verbose output
    println!("Executing LibreOffice command...");
    let output = Command::new("libreoffice")
        .args([
            "--headless",
            "--convert-to",
            "pdf",
            "--outdir",
            output_dir,
            input,
        ])
        .output()?;
    
    println!("LibreOffice stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("LibreOffice stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    let status = output.status;
    if !status.success() {
        eprintln!("Conversion failed with status: {}", status);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "LibreOffice conversion failed"));
    }

    // Get the base filename without extension
    let input_path = Path::new(input);
    let input_stem = input_path.file_stem().unwrap().to_str().unwrap();
    
    // Calculate the default output path from LibreOffice
    let default_output = format!("{}/{}.pdf", output_dir, input_stem);
    println!("Default output path: {}", default_output);
    
    // Check if the default output file exists
    if !Path::new(&default_output).exists() {
        println!("Error: Default output file not created: {}", default_output);
        // List files in the output directory
        println!("Files in output directory:");
        if let Ok(entries) = fs::read_dir(output_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  {}", entry.path().display());
                }
            }
        }
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Output PDF not created"));
    }
    
    // If custom output filename is provided, rename the file
    if let Some(output_name) = output_filename {
        let custom_output = format!("{}/{}", output_dir, output_name);
        println!("Renaming to custom output: {}", custom_output);
        fs::rename(&default_output, &custom_output)?;
        println!("Successfully renamed to: {}", custom_output);
        Ok(custom_output)
    } else {
        println!("Using default output path: {}", default_output);
        Ok(default_output)
    }
}
