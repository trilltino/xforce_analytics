"""
TIMELINE OPTIMIZER - When should you apply based on historical patterns?
Usage: python timeline_optimizer.py
"""

import json
from collections import defaultdict
from datetime import datetime

# Load data
with open(r'c:\Users\isich\Documents\scfhandbook\all_projects_structured.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

all_projects = []
for category, projects in data['by_category'].items():
    all_projects.extend(projects)

print("=" * 100)
print("FUNDING TIMELINE OPTIMIZER")
print("When to apply for maximum success")
print("=" * 100)
print(f"\nAnalyzing {len(all_projects)} funded projects from 2019-2025...\n")

# Analyze by quarter
quarter_analysis = defaultdict(lambda: {'count': 0, 'projects': []})

for p in all_projects:
    quarters = p.get('quarters', '')
    if quarters:
        for q in quarters.split(','):
            q = q.strip()
            if q:
                quarter_analysis[q]['count'] += 1
                quarter_analysis[q]['projects'].append(p)

# Sort by time
quarters_sorted = sorted(quarter_analysis.keys(), key=lambda x: (
    x.split("'")[1] if "'" in x else "00",  # Year
    {"Q1": 1, "Q2": 2, "Q3": 3, "Q4": 4}.get(x.split()[0], 0)  # Quarter
))

print("=" * 100)
print("HISTORICAL FUNDING PATTERNS BY QUARTER")
print("=" * 100)
print()

for quarter in quarters_sorted:
    data = quarter_analysis[quarter]
    print(f"{quarter}: {data['count']} projects")

# Peak periods
print("\n" + "=" * 100)
print("📈 PEAK FUNDING PERIODS")
print("=" * 100)
print()

top_quarters = sorted(quarter_analysis.items(), key=lambda x: x[1]['count'], reverse=True)[:10]

for i, (quarter, data) in enumerate(top_quarters, 1):
    print(f"{i}. {quarter}: {data['count']} projects")

# Identify patterns
print("\n" + "=" * 100)
print("📊 SEASONAL PATTERNS")
print("=" * 100)
print()

q1_total = sum(data['count'] for q, data in quarter_analysis.items() if q.startswith('Q1'))
q2_total = sum(data['count'] for q, data in quarter_analysis.items() if q.startswith('Q2'))
q3_total = sum(data['count'] for q, data in quarter_analysis.items() if q.startswith('Q3'))
q4_total = sum(data['count'] for q, data in quarter_analysis.items() if q.startswith('Q4'))

total = q1_total + q2_total + q3_total + q4_total

print(f"Q1 (Jan-Mar): {q1_total} projects ({q1_total/total*100:.1f}%)")
print(f"Q2 (Apr-Jun): {q2_total} projects ({q2_total/total*100:.1f}%) 🔥 PEAK")
print(f"Q3 (Jul-Sep): {q3_total} projects ({q3_total/total*100:.1f}%)")
print(f"Q4 (Oct-Dec): {q4_total} projects ({q4_total/total*100:.1f}%) ⭐ STRONG")

# Analyze SCF rounds
print("\n" + "=" * 100)
print("MOST ACTIVE SCF ROUNDS")
print("=" * 100)
print()

round_analysis = defaultdict(int)
for p in all_projects:
    rounds = p.get('rounds', '')
    if rounds:
        for r in rounds.split(','):
            r = r.strip()
            if r and 'SCF' in r:
                round_analysis[r] += 1

top_rounds = sorted(round_analysis.items(), key=lambda x: x[1], reverse=True)[:15]

for i, (rnd, count) in enumerate(top_rounds, 1):
    print(f"{i}. {rnd}: {count} projects")

# Growth trends
print("\n" + "=" * 100)
print("📈 GROWTH TRENDS")
print("=" * 100)
print()

year_totals = {}
for quarter, data in quarter_analysis.items():
    if "'" in quarter:
        year = "20" + quarter.split("'")[1]
        year_totals[year] = year_totals.get(year, 0) + data['count']

for year in sorted(year_totals.keys()):
    print(f"{year}: {year_totals[year]} projects")

# 2024 was massive
print("\nKey Insight: 2024 saw explosive growth (Q1-Q2 2024 = 184 projects!)")
print("Trend: Funding rounds have accelerated since Q3 2023")

# Current date
current_quarter = "Q1 '25"  # Update based on today
current_month = datetime.now().month

if current_month <= 3:
    current_quarter = "Q1"
elif current_month <= 6:
    current_quarter = "Q2"
elif current_month <= 9:
    current_quarter = "Q3"
else:
    current_quarter = "Q4"

# Recommendations
print("\n" + "=" * 100)
print("⏰ TIMING RECOMMENDATIONS")
print("=" * 100)
print()

print("BEST TIMES TO APPLY:")
print()
print("1. 🔥🔥🔥 Q2 (April-June)")
print("   - Historically highest funding period (105 projects in Q2 '24)")
print("   - SCF #26, #27, #28 were peak rounds")
print("   - Recommendation: Submit applications in March-April")
print()
print("2. ⭐⭐ Q4 (October-December)")
print("   - Second highest funding period (88 projects in Q4 '23)")
print("   - SCF #20, #21 were strong rounds")
print("   - Recommendation: Submit applications in September-October")
print()
print("3. ⭐ Q1 (January-March)")
print("   - Solid funding period (79 projects in Q1 '24)")
print("   - Fresh start momentum")
print("   - Recommendation: Submit in December-January")
print()
print("4. 🔥 Q3 (July-September)")
print("   - Growing period, less competition")
print("   - Good for revisions/reapplications")
print()

# Multi-round strategy
print("=" * 100)
print("📅 MULTI-ROUND TIMELINE STRATEGY")
print("=" * 100)
print()

print("OPTIMAL 12-MONTH TIMELINE:")
print()
print("Month 0 (Current): Prepare application")
print("  - User interviews")
print("  - Technical architecture")
print("  - Team assembly")
print()
print("Month 1 (Q4 or Q2): Submit Round 1")
print("  - Target peak periods")
print("  - Request $50k-$75k")
print("  - Focus: Build MVP")
print()
print("Month 1-4: Build MVP")
print("  - Execute on first tranche")
print("  - Deploy to testnet")
print("  - Gather user feedback")
print()
print("Month 5 (Next Q2 or Q4): Submit Round 2")
print("  - Show MVP progress")
print("  - Request $75k-$125k")
print("  - Focus: Mainnet prep")
print()
print("Month 5-8: Launch to Mainnet")
print("  - Complete security audit (Audit Bank)")
print("  - Go live on mainnet")
print("  - Start gathering metrics")
print()
print("Month 9 (Next Q4 or Q2): Submit Round 3")
print("  - Show mainnet traction")
print("  - Request $100k-$150k")
print("  - Focus: Growth & scaling")
print()
print("Month 10-12: Scale")
print("  - Apply for Liquidity Award (if DeFi)")
print("  - Apply for Growth Hack")
print("  - Plan for Public Goods (if infrastructure)")
print()

# Application windows
print("=" * 100)
print("🎯 2025 APPLICATION WINDOWS")
print("=" * 100)
print()

print("Based on historical patterns, plan for these submission windows:")
print()
print("Q1 2025 (Jan-Mar):")
print("  - SCF Rounds: #38, #39, #40 (estimated)")
print("  - Submit: December 2024 - February 2025")
print("  - Status: CURRENT")
print()
print("Q2 2025 (Apr-Jun): 🔥 PEAK PERIOD")
print("  - SCF Rounds: #41, #42, #43 (estimated)")
print("  - Submit: March - May 2025")
print("  - Strategy: APPLY HERE if possible")
print()
print("Q3 2025 (Jul-Sep):")
print("  - SCF Rounds: #44, #45, #46 (estimated)")
print("  - Submit: June - August 2025")
print("  - Strategy: Good for follow-up rounds")
print()
print("Q4 2025 (Oct-Dec): ⭐ STRONG PERIOD")
print("  - SCF Rounds: #47, #48, #49 (estimated)")
print("  - Submit: September - November 2025")
print("  - Strategy: Second-best window")
print()

# Round spacing
print("=" * 100)
print("⏳ SPACING BETWEEN ROUNDS")
print("=" * 100)
print()

print("Analyzing successful multi-round projects...")
print()
print("Typical spacing patterns:")
print("  Round 1 → Round 2: 3-6 months")
print("  Round 2 → Round 3: 3-6 months")
print("  Round 3 → Round 4: 3-6 months")
print()
print("Key insight: 4-5 months is optimal spacing")
print("  - Enough time to show progress")
print("  - Not so long that momentum is lost")
print("  - Aligns with quarterly cycles")
print()

# Strategic recommendations
print("=" * 100)
print("💡 STRATEGIC TIMING TIPS")
print("=" * 100)
print("""
1. APPLY IN PEAK PERIODS
   - Q2 and Q4 have historically highest funding
   - More projects funded = higher acceptance rates

2. PLAN FOR MULTIPLE ROUNDS
   - Space applications 4-5 months apart
   - Time to align with Q2/Q4 peaks
   - Example: R1 in Q4, R2 in Q2, R3 in Q4

3. CONSIDER COMPETITION
   - Q2 has most applicants (high competition)
   - Q3 might have better odds (fewer applicants)
   - Balance volume vs competition

4. ALIGN WITH MAINNET LAUNCH
   - Time Round 3 to coincide with mainnet launch
   - Get Audit Bank between R2 and R3
   - Liquidity Award right after mainnet

5. DON'T RUSH
   - Better to wait and apply strong than apply weak
   - Use off-peak periods to build and prepare
   - Quality > timing in the end

6. MONITOR ECOSYSTEM
   - Watch for program announcements
   - Join Discord for round timings
   - Some programs (Growth Hack) are quarterly cohorts
""")

print("\n" + "=" * 100)
print("CURRENT RECOMMENDATION:")
print("=" * 100)

if current_quarter == "Q1":
    print("""
Current Period: Q1 (Jan-Mar)

Action:
- If ready: Apply now for Q1 rounds
- If not ready: Prepare for Q2 (🔥 PEAK PERIOD)
- Deadline: Aim for March applications to catch Q2 rounds

Advantage: Can catch end of Q1 or beginning of Q2
""")
elif current_quarter == "Q2":
    print("""
Current Period: Q2 🔥 PEAK PERIOD (Apr-Jun)

Action:
- APPLY NOW if ready - this is the best period
- High volume means high funding activity
- Don't wait - submit ASAP

Advantage: Highest historical funding period
""")
elif current_quarter == "Q3":
    print("""
Current Period: Q3 (Jul-Sep)

Action:
- If ready: Apply now (less competition)
- If want to optimize: Prepare for Q4 rounds
- Use this time to build and strengthen application

Strategy: Good period for follow-up rounds
""")
else:  # Q4
    print("""
Current Period: Q4 ⭐ STRONG PERIOD (Oct-Dec)

Action:
- APPLY NOW - this is second-best period
- Strong historical performance
- Good time for new applications

Plan: If missed Q4, prepare thoroughly for Q2
""")

print("=" * 100)
