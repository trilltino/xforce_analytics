"""
Polars-Based Analytics Engine
Ultra-fast data processing using Polars for all 443 projects
"""

import polars as pl
import json
from pathlib import Path
from datetime import datetime
import re


def load_projects_as_polars():
    """Load all projects into a Polars DataFrame"""
    with open('data/raw/all_projects_structured.json', 'r', encoding='utf-8') as f:
        data = json.load(f)

    # Flatten the nested structure
    all_projects = []
    for category, projects in data['by_category'].items():
        all_projects.extend(projects)

    # Create DataFrame
    df = pl.DataFrame(all_projects)

    # Data type conversions and cleaning
    df = df.with_columns([
        pl.col('total_awarded').cast(pl.Float64).fill_null(0),
        pl.col('soroban').fill_null(False),
    ])

    print(f"Loaded {len(df)} projects into Polars DataFrame")
    print(f"Columns: {df.columns}")
    print(f"Shape: {df.shape}")

    return df


def parse_quarters_column(df):
    """Parse quarters column into structured data"""
    # Extract first and last quarter
    df = df.with_columns([
        pl.col('quarters').str.split(',').list.first().alias('first_quarter'),
        pl.col('quarters').str.split(',').list.last().alias('last_quarter'),
        pl.col('quarters').str.split(',').list.len().alias('quarter_count'),
    ])

    return df


def parse_rounds_column(df):
    """Parse rounds column into structured data"""
    df = df.with_columns([
        pl.col('rounds').str.split(',').list.len().alias('round_count').fill_null(0),
        pl.col('rounds').str.contains('SCF').fill_null(False).alias('has_scf_rounds'),
    ])

    return df


def parse_programs_column(df):
    """Parse programs column"""
    df = df.with_columns([
        pl.col('programs').str.contains('SCF Build').fill_null(False).alias('has_build'),
        pl.col('programs').str.contains('Audit Bank').fill_null(False).alias('has_audit'),
        pl.col('programs').str.split(',').list.len().alias('program_count').fill_null(0),
    ])

    return df


def calculate_success_score(df):
    """Calculate comprehensive success score"""
    df = df.with_columns([
        (
            (pl.col('total_awarded') / 50000.0 * 2) +  # Funding (max 2 points per 50k)
            (pl.when(pl.col('integration_status') == 'Mainnet').then(3).otherwise(0)) +
            (pl.when(pl.col('soroban')).then(2).otherwise(0)) +
            (pl.col('round_count') * 1.5) +
            (pl.when(pl.col('has_build') & pl.col('has_audit')).then(2).otherwise(0)) +
            (pl.when(pl.col('open_source').str.contains('fully open-source')).then(1).otherwise(0))
        ).alias('success_score')
    ])

    return df


def analyze_by_category(df):
    """Group by category and calculate aggregations"""
    category_stats = df.group_by('category').agg([
        pl.count().alias('project_count'),
        pl.col('total_awarded').sum().alias('total_funding'),
        pl.col('total_awarded').mean().alias('avg_funding'),
        pl.col('total_awarded').median().alias('median_funding'),
        pl.col('total_awarded').max().alias('max_funding'),
        pl.col('soroban').sum().alias('soroban_count'),
        (pl.col('integration_status') == 'Mainnet').sum().alias('mainnet_count'),
        pl.col('round_count').mean().alias('avg_rounds'),
        pl.col('success_score').mean().alias('avg_success_score'),
    ]).with_columns([
        (pl.col('soroban_count') / pl.col('project_count')).alias('soroban_rate'),
        (pl.col('mainnet_count') / pl.col('project_count')).alias('mainnet_rate'),
    ]).sort('total_funding', descending=True)

    return category_stats


def analyze_by_country(df):
    """Geographic analysis"""
    country_stats = df.group_by('country').agg([
        pl.count().alias('project_count'),
        pl.col('total_awarded').sum().alias('total_funding'),
        pl.col('total_awarded').mean().alias('avg_funding'),
        (pl.col('integration_status') == 'Mainnet').sum().alias('mainnet_count'),
    ]).filter(
        pl.col('country').is_not_null()
    ).with_columns([
        (pl.col('mainnet_count') / pl.col('project_count')).alias('mainnet_rate'),
    ]).sort('total_funding', descending=True)

    return country_stats


