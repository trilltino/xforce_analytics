"""
Geographic Enrichment Script
Standardizes countries and adds region/continent mappings
"""

import json
from collections import defaultdict

# Country to region mapping
COUNTRY_TO_REGION = {
    # Europe & Central Asia
    "Portugal": "Europe & Central Asia",
    "Romania": "Europe & Central Asia",
    "Switzerland": "Europe & Central Asia",
    "Germany": "Europe & Central Asia",
    "United Kingdom": "Europe & Central Asia",
    "Ukraine": "Europe & Central Asia",
    "Poland": "Europe & Central Asia",
    "France": "Europe & Central Asia",
    "Spain": "Europe & Central Asia",
    "Italy": "Europe & Central Asia",
    "Netherlands": "Europe & Central Asia",
    "Belgium": "Europe & Central Asia",
    "Austria": "Europe & Central Asia",
    "Sweden": "Europe & Central Asia",
    "Norway": "Europe & Central Asia",
    "Denmark": "Europe & Central Asia",
    "Finland": "Europe & Central Asia",
    "Czech Republic": "Europe & Central Asia",
    "Greece": "Europe & Central Asia",
    "Turkey": "Europe & Central Asia",
    "Russia": "Europe & Central Asia",

    # North America
    "United States": "North America",
    "Canada": "North America",

    # Latin America & Caribbean
    "Mexico": "Latin America & Caribbean",
    "Argentina": "Latin America & Caribbean",
    "Colombia": "Latin America & Caribbean",
    "Brazil": "Latin America & Caribbean",
    "Chile": "Latin America & Caribbean",
    "Peru": "Latin America & Caribbean",
    "Venezuela": "Latin America & Caribbean",
    "Ecuador": "Latin America & Caribbean",
    "Uruguay": "Latin America & Caribbean",
    "Paraguay": "Latin America & Caribbean",
    "Bolivia": "Latin America & Caribbean",
    "Costa Rica": "Latin America & Caribbean",
    "Panama": "Latin America & Caribbean",
    "Guatemala": "Latin America & Caribbean",
    "Honduras": "Latin America & Caribbean",
    "El Salvador": "Latin America & Caribbean",
    "Nicaragua": "Latin America & Caribbean",
    "Dominican Republic": "Latin America & Caribbean",
    "Jamaica": "Latin America & Caribbean",
    "Trinidad and Tobago": "Latin America & Caribbean",
    "Bahamas": "Latin America & Caribbean",

    # Sub-Saharan Africa
    "Nigeria": "Sub-Saharan Africa",
    "Kenya": "Sub-Saharan Africa",
    "South Africa": "Sub-Saharan Africa",
    "Ghana": "Sub-Saharan Africa",
    "Uganda": "Sub-Saharan Africa",
    "Tanzania": "Sub-Saharan Africa",
    "Rwanda": "Sub-Saharan Africa",
    "Ethiopia": "Sub-Saharan Africa",
    "Senegal": "Sub-Saharan Africa",
    "Ivory Coast": "Sub-Saharan Africa",
    "Zimbabwe": "Sub-Saharan Africa",
    "Zambia": "Sub-Saharan Africa",
    "Mozambique": "Sub-Saharan Africa",
    "Botswana": "Sub-Saharan Africa",
    "Namibia": "Sub-Saharan Africa",
    "Angola": "Sub-Saharan Africa",
    "Cameroon": "Sub-Saharan Africa",
    "Congo": "Sub-Saharan Africa",

    # Middle East & North Africa
    "United Arab Emirates": "Middle East & North Africa",
    "Saudi Arabia": "Middle East & North Africa",
    "Israel": "Middle East & North Africa",
    "Egypt": "Middle East & North Africa",
    "Morocco": "Middle East & North Africa",
    "Jordan": "Middle East & North Africa",
    "Lebanon": "Middle East & North Africa",
    "Tunisia": "Middle East & North Africa",
    "Algeria": "Middle East & North Africa",
    "Bahrain": "Middle East & North Africa",
    "Kuwait": "Middle East & North Africa",
    "Oman": "Middle East & North Africa",
    "Qatar": "Middle East & North Africa",

    # South Asia
    "India": "South Asia",
    "Pakistan": "South Asia",
    "Bangladesh": "South Asia",
    "Sri Lanka": "South Asia",
    "Nepal": "South Asia",
    "Afghanistan": "South Asia",
    "Maldives": "South Asia",
    "Bhutan": "South Asia",

    # East Asia & Pacific
    "China": "East Asia & Pacific",
    "Japan": "East Asia & Pacific",
    "South Korea": "East Asia & Pacific",
    "Singapore": "East Asia & Pacific",
    "Hong Kong": "East Asia & Pacific",
    "Taiwan": "East Asia & Pacific",
    "Indonesia": "East Asia & Pacific",
    "Malaysia": "East Asia & Pacific",
    "Thailand": "East Asia & Pacific",
    "Vietnam": "East Asia & Pacific",
    "Philippines": "East Asia & Pacific",
    "Australia": "East Asia & Pacific",
    "New Zealand": "East Asia & Pacific",
    "Cambodia": "East Asia & Pacific",
    "Laos": "East Asia & Pacific",
    "Myanmar": "East Asia & Pacific",
}


def standardize_country(country_str):
    """Standardize country names"""
    if not country_str:
        return None

    # Remove leading/trailing whitespace and special chars
    country = country_str.strip().lstrip(' \xa0')

    # Try direct match
    if country in COUNTRY_TO_REGION:
        return country

    # Common variations
    variations = {
        "USA": "United States",
        "U.S.": "United States",
        "UK": "United Kingdom",
        "UAE": "United Arab Emirates",
    }

    return variations.get(country, country)


