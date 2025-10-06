# XForce Analytics - Data Enrichment Scripts

Comprehensive data enrichment pipeline for the 443 SDF-funded Stellar projects.

## 📦 Installation

```bash
pip install -r requirements.txt
```

## 🚀 Quick Start

### Run All Enrichment Scripts
```bash
python scripts/run_all_enrichment.py
```

### Run Individual Scripts

#### 1. Temporal Analysis
```bash
python scripts/temporal_analysis.py
```
**Generates:**
- `data/analytics/temporal/funding_velocity.json` - Round progression speed
- `data/analytics/temporal/time_to_mainnet.json` - Development timeline
- `data/analytics/temporal/quarterly_cohorts.json` - Cohort analysis
- `data/analytics/temporal/round_progression.json` - Retention rates
- `data/analytics/temporal/seasonal_patterns.json` - Q1-Q4 patterns

#### 2. Geographic Enrichment
```bash
python scripts/geographic_enrichment.py
```
**Generates:**
- `data/enriched/projects_with_regions.json` - Projects + region data
- `data/analytics/geographic/country_rankings.json` - 63 countries ranked
- `data/analytics/geographic/regional_analysis.json` - Regional stats
- `data/analytics/geographic/geographic_gaps.json` - Opportunity gaps

#### 3. HTML Data Extraction
```bash
python scripts/html_parser.py
```
**Generates:**
- `data/enriched/social_links.json` - Social media links
- `data/enriched/team_profiles.json` - Team descriptions & scores
- `data/enriched/traction_metrics.json` - Traction data
- `data/enriched/undp_projects.json` - UNDP-related projects

#### 4. Deep Analytics
```bash
python scripts/deep_analytics.py
```
**Generates:**
- `data/analytics/success_patterns.json` - Top 10% characteristics
- `data/analytics/program_combinations.json` - Optimal combos
- `data/analytics/open_source_correlation.json` - OS premium
- `data/analytics/multichain_analysis.json` - Multichain vs Stellar-only
- `data/analytics/funding_tiers.json` - Tier breakdown
- `data/analytics/category_deep_dive.json` - Per-category analysis

#### 5. GitHub Data (Optional - Requires API Token)
```bash
export GITHUB_TOKEN=your_token_here
python scripts/github_fetcher.py
```
**Generates:**
- `data/enriched/github_metrics.json` - Stars, forks, activity scores

#### 6. Website Scraping (New!)
```bash
python scripts/website_scraper.py
```
**Generates:**
- `data/enriched/website_metadata.json` - Meta tags, tech stack, social discovery

**Features:**
- Scrapes 443 project websites
- Extracts metadata (title, description, Open Graph)
- Discovers social media links
- Detects technology stack from page content
- Rate-limited concurrent scraping (5 workers)

#### 7. Social Media Scraping (New!)
```bash
python scripts/social_scraper.py
```
**Generates:**
- `data/enriched/social_accounts_detailed.json` - Enhanced social data

**Features:**
- Discord server stats (member count, online count) via public API
- Twitter/X handle extraction
- GitHub org/user extraction
- YouTube channel identification
- LinkedIn company slugs
- No authentication required for Discord public invites!

## 📊 Generated Data Structure

### Enriched Data (`data/enriched/`)
- **projects_with_regions.json** - All 443 projects with standardized regions
- **social_links.json** - Social media URLs and scores
- **team_profiles.json** - Team size, roles, expertise, strength scores
- **traction_metrics.json** - Downloads, users, revenue, partnerships
- **github_metrics.json** - Stars, forks, activity scores (optional)
- **website_metadata.json** - Scraped website data (new)
- **social_accounts_detailed.json** - Enhanced social stats (new)

### Analytics (`data/analytics/`)

#### Temporal (`data/analytics/temporal/`)
- Funding velocity by category
- Time-to-mainnet analysis
- Quarterly cohorts
- Round progression probabilities
- Seasonal success patterns

#### Geographic (`data/analytics/geographic/`)
- Country rankings (63 countries)
- Regional analysis (7 regions)
- Geographic opportunity gaps

#### Core Analytics
- **success_patterns.json** - What makes top 10% successful
- **program_combinations.json** - Best SCF program combos
- **open_source_correlation.json** - OS funding premium analysis
- **multichain_analysis.json** - Stellar-only vs multichain
- **funding_tiers.json** - $50k/$100k/$150k tier stats
- **category_deep_dive.json** - Deep per-category breakdown

