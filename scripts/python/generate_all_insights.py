#!/usr/bin/env python3
"""
Generate all analytics insights from SDF funding data
"""
import json
import os
from pathlib import Path

def main():
    # Paths
    project_root = Path(__file__).parent.parent.parent
    data_raw = project_root / "data" / "raw"
    data_processed = project_root / "data" / "processed"

    # Create processed directory
    data_processed.mkdir(parents=True, exist_ok=True)

    # Load projects
    with open(data_raw / "all_projects_structured.json") as f:
        projects = json.load(f)

    print(f"📊 Loaded {len(projects)} projects")

    # Generate insights
    insights = {
        "total_projects": len(projects),
        "total_funding": sum(
            extract_funding(p.get("funding_amount", ""))
            for p in projects
        ),
        "categories": list(set(
            p.get("category", "Other")
            for p in projects
            if p.get("category")
        )),
        "soroban_count": sum(
            1 for p in projects
            if p.get("soroban", "").lower() in ["yes", "true"]
        ),
    }

    # Save insights
    with open(data_processed / "insights.json", "w") as f:
        json.dump(insights, f, indent=2)

    print(f"✅ Generated insights:")
    print(f"   - Total Projects: {insights['total_projects']}")
    print(f"   - Total Funding: ${insights['total_funding']:,.2f}")
    print(f"   - Soroban Projects: {insights['soroban_count']}")
    print(f"   - Categories: {len(insights['categories'])}")

    # Generate predictions placeholder
    predictions = {
        "categories": {
            cat: {
                "avg_funding": insights["total_funding"] / len(projects),
                "project_count": sum(1 for p in projects if p.get("category") == cat)
            }
            for cat in insights["categories"]
        }
    }

    with open(data_processed / "predictions.json", "w") as f:
        json.dump(predictions, f, indent=2)

    print("✅ Generated predictions")

    # Generate opportunities placeholder
    opportunities = {
        "gaps": [
            "DeFi infrastructure for Soroban",
            "Developer tooling for smart contracts",
            "Analytics and data platforms"
        ],
        "high_potential": insights["categories"][:5]
    }

    with open(data_processed / "opportunities.json", "w") as f:
        json.dump(opportunities, f, indent=2)

    print("✅ Generated opportunities")
    print("\n🎉 All insights generated successfully!")

def extract_funding(amount_str):
    """Extract numeric funding amount from string"""
    if not amount_str:
        return 0

    # Remove $ and commas
    cleaned = amount_str.replace("$", "").replace(",", "")

    # Handle M and K suffixes
    if "M" in cleaned:
        return float(cleaned.replace("M", "")) * 1_000_000
    elif "K" in cleaned:
        return float(cleaned.replace("K", "")) * 1_000

    try:
        return float(cleaned)
    except:
        return 0

if __name__ == "__main__":
    main()