def analyze_funding_tiers(df):
    """Analyze by funding tier"""
    df = df.with_columns([
        pl.when(pl.col('total_awarded') < 50000).then('$0-50k')
        .when(pl.col('total_awarded') < 100000).then('$50-100k')
        .when(pl.col('total_awarded') < 150000).then('$100-150k')
        .when(pl.col('total_awarded') < 200000).then('$150-200k')
        .otherwise('$200k+')
        .alias('funding_tier')
    ])

    tier_stats = df.group_by('funding_tier').agg([
        pl.count().alias('project_count'),
        pl.col('total_awarded').mean().alias('avg_funding'),
        (pl.col('integration_status') == 'Mainnet').sum().alias('mainnet_count'),
        pl.col('round_count').mean().alias('avg_rounds'),
        pl.col('soroban').sum().alias('soroban_count'),
    ]).with_columns([
        (pl.col('mainnet_count') / pl.col('project_count')).alias('mainnet_rate'),
        (pl.col('soroban_count') / pl.col('project_count')).alias('soroban_rate'),
    ])

    # Sort by tier order
    tier_order = ['$0-50k', '$50-100k', '$100-150k', '$150-200k', '$200k+']
    tier_stats = tier_stats.sort(
        pl.col('funding_tier').map_dict({tier: i for i, tier in enumerate(tier_order)})
    )

    return tier_stats


def analyze_program_combinations(df):
    """Analyze program combinations"""
    program_stats = df.group_by('programs').agg([
        pl.count().alias('project_count'),
        pl.col('total_awarded').sum().alias('total_funding'),
        pl.col('total_awarded').mean().alias('avg_funding'),
        (pl.col('integration_status') == 'Mainnet').sum().alias('mainnet_count'),
        pl.col('round_count').mean().alias('avg_rounds'),
    ]).filter(
        pl.col('programs').is_not_null()
    ).with_columns([
        (pl.col('mainnet_count') / pl.col('project_count')).alias('mainnet_rate'),
    ]).sort('avg_funding', descending=True)

    return program_stats


def analyze_soroban_advantage(df):
    """Compare Soroban vs non-Soroban projects"""
    soroban_stats = df.group_by('soroban').agg([
        pl.count().alias('project_count'),
        pl.col('total_awarded').mean().alias('avg_funding'),
        pl.col('total_awarded').median().alias('median_funding'),
        (pl.col('integration_status') == 'Mainnet').sum().alias('mainnet_count'),
        pl.col('round_count').mean().alias('avg_rounds'),
    ]).with_columns([
        (pl.col('mainnet_count') / pl.col('project_count')).alias('mainnet_rate'),
    ])

    return soroban_stats


def analyze_top_performers(df, top_n=10):
    """Identify top performers by various metrics"""
    top_funding = df.select([
        'title', 'category', 'total_awarded', 'integration_status', 'soroban'
    ]).sort('total_awarded', descending=True).head(top_n)

    top_rounds = df.filter(
        pl.col('round_count') > 0
    ).select([
        'title', 'category', 'round_count', 'total_awarded', 'rounds'
    ]).sort('round_count', descending=True).head(top_n)

    top_success_score = df.select([
        'title', 'category', 'success_score', 'total_awarded', 'integration_status'
    ]).sort('success_score', descending=True).head(top_n)

    return {
        'top_by_funding': top_funding,
        'top_by_rounds': top_rounds,
        'top_by_success_score': top_success_score
    }


def analyze_multichain(df):
    """Analyze multichain vs Stellar-only"""
    df = df.with_columns([
        pl.when(
            pl.col('other_chains').is_null() |
            (pl.col('other_chains') == '') |
            (pl.col('other_chains').str.to_lowercase().str.contains('not multichain'))
        ).then('Stellar Only')
        .otherwise('Multichain')
        .alias('chain_strategy')
    ])

    chain_stats = df.group_by('chain_strategy').agg([
        pl.count().alias('project_count'),
        pl.col('total_awarded').mean().alias('avg_funding'),
        pl.col('total_awarded').median().alias('median_funding'),
        (pl.col('integration_status') == 'Mainnet').sum().alias('mainnet_count'),
        pl.col('round_count').mean().alias('avg_rounds'),
    ]).with_columns([
        (pl.col('mainnet_count') / pl.col('project_count')).alias('mainnet_rate'),
    ])

    return chain_stats


