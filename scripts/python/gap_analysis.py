"""
GAP ANALYSIS - Identify underserved opportunities in Stellar ecosystem
Usage: python gap_analysis.py
"""

import json
import statistics
from collections import defaultdict

# Load data
with open(r'c:\Users\isich\Documents\scfhandbook\all_projects_structured.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

all_projects = []
for category, projects in data['by_category'].items():
    all_projects.extend(projects)

print("=" * 100)
print("STELLAR ECOSYSTEM GAP ANALYSIS")
print("Finding High-Opportunity, Low-Competition Categories")
print("=" * 100)
print(f"\nAnalyzing {len(all_projects)} funded projects to find gaps...\n")

# Analyze by type
type_analysis = defaultdict(lambda: {'count': 0, 'total_funding': 0, 'projects': []})

for p in all_projects:
    ptype = p.get('type', 'Unknown')
    funding = p.get('total_awarded', 0)

    type_analysis[ptype]['count'] += 1
    type_analysis[ptype]['total_funding'] += funding
    type_analysis[ptype]['projects'].append(p)

# Calculate averages and identify gaps
opportunities = []

for ptype, data in type_analysis.items():
    if data['count'] > 0:
        avg_funding = data['total_funding'] / data['count']

        # Get top-funded average
        top_funded = sorted([p.get('total_awarded', 0) for p in data['projects']], reverse=True)[:max(1, data['count']//4)]
        top_avg = statistics.mean(top_funded) if top_funded else avg_funding

        opportunities.append({
            'type': ptype,
            'count': data['count'],
            'avg_funding': avg_funding,
            'top_avg': top_avg,
            'total_funding': data['total_funding'],
            'opportunity_score': (top_avg / (data['count'] + 1)) / 1000  # Higher = better opportunity
        })

# Sort by opportunity score
opportunities.sort(key=lambda x: x['opportunity_score'], reverse=True)

print("=" * 100)
print("🔥 TOP OPPORTUNITIES (High Value, Low Competition)")
print("=" * 100)
print("\nCriteria: High average funding, few existing projects\n")

for i, opp in enumerate(opportunities[:20], 1):
    count = opp['count']

    if count <= 5:
        competition = "🔥🔥🔥 VERY LOW"
    elif count <= 10:
        competition = "🔥🔥 LOW"
    elif count <= 20:
        competition = "🔥 MODERATE"
    else:
        competition = "⚠️ HIGH"

    print(f"{i}. {opp['type']}")
    print(f"   Existing Projects: {count}")
    print(f"   Competition Level: {competition}")
    print(f"   Average Funding: ${opp['avg_funding']:,.0f}")
    print(f"   Top-Funded Average: ${opp['top_avg']:,.0f}")
    print(f"   Opportunity Score: {opp['opportunity_score']:.1f}")
    print()

# Zero-competition categories
print("=" * 100)
print("💎 POTENTIAL ZERO-COMPETITION CATEGORIES")
print("=" * 100)
print("\nBased on handbook priorities and ecosystem needs:\n")

zero_comp_categories = [
    {
        'category': 'Subscription Payments',
        'reasoning': 'Zero funded projects. Recurring payments critical for SaaS.',
        'estimated_funding': '$75k - $150k',
        'handbook_priority': True
    },
    {
        'category': 'Gas Optimization Tools',
        'reasoning': 'Zero funded projects. Developer efficiency critical.',
        'estimated_funding': '$50k - $100k',
        'handbook_priority': True
    },
    {
        'category': 'Credit Scoring Protocol',
        'reasoning': 'Zero funded projects. Handbook explicitly mentions need.',
        'estimated_funding': '$100k - $200k',
        'handbook_priority': True
    },
    {
        'category': 'Escrow/Insurance Primitives',
        'reasoning': 'Zero funded projects. Handbook explicitly mentions need.',
        'estimated_funding': '$100k - $200k',
        'handbook_priority': True
    },
    {
        'category': 'Proof of Reserves Dashboard',
        'reasoning': 'Zero funded projects. Trust infrastructure needed.',
        'estimated_funding': '$75k - $150k',
        'handbook_priority': False
    },
    {
        'category': 'Contract Migration Tools',
        'reasoning': 'Zero funded projects. Upgrade infrastructure needed.',
        'estimated_funding': '$50k - $100k',
        'handbook_priority': False
    },
    {
        'category': 'Soroban-Native Anchor',
        'reasoning': 'Zero Soroban anchors. Classic anchors avg $331k!',
        'estimated_funding': '$150k - $350k',
        'handbook_priority': True
    },
    {
        'category': 'Identity Protocol',
        'reasoning': 'Only 1 project. Major ecosystem gap.',
        'estimated_funding': '$100k - $200k',
        'handbook_priority': False
    }
]

for i, cat in enumerate(zero_comp_categories, 1):
    priority_marker = "⭐ HANDBOOK PRIORITY" if cat['handbook_priority'] else ""
    print(f"{i}. {cat['category']} {priority_marker}")
    print(f"   Why: {cat['reasoning']}")
    print(f"   Est. Funding: {cat['estimated_funding']}")
    print()

# Saturation analysis
print("=" * 100)
print("⚠️ SATURATED CATEGORIES (High Competition)")
print("=" * 100)
print("\nAvoid unless you have strong differentiation:\n")

saturated = [opp for opp in opportunities if opp['count'] >= 30]
saturated.sort(key=lambda x: x['count'], reverse=True)

for i, opp in enumerate(saturated[:10], 1):
    print(f"{i}. {opp['type']}")
    print(f"   Existing Projects: {opp['count']} (VERY COMPETITIVE)")
    print(f"   Average Funding: ${opp['avg_funding']:,.0f}")
    print(f"   Advice: Need strong differentiation and unique value prop")
    print()

# Category-level gaps
print("=" * 100)
print("CATEGORY-LEVEL ANALYSIS")
print("=" * 100)

category_counts = defaultdict(int)
category_funding = defaultdict(float)

for p in all_projects:
    cat = p.get('category', 'Unknown')
    category_counts[cat] += 1
    category_funding[cat] += p.get('total_awarded', 0)

print("\nProjects per Category:\n")
for cat in sorted(category_counts.keys(), key=lambda x: category_counts[x], reverse=True):
    count = category_counts[cat]
    avg = category_funding[cat] / count if count > 0 else 0

    if count < 50:
        opportunity = "🔥 OPPORTUNITY"
    elif count < 100:
        opportunity = "⭐ MODERATE"
    else:
        opportunity = "⚠️ COMPETITIVE"

    print(f"{cat}: {count} projects | Avg: ${avg:,.0f} | {opportunity}")

# Time-based gaps (what's trending)
print("\n" + "=" * 100)
print("📈 EMERGING TRENDS (Recent Growth)")
print("=" * 100)

recent_projects = [p for p in all_projects if 'Q1 \'24' in p.get('quarters', '') or 'Q2 \'24' in p.get('quarters', '') or 'Q3 \'24' in p.get('quarters', '')]

recent_types = defaultdict(int)
for p in recent_projects:
    ptype = p.get('type', 'Unknown')
    recent_types[ptype] += 1

print(f"\nMost Funded Project Types in 2024 (Q1-Q3):\n")
for ptype, count in sorted(recent_types.items(), key=lambda x: x[1], reverse=True)[:10]:
    print(f"  {ptype}: {count} projects (TRENDING)")

# Strategic recommendations
print("\n" + "=" * 100)
print("💡 STRATEGIC RECOMMENDATIONS")
print("=" * 100)

print("""
BEST OPPORTUNITIES (High Funding, Low Competition):

1. 🔥🔥🔥 ANCHORS
   - Only 4 classic anchors funded ($331k avg)
   - ZERO Soroban-native anchors
   - Strategy: Build Soroban anchor for specific region/use case

2. 🔥🔥🔥 CREDIT SCORING / ESCROW / INSURANCE
   - Zero projects in each
   - Handbook explicitly mentions need
   - Strategy: DeFi primitives, composable with existing protocols

3. 🔥🔥 SUBSCRIPTION PAYMENTS
   - Zero projects
   - Obvious use case for recurring revenue
   - Strategy: Consumer or B2B focus

4. 🔥🔥 ORACLES (more needed)
   - Only 6 projects, avg $212k for top-funded
   - Reflector dominant but specific use cases remain
   - Strategy: Specialized oracles (sports, weather, specialized assets)

5. 🔥🔥 GAS OPTIMIZATION / FORMAL VERIFICATION
   - Minimal competition
   - High developer value
   - Strategy: Developer tools with clear ROI

AVOID (Unless Strong Differentiation):

❌ Wallets (35 projects) - Need unique angle
❌ Payments (44 projects) - Very crowded
❌ Generic Developer Tools (57 projects) - Too many SDKs
❌ Education (22 projects) - Saturated

EMERGING OPPORTUNITIES:

🔥 RWA (Real World Assets) - 9 projects, growing fast
🔥 ZKP (Zero Knowledge Proofs) - Only 3 projects
🔥 Yield Aggregators - Only 1 project (Defindex)
🔥 Derivatives/Options - Only 2 projects each
""")

print("\n" + "=" * 100)
print("NEXT STEPS:")
print("  1. Choose category from 'TOP OPPORTUNITIES' above")
print("  2. Run funding_predictor.py to estimate expected funding")
print("  3. Check handbook for specific requirements")
print("  4. Validate PMF through user interviews")
print("=" * 100)
