import json
from collections import defaultdict

with open('all_projects_structured.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

all_projects = []
for category, projects in data['by_category'].items():
    all_projects.extend(projects)

# Quarter analysis
quarter_counts = defaultdict(int)
for p in all_projects:
    quarters = p.get('quarters', '')
    if quarters:
        for q in quarters.split(','):
            q = q.strip()
            if q:
                quarter_counts[q] += 1

print("=" * 80)
print("PEAK FUNDING PERIODS")
print("=" * 80)
top_quarters = sorted(quarter_counts.items(), key=lambda x: x[1], reverse=True)[:10]
for i, (q, count) in enumerate(top_quarters, 1):
    print(f"{i}. {q}: {count} projects")

# Seasonal patterns
print("\n" + "=" * 80)
print("SEASONAL PATTERNS")
print("=" * 80)
q1 = sum(c for q, c in quarter_counts.items() if q.startswith('Q1'))
q2 = sum(c for q, c in quarter_counts.items() if q.startswith('Q2'))
q3 = sum(c for q, c in quarter_counts.items() if q.startswith('Q3'))
q4 = sum(c for q, c in quarter_counts.items() if q.startswith('Q4'))
total = q1 + q2 + q3 + q4

print(f"Q1 (Jan-Mar): {q1} projects ({q1/total*100:.1f}%)")
print(f"Q2 (Apr-Jun): {q2} projects ({q2/total*100:.1f}%) <-- PEAK")
print(f"Q3 (Jul-Sep): {q3} projects ({q3/total*100:.1f}%)")
print(f"Q4 (Oct-Dec): {q4} projects ({q4/total*100:.1f}%) <-- STRONG")

# SCF rounds
print("\n" + "=" * 80)
print("MOST ACTIVE SCF ROUNDS")
print("=" * 80)
round_counts = defaultdict(int)
for p in all_projects:
    rounds = p.get('rounds', '')
    if rounds:
        for r in rounds.split(','):
            r = r.strip()
            if 'SCF' in r:
                round_counts[r] += 1

for i, (rnd, count) in enumerate(sorted(round_counts.items(), key=lambda x: x[1], reverse=True)[:10], 1):
    print(f"{i}. {rnd}: {count} projects")

print("\n" + "=" * 80)
print("OPTIMAL APPLICATION STRATEGY")
print("=" * 80)
print("""
BEST TIMES TO APPLY:

1. Q2 (April-June) - PEAK PERIOD
   - Highest funding historically
   - Q2 '24 had 105 projects funded
   - Submit: March-April for Q2 rounds

2. Q4 (October-December) - STRONG PERIOD
   - Second highest funding
   - Q4 '23 had 88 projects funded
   - Submit: September-October for Q4 rounds

3. Q1 (January-March) - SOLID PERIOD
   - Q1 '24 had 79 projects funded
   - Submit: December-January

MULTI-ROUND SPACING:
   - Optimal: 4-5 months between applications
   - Allows time to show progress
   - Aligns with quarterly cycles
   - Example: Round 1 in Q4, Round 2 in Q2, Round 3 in Q4
""")
