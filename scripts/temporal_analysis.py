"""
Temporal Analysis Script
Parses quarters and rounds data to generate temporal insights
"""

import json
from datetime import datetime
from collections import defaultdict
import re


def parse_quarter(quarter_str):
    """Parse quarter string like 'Q2 '23' into date"""
    if not quarter_str or quarter_str == "":
        return None

    match = re.match(r"Q(\d)\s*'(\d{2})", quarter_str.strip())
    if match:
        q, year = match.groups()
        year = int("20" + year)
        month = (int(q) - 1) * 3 + 1
        return datetime(year, month, 1)
    return None


def calculate_time_to_mainnet(project):
    """Calculate time from first funding to mainnet"""
    if not project.get('quarters') or project.get('integration_status') != 'Mainnet':
        return None

    quarters = project['quarters'].split(',')
    dates = [parse_quarter(q) for q in quarters if parse_quarter(q)]

    if not dates:
        return None

    first_date = min(dates)
    last_date = max(dates)

    return (last_date - first_date).days


def analyze_funding_velocity(projects):
    """Analyze how fast projects progress through rounds"""
    velocity_data = []

    for project in projects:
        if not project.get('rounds') or not project.get('quarters'):
            continue

        rounds_list = [r.strip() for r in project['rounds'].split(',')]
        quarters_list = [q.strip() for q in project['quarters'].split(',')]

        if len(rounds_list) < 2 or len(quarters_list) < 2:
            continue

        dates = [parse_quarter(q) for q in quarters_list]
        dates = [d for d in dates if d is not None]

        if len(dates) >= 2:
            dates.sort()
            intervals = []
            for i in range(len(dates) - 1):
                days = (dates[i+1] - dates[i]).days
                intervals.append(days)

            velocity_data.append({
                'title': project['title'],
                'category': project.get('category'),
                'round_count': len(rounds_list),
                'avg_interval_days': sum(intervals) / len(intervals) if intervals else 0,
                'intervals': intervals,
                'total_funding': project.get('total_awarded', 0)
            })

    return velocity_data


def analyze_quarterly_cohorts(projects):
    """Analyze cohorts by first funding quarter"""
    cohorts = defaultdict(lambda: {
        'projects': [],
        'total_funding': 0,
        'mainnet_count': 0,
        'avg_funding': 0,
        'soroban_count': 0
    })

    for project in projects:
        if not project.get('quarters'):
            continue

        quarters_list = [q.strip() for q in project['quarters'].split(',')]
        first_quarter = quarters_list[0] if quarters_list else None

        if first_quarter:
            cohort = cohorts[first_quarter]
            cohort['projects'].append(project['title'])
            cohort['total_funding'] += project.get('total_awarded', 0)

            if project.get('integration_status') == 'Mainnet':
                cohort['mainnet_count'] += 1

            if project.get('soroban'):
                cohort['soroban_count'] += 1

    # Calculate averages
    for quarter, data in cohorts.items():
        if data['projects']:
            data['avg_funding'] = data['total_funding'] / len(data['projects'])
            data['mainnet_rate'] = data['mainnet_count'] / len(data['projects'])
            data['soroban_rate'] = data['soroban_count'] / len(data['projects'])
            data['project_count'] = len(data['projects'])
            # Remove project list for cleaner output
            del data['projects']

    return dict(cohorts)


def analyze_round_progression(projects):
    """Analyze round progression patterns"""
    progression_patterns = defaultdict(int)
    round_retention = defaultdict(lambda: {'total': 0, 'progressed': 0})

    for project in projects:
        if not project.get('rounds'):
            continue

        rounds_list = [r.strip() for r in project['rounds'].split(',')]
        round_count = len(rounds_list)

        progression_patterns[round_count] += 1

        # Track retention
        for i in range(round_count):
            round_retention[i + 1]['total'] += 1
            if i + 1 < round_count:
                round_retention[i + 1]['progressed'] += 1

    # Calculate retention rates
    retention_rates = {}
    for round_num, data in round_retention.items():
        if data['total'] > 0:
            retention_rates[f'round_{round_num}'] = {
                'total_projects': data['total'],
                'progressed_to_next': data['progressed'],
                'retention_rate': data['progressed'] / data['total']
            }

    return {
        'progression_patterns': dict(progression_patterns),
        'retention_rates': retention_rates
    }


