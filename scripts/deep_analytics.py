"""
Deep Analytics Generator
Generates advanced analytics: success patterns, program combinations, correlations
"""

import json
from collections import defaultdict
import statistics


def load_all_data():
    """Load all available data files"""
    with open('data/raw/all_projects_structured.json', 'r', encoding='utf-8') as f:
        raw_data = json.load(f)

    # Flatten projects
    all_projects = []
    for category, projects in raw_data['by_category'].items():
        all_projects.extend(projects)

    # Try to load enriched data if available
    enriched = {}
    try:
        with open('data/enriched/github_metrics.json', 'r', encoding='utf-8') as f:
            enriched['github'] = {p['title']: p for p in json.load(f)['projects']}
    except:
        enriched['github'] = {}

    try:
        with open('data/enriched/social_links.json', 'r', encoding='utf-8') as f:
            enriched['social'] = {p['title']: p for p in json.load(f)['projects']}
    except:
        enriched['social'] = {}

    try:
        with open('data/enriched/team_profiles.json', 'r', encoding='utf-8') as f:
            enriched['team'] = {p['title']: p for p in json.load(f)['projects']}
    except:
        enriched['team'] = {}

    return all_projects, enriched


def analyze_success_patterns(projects, enriched):
    """Identify characteristics of top 10% funded projects"""
    # Sort by funding
    sorted_projects = sorted(projects, key=lambda x: x.get('total_awarded', 0), reverse=True)
    top_10_percent_count = max(1, len(sorted_projects) // 10)

    top_10_percent = sorted_projects[:top_10_percent_count]
    bottom_90_percent = sorted_projects[top_10_percent_count:]

    def calculate_characteristics(project_list):
        total = len(project_list)
        if total == 0:
            return {}

        return {
            'count': total,
            'avg_funding': sum(p.get('total_awarded', 0) for p in project_list) / total,
            'soroban_rate': sum(1 for p in project_list if p.get('soroban')) / total,
            'mainnet_rate': sum(1 for p in project_list if p.get('integration_status') == 'Mainnet') / total,
            'open_source_rate': sum(1 for p in project_list if 'fully open-source' in str(p.get('open_source', '')).lower()) / total,
            'avg_rounds': sum(len(str(p.get('rounds', '')).split(',')) for p in project_list) / total,
            'multi_program_rate': sum(1 for p in project_list if ',' in str(p.get('programs', ''))) / total,
        }

    top_chars = calculate_characteristics(top_10_percent)
    bottom_chars = calculate_characteristics(bottom_90_percent)

    # Calculate differentiators
    differentiators = {}
    for key in top_chars:
        if key != 'count' and key != 'avg_funding':
            top_val = top_chars.get(key, 0)
            bottom_val = bottom_chars.get(key, 0)
            if bottom_val > 0:
                differentiators[key] = {
                    'top_10_percent': top_val,
                    'bottom_90_percent': bottom_val,
                    'advantage_ratio': top_val / bottom_val if bottom_val > 0 else float('inf')
                }

    return {
        'top_10_percent_characteristics': top_chars,
        'bottom_90_percent_characteristics': bottom_chars,
        'differentiators': differentiators,
        'top_10_percent_threshold': top_10_percent[-1].get('total_awarded', 0) if top_10_percent else 0
    }


def analyze_program_combinations(projects):
    """Analyze optimal program combinations"""
    combo_stats = defaultdict(lambda: {
        'projects': [],
        'total_funding': 0,
        'avg_funding': 0,
        'mainnet_count': 0,
        'avg_rounds': 0
    })

    for project in projects:
        programs = project.get('programs', '')
        if not programs:
            programs = 'No Program'

        # Normalize program names
        programs = programs.replace('SCF Build', 'Build').replace('Audit Bank', 'Audit')

        stats = combo_stats[programs]
        stats['projects'].append(project['title'])
        stats['total_funding'] += project.get('total_awarded', 0)
        if project.get('integration_status') == 'Mainnet':
            stats['mainnet_count'] += 1

        rounds = len(str(project.get('rounds', '')).split(','))
        stats['avg_rounds'] += rounds

    # Calculate averages
    for program, stats in combo_stats.items():
        count = len(stats['projects'])
        stats['project_count'] = count
        stats['avg_funding'] = stats['total_funding'] / count if count > 0 else 0
        stats['mainnet_rate'] = stats['mainnet_count'] / count if count > 0 else 0
        stats['avg_rounds'] = stats['avg_rounds'] / count if count > 0 else 0
        del stats['projects']  # Remove for cleaner output

    # Sort by average funding
    combinations = sorted(
        [{'programs': k, **v} for k, v in combo_stats.items()],
        key=lambda x: x['avg_funding'],
        reverse=True
    )

    return {
        'combinations': combinations,
        'total_unique_combinations': len(combinations)
    }


def analyze_open_source_correlation(projects):
    """Calculate open source correlation with funding success"""
    fully_open = []
    partly_open = []
    closed = []

    for project in projects:
        open_source = str(project.get('open_source', '')).lower()

        if 'fully open-source' in open_source:
            fully_open.append(project)
        elif 'partly' in open_source or 'public github' in open_source:
            partly_open.append(project)
        else:
            closed.append(project)

    def calc_stats(project_list):
        if not project_list:
            return {}

        return {
            'count': len(project_list),
            'avg_funding': sum(p.get('total_awarded', 0) for p in project_list) / len(project_list),
            'mainnet_rate': sum(1 for p in project_list if p.get('integration_status') == 'Mainnet') / len(project_list),
            'github_provided_rate': sum(1 for p in project_list if p.get('github')) / len(project_list)
        }

    fully_stats = calc_stats(fully_open)
    partly_stats = calc_stats(partly_open)
    closed_stats = calc_stats(closed)

    # Calculate premium
    all_avg = fully_stats.get('avg_funding', 0) * 0.4 + partly_stats.get('avg_funding', 0) * 0.4 + closed_stats.get('avg_funding', 0) * 0.2
    open_source_premium = ((fully_stats.get('avg_funding', 0) - all_avg) / all_avg * 100) if all_avg > 0 else 0

    return {
        'fully_open_source': fully_stats,
        'partly_open_source': partly_stats,
        'closed_source': closed_stats,
        'open_source_premium_percent': round(open_source_premium, 2)
    }


def analyze_multichain_impact(projects):
    """Analyze if multichain projects are more successful"""
    stellar_only = []
    multichain = []

    for project in projects:
        other_chains = project.get('other_chains', '')

        if other_chains and other_chains.strip() and other_chains.lower() not in ['not multichain', 'none', '']:
            multichain.append(project)
        else:
            stellar_only.append(project)

    def calc_stats(project_list):
        if not project_list:
            return {}

        return {
            'count': len(project_list),
            'avg_funding': sum(p.get('total_awarded', 0) for p in project_list) / len(project_list),
            'mainnet_rate': sum(1 for p in project_list if p.get('integration_status') == 'Mainnet') / len(project_list),
            'avg_rounds': sum(len(str(p.get('rounds', '')).split(',')) for p in project_list) / len(project_list)
        }

    return {
        'stellar_only': calc_stats(stellar_only),
        'multichain': calc_stats(multichain),
        'stellar_exclusivity_rate': len(stellar_only) / (len(stellar_only) + len(multichain)) if (len(stellar_only) + len(multichain)) > 0 else 0
    }


def analyze_funding_tiers(projects):
    """Analyze funding tier distributions"""
    tiers = {
        '$0-50k': [],
        '$50-100k': [],
        '$100-150k': [],
        '$150-200k': [],
        '$200k+': []
    }

    for project in projects:
        funding = project.get('total_awarded', 0)

        if funding < 50000:
            tiers['$0-50k'].append(project)
        elif funding < 100000:
            tiers['$50-100k'].append(project)
        elif funding < 150000:
            tiers['$100-150k'].append(project)
        elif funding < 200000:
            tiers['$150-200k'].append(project)
        else:
            tiers['$200k+'].append(project)

    tier_stats = {}
    for tier_name, tier_projects in tiers.items():
        if tier_projects:
            tier_stats[tier_name] = {
                'count': len(tier_projects),
                'avg_funding': sum(p.get('total_awarded', 0) for p in tier_projects) / len(tier_projects),
                'mainnet_rate': sum(1 for p in tier_projects if p.get('integration_status') == 'Mainnet') / len(tier_projects),
                'avg_rounds': sum(len(str(p.get('rounds', '')).split(',')) for p in tier_projects) / len(tier_projects),
                'soroban_rate': sum(1 for p in tier_projects if p.get('soroban')) / len(tier_projects),
            }

    return tier_stats


def analyze_category_deep_dive(projects):
    """Deep analysis per category"""
    categories = defaultdict(lambda: {
        'projects': [],
        'types': defaultdict(list)
    })

    for project in projects:
        category = project.get('category', 'Unknown')
        categories[category]['projects'].append(project)

        ptype = project.get('type', 'Unknown')
        categories[category]['types'][ptype].append(project)

    category_analysis = {}
    for cat, data in categories.items():
        projs = data['projects']

        # Type analysis
        type_stats = []
        for ptype, type_projects in data['types'].items():
            type_stats.append({
                'type': ptype,
                'count': len(type_projects),
                'total_funding': sum(p.get('total_awarded', 0) for p in type_projects),
                'avg_funding': sum(p.get('total_awarded', 0) for p in type_projects) / len(type_projects),
                'mainnet_rate': sum(1 for p in type_projects if p.get('integration_status') == 'Mainnet') / len(type_projects)
            })

        category_analysis[cat] = {
            'total_projects': len(projs),
            'total_funding': sum(p.get('total_awarded', 0) for p in projs),
            'avg_funding': sum(p.get('total_awarded', 0) for p in projs) / len(projs),
            'mainnet_rate': sum(1 for p in projs if p.get('integration_status') == 'Mainnet') / len(projs),
            'soroban_rate': sum(1 for p in projs if p.get('soroban')) / len(projs),
            'types': sorted(type_stats, key=lambda x: x['total_funding'], reverse=True)
        }

    return category_analysis


def main():
    print("Generating deep analytics...")

    projects, enriched = load_all_data()
    print(f"Loaded {len(projects)} projects")

    # 1. Success Patterns
    print("\n1. Analyzing success patterns...")
    success_patterns = analyze_success_patterns(projects, enriched)
    with open('data/analytics/success_patterns.json', 'w', encoding='utf-8') as f:
        json.dump(success_patterns, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved success_patterns.json")

    # 2. Program Combinations
    print("\n2. Analyzing program combinations...")
    program_analysis = analyze_program_combinations(projects)
    with open('data/analytics/program_combinations.json', 'w', encoding='utf-8') as f:
        json.dump(program_analysis, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved program_combinations.json ({program_analysis['total_unique_combinations']} combinations)")

    # 3. Open Source Correlation
    print("\n3. Analyzing open source correlation...")
    open_source_analysis = analyze_open_source_correlation(projects)
    with open('data/analytics/open_source_correlation.json', 'w', encoding='utf-8') as f:
        json.dump(open_source_analysis, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved open_source_correlation.json")

    # 4. Multichain Impact
    print("\n4. Analyzing multichain vs Stellar-only...")
    multichain_analysis = analyze_multichain_impact(projects)
    with open('data/analytics/multichain_analysis.json', 'w', encoding='utf-8') as f:
        json.dump(multichain_analysis, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved multichain_analysis.json")

    # 5. Funding Tiers
    print("\n5. Analyzing funding tiers...")
    tier_analysis = analyze_funding_tiers(projects)
    with open('data/analytics/funding_tiers.json', 'w', encoding='utf-8') as f:
        json.dump(tier_analysis, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved funding_tiers.json")

    # 6. Category Deep Dive
    print("\n6. Generating category deep dive...")
    category_analysis = analyze_category_deep_dive(projects)
    with open('data/analytics/category_deep_dive.json', 'w', encoding='utf-8') as f:
        json.dump(category_analysis, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved category_deep_dive.json")

    print("\n[OK] Deep analytics complete!")

    # Print key insights
    print(f"\n🔍 Key Insights:")
    if success_patterns['differentiators']:
        print(f"\nTop 10% vs Rest:")
        for key, val in list(success_patterns['differentiators'].items())[:3]:
            ratio = val['advantage_ratio']
            print(f"   * {key}: {ratio:.2f}x advantage")

    if open_source_analysis.get('open_source_premium_percent'):
        premium = open_source_analysis['open_source_premium_percent']
        print(f"\n   * Open source projects get {premium:.1f}% more funding on average")

    best_combo = program_analysis['combinations'][0] if program_analysis['combinations'] else None
    if best_combo:
        print(f"\n   * Best program combo: {best_combo['programs']} (${best_combo['avg_funding']/1000:.0f}k avg)")


if __name__ == "__main__":
    import os
    os.makedirs('data/analytics', exist_ok=True)
    main()
