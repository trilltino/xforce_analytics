import re
from collections import defaultdict
from bs4 import BeautifulSoup
import json

# Read the HTML file
with open(r'c:\Users\isich\Documents\scfhandbook\Shared-to-Ecosystem-Projects.html', 'r', encoding='utf-8') as f:
    html_content = f.read()

soup = BeautifulSoup(html_content, 'html.parser')

# Find all rows
rows = soup.find_all('tr')

# Extract headers
headers = [th.get_text(strip=True) for th in rows[0].find_all('td')]

# Parse all projects
projects = []
for row in rows[1:]:  # Skip header
    cells = row.find_all('td')
    if len(cells) >= 27:
        project = {}
        for i, header in enumerate(headers):
            if i < len(cells):
                project[header] = cells[i].get_text(strip=True)
        projects.append(project)

# Function to extract funding amount
def get_funding(p):
    funding_str = p.get('Total Awarded', '$0')
    funding_match = re.search(r'\$?([\d,]+\.?\d*)', funding_str.replace(',', ''))
    if funding_match:
        try:
            return float(funding_match.group(1))
        except:
            return 0
    return 0

# Export structured data
output_data = {
    'total_projects': len(projects),
    'by_category': {},
    'by_type': {},
    'by_country': {},
    'by_integration_status': {},
    'all_projects': []
}

for p in projects:
    funding = get_funding(p)

    project_data = {
        'title': p.get('Title', ''),
        'category': p.get('Category', ''),
        'type': p.get('Type', ''),
        'country': p.get('Country', '').strip(),
        'description': p.get('Description', ''),
        'total_awarded': funding,
        'programs': p.get('Programs / Initiatives', ''),
        'rounds': p.get('Awarded Rounds', ''),
        'quarters': p.get('Quarter, Year (from Round) (from Awarded Submissions)', ''),
        'integration_status': p.get('Integration Status', ''),
        'open_source': p.get('Open-Source?', ''),
        'website': p.get('Website', ''),
        'github': p.get('Github', ''),
        'soroban': p.get('Soroban?', '') == 'checked',
        'other_chains': p.get('Other Chains', ''),
        'regions': p.get('Regions of Operation', ''),
        'traction': p.get('Current Traction', '')[:200] if p.get('Current Traction', '') else ''
    }

    output_data['all_projects'].append(project_data)

    # Group by category
    cat = p.get('Category', 'Unknown')
    if cat not in output_data['by_category']:
        output_data['by_category'][cat] = []
    output_data['by_category'][cat].append(project_data)

    # Group by type
    ptype = p.get('Type', 'Unknown')
    if ptype not in output_data['by_type']:
        output_data['by_type'][ptype] = []
    output_data['by_type'][ptype].append(project_data)

    # Group by country
    country = p.get('Country', 'Unknown').strip()
    if country and country != '\xa0':
        if country not in output_data['by_country']:
            output_data['by_country'][country] = []
        output_data['by_country'][country].append(project_data)

    # Group by integration status
    status = p.get('Integration Status', 'Unknown')
    if status not in output_data['by_integration_status']:
        output_data['by_integration_status'][status] = []
    output_data['by_integration_status'][status].append(project_data)

# Sort projects within each category by funding
for cat in output_data['by_category']:
    output_data['by_category'][cat].sort(key=lambda x: x['total_awarded'], reverse=True)

for ptype in output_data['by_type']:
    output_data['by_type'][ptype].sort(key=lambda x: x['total_awarded'], reverse=True)

for country in output_data['by_country']:
    output_data['by_country'][country].sort(key=lambda x: x['total_awarded'], reverse=True)

# Save to JSON
with open(r'c:\Users\isich\Documents\scfhandbook\all_projects_structured.json', 'w', encoding='utf-8') as f:
    json.dump(output_data, f, indent=2, ensure_ascii=False)

print(f"Exported {len(projects)} projects to all_projects_structured.json")
print(f"\nCategories: {len(output_data['by_category'])}")
print(f"Types: {len(output_data['by_type'])}")
print(f"Countries: {len(output_data['by_country'])}")
print(f"Integration Statuses: {len(output_data['by_integration_status'])}")
