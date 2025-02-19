use docx_rs::{
    AlignmentType, Docx, Paragraph, Run, Table, TableCell, TableCellBorderPosition, TableRow,
    WidthType,
};

use crate::utils::{create_paragraph, empty_row};

pub fn page2() -> Docx {
    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new()
                .width(10, WidthType::Pct)
                .add_paragraph(Paragraph::new()),
            TableCell::new()
                .width(30, WidthType::Pct)
                .add_paragraph(create_paragraph("Item")),
            TableCell::new()
                .width(12, WidthType::Pct)
                .clear_border(TableCellBorderPosition::Right)
                .add_paragraph(create_paragraph("")),
            TableCell::new()
                .width(12, WidthType::Pct)
                .add_paragraph(create_paragraph("Budget")),
            TableCell::new()
                .width(12, WidthType::Pct)
                .clear_border(TableCellBorderPosition::Left)
                .add_paragraph(create_paragraph("")),
            TableCell::new()
                .width(24, WidthType::Pct)
                .add_paragraph(create_paragraph("Justification")),
        ]),
        TableRow::new(vec![
            TableCell::new().add_paragraph(create_paragraph("A")),
            TableCell::new().add_paragraph(create_paragraph("Recurring")),
            TableCell::new().add_paragraph(create_paragraph("1st year")),
            TableCell::new().add_paragraph(create_paragraph("2nd year")),
            TableCell::new().add_paragraph(create_paragraph("Total")),
            TableCell::new().add_paragraph(Paragraph::new()),
        ]),
        TableRow::new(empty_row(6)),
        TableRow::new(empty_row(6)),
        TableRow::new(empty_row(6)),
        TableRow::new(vec![
            TableCell::new().add_paragraph(create_paragraph("B")), // B for Equipment
            TableCell::new().add_paragraph(create_paragraph("Equipment")),
            TableCell::new(), // Empty Budget columns
            TableCell::new(),
            TableCell::new(),
            TableCell::new(),
        ]),
    ])
    .width(100, WidthType::Pct);

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
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new())
        .add_table(table)
        .add_paragraph(Paragraph::new())
        .add_paragraph(Paragraph::new())
        .add_paragraph(create_paragraph(
            "14. Any other information which the investigator may like to give in support of his proposal.",
        ));
    doc
}
