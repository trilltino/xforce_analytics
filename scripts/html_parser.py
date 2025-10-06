"""
HTML Parser Script
Extracts social links, team data, and other metadata from the HTML file
"""

import json
import re
from html.parser import HTMLParser
from collections import defaultdict


class ProjectHTMLParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.projects = []
        self.current_row = []
        self.in_cell = False
        self.current_cell = ""
        self.headers = []

    def handle_starttag(self, tag, attrs):
        if tag == "td":
            self.in_cell = True
            self.current_cell = ""

    def handle_endtag(self, tag):
        if tag == "td":
            self.in_cell = False
            self.current_row.append(self.current_cell.strip())
        elif tag == "tr":
            if self.current_row:
                if not self.headers:
                    # First row is headers
                    self.headers = self.current_row
                elif len(self.current_row) == len(self.headers):
                    # Create project dict
                    project = {self.headers[i]: self.current_row[i] for i in range(len(self.headers))}
                    self.projects.append(project)
                self.current_row = []

    def handle_data(self, data):
        if self.in_cell:
            self.current_cell += data


def extract_url(text, pattern):
    """Extract URL matching a pattern"""
    if not text:
        return None
    match = re.search(pattern, text)
    return match.group(0) if match else None


def parse_social_links(projects_html):
    """Parse social links from HTML projects"""
    social_data = []

    for project in projects_html:
        title = project.get('Title', '')
        if not title or title == 'Title':  # Skip header row
            continue

        social_links = {
            'title': title,
            'discord': project.get('Discord', '').strip() or None,
            'twitter': project.get('X', '').strip() or None,
            'linkedin': project.get('LinkedIn', '').strip() or None,
            'medium': project.get('Medium', '').strip() or None,
            'video': project.get('Video', '').strip() or None,
            'pitch_deck': project.get('Pitch Deck', '').strip() or None,
            'website': project.get('Website', '').strip() or None,
        }

        # Calculate social score (how many channels they have)
        channels = [social_links['discord'], social_links['twitter'], social_links['linkedin'],
                   social_links['medium'], social_links['video'], social_links['pitch_deck']]
        social_links['social_score'] = sum(1 for c in channels if c) / 6.0 * 10  # 0-10 scale

        social_data.append(social_links)

    return social_data


def parse_team_descriptions(projects_html):
    """Parse team descriptions and analyze them"""
    team_data = []

    for project in projects_html:
        title = project.get('Title', '')
        if not title or title == 'Title':
            continue

        team_desc = project.get('Team Description', '')

        if team_desc:
            # Extract team size (look for numbers like "team of 24", "founding team of three")
            team_size_match = re.search(r'team of (\d+|\w+)', team_desc, re.IGNORECASE)
            team_size = None
            if team_size_match:
                size_str = team_size_match.group(1)
                word_to_num = {'one': 1, 'two': 2, 'three': 3, 'four': 4, 'five': 5,
                              'six': 6, 'seven': 7, 'eight': 8, 'nine': 9, 'ten': 10}
                team_size = int(size_str) if size_str.isdigit() else word_to_num.get(size_str.lower())

            # Extract roles (CEO, CTO, etc.)
            roles = re.findall(r'\b(CEO|CTO|CMO|CFO|COO|CBO|CPO|CSO)\b', team_desc)

            # Count expertise keywords
            expertise_keywords = ['blockchain', 'fintech', 'web3', 'defi', 'crypto', 'stellar',
                                'engineering', 'development', 'security', 'audit', 'design',
                                'marketing', 'business', 'strategy', 'operations']

            expertise_found = [kw for kw in expertise_keywords if kw.lower() in team_desc.lower()]

            # Count LinkedIn profiles mentioned
            linkedin_count = team_desc.count('linkedin.com')

            team_data.append({
                'title': title,
                'team_description': team_desc[:500] if team_desc else None,  # Truncate for size
                'team_size': team_size,
                'roles_mentioned': list(set(roles)),
                'expertise_areas': expertise_found[:10],  # Top 10
                'linkedin_profiles': linkedin_count,
                'has_team_desc': bool(team_desc),
                'team_strength_score': min(10, (
                    (2 if team_size else 0) +
                    (len(set(roles)) * 1.5) +
                    (len(expertise_found) * 0.3) +
                    (linkedin_count * 0.5)
                ))
            })
        else:
            team_data.append({
                'title': title,
                'team_description': None,
                'team_size': None,
                'roles_mentioned': [],
                'expertise_areas': [],
                'linkedin_profiles': 0,
                'has_team_desc': False,
                'team_strength_score': 0
            })

    return team_data


