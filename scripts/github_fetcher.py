"""
GitHub API Fetcher
Fetches stars, forks, and activity metrics for all project GitHub repos
"""

import json
import re
import time
import requests
from collections import defaultdict


def extract_github_info(github_url):
    """Extract owner and repo from GitHub URL"""
    if not github_url:
        return None, None

    # Handle various GitHub URL formats
    patterns = [
        r'github\.com/([^/]+)/([^/\s]+)',  # Standard format
        r'github\.com/([^/]+)$',  # Just org/user
    ]

    for pattern in patterns:
        match = re.search(pattern, github_url)
        if match:
            if len(match.groups()) == 2:
                return match.group(1), match.group(2)
            else:
                return match.group(1), None

    return None, None


def fetch_github_stats(owner, repo, token=None):
    """Fetch GitHub repository statistics"""
    if not owner or not repo:
        return None

    headers = {}
    if token:
        headers['Authorization'] = f'token {token}'

    try:
        # Fetch repo info
        url = f'https://api.github.com/repos/{owner}/{repo}'
        response = requests.get(url, headers=headers, timeout=10)

        if response.status_code == 200:
            data = response.json()

            return {
                'owner': owner,
                'repo': repo,
                'stars': data.get('stargazers_count', 0),
                'forks': data.get('forks_count', 0),
                'watchers': data.get('watchers_count', 0),
                'open_issues': data.get('open_issues_count', 0),
                'language': data.get('language'),
                'created_at': data.get('created_at'),
                'updated_at': data.get('updated_at'),
                'pushed_at': data.get('pushed_at'),
                'size': data.get('size', 0),
                'has_wiki': data.get('has_wiki', False),
                'has_issues': data.get('has_issues', False),
                'license': data.get('license', {}).get('name') if data.get('license') else None,
                'description': data.get('description'),
                'topics': data.get('topics', []),
            }
        elif response.status_code == 404:
            return {'error': 'Repository not found', 'owner': owner, 'repo': repo}
        elif response.status_code == 403:
            print(f"   Rate limit reached. Consider using a GitHub token.")
            return {'error': 'Rate limit', 'owner': owner, 'repo': repo}
        else:
            return {'error': f'HTTP {response.status_code}', 'owner': owner, 'repo': repo}

    except Exception as e:
        return {'error': str(e), 'owner': owner, 'repo': repo}


def calculate_activity_score(stats):
    """Calculate GitHub activity score (0-10)"""
    if not stats or 'error' in stats:
        return 0

    score = 0

    # Stars (0-3 points)
    stars = stats.get('stars', 0)
    if stars > 1000:
        score += 3
    elif stars > 100:
        score += 2
    elif stars > 10:
        score += 1

    # Forks (0-2 points)
    forks = stats.get('forks', 0)
    if forks > 100:
        score += 2
    elif forks > 10:
        score += 1

    # Recency (0-3 points)
    pushed_at = stats.get('pushed_at')
    if pushed_at:
        try:
            from datetime import datetime
            last_push = datetime.fromisoformat(pushed_at.replace('Z', '+00:00'))
            days_since_push = (datetime.now(last_push.tzinfo) - last_push).days

            if days_since_push < 30:
                score += 3
            elif days_since_push < 90:
                score += 2
            elif days_since_push < 180:
                score += 1
        except:
            pass

    # Has documentation (0-1 point)
    if stats.get('has_wiki'):
        score += 0.5

    # Active issues (0-1 point)
    if stats.get('has_issues') and stats.get('open_issues', 0) > 0:
        score += 0.5

    return round(min(10, score), 2)


