import json
from collections import defaultdict

with open('all_projects_structured.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

all_projects = []
for category, projects in data['by_category'].items():
    all_projects.extend(projects)

# Type analysis for low competition
type_stats = defaultdict(lambda: {'count': 0, 'funding': []})
for p in all_projects:
    ptype = p.get('type', 'Unknown')
    funding = p.get('total_awarded', 0)
    type_stats[ptype]['count'] += 1
    if funding > 0:
        type_stats[ptype]['funding'].append(funding)

# Find opportunities (low count, high funding)
opportunities = []
for ptype, stats in type_stats.items():
    if stats['count'] <= 10 and stats['funding']:
        avg = sum(stats['funding']) / len(stats['funding'])
        max_fund = max(stats['funding'])
        opportunities.append({
            'type': ptype,
            'count': stats['count'],
            'avg': avg,
            'max': max_fund,
            'score': avg / (stats['count'] + 1)
        })

opportunities.sort(key=lambda x: x['score'], reverse=True)

print("=" * 80)
print("TOP 20 OPPORTUNITIES (Low Competition, High Value)")
print("=" * 80)
for i, opp in enumerate(opportunities[:20], 1):
    print(f"{i}. {opp['type']}")
    print(f"   Projects: {opp['count']} (LOW COMPETITION)")
    print(f"   Avg Funding: ${opp['avg']:,.0f}")
    print(f"   Max Funding: ${opp['max']:,.0f}")
    print()

# Saturated categories
print("=" * 80)
print("SATURATED CATEGORIES (Avoid Unless Strong Differentiation)")
print("=" * 80)
saturated = [(t, s['count'], sum(s['funding'])/len(s['funding']) if s['funding'] else 0)
             for t, s in type_stats.items() if s['count'] >= 30]
saturated.sort(key=lambda x: x[1], reverse=True)

for i, (ptype, count, avg) in enumerate(saturated[:10], 1):
    print(f"{i}. {ptype}: {count} projects (HIGH COMPETITION), ${avg:,.0f} avg")

# Recent trends (2024)
print("\n" + "=" * 80)
print("EMERGING TRENDS (2024 Growth)")
print("=" * 80)
recent = [p for p in all_projects if any(q in p.get('quarters', '') for q in ["Q1 '24", "Q2 '24", "Q3 '24"])]
recent_types = defaultdict(int)
for p in recent:
    recent_types[p.get('type', 'Unknown')] += 1

for ptype, count in sorted(recent_types.items(), key=lambda x: x[1], reverse=True)[:10]:
    print(f"  {ptype}: {count} projects in 2024")

# Zero competition categories from handbook
print("\n" + "=" * 80)
print("ZERO-COMPETITION OPPORTUNITIES (From Handbook)")
print("=" * 80)
print("""
Based on handbook priorities and ecosystem analysis:

1. Subscription Payment Infrastructure (ZERO projects)
   - Recurring payments critical for SaaS
   - Estimated: $75k - $150k

2. Credit Scoring Protocol (ZERO projects)
   - Handbook explicitly mentions need
   - DeFi primitive
   - Estimated: $100k - $200k

3. Escrow/Insurance Primitives (ZERO projects)
   - Handbook explicitly mentions need
   - DeFi building block
   - Estimated: $100k - $200k

4. Gas Optimization Tools (ZERO projects)
   - Developer efficiency critical
   - Estimated: $50k - $100k

5. Contract Migration Tools (ZERO projects)
   - Upgrade infrastructure needed
   - Estimated: $50k - $100k

6. Proof of Reserves Dashboard (ZERO projects)
   - Trust infrastructure
   - Estimated: $75k - $150k

7. Soroban-Native Anchor (ZERO identified)
   - Classic anchors avg $331k!
   - Estimated: $150k - $350k
""")