def get_region(country):
    """Get region for a country"""
    return COUNTRY_TO_REGION.get(country, "Other")


def main():
    # Load project data
    with open('data/raw/all_projects_structured.json', 'r', encoding='utf-8') as f:
        data = json.load(f)

    # Flatten all projects
    all_projects = []
    for category, projects in data['by_category'].items():
        all_projects.extend(projects)

    print(f"Processing {len(all_projects)} projects...")

    # Enrich geographic data
    enriched_projects = []
    region_stats = defaultdict(lambda: {
        'projects': [],
        'total_funding': 0,
        'categories': defaultdict(int),
        'avg_funding': 0,
        'mainnet_count': 0,
        'soroban_count': 0
    })

    country_stats = defaultdict(lambda: {
        'projects': [],
        'total_funding': 0,
        'categories': defaultdict(int),
        'avg_funding': 0,
        'mainnet_count': 0
    })

    unknown_countries = set()

    for project in all_projects:
        country_raw = project.get('country', '')
        country = standardize_country(country_raw)
        region = get_region(country) if country else None

        if country and region == "Other":
            unknown_countries.add(country)

        enriched_project = project.copy()
        enriched_project['country_standardized'] = country
        enriched_project['region'] = region
        enriched_projects.append(enriched_project)

        # Update stats
        if region and region != "Other":
            stats = region_stats[region]
            stats['projects'].append(project['title'])
            stats['total_funding'] += project.get('total_awarded', 0)
            stats['categories'][project.get('category', 'Unknown')] += 1
            if project.get('integration_status') == 'Mainnet':
                stats['mainnet_count'] += 1
            if project.get('soroban'):
                stats['soroban_count'] += 1

        if country:
            c_stats = country_stats[country]
            c_stats['projects'].append(project['title'])
            c_stats['total_funding'] += project.get('total_awarded', 0)
            c_stats['categories'][project.get('category', 'Unknown')] += 1
            if project.get('integration_status') == 'Mainnet':
                c_stats['mainnet_count'] += 1

    # Calculate averages and rates
    for region, stats in region_stats.items():
        project_count = len(stats['projects'])
        stats['project_count'] = project_count
        stats['avg_funding'] = stats['total_funding'] / project_count if project_count > 0 else 0
        stats['mainnet_rate'] = stats['mainnet_count'] / project_count if project_count > 0 else 0
        stats['soroban_rate'] = stats['soroban_count'] / project_count if project_count > 0 else 0
        stats['categories'] = dict(stats['categories'])
        del stats['projects']  # Remove for cleaner output

    for country, stats in country_stats.items():
        project_count = len(stats['projects'])
        stats['project_count'] = project_count
        stats['avg_funding'] = stats['total_funding'] / project_count if project_count > 0 else 0
        stats['mainnet_rate'] = stats['mainnet_count'] / project_count if project_count > 0 else 0
        stats['categories'] = dict(stats['categories'])
        stats['region'] = get_region(country)
        del stats['projects']  # Keep for country rankings

    # Save enriched project data
    enriched_data = {
        'total_projects': len(enriched_projects),
        'projects': enriched_projects
    }

    with open('data/enriched/projects_with_regions.json', 'w', encoding='utf-8') as f:
        json.dump(enriched_data, f, indent=2, ensure_ascii=False)
    print(f"[OK] Saved projects_with_regions.json")

    # Save regional analysis
    with open('data/analytics/geographic/regional_analysis.json', 'w', encoding='utf-8') as f:
        json.dump(dict(region_stats), f, indent=2, ensure_ascii=False)
    print(f"[OK] Saved regional_analysis.json ({len(region_stats)} regions)")

    # Save country rankings
    country_rankings = sorted(
        [{'country': k, **v} for k, v in country_stats.items()],
        key=lambda x: x['total_funding'],
        reverse=True
    )

    with open('data/analytics/geographic/country_rankings.json', 'w', encoding='utf-8') as f:
        json.dump({
            'rankings': country_rankings,
            'total_countries': len(country_rankings)
        }, f, indent=2, ensure_ascii=False)
    print(f"[OK] Saved country_rankings.json ({len(country_rankings)} countries)")

    # Geographic gaps analysis
    all_regions = set(COUNTRY_TO_REGION.values())
    funded_regions = set(region_stats.keys())
    unfunded_regions = all_regions - funded_regions

    gaps_analysis = {
        'funded_regions': list(funded_regions),
        'underrepresented_regions': [
            {
                'region': r,
                'project_count': region_stats[r]['project_count'],
                'avg_funding': region_stats[r]['avg_funding'],
                'opportunity_score': 100 - (region_stats[r]['project_count'] * 2)  # Higher score = more opportunity
            }
            for r in funded_regions
            if region_stats[r]['project_count'] < 20
        ],
        'unfunded_regions': list(unfunded_regions)
    }

    with open('data/analytics/geographic/geographic_gaps.json', 'w', encoding='utf-8') as f:
        json.dump(gaps_analysis, f, indent=2, ensure_ascii=False)
    print(f"[OK] Saved geographic_gaps.json")

    print("\n[OK] Geographic enrichment complete!")
    print(f"\nKey Insights:")
    print(f"   * {len(country_rankings)} countries represented")
    print(f"   * {len(region_stats)} regions active")

    if unknown_countries:
        print(f"\n⚠ Unknown countries found: {', '.join(sorted(unknown_countries))}")

    top_country = country_rankings[0]
    print(f"   * Top country: {top_country['country']} (${top_country['total_funding']/1e6:.1f}M, {top_country['project_count']} projects)")


if __name__ == "__main__":
    import os
    os.makedirs('data/enriched', exist_ok=True)
    os.makedirs('data/analytics/geographic', exist_ok=True)
    main()
