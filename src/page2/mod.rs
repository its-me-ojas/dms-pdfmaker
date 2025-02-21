use crate::utils::{create_paragraph, empty_row};
use docx_rs::{
    AlignmentType, Paragraph, Run, Table, TableCell, TableCellBorderPosition, TableRow, WidthType,
};

pub fn page2_content_withTable() -> (Vec<Paragraph>, Table) {
    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new()
                .width(500, WidthType::Dxa)
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(""))),
            TableCell::new()
                .width(2000, WidthType::Dxa)
                .add_paragraph(create_paragraph("Item")),
            TableCell::new()
                .width(1000, WidthType::Dxa)
                .clear_border(TableCellBorderPosition::Right)
                .add_paragraph(create_paragraph("")),
            TableCell::new()
                .width(1000, WidthType::Dxa)
                .add_paragraph(create_paragraph("Budget")),
            TableCell::new()
                .width(1000, WidthType::Dxa)
                .clear_border(TableCellBorderPosition::Left)
                .add_paragraph(create_paragraph("")),
            TableCell::new()
                .width(1500, WidthType::Dxa)
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
            TableCell::new().add_paragraph(create_paragraph("B")),
            TableCell::new().add_paragraph(create_paragraph("Equipment")),
            TableCell::new(),
            TableCell::new(),
            TableCell::new(),
            TableCell::new(),
        ]),
    ])
    .width(100, WidthType::Pct);

    let paragraphs = vec![
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Section A").bold().size(48)),
        Paragraph::new(),
        create_paragraph("1. Project Title:"),
        create_paragraph("2. Sub Area:"),
        create_paragraph("3. Total Cost:"),
        create_paragraph("4. Duration in months:"),
        create_paragraph("5. Name of the Investigator:"),
        create_paragraph("   • Designation:"),
        create_paragraph("   • E-Code:"),
        create_paragraph("   • Contact:"),
        create_paragraph("   • Email:"),
        create_paragraph("   • Department /School"),
        create_paragraph("   • Area of Specialization"),
        create_paragraph("   • Date of Joining the Institute"),
        create_paragraph("   • Date of Award of Ph.D Degree"),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Section B").bold().size(48)),
        Paragraph::new(),
        create_paragraph("6. Project Title"),
        create_paragraph("7. Project summary (maximum 500 words)"),
        create_paragraph("8. Key words:"),
        create_paragraph("9. Introduction (under the following heads):"),
        create_paragraph("   • Origin of the proposal"),
        create_paragraph("   • Definition of the problem"),
        create_paragraph("   • Objective"),
        create_paragraph("10. Review and status of Research and Development in the subject:"),
        create_paragraph("   • International Status"),
        create_paragraph("   • National Status"),
        create_paragraph(
            "   • Importance of the proposed project in the context of current status",
        ),
        create_paragraph("   • References"),
        create_paragraph("11. Work plan:"),
        create_paragraph("   • Methodology"),
        create_paragraph("   • Organization of work elements"),
        create_paragraph(
            "   • Time schedule of activities giving milestones (also append to bar diagram)",
        ),
        create_paragraph("   • Deliverables"),
        create_paragraph("12. Facilities available at TIET"),
        Paragraph::new(),
        Paragraph::new(),
        create_paragraph(
            "13. Budget requirement with justification (Consumables, Equipment, Contingency)",
        ),
        Paragraph::new(),
        Paragraph::new(),
    ];

    (paragraphs, table)
}

pub fn page2_content_signatures() -> Vec<Paragraph> {
    vec![
        Paragraph::new(),
        create_paragraph("14. Any other information which the investigator may like to give in support of his proposal"),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Left)
            .add_run(Run::new().add_text("Signature of the Applicant").size(24)),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new(),
        Paragraph::new()
            .align(AlignmentType::Right)
            .add_run(Run::new().add_text("Head of the Department").bold().size(26)),

    ]
}
