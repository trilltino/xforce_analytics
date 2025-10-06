"""
Website Scraper
Scrapes project websites for metadata, tech stack, and additional info
"""

import json
import requests
from bs4 import BeautifulSoup
import re
import time
from urllib.parse import urlparse
from collections import defaultdict
import concurrent.futures


def scrape_website(url, title):
    """Scrape a single website for metadata"""
    if not url or not url.startswith('http'):
        return None

    try:
        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
        }

        response = requests.get(url, headers=headers, timeout=10, allow_redirects=True)

        if response.status_code != 200:
            return {
                'title': title,
                'url': url,
                'status': response.status_code,
                'error': f'HTTP {response.status_code}'
            }

        soup = BeautifulSoup(response.text, 'html.parser')

        # Extract metadata
        metadata = {
            'title': title,
            'url': url,
            'status': 200,
            'final_url': response.url,  # After redirects
        }

        # Page title
        page_title = soup.find('title')
        metadata['page_title'] = page_title.get_text().strip() if page_title else None

        # Meta description
        meta_desc = soup.find('meta', attrs={'name': 'description'}) or soup.find('meta', attrs={'property': 'og:description'})
        metadata['meta_description'] = meta_desc.get('content', '').strip() if meta_desc else None

        # Meta keywords
        meta_keywords = soup.find('meta', attrs={'name': 'keywords'})
        metadata['meta_keywords'] = meta_keywords.get('content', '').strip() if meta_keywords else None

        # Open Graph data
        og_image = soup.find('meta', attrs={'property': 'og:image'})
        metadata['og_image'] = og_image.get('content', '') if og_image else None

        og_type = soup.find('meta', attrs={'property': 'og:type'})
        metadata['og_type'] = og_type.get('content', '') if og_type else None

        # Twitter card
        twitter_card = soup.find('meta', attrs={'name': 'twitter:card'})
        metadata['twitter_card'] = twitter_card.get('content', '') if twitter_card else None

        twitter_site = soup.find('meta', attrs={'name': 'twitter:site'})
        metadata['twitter_handle'] = twitter_site.get('content', '').lstrip('@') if twitter_site else None

        # Extract all links
        links = soup.find_all('a', href=True)

        # Find social media links
        social_patterns = {
            'twitter': r'(twitter\.com|x\.com)/([^/\s]+)',
            'discord': r'discord\.(gg|com)/([^\s]+)',
            'telegram': r't\.me/([^\s]+)',
            'github': r'github\.com/([^/\s]+)(?:/([^/\s]+))?',
            'medium': r'medium\.com/([^\s]+)',
            'linkedin': r'linkedin\.com/company/([^\s]+)',
            'youtube': r'youtube\.com/(channel/|c/|@)?([^\s]+)',
        }

        found_socials = {}
        for link in links:
            href = link.get('href', '')
            for platform, pattern in social_patterns.items():
                if platform not in found_socials:
                    match = re.search(pattern, href, re.IGNORECASE)
                    if match:
                        found_socials[platform] = href

        metadata['discovered_socials'] = found_socials

        # Extract technology indicators from page
        page_text = response.text.lower()

        tech_keywords = {
            'stellar': ['stellar', 'xlm', 'soroban'],
            'blockchain': ['blockchain', 'web3', 'crypto', 'defi'],
            'languages': ['rust', 'javascript', 'typescript', 'python', 'go', 'solidity'],
            'frameworks': ['react', 'vue', 'angular', 'next.js', 'django', 'flask', 'express'],
        }

        detected_tech = defaultdict(list)
        for category, keywords in tech_keywords.items():
            for keyword in keywords:
                if keyword in page_text:
                    detected_tech[category].append(keyword)

        metadata['detected_technologies'] = dict(detected_tech)

        # Count external links vs internal
        domain = urlparse(url).netloc
        external_links = [link.get('href') for link in links if link.get('href') and urlparse(link.get('href')).netloc and urlparse(link.get('href')).netloc != domain]
        metadata['external_link_count'] = len(external_links)
        metadata['total_link_count'] = len(links)

        # Check for common sections
        sections = {
            'has_docs': bool(soup.find(string=re.compile(r'documentation|docs', re.IGNORECASE))),
            'has_blog': bool(soup.find(string=re.compile(r'blog|news|articles', re.IGNORECASE))),
            'has_team': bool(soup.find(string=re.compile(r'team|about us|our team', re.IGNORECASE))),
            'has_contact': bool(soup.find(string=re.compile(r'contact|get in touch|email us', re.IGNORECASE))),
            'has_pricing': bool(soup.find(string=re.compile(r'pricing|plans|subscribe', re.IGNORECASE))),
        }
        metadata['sections'] = sections

        # Check for app stores
        app_stores = {
            'google_play': bool(soup.find('a', href=re.compile(r'play\.google\.com'))),
            'app_store': bool(soup.find('a', href=re.compile(r'apps\.apple\.com'))),
        }
        metadata['app_stores'] = app_stores

        return metadata

    except requests.Timeout:
        return {'title': title, 'url': url, 'error': 'Timeout'}
    except requests.ConnectionError:
        return {'title': title, 'url': url, 'error': 'Connection Error'}
    except Exception as e:
        return {'title': title, 'url': url, 'error': str(e)}


