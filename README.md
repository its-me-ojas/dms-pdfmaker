# DMS PDF Maker

A Rust-based web service that generates standardized PDF documents for SEED Money Grant Proposals using a predefined template.

## Overview

This service provides an API endpoint that generates a formatted PDF document containing a SEED Money Grant Proposal template. The document includes:
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
```

## Project Structure

```
dms-pdfmaker/
├── src/
│   ├── main.rs          # Main application entry point and server setup
│   ├── page1/           # Cover page content generation
│   │   └── mod.rs
│   ├── page2/           # Main content and table generation
│   │   └── mod.rs
│   └── utils/           # Utility functions
│       └── mod.rs
├── public/              # Static assets
│   └── thapar_logo.png
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

3. Build the project:
```bash
cargo build --release
```

## Usage

1. Start the server:
```bash
cargo run
```

2. The server will start on `http://0.0.0.0:3000` with the following endpoints:
- `GET /` - Returns a "Hello, World!" message (health check)
- `GET /generate` - Generates and returns the PDF document

3. To generate a PDF, send a GET request to `/generate`:
```bash
curl http://localhost:3000/generate --output document.pdf
```

## API Endpoints

### GET /generate
- Generates a PDF document based on the template
- Returns:
  - Status: 200 OK
  - Content-Type: application/pdf
  - Body: PDF file
- Error Responses:
  - 500 Internal Server Error: If PDF generation fails
