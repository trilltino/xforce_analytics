import re
from collections import defaultdict
from bs4 import BeautifulSoup
import statistics

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

print(f"Total projects extracted: {len(projects)}\n")

# 1. Category breakdown
categories = defaultdict(list)
for p in projects:
    cat = p.get('Category', 'Unknown')
    categories[cat].append(p)

print("=" * 80)
print("1. CATEGORY BREAKDOWN")
print("=" * 80)
for cat, projs in sorted(categories.items(), key=lambda x: len(x[1]), reverse=True):
    print(f"\n{cat}: {len(projs)} projects")

# 2. Funding analysis per category
print("\n" + "=" * 80)
print("2. FUNDING ANALYSIS BY CATEGORY")
print("=" * 80)

category_funding = defaultdict(list)
for p in projects:
    cat = p.get('Category', 'Unknown')
    funding_str = p.get('Total Awarded', '$0')
    # Extract numeric value
    funding_match = re.search(r'\$?([\d,]+\.?\d*)', funding_str.replace(',', ''))
    if funding_match:
        try:
            funding = float(funding_match.group(1))
            if funding > 0:
                category_funding[cat].append(funding)
        except:
            pass

for cat in sorted(categories.keys()):
    if cat in category_funding and category_funding[cat]:
        funds = category_funding[cat]
        print(f"\n{cat}:")
        print(f"  Projects with funding: {len(funds)}")
        print(f"  Min: ${min(funds):,.2f}")
        print(f"  Max: ${max(funds):,.2f}")
        print(f"  Average: ${statistics.mean(funds):,.2f}")
        print(f"  Median: ${statistics.median(funds):,.2f}")
        print(f"  Total: ${sum(funds):,.2f}")

# 3. Programs/Initiatives
print("\n" + "=" * 80)
print("3. PROGRAMS / INITIATIVES")
print("=" * 80)

programs = defaultdict(int)
for p in projects:
    prog = p.get('Programs / Initiatives', '')
    if prog:
        for prg in prog.split(','):
            prg = prg.strip()
            if prg:
                programs[prg] += 1

for prog, count in sorted(programs.items(), key=lambda x: x[1], reverse=True):
    print(f"{prog}: {count} projects")

# 4. Common characteristics
print("\n" + "=" * 80)
print("4. COMMON CHARACTERISTICS")
print("=" * 80)

# Open Source Status
print("\nOpen-Source Status:")
opensource = defaultdict(int)
for p in projects:
    status = p.get('Open-Source?', 'Unknown')
    opensource[status] += 1
for status, count in sorted(opensource.items(), key=lambda x: x[1], reverse=True):
    print(f"  {status}: {count}")

# Integration Status
print("\nIntegration Status:")
integration = defaultdict(int)
for p in projects:
    status = p.get('Integration Status', 'Unknown')
    integration[status] += 1
for status, count in sorted(integration.items(), key=lambda x: x[1], reverse=True):
    print(f"  {status}: {count}")

# Countries
print("\nTop 20 Countries:")
countries = defaultdict(int)
for p in projects:
    country = p.get('Country', 'Unknown').strip()
    if country and country != '\xa0':
        countries[country] += 1
for country, count in sorted(countries.items(), key=lambda x: x[1], reverse=True)[:20]:
    print(f"  {country}: {count}")

# Soroban projects
print("\nSoroban Projects:")
soroban_count = sum(1 for p in projects if p.get('Soroban?', '').lower() == 'checked')
print(f"  {soroban_count} projects marked as Soroban")

# 5. Type breakdown
print("\n" + "=" * 80)
print("5. PROJECT TYPE BREAKDOWN")
print("=" * 80)

types = defaultdict(int)
for p in projects:
    ptype = p.get('Type', 'Unknown')
    if ptype:
        types[ptype] += 1

for ptype, count in sorted(types.items(), key=lambda x: x[1], reverse=True):
    print(f"{ptype}: {count}")

# 6. Awarded Rounds analysis
print("\n" + "=" * 80)
print("6. AWARDED ROUNDS PATTERNS")
print("=" * 80)

rounds = defaultdict(int)
for p in projects:
    rounds_str = p.get('Awarded Rounds', '')
    if rounds_str:
        for rnd in rounds_str.split(','):
            rnd = rnd.strip()
            if rnd:
                rounds[rnd] += 1

print("\nTop 20 Most Active Rounds:")
for rnd, count in sorted(rounds.items(), key=lambda x: x[1], reverse=True)[:20]:
    print(f"  {rnd}: {count} projects")

# 7. Quarter/Year analysis
print("\n" + "=" * 80)
print("7. QUARTER/YEAR PATTERNS")
print("=" * 80)

quarters = defaultdict(int)
for p in projects:
    qtr_str = p.get('Quarter, Year (from Round) (from Awarded Submissions)', '')
    if qtr_str:
        for q in qtr_str.split(','):
            q = q.strip()
            if q:
                quarters[q] += 1

print("\nActive Quarters:")
for qtr, count in sorted(quarters.items(), key=lambda x: x[0]):
    print(f"  {qtr}: {count} projects")

# 8. Regional analysis
print("\n" + "=" * 80)
print("8. REGIONS OF OPERATION")
print("=" * 80)

regions = defaultdict(int)
for p in projects:
    region_str = p.get('Regions of Operation', '')
    if region_str and region_str.strip():
        for reg in region_str.split(','):
            reg = reg.strip()
            if reg:
                regions[reg] += 1

for reg, count in sorted(regions.items(), key=lambda x: x[1], reverse=True):
    print(f"  {reg}: {count}")

# 9. Other Chains
print("\n" + "=" * 80)
print("9. MULTICHAIN ANALYSIS")
print("=" * 80)

chains = defaultdict(int)
multichain_count = 0
stellar_only = 0

for p in projects:
    chain_str = p.get('Other Chains', '')
    if chain_str and chain_str.strip() and chain_str != 'Not Multichain':
        multichain_count += 1
        for chain in chain_str.split(','):
            chain = chain.strip()
            if chain:
                chains[chain] += 1
    elif chain_str == 'Not Multichain':
        stellar_only += 1

print(f"\nStellar-only projects: {stellar_only}")
print(f"Multichain projects: {multichain_count}")
print(f"\nTop chains (besides Stellar):")
for chain, count in sorted(chains.items(), key=lambda x: x[1], reverse=True)[:15]:
    print(f"  {chain}: {count}")

print("\n" + "=" * 80)
print("ANALYSIS COMPLETE")
print("=" * 80)