def scrape_websites_batch(projects, max_workers=5, delay=1):
    """Scrape multiple websites with rate limiting"""
    results = []

    with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures = []

        for project in projects:
            website = project.get('website')
            title = project.get('title', 'Unknown')

            if website:
                future = executor.submit(scrape_website, website, title)
                futures.append(future)
                time.sleep(delay)  # Rate limiting

        for i, future in enumerate(concurrent.futures.as_completed(futures)):
            result = future.result()
            if result:
                results.append(result)
                if result.get('status') == 200:
                    print(f"   [{i+1}/{len(futures)}] {result['title']}: OK")
                else:
                    error = result.get('error', 'Unknown')
                    print(f"   [{i+1}/{len(futures)}] {result['title']}: {error}")

    return results


def main():
    print("Loading project data...")

    # Load enriched data with social links
    with open('data/enriched/social_links.json', 'r', encoding='utf-8') as f:
        social_data = json.load(f)

    projects = social_data['projects']

    # Filter projects with websites
    projects_with_websites = [p for p in projects if p.get('website')]

    print(f"\nFound {len(projects_with_websites)} projects with websites")
    print(f"Starting web scraping (rate limited to 5 concurrent requests)...\n")

    # Scrape in batches
    scraped_data = scrape_websites_batch(projects_with_websites, max_workers=5, delay=0.5)

    # Calculate statistics
    successful = sum(1 for s in scraped_data if s.get('status') == 200)
    errors = len(scraped_data) - successful

    # Find projects with discovered social accounts
    new_socials_found = sum(1 for s in scraped_data if s.get('discovered_socials'))

    # Aggregate technology findings
    all_tech = defaultdict(int)
    for s in scraped_data:
        if s.get('detected_technologies'):
            for category, techs in s['detected_technologies'].items():
                for tech in techs:
                    all_tech[tech] += 1

    # Save results
    output = {
        'scraped_websites': scraped_data,
        'statistics': {
            'total_attempted': len(projects_with_websites),
            'successful': successful,
            'errors': errors,
            'success_rate': successful / len(projects_with_websites) if projects_with_websites else 0,
            'new_socials_discovered': new_socials_found
        },
        'technology_mentions': dict(sorted(all_tech.items(), key=lambda x: x[1], reverse=True))
    }

    with open('data/enriched/website_metadata.json', 'w', encoding='utf-8') as f:
        json.dump(output, f, indent=2, ensure_ascii=False)

    print(f"\n[OK] Website scraping complete!")
    print(f"\nStatistics:")
    print(f"   * Successful: {successful}/{len(projects_with_websites)}")
    print(f"   * Errors: {errors}")
    print(f"   * New social accounts found: {new_socials_found}")

    if all_tech:
        print(f"\nTop Technologies Mentioned:")
        for tech, count in list(all_tech.items())[:10]:
            print(f"   * {tech}: {count} projects")

    print(f"\n[OK] Saved to data/enriched/website_metadata.json")


if __name__ == "__main__":
    import os
    os.makedirs('data/enriched', exist_ok=True)
    main()
