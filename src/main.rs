use docx_rs::{
    AlignmentType, BorderType, Docx, Paragraph, Run, Table, TableCell, TableCellBorder, TableRow,
    WidthType,
};
fn main() {
    let path = std::path::Path::new("dms.docx");
    let file = std::fs::File::create(path).unwrap();

    fn create_paragraph(text: &str) -> Paragraph {
        Paragraph::new().add_run(Run::new().add_text(text).size(28).color("#2C3E50"))
    }
    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(create_paragraph("Budget"))
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Top)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Bottom)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Left)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Right)
                        .set_border_type(BorderType::Single),
                ),
            TableCell::new()
                .add_paragraph(create_paragraph("Justification"))
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Top)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Bottom)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Left)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Right)
                        .set_border_type(BorderType::Single),
                ),
            TableCell::new()
                .add_paragraph(create_paragraph("A"))
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Top)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Bottom)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Left)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Right)
                        .set_border_type(BorderType::Single),
                ),
        ]),
        TableRow::new(vec![
            TableCell::new()
                .add_paragraph(create_paragraph("B"))
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Top)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Bottom)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Left)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Right)
                        .set_border_type(BorderType::Single),
                ),
            TableCell::new()
                .add_paragraph(create_paragraph("Equipment"))
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Top)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Bottom)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Left)
                        .set_border_type(BorderType::Single),
                )
                .set_border(
                    TableCellBorder::new(TableCellBorderPosition::Right)
                        .set_border_type(BorderType::Single),
                ),
        ]),
    ])
    .width(WidthType::Auto, 0);

    let doc = Docx::new()
        .add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(Run::new().add_text("Section A").bold().size(48)),
        )
        .add_paragraph(Paragraph::new())
        .add_paragraph(create_paragraph("1. Project Title:"))
        .add_paragraph(create_paragraph("2. Sub Area:"))
        .add_paragraph(create_paragraph("3. Total Cost:"))
        .add_paragraph(create_paragraph("4. Duration in months:"))
        .add_paragraph(create_paragraph("5. Name of the Investigator:"))
        .add_paragraph(create_paragraph("   • Designation:"))
        .add_paragraph(create_paragraph("   • E-Code:"))
        .add_paragraph(create_paragraph("   • Contact:"))
        .add_paragraph(create_paragraph("   • Email:"))
        .add_paragraph(create_paragraph("   • Department /School"))
        .add_paragraph(create_paragraph("   • Area of Specialization"))
        .add_paragraph(create_paragraph("   • Date of Joining the Institute"))
        .add_paragraph(create_paragraph("   • Date of Award of Ph.D Degree"))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new())
        .add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(Run::new().add_text("Section B").bold().size(48)),
        )
        .add_paragraph(Paragraph::new())
        .add_paragraph(create_paragraph("6. Project Title"))
        .add_paragraph(create_paragraph("7. Project summary (maximum 500 words)"))
        .add_paragraph(create_paragraph("8. Key words:"))
        .add_paragraph(create_paragraph(
            "9. Introduction (under the following heads):",
        ))
        .add_paragraph(create_paragraph("   • Origin of the proposal"))
        .add_paragraph(create_paragraph("   • Definition of the problem"))
        .add_paragraph(create_paragraph("   • Objective"))
        .add_paragraph(create_paragraph(
            "10. Review and status of Research and Development in the subject:",
        ))
        .add_paragraph(create_paragraph("   • International Status"))
        .add_paragraph(create_paragraph("   • National Status"))
        .add_paragraph(create_paragraph(
            "   • Importance of the proposed project in the context of current status",
        ))
        .add_paragraph(create_paragraph("   • References"))
        .add_paragraph(create_paragraph("11. Work plan:"))
        .add_paragraph(create_paragraph("   • Methodology"))
        .add_paragraph(create_paragraph("   • Organization of work elements"))
        .add_paragraph(create_paragraph(
            "   • Time schedule of activities giving milestones (also append to bar diagram)",
        ))
        .add_paragraph(create_paragraph("   • Deliverables"))
        .add_paragraph(create_paragraph("12. Facilities available at TIET"))
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new())
        .add_paragraph(create_paragraph(
            "13. Budget requirement with justification (Consumables, Equipment, Contingency)",
        ))
        .add_table(table)
        .add_paragraph(create_paragraph(
            "14. Any other information which the investigator may like to give in support of his proposal.",
        ));

    doc.build().pack(file).unwrap();
}
