# DMS PDF Maker

A Rust-based web service that generates standardized PDF documents for SEED Money Grant Proposals using a predefined template.

## Overview

This service provides API endpoints that generate formatted PDF documents containing SEED Money Grant Proposal templates and fetch submission data. The document includes:
- A cover page with institution details and logo
- Sections for project details with proper spacing between numbered points
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
docx-rs = "0.4.17"
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

5. Run the server:
```bash
./target/release/dms-pdfmaker
```

## Usage

The server will start on `http://0.0.0.0:8080` with the following endpoints:
- `GET /` - Returns a "Hello, World!" message (health check)
- `GET /generate` - Generates and returns the PDF document using the first submitted application
- `GET /fetch-submissions` - Fetches all submissions from the DMS API
- `POST /generate-pdf` - Generates a PDF document based on the submitted JSON data

### Generate PDF from API data
```bash
curl http://localhost:8080/generate --output document.pdf
```

### Generate PDF from custom JSON
```bash
curl -X POST -H "Content-Type: application/json" --data @mock_submission.json http://localhost:8080/generate-pdf -o custom_document.pdf
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
- Generates a PDF document based on the first submitted application
- Returns:
  - Status: 200 OK
  - Content-Type: application/pdf
  - Body: PDF file
- Error Responses:
  - 500 Internal Server Error: If PDF generation fails
  - 404 Not Found: If no submitted applications found

### POST /generate-pdf
- Generates a PDF document based on the submitted JSON data
- Request:
  - Content-Type: application/json
  - Body: Submission JSON object
- Returns:
  - Status: 200 OK
  - Content-Type: application/pdf
  - Body: PDF file
- Error Responses:
  - 500 Internal Server Error: If PDF generation fails

### GET /fetch-submissions
- Fetches all submissions from the DMS API
- Returns:
  - Status: 200 OK
  - Content-Type: application/json
  - Body: Array of submission objects
- Error Responses:
  - 500 Internal Server Error: If fetching or parsing fails

## Document Formatting Features

### Line Spacing
- Enhanced readability with proper spacing between numbered points
- Consistent 12pt (240 twips) spacing after paragraphs
- Clear visual separation between sections

### Document Structure
- Professional formatting with bold headers and standardized font sizes
- Hierarchical organization of content with proper indentation
- Well-formatted budget tables with clear column separation

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
- PDF conversion issues

## File Management

- Generated DOCX files are temporary and automatically cleaned up
- PDFs are stored in the `output/` directory before being served
- Each file is named with a standardized format: `proposal_[unique_id].pdf`
- Timestamp-based temporary files to prevent conflicts

## Authentication

The service uses token-based authentication to communicate with the DMS API. Credentials are currently hardcoded for development purposes.

## Development

### Building
```bash
cargo build
```

### Testing with Mock Data
```bash
curl -X POST -H "Content-Type: application/json" --data @mock_submission.json http://localhost:8080/generate-pdf -o proposal.pdf
```

### Running in Docker
```bash
docker-compose up --build
```

## Troubleshooting

### Common Issues

1. **PDF Generation Fails**
   - Check that LibreOffice is installed and accessible
   - Verify the output directory exists and is writable
   - Ensure the template has the correct structure

2. **Missing Logo**
   - Confirm that `public/thapar_logo.png` exists
   - Check file permissions

3. **API Connectivity Issues**
   - Verify network connection
   - Check authentication credentials