def parse_traction_data(projects_html):
    """Parse and structure traction metrics"""
    traction_data = []

    for project in projects_html:
        title = project.get('Title', '')
        if not title or title == 'Title':
            continue

        traction_text = project.get('Current Traction', '')

        metrics = {
            'title': title,
            'traction_text': traction_text[:1000] if traction_text else None,
            'has_traction': bool(traction_text),
        }

        if traction_text:
            # Extract numeric metrics
            # Downloads
            downloads_match = re.search(r'([\d,]+)\+?\s*(app\s+)?downloads', traction_text, re.IGNORECASE)
            if downloads_match:
                metrics['downloads'] = int(downloads_match.group(1).replace(',', ''))

            # Users
            users_match = re.search(r'([\d,]+)\+?\s*users', traction_text, re.IGNORECASE)
            if users_match:
                metrics['users'] = int(users_match.group(1).replace(',', ''))

            # Community size
            community_match = re.search(r'([\d,]+)\+?\s*community\s+members', traction_text, re.IGNORECASE)
            if community_match:
                metrics['community_size'] = int(community_match.group(1).replace(',', ''))

            # Revenue
            revenue_match = re.search(r'[\$£€]([\d,]+)\s*(in\s+)?revenue', traction_text, re.IGNORECASE)
            if revenue_match:
                metrics['revenue'] = int(revenue_match.group(1).replace(',', ''))

            # Partnerships
            partnerships = re.findall(r'partner(?:ship)?.*?with\s+([A-Z][A-Za-z\s&]+)', traction_text)
            if partnerships:
                metrics['partnerships'] = partnerships[:10]  # Top 10

            # Calculate traction score
            score = 0
            if metrics.get('downloads', 0) > 0:
                score += min(3, metrics['downloads'] / 50000)
            if metrics.get('users', 0) > 0:
                score += min(3, metrics['users'] / 10000)
            if metrics.get('revenue', 0) > 0:
                score += min(2, metrics['revenue'] / 100000)
            if metrics.get('partnerships'):
                score += min(2, len(metrics['partnerships']) * 0.5)

            metrics['traction_score'] = round(score, 2)

        traction_data.append(metrics)

    return traction_data


def main():
    print("Parsing HTML file...")

    # Read and parse HTML
    with open('data/raw/Shared-to-Ecosystem-Projects.html', 'r', encoding='utf-8') as f:
        html_content = f.read()

    parser = ProjectHTMLParser()
    parser.feed(html_content)
    print(f"   [OK] Parsed {len(parser.projects)} projects from HTML")

    # 1. Social Links
    print("\n1. Extracting social links...")
    social_data = parse_social_links(parser.projects)

    with open('data/enriched/social_links.json', 'w', encoding='utf-8') as f:
        json.dump({
            'projects': social_data,
            'statistics': {
                'total': len(social_data),
                'with_twitter': sum(1 for p in social_data if p['twitter']),
                'with_discord': sum(1 for p in social_data if p['discord']),
                'with_linkedin': sum(1 for p in social_data if p['linkedin']),
                'with_video': sum(1 for p in social_data if p['video']),
                'avg_social_score': sum(p['social_score'] for p in social_data) / len(social_data) if social_data else 0
            }
        }, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved social_links.json ({len(social_data)} projects)")

    # 2. Team Descriptions
    print("\n2. Parsing team descriptions...")
    team_data = parse_team_descriptions(parser.projects)

    with open('data/enriched/team_profiles.json', 'w', encoding='utf-8') as f:
        json.dump({
            'projects': team_data,
            'statistics': {
                'total': len(team_data),
                'with_team_desc': sum(1 for p in team_data if p['has_team_desc']),
                'avg_team_size': sum(p['team_size'] for p in team_data if p['team_size']) / sum(1 for p in team_data if p['team_size']) if any(p['team_size'] for p in team_data) else 0,
                'avg_strength_score': sum(p['team_strength_score'] for p in team_data) / len(team_data) if team_data else 0
            }
        }, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved team_profiles.json ({sum(1 for p in team_data if p['has_team_desc'])} with descriptions)")

    # 3. Traction Metrics
    print("\n3. Extracting traction metrics...")
    traction_data = parse_traction_data(parser.projects)

    with open('data/enriched/traction_metrics.json', 'w', encoding='utf-8') as f:
        json.dump({
            'projects': traction_data,
            'statistics': {
                'total': len(traction_data),
                'with_traction': sum(1 for p in traction_data if p['has_traction']),
                'with_downloads': sum(1 for p in traction_data if p.get('downloads')),
                'with_users': sum(1 for p in traction_data if p.get('users')),
                'with_revenue': sum(1 for p in traction_data if p.get('revenue')),
            }
        }, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved traction_metrics.json ({sum(1 for p in traction_data if p['has_traction'])} with traction)")

    # 4. UNDP Projects
    print("\n4. Identifying UNDP projects...")
    undp_projects = [p for p in parser.projects if p.get('UNDP', '').strip()]

    with open('data/enriched/undp_projects.json', 'w', encoding='utf-8') as f:
        json.dump({
            'projects': [{'title': p['Title'], 'category': p.get('Category', ''), 'type': p.get('Type', '')}
                        for p in undp_projects],
            'total': len(undp_projects)
        }, f, indent=2, ensure_ascii=False)
    print(f"   [OK] Saved undp_projects.json ({len(undp_projects)} UNDP projects)")

    print("\n[OK] HTML parsing complete!")


if __name__ == "__main__":
    import os
    os.makedirs('data/enriched', exist_ok=True)
    main()
