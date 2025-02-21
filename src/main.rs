use docx_rs::{Docx, Paragraph};
use page1::page1_content;
use page2::{page2_content_signatures, page2_content_withTable};

mod page1;
mod page2;
mod utils;

fn main() {
    let path = std::path::Path::new("dms.docx");
    let file = std::fs::File::create(path).unwrap();

    let mut doc = Docx::new();
    // page 1 content
    for paragraph in page1_content() {
        doc = doc.add_paragraph(paragraph);
    }

    // page break
    doc = doc.add_paragraph(Paragraph::new().page_break_before(true));

    // page 2 content
    let (paragraphs, table) = page2_content_withTable();
    for paragraph in paragraphs {
        doc = doc.add_paragraph(paragraph);
    }
    doc = doc.add_table(table);
    for paragraph in page2_content_signatures() {
        doc = doc.add_paragraph(paragraph);
    }

    doc.build().pack(file).unwrap();

    // converting to pdf
    utils::convert_docx_to_pdf("dms.docx", "output").unwrap();
}
