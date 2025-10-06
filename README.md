# XForce Analytics

**Stellar Development Foundation (SDF) Funding Analytics Platform**

A comprehensive fullstack Rust application for analyzing and visualizing SDF funding data across 443 projects totaling $42.1M in funding.

## Overview

XForce Analytics provides data-driven insights into the Stellar Development Foundation funding ecosystem, helping developers and entrepreneurs understand:

- Funding patterns across categories and regions
- Success factors for top-funded projects
- Gap analysis for underserved markets
- Competitive landscape intelligence
- Strategic recommendations for funding applications

## Tech Stack

### Frontend
- **Yew** - Rust WebAssembly frontend framework
- **Trunk** - WASM web application bundler
- **Gloo** - Rust/WASM utility library
- **CSS** - Sharp black & red theme with monospace fonts

### Backend
- **Axum** - Ergonomic Rust web framework
- **SQLx** - Async SQL toolkit
- **PostgreSQL** - Primary database
- **Tower** - Modular service components
- **Serde** - Serialization framework

### Data Analysis
- **Python** - Data enrichment scripts
- **Polars** - High-performance DataFrames
- **BeautifulSoup** - Web scraping
- **Requests** - HTTP client

## Features

### Intelligence Hub
Data-driven analytics interface with 8 specialized tabs:

1. **Competitive Landscape** - Filter and analyze all 443 funded projects by category, funding, status, and technology
2. **Gap Analysis** - Identify zero-competition opportunities and underserved categories
3. **Success Patterns** - Analyze what separates top-funded projects from average ones
4. **Funding Calculator** - Calculate expected funding based on real project data
5. **Social Intelligence** - Social media presence analysis across all projects
6. **Team Insights** - Team composition and experience patterns
7. **Geographic Trends** - Funding patterns by country and region
8. **Tech Stack Analysis** - Technology choices and funding correlation

### Application Guide
Comprehensive guide covering:
- All SDF funding programs (Kickstart, Build, Growth Hack, Audit Bank)
- Category-specific strategies
- Real data from 443 projects
- Success formulas and optimization tips

### Analytics Dashboard
- Live project statistics
- Category breakdown
- Timeline analysis
- Funding distribution

### Project Explorer
- Browse all 443 funded projects
- Advanced filtering and search
- Project detail pages
- Social media links

## Quick Start

### Prerequisites
- Rust 1.70+ (`rustup update`)
- PostgreSQL 15+
- Trunk (`cargo install trunk`)
- Python 3.10+ (for data scripts)

### Setup

1. **Clone & Install**
```bash
git clone <repository-url>
cd xforce_analytics
cargo build
```

2. **Database Setup**
```bash
# Create database
createdb xforce_analytics

# Set environment variables
cp .env.example .env
# Edit .env with your DATABASE_URL
```

3. **Run Migrations**
```bash
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

4. **Start Backend**
```bash
cargo run --bin web-server
# Server runs on http://localhost:3000
```

5. **Start Frontend** (in new terminal)
```bash
cd frontend
trunk serve --port 8080
# Frontend runs on http://localhost:8080
```

6. **Access Application**
```
Open http://localhost:8080
```

## Project Structure

```
xforce_analytics/
├── backend/
│   ├── libs/           # Shared backend libraries
│   │   ├── lib-auth/   # Authentication
│   │   ├── lib-core/   # Core types
│   │   ├── lib-rpc/    # RPC utilities
│   │   └── lib-utils/  # Common utilities
│   └── services/
│       └── web-server/ # Main API server
├── frontend/
│   ├── src/
│   │   ├── api/        # API client layer
│   │   ├── components/ # Reusable UI components
│   │   ├── pages/      # Application pages
│   │   └── styles/     # CSS stylesheets
│   └── static/         # Static assets
├── shared/             # Shared types between frontend/backend
│   └── src/models/     # Data models
├── data/
│   ├── raw/            # Raw JSON data files
│   └── enriched/       # Enriched data with analysis
├── scripts/            # Python data analysis scripts
└── sql/
    ├── migrations/     # Database migrations
    └── seeds/          # Seed data
```

## Data Pipeline

### Raw Data
- `data/raw/all_projects_structured.json` - 443 SDF-funded projects

### Enriched Data
Generated via Python scripts in `scripts/`:

- `projects_with_regions.json` - Projects with geographic data
- `social_accounts_detailed.json` - Social media presence analysis
- `team_profiles.json` - Team composition data
- `website_metadata.json` - Website metadata

### Data Enrichment Scripts
```bash
cd scripts

# Install dependencies
pip install -r requirements.txt

# Run enrichment
python run_all_enrichment.py

# Individual scripts:
python geographic_enrichment.py
python social_scraper.py
python website_scraper.py
python github_fetcher.py
```

## API Endpoints

### Projects
- `GET /api/projects` - List all projects
- `GET /api/projects/:id` - Get project details
- `GET /api/projects/search` - Search projects

### Analytics
- `GET /api/analytics/stats` - Overall statistics
- `GET /api/analytics/categories` - Category breakdown
- `GET /api/analytics/timeline` - Temporal analysis
- `GET /api/analytics/dashboard` - Live dashboard data

### Enriched Data
- `GET /api/enriched/projects-with-regions` - Geographic data
- `GET /api/enriched/social-accounts` - Social media data
- `GET /api/enriched/team-profiles` - Team data
- `GET /api/enriched/website-metadata` - Website metadata

## Key Insights (from Real Data)

### Funding Patterns
- **Average Funding**: $95,067 across all projects
- **Top 25% Threshold**: $144,000
- **Multiple Rounds Effect**: 4 rounds = 5.0x multiplier ($68K → $340K avg)

### Category Performance
1. **Financial Protocols**: $130,893 avg (highest)
2. **Infrastructure & Services**: $101,757 avg
3. **Applications**: $93,899 avg
4. **Developer Tooling**: $79,752 avg

### Geographic Sweet Spots
1. **Portugal**: $297K avg
2. **Germany**: $285K avg
3. **Netherlands**: $263K avg
4. **Chile**: $248K avg
5. **Colombia**: $219K avg

### Technology Correlation
- **Soroban-native**: 67.6% of top-funded projects
- **Stellar-only**: 74.8% of top-funded projects
- **Mainnet status**: +30% funding premium
- **Open source**: 70.3% have public GitHub
- **Audit Bank**: +28% funding correlation

### Zero-Competition Opportunities
- Credit Scoring Infrastructure (0 projects)
- Subscription Payment Platform (0 projects)
- Escrow & Insurance Protocol (0 projects)
- Gas Optimization Tools (0 projects)
- Soroban-native Anchor (4 Classic anchors only)

## Development

### Build for Production
```bash
# Backend
cargo build --release

# Frontend
cd frontend
trunk build --release
```

### Run Tests
```bash
cargo test
```

### Code Quality
```bash
# Format code
cargo fmt

# Check lints
cargo clippy

# Auto-fix warnings
cargo fix --allow-dirty
```

## Design Philosophy

**Sharp Black & Red Theme**
- Pure black backgrounds (#000000)
- Crimson red accents (#dc143c)
- No rounded corners (sharp, angular design)
- Monospace fonts (Courier New, Consolas)
- Uppercase typography for headers
- Data-first, no unnecessary visualizations

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## License

[Your License Here]

## Acknowledgments

- Data sourced from Stellar Development Foundation
- Built with Rust and Yew framework
- Analysis powered by Python and Polars

## Support

For issues, questions, or contributions:
- GitHub Issues: [repository-url]/issues
- Documentation: [docs-url]
