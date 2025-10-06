"""
COMPETITIVE LANDSCAPE ANALYZER - Compare your project against existing funded projects
Usage: python competitive_analyzer.py
"""

import json
from collections import defaultdict

# Load data
with open(r'c:\Users\isich\Documents\scfhandbook\all_projects_structured.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

all_projects = []
for category, projects in data['by_category'].items():
    all_projects.extend(projects)

print("=" * 100)
print("COMPETITIVE LANDSCAPE ANALYZER")
print("Analyze competition in your chosen category")
print("=" * 100)
print(f"\nSearching {len(all_projects)} funded projects...\n")

# Get user input
print("What type of project are you building?")
print("Examples: Wallet, DEX, Oracle, SDK, Anchor, NFTs, Lending, Gaming, etc.\n")

search_term = input("Enter project type: ").strip()

# Find similar projects
similar = []
for p in all_projects:
    ptype = p.get('type', '').lower()
    description = p.get('description', '').lower()

    if search_term.lower() in ptype or search_term.lower() in description:
        similar.append(p)

if not similar:
    print(f"\nNo direct competitors found for '{search_term}'")
    print("\n🔥🔥🔥 OPPORTUNITY: Low/zero competition category!")
    print("\nConsider:")
    print("  - You may be first-to-market in this niche")
    print("  - Validate there's actual demand")
    print("  - Reference similar successful categories in application")

else:
    print(f"\n{'=' * 100}")
    print(f"COMPETITIVE ANALYSIS: {len(similar)} similar projects found")
    print(f"{'=' * 100}\n")

    # Sort by funding
    similar.sort(key=lambda x: x.get('total_awarded', 0), reverse=True)

    # Summary statistics
    total_funding = sum(p.get('total_awarded', 0) for p in similar)
    avg_funding = total_funding / len(similar) if similar else 0

    mainnet_count = sum(1 for p in similar if p.get('integration_status') == 'Mainnet')
    soroban_count = sum(1 for p in similar if p.get('soroban', False))
    opensource_count = sum(1 for p in similar if 'open-source' in p.get('open_source', '').lower())

    print("OVERVIEW:")
    print(f"  Total Competitors: {len(similar)}")
    print(f"  Total Funding Distributed: ${total_funding:,.0f}")
    print(f"  Average Funding: ${avg_funding:,.0f}")
    print(f"  On Mainnet: {mainnet_count} ({mainnet_count/len(similar)*100:.1f}%)")
    print(f"  Soroban-based: {soroban_count} ({soroban_count/len(similar)*100:.1f}%)")
    print(f"  Open-Source: {opensource_count} ({opensource_count/len(similar)*100:.1f}%)")

    # Competition level
    if len(similar) <= 5:
        comp_level = "🔥 LOW - Good opportunity"
    elif len(similar) <= 15:
        comp_level = "⭐ MODERATE - Need differentiation"
    else:
        comp_level = "⚠️ HIGH - Strong differentiation required"

    print(f"\n  Competition Level: {comp_level}\n")

    # Top competitors
    print(f"{'=' * 100}")
    print("TOP FUNDED COMPETITORS")
    print(f"{'=' * 100}\n")

    for i, proj in enumerate(similar[:10], 1):
        title = proj.get('title', 'Unknown')
        funding = proj.get('total_awarded', 0)
        country = proj.get('country', 'Unknown')
        status = proj.get('integration_status', 'Unknown')
        rounds = proj.get('rounds', 'Unknown')
        website = proj.get('website', 'N/A')
        github = proj.get('github', 'N/A')
        description = proj.get('description', 'No description')

        print(f"{i}. {title} - ${funding:,.0f}")
        print(f"   Country: {country}")
        print(f"   Status: {status}")
        print(f"   Rounds: {rounds}")
        print(f"   Website: {website}")
        print(f"   Github: {github}")
        print(f"   Description: {description[:200]}...")
        print()

    # Geographic distribution
    print(f"{'=' * 100}")
    print("GEOGRAPHIC DISTRIBUTION")
    print(f"{'=' * 100}\n")

    countries = defaultdict(int)
    for p in similar:
        country = p.get('country', 'Unknown')
        if country and country.strip():
            countries[country] += 1

    for country, count in sorted(countries.items(), key=lambda x: x[1], reverse=True):
        print(f"  {country}: {count} project(s)")

    # Integration status
    print(f"\n{'=' * 100}")
    print("INTEGRATION STATUS DISTRIBUTION")
    print(f"{'=' * 100}\n")

    statuses = defaultdict(int)
    for p in similar:
        status = p.get('integration_status', 'Unknown')
        statuses[status] += 1

    for status, count in sorted(statuses.items(), key=lambda x: x[1], reverse=True):
        pct = count / len(similar) * 100
        print(f"  {status}: {count} ({pct:.1f}%)")

    # Differentiation opportunities
    print(f"\n{'=' * 100}")
    print("💡 DIFFERENTIATION OPPORTUNITIES")
    print(f"{'=' * 100}\n")

    # Check for gaps
    if mainnet_count < len(similar) * 0.5:
        print("✓ Most competitors not on mainnet yet - ship fast to win")

    if soroban_count < len(similar) * 0.5:
        print("✓ Consider Soroban-native approach (68% of top-funded use it)")

    if opensource_count < len(similar) * 0.7:
        print("✓ Open-source could be differentiator (70% of top-funded are)")

    # Geographic gaps
    if 'United States' not in countries or countries['United States'] < len(similar) * 0.3:
        print("✓ US market underserved")

    if not any(c in ['Nigeria', 'Kenya', 'South Africa', 'Ghana'] for c in countries):
        print("✓ African market opportunity")

    if not any(c in ['India', 'Singapore', 'Japan', 'South Korea'] for c in countries):
        print("✓ APAC market opportunity")

    if not any(c in ['Argentina', 'Chile', 'Mexico', 'Colombia', 'Brazil'] for c in countries):
        print("✓ Latin America opportunity")

    # Feature gaps
    print("\n" + "=" * 100)
    print("DIFFERENTIATION STRATEGIES")
    print("=" * 100)
    print("""
To compete in this category, consider:

1. TECHNICAL DIFFERENTIATION
   - Novel algorithm or architecture
   - Better performance/lower costs
   - Superior UX/developer experience
   - Composability with existing protocols
   - Security-first approach (get audited)

2. MARKET DIFFERENTIATION
   - Different target audience
   - Geographic focus (APAC, LatAm, Africa)
   - Vertical specialization (gaming, RWA, etc.)
   - B2B vs B2C positioning

3. EXECUTION DIFFERENTIATION
   - Ship faster to mainnet
   - Better documentation/support
   - Stronger partnerships
   - Community engagement
   - Open-source with active contributions

4. ECONOMIC DIFFERENTIATION
   - Better fee structure
   - Sustainability model
   - Token economics (if applicable)
   - Value sharing with ecosystem
""")

# Strategy recommendations
print("=" * 100)
print("STRATEGIC RECOMMENDATIONS")
print("=" * 100)

if len(similar) == 0:
    print("""
🔥🔥🔥 ZERO COMPETITION - HIGHEST OPPORTUNITY

Action Plan:
1. Validate there's actual demand (10+ user interviews)
2. Reference similar successful categories in application
3. Explain WHY this category is needed
4. Apply with confidence - first-mover advantage
5. Expected funding: $75k - $150k Round 1

Risk: Ensure it's a real gap, not just underfunded for good reason
""")

elif len(similar) <= 5:
    print(f"""
🔥 LOW COMPETITION ({len(similar)} projects)

Action Plan:
1. Study top-funded competitor closely
2. Identify specific gaps in their offering
3. Emphasize your differentiation clearly
4. Show you've researched the space
5. Expected funding: ${avg_funding * 0.8:,.0f} - ${avg_funding * 1.2:,.0f}

Advantage: Room for multiple winners
""")

elif len(similar) <= 15:
    print(f"""
⭐ MODERATE COMPETITION ({len(similar)} projects)

Action Plan:
1. Analyze top 3 competitors in detail
2. Find a niche or unique angle
3. Demonstrate clear differentiation in application
4. Consider geographic or vertical specialization
5. Expected funding: ${avg_funding * 0.7:,.0f} - ${avg_funding:,.0f}

Challenge: Need strong positioning
""")

else:
    print(f"""
⚠️ HIGH COMPETITION ({len(similar)} projects)

Action Plan:
1. Seriously question if you should pursue this
2. If yes: MUST have exceptional differentiation
3. Study ALL top-funded competitors
4. Find underserved sub-niche
5. Consider pivoting to less competitive category
6. Expected funding: ${avg_funding * 0.5:,.0f} - ${avg_funding * 0.8:,.0f}

Warning: Reviewers will compare you directly to existing projects
Need to explain why ecosystem needs another {search_term}
""")

print("\n" + "=" * 100)
print("NEXT STEPS:")
print("  1. Research competitors listed above")
print("  2. Visit their websites and try their products")
print("  3. Identify specific gaps or weaknesses")
print("  4. Document your differentiation clearly")
print("  5. Run gap_analysis.py to find less competitive alternatives")
print("=" * 100)
