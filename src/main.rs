use page2::page2;

mod page2;
fn main() {
    let path = std::path::Path::new("dms.docx");
    let file = std::fs::File::create(path).unwrap();

    let doc = page2();

    doc.build().pack(file).unwrap();
}
