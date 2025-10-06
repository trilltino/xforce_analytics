"""
Social Media Scraper
Scrapes Twitter/X, Discord invite stats, and other social metrics
"""

import json
import requests
import re
from urllib.parse import urlparse
import time


def get_twitter_info(twitter_url):
    """Extract Twitter/X handle and attempt to get basic info"""
    if not twitter_url:
        return None

    # Extract handle
    match = re.search(r'(?:twitter\.com|x\.com)/([^/\s?]+)', twitter_url)
    if not match:
        return None

    handle = match.group(1).lstrip('@')

    return {
        'handle': handle,
        'url': twitter_url,
        'platform': 'x.com' if 'x.com' in twitter_url else 'twitter.com'
    }


def get_discord_invite_info(discord_url):
    """Get Discord invite information"""
    if not discord_url:
        return None

    # Extract invite code
    match = re.search(r'discord\.(?:gg|com/invite)/([^\s/]+)', discord_url)
    if not match:
        return None

    invite_code = match.group(1)

    try:
        # Discord API endpoint for invite info (no auth needed for public invites)
        api_url = f'https://discord.com/api/v10/invites/{invite_code}?with_counts=true'

        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        }

        response = requests.get(api_url, headers=headers, timeout=10)

        if response.status_code == 200:
            data = response.json()

            return {
                'invite_code': invite_code,
                'server_name': data.get('guild', {}).get('name'),
                'member_count': data.get('approximate_member_count', 0),
                'online_count': data.get('approximate_presence_count', 0),
                'server_id': data.get('guild', {}).get('id'),
                'url': discord_url
            }
        else:
            return {
                'invite_code': invite_code,
                'url': discord_url,
                'error': f'HTTP {response.status_code}'
            }

    except Exception as e:
        return {
            'invite_code': invite_code,
            'url': discord_url,
            'error': str(e)
        }


def get_github_social_info(github_url):
    """Extract GitHub org/user from URL"""
    if not github_url:
        return None

    match = re.search(r'github\.com/([^/\s]+)', github_url)
    if not match:
        return None

    return {
        'owner': match.group(1),
        'url': github_url
    }


def get_youtube_info(youtube_url):
    """Extract YouTube channel info"""
    if not youtube_url:
        return None

    patterns = [
        r'youtube\.com/channel/([^/\s]+)',
        r'youtube\.com/c/([^/\s]+)',
        r'youtube\.com/@([^/\s]+)',
        r'youtube\.com/user/([^/\s]+)'
    ]

    for pattern in patterns:
        match = re.search(pattern, youtube_url)
        if match:
            return {
                'identifier': match.group(1),
                'url': youtube_url
            }

    return None


def get_linkedin_info(linkedin_url):
    """Extract LinkedIn company info"""
    if not linkedin_url:
        return None

    match = re.search(r'linkedin\.com/company/([^/\s]+)', linkedin_url)
    if not match:
        return None

    return {
        'company_slug': match.group(1),
        'url': linkedin_url
    }


def scrape_social_accounts(projects):
    """Scrape social account information"""
    social_data = []

    for i, project in enumerate(projects):
        title = project.get('title', 'Unknown')

        project_social = {
            'title': title,
            'twitter': None,
            'discord': None,
            'github': None,
            'youtube': None,
            'linkedin': None,
            'medium': None
        }

        # Twitter/X
        if project.get('twitter'):
            project_social['twitter'] = get_twitter_info(project['twitter'])

        # Discord
        if project.get('discord'):
            discord_info = get_discord_invite_info(project['discord'])
            if discord_info:
                project_social['discord'] = discord_info
                if discord_info.get('member_count'):
                    print(f"   [{i+1}/{len(projects)}] {title}: Discord {discord_info['member_count']} members")
                time.sleep(0.5)  # Rate limiting for Discord API

        # GitHub
        if project.get('website') and 'github.com' in project.get('website', ''):
            project_social['github'] = get_github_social_info(project['website'])

        # YouTube
        if project.get('video') and 'youtube.com' in project.get('video', ''):
            project_social['youtube'] = get_youtube_info(project['video'])

        # LinkedIn
        if project.get('linkedin'):
            project_social['linkedin'] = get_linkedin_info(project['linkedin'])

        # Medium
        if project.get('medium'):
            project_social['medium'] = {
                'url': project['medium'],
                'handle': re.search(r'medium\.com/([^/\s]+)', project['medium']).group(1) if re.search(r'medium\.com/([^/\s]+)', project['medium']) else None
            }

        social_data.append(project_social)

    return social_data


def main():
    print("Loading social links data...")

    with open('data/enriched/social_links.json', 'r', encoding='utf-8') as f:
        data = json.load(f)

    projects = data['projects']

    print(f"\nScraping social accounts for {len(projects)} projects...")
    print("Note: Only Discord provides public stats without authentication\n")

    social_data = scrape_social_accounts(projects)

    # Calculate statistics
    stats = {
        'total_projects': len(social_data),
        'with_twitter': sum(1 for s in social_data if s['twitter']),
        'with_discord': sum(1 for s in social_data if s['discord']),
        'with_github': sum(1 for s in social_data if s['github']),
        'with_youtube': sum(1 for s in social_data if s['youtube']),
        'with_linkedin': sum(1 for s in social_data if s['linkedin']),
        'with_medium': sum(1 for s in social_data if s['medium']),
    }

    # Discord stats
    discord_servers = [s['discord'] for s in social_data if s['discord'] and s['discord'].get('member_count')]
    if discord_servers:
        total_members = sum(s['member_count'] for s in discord_servers)
        avg_members = total_members / len(discord_servers)
        stats['discord_stats'] = {
            'servers_with_data': len(discord_servers),
            'total_members': total_members,
            'avg_members_per_server': round(avg_members, 2),
            'largest_server': max(discord_servers, key=lambda x: x['member_count'])['server_name'],
            'largest_member_count': max(s['member_count'] for s in discord_servers)
        }

    # Save results
    output = {
        'social_accounts': social_data,
        'statistics': stats
    }

    with open('data/enriched/social_accounts_detailed.json', 'w', encoding='utf-8') as f:
        json.dump(output, f, indent=2, ensure_ascii=False)

    print(f"\n[OK] Social scraping complete!")
    print(f"\nStatistics:")
    print(f"   * Twitter/X: {stats['with_twitter']} projects")
    print(f"   * Discord: {stats['with_discord']} projects")
    print(f"   * GitHub: {stats['with_github']} projects")
    print(f"   * LinkedIn: {stats['with_linkedin']} projects")
    print(f"   * YouTube: {stats['with_youtube']} projects")

    if stats.get('discord_stats'):
        ds = stats['discord_stats']
        print(f"\nDiscord Community Stats:")
        print(f"   * Servers analyzed: {ds['servers_with_data']}")
        print(f"   * Total members: {ds['total_members']:,}")
        print(f"   * Avg per server: {ds['avg_members_per_server']:,.0f}")
        print(f"   * Largest: {ds['largest_server']} ({ds['largest_member_count']:,} members)")

    print(f"\n[OK] Saved to data/enriched/social_accounts_detailed.json")


if __name__ == "__main__":
    import os
    os.makedirs('data/enriched', exist_ok=True)
    main()
