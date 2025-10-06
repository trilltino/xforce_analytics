"""
Master Enrichment Script
Runs all data enrichment and analytics scripts in the correct order
"""

import subprocess
import sys
import os


def run_script(script_name, description):
    """Run a Python script and print results"""
    print(f"\n{'='*80}")
    print(f">> {description}")
    print(f"{'='*80}\n")

    try:
        result = subprocess.run(
            [sys.executable, f'scripts/{script_name}'],
            capture_output=False,
            text=True,
            check=True
        )
        print(f"\n[OK] {description} - COMPLETE\n")
        return True
    except subprocess.CalledProcessError as e:
        print(f"\n[FAIL] {description} - FAILED")
        print(f"Error: {e}")
        return False
    except Exception as e:
        print(f"\n[ERROR] {description} - ERROR")
        print(f"Error: {e}")
        return False


def main():
    print("""
===============================================================================

                     XFORCE ANALYTICS DATA ENRICHMENT

                    Comprehensive Data Analysis Pipeline

===============================================================================
    """)

    scripts = [
        ('temporal_analysis.py', 'PHASE 1: Temporal Analysis'),
        ('geographic_enrichment.py', 'PHASE 2: Geographic Enrichment'),
        ('html_parser.py', 'PHASE 3: HTML Data Extraction'),
        ('deep_analytics.py', 'PHASE 4: Deep Analytics Generation'),
        # GitHub fetcher is optional due to rate limits
        # ('github_fetcher.py', 'PHASE 5: GitHub Data Fetching (Optional)'),
    ]

    successful = 0
    failed = 0

    for script, description in scripts:
        if run_script(script, description):
            successful += 1
        else:
            failed += 1
            print(f"\n[WARN]  Continuing despite failure in {script}...")

    print(f"\n{'='*80}")
    print(f"ENRICHMENT SUMMARY")
    print(f"{'='*80}\n")
    print(f"   [OK] Successful: {successful}/{len(scripts)}")
    print(f"   [FAIL] Failed: {failed}/{len(scripts)}")

    if failed == 0:
        print(f"\nALL ENRICHMENT COMPLETE!")
    else:
        print(f"\n[WARN] Some scripts failed, but enrichment partially complete")

    print(f"\n{'='*80}")
    print(f"GENERATED FILES")
    print(f"{'='*80}\n")

    # List all generated files
    directories = [
        'data/analytics/temporal',
        'data/analytics/geographic',
        'data/analytics',
        'data/enriched'
    ]

    for directory in directories:
        if os.path.exists(directory):
            files = [f for f in os.listdir(directory) if f.endswith('.json')]
            if files:
                print(f"\n[DIR] {directory}/")
                for file in sorted(files):
                    file_path = os.path.join(directory, file)
                    size = os.path.getsize(file_path) / 1024  # KB
                    print(f"   +-- {file} ({size:.1f} KB)")

    print(f"\n{'='*80}")
    print(f"\nTo fetch GitHub data (optional, requires API token):")
    print(f"   set GITHUB_TOKEN=your_token_here")
    print(f"   python scripts/github_fetcher.py")
    print(f"\n{'='*80}\n")


if __name__ == "__main__":
    main()