def export_to_parquet(df, base_path='data/parquet'):
    """Export to Parquet for fast loading"""
    Path(base_path).mkdir(parents=True, exist_ok=True)

    # Main dataset
    df.write_parquet(f'{base_path}/all_projects.parquet')
    print(f"[OK] Exported to {base_path}/all_projects.parquet")

    # Partitioned by category
    for category in df['category'].unique():
        category_df = df.filter(pl.col('category') == category)
        safe_name = category.lower().replace(' & ', '_').replace(' ', '_')
        category_df.write_parquet(f'{base_path}/by_category_{safe_name}.parquet')


def main():
    print("="*80)
    print("POLARS ANALYTICS ENGINE")
    print("="*80)

    # Load data
    print("\n[1/9] Loading projects...")
    df = load_projects_as_polars()

    # Parse columns
    print("[2/9] Parsing quarters...")
    df = parse_quarters_column(df)

    print("[3/9] Parsing rounds...")
    df = parse_rounds_column(df)

    print("[4/9] Parsing programs...")
    df = parse_programs_column(df)

    print("[5/9] Calculating success scores...")
    df = calculate_success_score(df)

    # Analyze
    print("[6/9] Analyzing by category...")
    category_stats = analyze_by_category(df)
    print(category_stats)

    print("\n[7/9] Analyzing by country...")
    country_stats = analyze_by_country(df)
    print(country_stats.head(10))

    print("\n[8/9] Analyzing funding tiers...")
    tier_stats = analyze_funding_tiers(df)
    print(tier_stats)

    print("\n[9/9] Additional analyses...")
    soroban_stats = analyze_soroban_advantage(df)
    print("\nSoroban vs Non-Soroban:")
    print(soroban_stats)

    chain_stats = analyze_multichain(df)
    print("\nMultichain Analysis:")
    print(chain_stats)

    program_stats = analyze_program_combinations(df)
    print("\nTop Program Combinations:")
    print(program_stats.head(10))

    top_performers = analyze_top_performers(df, top_n=10)
    print("\nTop 10 by Funding:")
    print(top_performers['top_by_funding'])

    # Export results
    print("\n" + "="*80)
    print("EXPORTING RESULTS")
    print("="*80)

    # Save to JSON
    Path('data/analytics/polars').mkdir(parents=True, exist_ok=True)

    category_stats.write_json('data/analytics/polars/category_stats.json')
    country_stats.write_json('data/analytics/polars/country_stats.json')
    tier_stats.write_json('data/analytics/polars/tier_stats.json')
    soroban_stats.write_json('data/analytics/polars/soroban_comparison.json')
    chain_stats.write_json('data/analytics/polars/chain_strategy.json')
    program_stats.write_json('data/analytics/polars/program_combinations.json')

    print("[OK] Saved JSON files to data/analytics/polars/")

    # Export to Parquet for fast reloading
    export_to_parquet(df)

    # Save enriched DataFrame
    df.write_json('data/enriched/projects_polars_enriched.json')
    print("[OK] Saved enriched dataset to data/enriched/projects_polars_enriched.json")

    print("\n" + "="*80)
    print("KEY INSIGHTS")
    print("="*80)

    total_projects = len(df)
    total_funding = df['total_awarded'].sum()
    avg_funding = df['total_awarded'].mean()

    soroban_avg = soroban_stats.filter(pl.col('soroban') == True)['avg_funding'][0]
    non_soroban_avg = soroban_stats.filter(pl.col('soroban') == False)['avg_funding'][0]

    print(f"\nDataset Overview:")
    print(f"  * Total Projects: {total_projects}")
    print(f"  * Total Funding: ${total_funding:,.0f}")
    print(f"  * Average Funding: ${avg_funding:,.0f}")

    print(f"\nSoroban Impact:")
    print(f"  * Soroban Avg: ${soroban_avg:,.0f}")
    print(f"  * Non-Soroban Avg: ${non_soroban_avg:,.0f}")
    print(f"  * Premium: {((soroban_avg - non_soroban_avg) / non_soroban_avg * 100):.1f}%")

    top_category = category_stats.head(1)
    print(f"\nTop Category: {top_category['category'][0]}")
    print(f"  * Projects: {top_category['project_count'][0]}")
    print(f"  * Total Funding: ${top_category['total_funding'][0]:,.0f}")

    print("\n" + "="*80)
    print("[SUCCESS] Polars analytics complete!")
    print("="*80)


if __name__ == "__main__":
    main()
