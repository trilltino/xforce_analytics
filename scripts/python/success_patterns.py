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

print("=" * 100)
print("SUCCESS PATTERNS ANALYSIS FOR SDF APPLICATION GUIDE")
print("=" * 100)

# Analyze high-funded projects (top 25%)
all_funded = [(p, get_funding(p)) for p in projects if get_funding(p) > 0]
all_funded.sort(key=lambda x: x[1], reverse=True)

top_quartile_threshold = all_funded[len(all_funded)//4][1]
print(f"\nTop 25% funding threshold: ${top_quartile_threshold:,.2f}")

top_funded = [p for p, f in all_funded if f >= top_quartile_threshold]

print(f"\nAnalyzing {len(top_funded)} highly-funded projects...\n")

# Pattern 1: Open Source vs Closed
print("=" * 100)
print("PATTERN 1: OPEN SOURCE STATUS")
print("=" * 100)

opensource_dist = defaultdict(lambda: {'count': 0, 'funding': []})
for p in top_funded:
    status = p.get('Open-Source?', 'Unknown')
    opensource_dist[status]['count'] += 1
    opensource_dist[status]['funding'].append(get_funding(p))

print("\nIn Top-Funded Projects:")
for status, data in sorted(opensource_dist.items(), key=lambda x: x[1]['count'], reverse=True):
    avg_funding = statistics.mean(data['funding'])
    print(f"{status}: {data['count']} projects (avg: ${avg_funding:,.2f})")

# Pattern 2: Integration Status at time of funding
print("\n" + "=" * 100)
print("PATTERN 2: INTEGRATION STATUS AT FUNDING")
print("=" * 100)

integration_dist = defaultdict(lambda: {'count': 0, 'funding': []})
for p in top_funded:
    status = p.get('Integration Status', 'Unknown')
    integration_dist[status]['count'] += 1
    integration_dist[status]['funding'].append(get_funding(p))

print("\nIn Top-Funded Projects:")
for status, data in sorted(integration_dist.items(), key=lambda x: x[1]['count'], reverse=True):
    avg_funding = statistics.mean(data['funding'])
    print(f"{status}: {data['count']} projects (avg: ${avg_funding:,.2f})")

# Pattern 3: Geographic trends
print("\n" + "=" * 100)
print("PATTERN 3: GEOGRAPHIC TRENDS (Top Countries)")
print("=" * 100)

country_dist = defaultdict(lambda: {'count': 0, 'funding': []})
for p in top_funded:
    country = p.get('Country', 'Unknown').strip()
    if country and country != '\xa0':
        country_dist[country]['count'] += 1
        country_dist[country]['funding'].append(get_funding(p))

print("\nTop 15 Countries in Highly-Funded Projects:")
for country, data in sorted(country_dist.items(), key=lambda x: x[1]['count'], reverse=True)[:15]:
    avg_funding = statistics.mean(data['funding'])
    print(f"{country}: {data['count']} projects (avg: ${avg_funding:,.2f})")

# Pattern 4: Multiple rounds correlation
print("\n" + "=" * 100)
print("PATTERN 4: MULTIPLE FUNDING ROUNDS")
print("=" * 100)

def count_rounds(p):
    rounds_str = p.get('Awarded Rounds', '')
    if not rounds_str:
        return 0
    return len([r for r in rounds_str.split(',') if r.strip()])

rounds_analysis = defaultdict(list)
for p in all_funded:
    num_rounds = count_rounds(p[0])
    rounds_analysis[num_rounds].append(p[1])

print("\nFunding vs Number of Rounds:")
for num_rounds in sorted(rounds_analysis.keys()):
    if rounds_analysis[num_rounds]:
        avg_funding = statistics.mean(rounds_analysis[num_rounds])
        count = len(rounds_analysis[num_rounds])
        print(f"{num_rounds} round(s): {count} projects (avg funding: ${avg_funding:,.2f})")

# Pattern 5: Type/Category success patterns
print("\n" + "=" * 100)
print("PATTERN 5: SUCCESSFUL PROJECT TYPES")
print("=" * 100)

type_dist = defaultdict(lambda: {'count': 0, 'funding': []})
for p in top_funded:
    ptype = p.get('Type', 'Unknown')
    type_dist[ptype]['count'] += 1
    type_dist[ptype]['funding'].append(get_funding(p))

print("\nTop 15 Types in Highly-Funded Projects:")
for ptype, data in sorted(type_dist.items(), key=lambda x: x[1]['count'], reverse=True)[:15]:
    avg_funding = statistics.mean(data['funding'])
    print(f"{ptype}: {data['count']} projects (avg: ${avg_funding:,.2f})")

# Pattern 6: Soroban vs Classic
print("\n" + "=" * 100)
print("PATTERN 6: SOROBAN VS CLASSIC STELLAR")
print("=" * 100)

soroban_projects = [p for p in top_funded if p.get('Soroban?', '').lower() == 'checked']
non_soroban_projects = [p for p in top_funded if p.get('Soroban?', '').lower() != 'checked']

soroban_avg = statistics.mean([get_funding(p) for p in soroban_projects])
non_soroban_avg = statistics.mean([get_funding(p) for p in non_soroban_projects]) if non_soroban_projects else 0

print(f"\nSoroban projects: {len(soroban_projects)} (avg: ${soroban_avg:,.2f})")
print(f"Classic Stellar projects: {len(non_soroban_projects)} (avg: ${non_soroban_avg:,.2f})")

# Pattern 7: Multichain vs Stellar-only
print("\n" + "=" * 100)
print("PATTERN 7: MULTICHAIN VS STELLAR-ONLY")
print("=" * 100)

multichain = []
stellar_only = []

for p in top_funded:
    chains = p.get('Other Chains', '')
    if chains and chains.strip() and chains != 'Not Multichain':
        multichain.append(p)
    else:
        stellar_only.append(p)

mc_avg = statistics.mean([get_funding(p) for p in multichain]) if multichain else 0
so_avg = statistics.mean([get_funding(p) for p in stellar_only]) if stellar_only else 0

print(f"\nMultichain projects: {len(multichain)} (avg: ${mc_avg:,.2f})")
print(f"Stellar-only projects: {len(stellar_only)} (avg: ${so_avg:,.2f})")

# Pattern 8: Team/Traction indicators
print("\n" + "=" * 100)
print("PATTERN 8: TRACTION INDICATORS")
print("=" * 100)

with_traction = [p for p in top_funded if len(p.get('Current Traction', '').strip()) > 50]
without_traction = [p for p in top_funded if len(p.get('Current Traction', '').strip()) <= 50]

wt_avg = statistics.mean([get_funding(p) for p in with_traction])
nt_avg = statistics.mean([get_funding(p) for p in without_traction]) if without_traction else 0

print(f"\nProjects with detailed traction description: {len(with_traction)} (avg: ${wt_avg:,.2f})")
print(f"Projects without detailed traction: {len(without_traction)} (avg: ${nt_avg:,.2f})")

# Pattern 9: Program participation
print("\n" + "=" * 100)
print("PATTERN 9: PROGRAM PARTICIPATION")
print("=" * 100)

program_patterns = defaultdict(lambda: {'count': 0, 'funding': []})
for p in top_funded:
    programs = p.get('Programs / Initiatives', '')
    if programs:
        for prog in programs.split(','):
            prog = prog.strip()
            if prog:
                program_patterns[prog]['count'] += 1
                program_patterns[prog]['funding'].append(get_funding(p))

print("\nProgram participation in highly-funded projects:")
for prog, data in sorted(program_patterns.items(), key=lambda x: x[1]['count'], reverse=True):
    avg_funding = statistics.mean(data['funding'])
    print(f"{prog}: {data['count']} projects (avg: ${avg_funding:,.2f})")

# Pattern 10: Key success indicators
print("\n" + "=" * 100)
print("PATTERN 10: KEY SUCCESS INDICATORS SUMMARY")
print("=" * 100)

# Analyze mainnet projects
mainnet_top = [p for p in top_funded if p.get('Integration Status') == 'Mainnet']
print(f"\n* {len(mainnet_top)}/{len(top_funded)} ({len(mainnet_top)/len(top_funded)*100:.1f}%) of top-funded projects reached Mainnet")

# Multiple rounds
multi_round = [p for p in top_funded if count_rounds(p) >= 2]
print(f"* {len(multi_round)}/{len(top_funded)} ({len(multi_round)/len(top_funded)*100:.1f}%) received funding across multiple rounds")

# Open source
opensource_top = [p for p in top_funded if 'open-source' in p.get('Open-Source?', '').lower()]
print(f"* {len(opensource_top)}/{len(top_funded)} ({len(opensource_top)/len(top_funded)*100:.1f}%) are fully or partially open-source")

# Has Github
with_github = [p for p in top_funded if p.get('Github', '').strip()]
print(f"* {len(with_github)}/{len(top_funded)} ({len(with_github)/len(top_funded)*100:.1f}%) have public Github repositories")

# Has website
with_website = [p for p in top_funded if p.get('Website', '').strip()]
print(f"* {len(with_website)}/{len(top_funded)} ({len(with_website)/len(top_funded)*100:.1f}%) have a website")

# Soroban
soroban_pct = len(soroban_projects) / len(top_funded) * 100
print(f"* {len(soroban_projects)}/{len(top_funded)} ({soroban_pct:.1f}%) are Soroban-based")

print("\n" + "=" * 100)
print("ANALYSIS COMPLETE")
print("=" * 100)