def main():
    # Load project data
    with open('data/raw/all_projects_structured.json', 'r', encoding='utf-8') as f:
        data = json.load(f)

    # Flatten all projects
    all_projects = []
    for category, projects in data['by_category'].items():
        all_projects.extend(projects)

    print(f"Analyzing {len(all_projects)} projects...")

    # 1. Funding Velocity
    print("\n1. Calculating funding velocity...")
    velocity_data = analyze_funding_velocity(all_projects)

    # Calculate category averages
    category_velocity = defaultdict(list)
    for v in velocity_data:
        if v['category']:
            category_velocity[v['category']].append(v['avg_interval_days'])

    velocity_summary = {
        'by_project': velocity_data,
        'by_category': {
            cat: {
                'avg_days_between_rounds': sum(intervals) / len(intervals) if intervals else 0,
                'project_count': len(intervals)
            }
            for cat, intervals in category_velocity.items()
        }
    }

    with open('data/analytics/temporal/funding_velocity.json', 'w', encoding='utf-8') as f:
        json.dump(velocity_summary, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved funding_velocity.json ({len(velocity_data)} projects analyzed)")

    # 2. Time to Mainnet
    print("\n2. Calculating time to mainnet...")
    time_to_mainnet_data = []
    for project in all_projects:
        days = calculate_time_to_mainnet(project)
        if days is not None:
            time_to_mainnet_data.append({
                'title': project['title'],
                'category': project.get('category'),
                'days_to_mainnet': days,
                'total_funding': project.get('total_awarded', 0),
                'soroban': project.get('soroban', False)
            })

    # Category averages
    category_ttm = defaultdict(list)
    soroban_ttm = []
    non_soroban_ttm = []

    for t in time_to_mainnet_data:
        if t['category']:
            category_ttm[t['category']].append(t['days_to_mainnet'])

        if t['soroban']:
            soroban_ttm.append(t['days_to_mainnet'])
        else:
            non_soroban_ttm.append(t['days_to_mainnet'])

    ttm_summary = {
        'by_project': time_to_mainnet_data,
        'by_category': {
            cat: {
                'avg_days': sum(days) / len(days) if days else 0,
                'min_days': min(days) if days else 0,
                'max_days': max(days) if days else 0,
                'project_count': len(days)
            }
            for cat, days in category_ttm.items()
        },
        'soroban_advantage': {
            'soroban_avg_days': sum(soroban_ttm) / len(soroban_ttm) if soroban_ttm else 0,
            'non_soroban_avg_days': sum(non_soroban_ttm) / len(non_soroban_ttm) if non_soroban_ttm else 0,
            'speedup_factor': (sum(non_soroban_ttm) / len(non_soroban_ttm)) / (sum(soroban_ttm) / len(soroban_ttm)) if soroban_ttm and non_soroban_ttm else 0
        }
    }

    with open('data/analytics/temporal/time_to_mainnet.json', 'w', encoding='utf-8') as f:
        json.dump(ttm_summary, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved time_to_mainnet.json ({len(time_to_mainnet_data)} projects)")

    # 3. Quarterly Cohorts
    print("\n3. Analyzing quarterly cohorts...")
    cohorts = analyze_quarterly_cohorts(all_projects)

    with open('data/analytics/temporal/quarterly_cohorts.json', 'w', encoding='utf-8') as f:
        json.dump(cohorts, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved quarterly_cohorts.json ({len(cohorts)} cohorts)")

    # 4. Round Progression
    print("\n4. Analyzing round progression...")
    progression = analyze_round_progression(all_projects)

    with open('data/analytics/temporal/round_progression.json', 'w', encoding='utf-8') as f:
        json.dump(progression, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved round_progression.json")

    # 5. Seasonal Patterns
    print("\n5. Analyzing seasonal patterns...")
    seasonal = defaultdict(lambda: {'projects': 0, 'total_funding': 0, 'mainnet_count': 0})

    for project in all_projects:
        if not project.get('quarters'):
            continue

        quarters_list = [q.strip() for q in project['quarters'].split(',')]
        first_quarter = quarters_list[0] if quarters_list else None

        if first_quarter:
            # Extract just the quarter (Q1, Q2, Q3, Q4)
            match = re.match(r"(Q\d)", first_quarter)
            if match:
                q = match.group(1)
                seasonal[q]['projects'] += 1
                seasonal[q]['total_funding'] += project.get('total_awarded', 0)
                if project.get('integration_status') == 'Mainnet':
                    seasonal[q]['mainnet_count'] += 1

    # Calculate rates
    for quarter, data in seasonal.items():
        if data['projects'] > 0:
            data['avg_funding'] = data['total_funding'] / data['projects']
            data['mainnet_rate'] = data['mainnet_count'] / data['projects']

    with open('data/analytics/temporal/seasonal_patterns.json', 'w', encoding='utf-8') as f:
        json.dump(dict(seasonal), f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved seasonal_patterns.json")

    print("\n[OK] Temporal analysis complete!")
    print(f"\nKey Insights:")
    if soroban_ttm and non_soroban_ttm:
        speedup = ttm_summary['soroban_advantage']['speedup_factor']
        print(f"   * Soroban projects reach mainnet {speedup:.1f}x faster")

    if progression['retention_rates']:
        r1_to_r2 = progression['retention_rates'].get('round_1', {}).get('retention_rate', 0)
        print(f"   * {r1_to_r2*100:.1f}% of Round 1 projects get Round 2 funding")


if __name__ == "__main__":
    import os
    os.makedirs('data/analytics/temporal', exist_ok=True)
    main()
