import re
from collections import defaultdict
from bs4 import BeautifulSoup

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

# Group by category
categories = defaultdict(list)
for p in projects:
    cat = p.get('Category', 'Unknown')
    categories[cat].append(p)

# For each category, get top 3 funded projects on Mainnet
print("=" * 100)
print("SAMPLE SUCCESSFUL PROJECTS BY CATEGORY")
print("=" * 100)

for cat in sorted(categories.keys()):
    if cat == 'Unknown':
        continue

    print(f"\n{'=' * 100}")
    print(f"CATEGORY: {cat}")
    print(f"{'=' * 100}\n")

    # Filter for mainnet projects with funding
    mainnet_projects = [p for p in categories[cat]
                       if p.get('Integration Status') == 'Mainnet'
                       and get_funding(p) > 0]

    # Sort by funding
    mainnet_projects.sort(key=get_funding, reverse=True)

    # Get top 3
    top_projects = mainnet_projects[:3]

    for i, p in enumerate(top_projects, 1):
        print(f"\n{'-' * 100}")
        print(f"Example #{i}: {p.get('Title', 'N/A')}")
        print(f"{'-' * 100}")
        print(f"Type: {p.get('Type', 'N/A')}")
        print(f"Country: {p.get('Country', 'N/A')}")
        print(f"Total Awarded: {p.get('Total Awarded', 'N/A')}")
        print(f"Programs: {p.get('Programs / Initiatives', 'N/A')}")
        print(f"Awarded Rounds: {p.get('Awarded Rounds', 'N/A')}")
        print(f"Quarter/Year: {p.get('Quarter, Year (from Round) (from Awarded Submissions)', 'N/A')}")
        print(f"Integration Status: {p.get('Integration Status', 'N/A')}")
        print(f"Open-Source: {p.get('Open-Source?', 'N/A')}")

        if p.get('Website'):
            print(f"Website: {p.get('Website')}")
        if p.get('Github'):
            print(f"Github: {p.get('Github')}")

        desc = p.get('Description', '')
        if desc:
            print(f"\nDescription:")
            print(f"{desc[:500]}..." if len(desc) > 500 else desc)

        traction = p.get('Current Traction', '')
        if traction and len(traction) > 10:
            print(f"\nCurrent Traction:")
            print(f"{traction[:500]}..." if len(traction) > 500 else traction)

        other_chains = p.get('Other Chains', '')
        if other_chains and other_chains != 'Not Multichain':
            print(f"\nOther Chains: {other_chains}")

        regions = p.get('Regions of Operation', '')
        if regions:
            print(f"Regions of Operation: {regions}")

        soroban = p.get('Soroban?', '')
        if soroban == 'checked':
            print(f"Soroban: Yes")

        print()

# Also extract some development stage projects with high funding for learning
print("\n" + "=" * 100)
print("HIGH-FUNDED DEVELOPMENT STAGE PROJECTS (Learning Examples)")
print("=" * 100)

dev_projects = [p for p in projects
               if p.get('Integration Status') == 'Development'
               and get_funding(p) > 100000]

dev_projects.sort(key=get_funding, reverse=True)

for i, p in enumerate(dev_projects[:5], 1):
    print(f"\n{'-' * 100}")
    print(f"Example #{i}: {p.get('Title', 'N/A')}")
    print(f"{'-' * 100}")
    print(f"Category: {p.get('Category', 'N/A')}")
    print(f"Type: {p.get('Type', 'N/A')}")
    print(f"Country: {p.get('Country', 'N/A')}")
    print(f"Total Awarded: {p.get('Total Awarded', 'N/A')}")
    print(f"Programs: {p.get('Programs / Initiatives', 'N/A')}")
    print(f"Open-Source: {p.get('Open-Source?', 'N/A')}")

    if p.get('Website'):
        print(f"Website: {p.get('Website')}")

    desc = p.get('Description', '')
    if desc:
        print(f"\nDescription:")
        print(f"{desc[:400]}..." if len(desc) > 400 else desc)

    integration_desc = p.get('Integration Description', '')
    if integration_desc and len(integration_desc) > 20:
        print(f"\nIntegration Plan:")
        print(f"{integration_desc[:400]}..." if len(integration_desc) > 400 else integration_desc)

print("\n" + "=" * 100)
print("ANALYSIS COMPLETE")
print("=" * 100)
