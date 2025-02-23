# DMS PDF Maker

A Rust-based web service that generates standardized PDF documents for SEED Money Grant Proposals using a predefined template.

## Overview

This service provides API endpoints that generate formatted PDF documents containing SEED Money Grant Proposal templates and fetch submission data. The document includes:
- A cover page with institution details and logo
- Sections for project details
- A structured table for budget information
- Signature sections

## Prerequisites

### Local Development
- Rust (latest stable version)
- LibreOffice (for PDF conversion)
- Cargo (Rust's package manager)

### Docker Deployment
- Docker
- Docker Compose

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
│   └── thapar_logo.png  # Required logo file
├── output/              # Generated PDF output directory
├── Dockerfile
├── docker-compose.yml
├── Cargo.toml
└── README.md
```

## Installation

### Docker Installation (Recommended)

1. Clone the repository:
```bash
git clone https://github.com/yourusername/dms-pdfmaker.git
cd dms-pdfmaker
```

2. Ensure the `public` folder contains the required `thapar_logo.png` file

3. Build and run using Docker Compose:
```bash
docker-compose up --build
```

### Local Installation

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

The server will start on `http://0.0.0.0:8080` with the following endpoints:
- `GET /` - Returns a "Hello, World!" message (health check)
- `GET /generate` - Generates and returns the PDF document
- `GET /fetch-submissions` - Fetches all submissions from the DMS API

### Generate PDF
```bash
curl http://localhost:8080/generate --output document.pdf
```

### Fetch Submissions
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

## Docker Configuration

### Environment Variables
- `RUST_LOG=info` - Sets logging level

### Volumes
- `./data:/usr/src/app/data` - Mounted for temporary file storage

### Ports
- `8080:8080` - Exposed for HTTP service

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