## 🔍 Key Insights from Enrichment

### Temporal Patterns
- **Soroban projects reach mainnet 0.7x faster** than non-Soroban
- **31.6% of Round 1 projects** secure Round 2 funding
- **Q2 and Q4** show highest funding success rates

### Geographic Distribution
- **63 countries** represented across **7 regions**
- Sub-Saharan Africa & Latin America: high growth potential
- Top countries vary by category

### Success Factors (Top 10% vs Rest)
- Higher Soroban adoption rate
- More multi-round participation
- Better open-source practices
- Multi-program participation (Build + Audit Bank)

### Technology Trends
- **60.7% Soroban adoption** (269/443 projects)
- **Open source premium**: Higher funding correlation
- **~47% multichain** projects

## 🛠️ Web Scraping Features

### Website Scraper (`website_scraper.py`)
**What it collects:**
- Page titles and meta descriptions
- Open Graph images and metadata
- Twitter cards
- Discovered social media links not in original data
- Technology keywords (Stellar, Rust, React, etc.)
- Website sections (docs, blog, team, contact)
- App store links (Google Play, Apple App Store)
- External link analysis

**Rate Limiting:**
- 5 concurrent requests max
- 0.5s delay between requests
- Respects website robots.txt

### Social Scraper (`social_scraper.py`)
**What it collects:**
- **Discord**: Server name, member count, online count (via public API)
- **Twitter/X**: Handle extraction
- **GitHub**: Org/user from URLs
- **YouTube**: Channel identifiers
- **LinkedIn**: Company slugs
- **Medium**: Publication handles

**No Authentication Required:**
- Discord public invite API provides stats
- Other platforms: URL parsing and metadata extraction

## 📈 Usage in Analytics Platform

### Backend Integration
```rust
// Load enriched data in Axum backend
let enriched_data = load_json("data/enriched/projects_with_regions.json");
let analytics = load_json("data/analytics/success_patterns.json");
```

### Frontend Visualization
```javascript
// Fetch enriched data
const response = await fetch('/api/analytics/geographic/country_rankings');
const data = await response.json();

// Render charts with Chart.js
renderCountryMap(data.rankings);
```

### API Endpoints to Create
- `GET /api/analytics/temporal/velocity` - Funding velocity
- `GET /api/analytics/geographic/rankings` - Country rankings
- `GET /api/analytics/success-patterns` - Success factors
- `GET /api/analytics/program-combos` - Best programs
- `GET /api/enriched/social-stats` - Social media metrics
- `GET /api/enriched/website-metadata` - Scraped website data

## 🔐 Environment Variables

```bash
# Optional: For GitHub scraping (avoid rate limits)
export GITHUB_TOKEN=ghp_your_token_here

# Optional: For future Twitter API integration
export TWITTER_BEARER_TOKEN=your_bearer_token

# Optional: For future analytics APIs
export DISCORD_BOT_TOKEN=your_bot_token
```

## 📝 Notes

- **Rate Limiting**: All scrapers implement delays to avoid being blocked
- **Error Handling**: Graceful fallbacks for failed requests
- **No Authentication**: Social scraper works without API keys (Discord only provides limited data)
- **Incremental Updates**: Run scripts individually to update specific datasets

## 🚨 Troubleshooting

### Unicode Encoding Errors
All emoji/unicode characters have been replaced with ASCII equivalents in scripts.

### Import Errors
```bash
pip install requests beautifulsoup4 lxml
```

### Rate Limits
- GitHub: Use `GITHUB_TOKEN` environment variable
- Websites: Increase delay in `website_scraper.py` (line 236)
- Discord: Already rate-limited at 0.5s per request

## 📊 Data Size
Total enriched data: **~1.5MB** across 25+ JSON files

## 🎯 Next Steps

1. **Integrate into Backend API**: Serve enriched data via Axum endpoints
2. **Build Frontend Visualizations**: Use Chart.js with enriched data
3. **ML Model Training**: Use success_patterns.json for predictions
4. **Real-time Updates**: Schedule scraper runs (daily/weekly)
5. **Additional Data Sources**: CoinGecko, DefiLlama APIs for on-chain data
