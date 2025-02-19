use docx_rs::{Paragraph, Run, TableCell};

pub fn create_paragraph(text: &str) -> Paragraph {
    Paragraph::new().add_run(Run::new().add_text(text).size(28).color("#2C3E50"))
}
pub fn empty_row(cols: usize) -> Vec<TableCell> {
    (0..cols)
        .map(|_| TableCell::new().add_paragraph(Paragraph::new()))
        .collect()
}
