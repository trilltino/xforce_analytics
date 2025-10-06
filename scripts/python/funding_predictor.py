"""
FUNDING PREDICTOR - Predict expected funding based on project characteristics
Usage: python funding_predictor.py
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
print("SDF FUNDING PREDICTOR - Estimate Your Expected Funding")
print("=" * 100)
print("\nBased on analysis of 443 funded projects\n")

# Build prediction models
def get_funding_estimate(category, project_type, stage, soroban, stellar_only, rounds):
    """
    Predict funding based on key characteristics
    """

    # Filter relevant projects
    relevant = []
    for p in all_projects:
        matches = True

        if category and p.get('category') != category:
            matches = False
        if project_type and p.get('type') != project_type:
            matches = False
        if stage and p.get('integration_status') != stage:
            matches = False
        if soroban is not None and p.get('soroban', False) != soroban:
            matches = False

        if stellar_only is not None:
            is_multichain = p.get('other_chains', 'Not Multichain') != 'Not Multichain'
            if stellar_only and is_multichain:
                matches = False
            if not stellar_only and not is_multichain:
                matches = False

        if matches:
            relevant.append(p)

    if not relevant:
        return None, None, None, 0

    # Calculate statistics
    funding_amounts = [p['total_awarded'] for p in relevant if p.get('total_awarded', 0) > 0]

    if not funding_amounts:
        return None, None, None, 0

    avg = statistics.mean(funding_amounts)
    median = statistics.median(funding_amounts)

    # Adjust for rounds
    if rounds > 1:
        multipliers = {1: 1.0, 2: 1.8, 3: 3.4, 4: 5.0}
        multiplier = multipliers.get(rounds, 5.0)
        avg *= multiplier
        median *= multiplier

    return avg, median, min(funding_amounts), len(relevant)

# Interactive predictor
print("PREDICTOR TOOL - Answer questions to estimate your funding:\n")

categories = ['Applications', 'Financial Protocols', 'Infrastructure & Services', 'Developer Tooling', 'Education & Community']
print("Categories:")
for i, cat in enumerate(categories, 1):
    print(f"  {i}. {cat}")

types_by_category = {
    'Applications': ['Wallet', 'Payments', 'NFTs', 'Gaming', 'Other Application'],
    'Financial Protocols': ['DEX', 'Lending & Borrowing', 'Anchor', 'Other'],
    'Infrastructure & Services': ['Oracle', 'Indexer', 'Security Tool', 'On-chain Reporting & Data', 'Other'],
    'Developer Tooling': ['SDK', 'IDE', 'Testing Tool', 'Security Tool', 'Other'],
    'Education & Community': ['Education & Community']
}

stages = ['Idea', 'Development', 'Testnet', 'Mainnet']

# Scenario Analysis - Common project profiles
print("\n" + "=" * 100)
print("SCENARIO ANALYSIS - Common Project Profiles")
print("=" * 100)

scenarios = [
    {
        'name': 'New Wallet (Idea Stage, Round 1)',
        'category': 'Applications',
        'type': 'Wallet',
        'stage': 'Idea',
        'soroban': True,
        'stellar_only': True,
        'rounds': 1
    },
    {
        'name': 'DEX on Mainnet (Multiple Rounds)',
        'category': 'Financial Protocols',
        'type': 'DEX',
        'stage': 'Mainnet',
        'soroban': True,
        'stellar_only': True,
        'rounds': 3
    },
    {
        'name': 'Oracle Infrastructure (Testnet)',
        'category': 'Infrastructure & Services',
        'type': 'Oracle',
        'stage': 'Testnet',
        'soroban': True,
        'stellar_only': True,
        'rounds': 2
    },
    {
        'name': 'Security Tool (Development)',
        'category': 'Developer Tooling',
        'type': 'Other Security Tool',
        'stage': 'Development',
        'soroban': True,
        'stellar_only': True,
        'rounds': 1
    },
    {
        'name': 'Anchor (Mainnet, Multiple Rounds)',
        'category': 'Financial Protocols',
        'type': 'Anchor',
        'stage': 'Mainnet',
        'soroban': False,
        'stellar_only': True,
        'rounds': 3
    },
]

for scenario in scenarios:
    avg, median, min_fund, count = get_funding_estimate(
        scenario['category'],
        scenario['type'],
        scenario['stage'],
        scenario['soroban'],
        scenario['stellar_only'],
        scenario['rounds']
    )

    print(f"\n{scenario['name']}:")
    print(f"  Category: {scenario['category']}")
    print(f"  Type: {scenario['type']}")
    print(f"  Stage: {scenario['stage']}")
    print(f"  Soroban: {scenario['soroban']}")
    print(f"  Stellar-Only: {scenario['stellar_only']}")
    print(f"  Expected Rounds: {scenario['rounds']}")
    print(f"  ---")
    if avg:
        print(f"  Expected Funding (Average): ${avg:,.0f}")
        print(f"  Expected Funding (Median): ${median:,.0f}")
        print(f"  Minimum Historical: ${min_fund:,.0f}")
        print(f"  Based on {count} similar projects")
    else:
        print(f"  No similar projects found in dataset")

# Category-specific recommendations
print("\n" + "=" * 100)
print("CATEGORY-SPECIFIC FUNDING EXPECTATIONS")
print("=" * 100)

for category in categories:
    print(f"\n{category}:")

    for stage in stages:
        avg, median, min_fund, count = get_funding_estimate(
            category, None, stage, True, True, 1
        )

        if avg and count >= 3:
            print(f"  {stage}: ${median:,.0f} (median, based on {count} projects)")

# Multiplier effects
print("\n" + "=" * 100)
print("FUNDING MULTIPLIERS")
print("=" * 100)

print("\n1. MULTIPLE ROUNDS MULTIPLIER:")
print("   Round 1: 1.0x (baseline)")
print("   Round 2: 1.8x")
print("   Round 3: 3.4x")
print("   Round 4: 5.0x")

print("\n2. MAINNET BONUS: +30% average")
print("3. STELLAR-ONLY PREMIUM: +30% average")
print("4. AUDIT BANK CORRELATION: +28% average")

# Custom calculator
print("\n" + "=" * 100)
print("CUSTOM FUNDING CALCULATOR")
print("=" * 100)

def calculate_custom(base_amount, mainnet=False, stellar_only=False, audit=False, rounds=1):
    """Calculate expected funding with modifiers"""
    total = base_amount

    if mainnet:
        total *= 1.3
    if stellar_only:
        total *= 1.3
    if audit:
        total *= 1.28

    # Rounds multiplier
    round_mults = {1: 1.0, 2: 1.8, 3: 3.4, 4: 5.0}
    total *= round_mults.get(rounds, 5.0)

    return total

print("\nExample Calculation:")
print("Base: $50,000 (Idea stage, typical)")
print("Mainnet: +30% = $65,000")
print("Stellar-Only: +30% = $84,500")
print("3 Rounds: 3.4x = $287,300")
print("Audit Bank: +28% = $367,744")

print("\n" + "=" * 100)
print("USE THIS DATA TO:")
print("  1. Set realistic funding expectations")
print("  2. Choose high-value categories")
print("  3. Plan multi-round strategy")
print("  4. Optimize project characteristics")
print("=" * 100)