def main():
    # Load project data
    with open('data/raw/all_projects_structured.json', 'r', encoding='utf-8') as f:
        data = json.load(f)

    # Flatten all projects
    all_projects = []
    for category, projects in data['by_category'].items():
        all_projects.extend(projects)

    print(f"Fetching GitHub data for {len(all_projects)} projects...")
    print("Note: This may take a while. Consider setting GITHUB_TOKEN env var to avoid rate limits.")

    # Get GitHub token from environment if available
    import os
    github_token = os.environ.get('GITHUB_TOKEN')

    github_data = []
    successful = 0
    no_github = 0
    errors = 0

    for i, project in enumerate(all_projects):
        github_url = project.get('github', '')
        title = project.get('title', 'Unknown')

        if not github_url:
            no_github += 1
            github_data.append({
                'title': title,
                'category': project.get('category'),
                'github_url': None,
                'stats': None,
                'activity_score': 0
            })
            continue

        owner, repo = extract_github_info(github_url)

        if not owner or not repo:
            errors += 1
            github_data.append({
                'title': title,
                'category': project.get('category'),
                'github_url': github_url,
                'stats': {'error': 'Could not parse URL'},
                'activity_score': 0
            })
            continue

        stats = fetch_github_stats(owner, repo, github_token)

        if stats and 'error' not in stats:
            successful += 1
            activity_score = calculate_activity_score(stats)
            print(f"   [{i+1}/{len(all_projects)}] {title}: ⭐{stats['stars']} 🍴{stats['forks']} Score:{activity_score}")
        else:
            errors += 1
            activity_score = 0
            error_msg = stats.get('error', 'Unknown error') if stats else 'Unknown error'
            print(f"   [{i+1}/{len(all_projects)}] {title}: [FAIL] {error_msg}")

        github_data.append({
            'title': title,
            'category': project.get('category'),
            'github_url': github_url,
            'owner': owner,
            'repo': repo,
            'stats': stats,
            'activity_score': activity_score
        })

        # Rate limiting - sleep between requests
        if (i + 1) % 10 == 0:
            time.sleep(1)  # Sleep 1 second every 10 requests

    # Calculate category averages
    category_stats = defaultdict(lambda: {'scores': [], 'stars': [], 'forks': []})

    for entry in github_data:
        if entry['stats'] and 'error' not in entry['stats']:
            cat = entry.get('category', 'Unknown')
            category_stats[cat]['scores'].append(entry['activity_score'])
            category_stats[cat]['stars'].append(entry['stats'].get('stars', 0))
            category_stats[cat]['forks'].append(entry['stats'].get('forks', 0))

    category_summary = {}
    for cat, data in category_stats.items():
        if data['scores']:
            category_summary[cat] = {
                'avg_activity_score': sum(data['scores']) / len(data['scores']),
                'avg_stars': sum(data['stars']) / len(data['stars']),
                'avg_forks': sum(data['forks']) / len(data['forks']),
                'project_count': len(data['scores'])
            }

    # Save results
    output = {
        'projects': github_data,
        'statistics': {
            'total_projects': len(all_projects),
            'with_github': len(all_projects) - no_github,
            'successful_fetches': successful,
            'errors': errors,
            'no_github_url': no_github
        },
        'category_summary': category_summary
    }

    with open('data/enriched/github_metrics.json', 'w', encoding='utf-8') as f:
        json.dump(output, f, indent=2, ensure_ascii=False)

    print(f"\n[OK] GitHub data fetching complete!")
    print(f"   [OK] Successful: {successful}")
    print(f"   [OK] No GitHub URL: {no_github}")
    print(f"   [OK] Errors: {errors}")

    # Top projects by stars
    top_starred = sorted(
        [p for p in github_data if p['stats'] and 'error' not in p['stats']],
        key=lambda x: x['stats'].get('stars', 0),
        reverse=True
    )[:10]

    if top_starred:
        print(f"\n🌟 Top 10 Most Starred:")
        for i, p in enumerate(top_starred, 1):
            print(f"   {i}. {p['title']}: {p['stats']['stars']} stars")


if __name__ == "__main__":
    import os
    os.makedirs('data/enriched', exist_ok=True)
    main()
