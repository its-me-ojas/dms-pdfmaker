# DMS PDF Maker

A Rust-based web service that generates standardized PDF documents for SEED Money Grant Proposals using a predefined template.

## Overview

This service provides API endpoints that generate formatted PDF documents containing SEED Money Grant Proposal templates and fetch submission data. The document includes:
- A cover page with institution details and logo
- Sections for project details
- A structured table for budget information
- Signature sections

## Prerequisites

- Rust (latest stable version)
- LibreOffice (for PDF conversion)
- Cargo (Rust's package manager)

## Dependencies

```toml
[dependencies]
axum = "0.7.1"
tokio = { version = "1.0", features = ["full"] }
docx-rs = "0.4.6"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
image = "0.24"
```

## Project Structure

```
dms-pdfmaker/
├── src/
│   ├── main.rs          # Main application entry point and server setup
│   ├── lib.rs           # Library module declarations
│   ├── models.rs        # Data structures and response types
│   ├── page1/           # Cover page content generation
│   │   └── mod.rs
│   ├── page2/           # Main content and table generation
│   │   └── mod.rs
│   └── utils/           # Utility functions
│       └── mod.rs
├── public/              # Static assets
│   └── thapar_logo.png
├── output/              # Generated PDF output directory
├── Cargo.toml
└── README.md
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/dms-pdfmaker.git
cd dms-pdfmaker
```

2. Install LibreOffice (if not already installed):
- Ubuntu/Debian:
  ```bash
  sudo apt-get install libreoffice
  ```
- MacOS:
  ```bash
  brew install --cask libreoffice
  ```
- Windows:
  Download and install from [LibreOffice website](https://www.libreoffice.org/download/download/)

3. Create output directory:
```bash
mkdir output
```

4. Build the project:
```bash
cargo build --release
```

## Usage

1. Start the server:
```bash
cargo run
```

2. The server will start on `http://0.0.0.0:8080` with the following endpoints:
- `GET /` - Returns a "Hello, World!" message (health check)
- `GET /generate` - Generates and returns the PDF document
- `GET /fetch-submissions` - Fetches all submissions from the DMS API

3. To generate a PDF:
```bash
curl http://localhost:8080/generate --output document.pdf
```

4. To fetch submissions:
```bash
curl http://localhost:8080/fetch-submissions
```

## API Endpoints

### GET /
- Health check endpoint
- Returns: "Hello, World!"

### GET /generate
- Generates a PDF document based on the template
- Returns:
  - Status: 200 OK
  - Content-Type: application/pdf
  - Body: PDF file
- Error Responses:
  - 500 Internal Server Error: If PDF generation fails
  - 404 Not Found: If no submitted applications found

### GET /fetch-submissions
- Fetches all submissions from the DMS API
- Returns:
  - Status: 200 OK
  - Content-Type: application/json
  - Body: Array of submission objects
- Error Responses:
  - 500 Internal Server Error: If fetching or parsing fails

## Error Handling

The service includes error handling for:
- API authentication failures
- Failed PDF generation
- File system operations
- Missing submissions
- JSON parsing errors

## File Management

- Generated DOCX files are temporary and automatically cleaned up
- PDFs are stored in the `output/` directory before being served
- Each file is named with the submission's unique ID

## Authentication

The service uses token-based authentication to communicate with the DMS API. Credentials are currently hardcoded for development purposes.
