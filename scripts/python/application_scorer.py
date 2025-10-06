"""
APPLICATION READINESS SCORER - Score your application before submitting
Usage: python application_scorer.py
"""

print("=" * 100)
print("SDF APPLICATION READINESS SCORER")
print("Answer questions to get your readiness score")
print("=" * 100)
print("\nBased on analysis of 443 funded projects and top success patterns\n")

def ask_yes_no(question):
    """Ask a yes/no question"""
    while True:
        response = input(f"{question} (y/n): ").lower().strip()
        if response in ['y', 'yes']:
            return True
        elif response in ['n', 'no']:
            return False
        print("Please answer y or n")

def ask_choice(question, choices):
    """Ask multiple choice question"""
    print(f"\n{question}")
    for i, choice in enumerate(choices, 1):
        print(f"  {i}. {choice}")

    while True:
        try:
            response = int(input("Enter number: "))
            if 1 <= response <= len(choices):
                return choices[response - 1]
        except:
            pass
        print(f"Please enter a number between 1 and {len(choices)}")

def ask_number(question, min_val, max_val):
    """Ask for a number in range"""
    while True:
        try:
            response = int(input(f"{question} ({min_val}-{max_val}): "))
            if min_val <= response <= max_val:
                return response
        except:
            pass
        print(f"Please enter a number between {min_val} and {max_val}")

# Start scoring
score = 0
max_score = 0
feedback = []

print("\n" + "=" * 100)
print("SECTION 1: PRODUCT-MARKET FIT (30 points)")
print("=" * 100)

# PMF validation
if ask_yes_no("\n1. Have you conducted 10+ user interviews?"):
    score += 10
    feedback.append("✓ Strong PMF validation")
else:
    feedback.append("❌ Conduct 10+ user interviews (CRITICAL)")

if ask_yes_no("2. Do you have beta testers or users?"):
    score += 10
    feedback.append("✓ User validation")
else:
    feedback.append("⚠️ Get beta testers before applying")

if ask_yes_no("3. Do you have letters of intent or partnerships lined up?"):
    score += 10
    feedback.append("✓ Partnership validation")
else:
    feedback.append("⚠️ Consider getting LOIs from potential partners")

max_score += 30

print("\n" + "=" * 100)
print("SECTION 2: TECHNICAL READINESS (30 points)")
print("=" * 100)

# Technical architecture
if ask_yes_no("\n4. Have you written a detailed technical architecture document?"):
    score += 10
    feedback.append("✓ Technical documentation ready")
else:
    feedback.append("❌ Write technical architecture doc (REQUIRED)")

if ask_yes_no("5. Do you have contract/data flow diagrams?"):
    score += 5
    feedback.append("✓ Visual documentation")
else:
    feedback.append("⚠️ Create diagrams (highly recommended)")

if ask_yes_no("6. Have you built a prototype or MVP?"):
    score += 10
    feedback.append("✓ Prototype exists - STRONG")
else:
    feedback.append("⚠️ Prototype gives major advantage")

if ask_yes_no("7. Do you have a Github repository with code?"):
    score += 5
    feedback.append("✓ Github repo (70% of top-funded have this)")
else:
    feedback.append("⚠️ Github repo recommended (70% of top-funded have one)")

max_score += 30

print("\n" + "=" * 100)
print("SECTION 3: STELLAR INTEGRATION (20 points)")
print("=" * 100)

# Stellar integration
platform = ask_choice("\n8. What are you building on?", [
    "Soroban smart contracts",
    "Classic Stellar",
    "Both",
    "Not sure yet"
])

if "Soroban" in platform:
    score += 10
    feedback.append("✓ Soroban - 68% of top-funded use it")
elif "Classic" in platform:
    score += 7
    feedback.append("✓ Classic Stellar - good choice")
elif "Both" in platform:
    score += 8
    feedback.append("✓ Hybrid approach")
else:
    feedback.append("❌ Decide on Stellar/Soroban ASAP")

multichain = ask_yes_no("9. Will you support other chains besides Stellar?")
if not multichain:
    score += 10
    feedback.append("✓ Stellar-only (+30% funding premium)")
else:
    score += 5
    feedback.append("⚠️ Multichain gets 30% less funding on average")

max_score += 20

print("\n" + "=" * 100)
print("SECTION 4: TEAM & EXECUTION (15 points)")
print("=" * 100)

# Team
if ask_yes_no("\n10. Does your team have blockchain development experience?"):
    score += 5
    feedback.append("✓ Experienced team")
else:
    feedback.append("⚠️ Demonstrate ability to execute")

if ask_yes_no("11. Does your team have Stellar/Soroban experience?"):
    score += 5
    feedback.append("✓ Stellar experience - STRONG")
else:
    feedback.append("⚠️ Learn Stellar before applying")

if ask_yes_no("12. Do you have a public track record (Github, previous projects)?"):
    score += 5
    feedback.append("✓ Proven track record")
else:
    feedback.append("⚠️ Build public portfolio")

max_score += 15

print("\n" + "=" * 100)
print("SECTION 5: COMPETITIVE POSITIONING (15 points)")
print("=" * 100)

category = ask_choice("\n13. What category are you applying to?", [
    "Financial Protocols ($130k avg)",
    "Infrastructure & Services ($101k avg)",
    "Applications ($93k avg)",
    "Developer Tooling ($79k avg)",
    "Education & Community ($66k avg)"
])

if "Financial" in category:
    score += 10
    feedback.append("✓ Highest-funded category ($130k avg)")
elif "Infrastructure" in category:
    score += 9
    feedback.append("✓ High-value category ($101k avg)")
elif "Applications" in category:
    score += 7
    feedback.append("✓ Good category ($93k avg)")
else:
    score += 6
    feedback.append("⚠️ Lower average funding category")

if ask_yes_no("14. Is your specific project type underserved (<10 funded projects)?"):
    score += 5
    feedback.append("✓ Low competition - EXCELLENT")
else:
    feedback.append("⚠️ Need strong differentiation")

max_score += 15

print("\n" + "=" * 100)
print("SECTION 6: APPLICATION QUALITY (10 points)")
print("=" * 100)

# Application materials
if ask_yes_no("\n15. Do you have a website?"):
    score += 5
    feedback.append("✓ Website (100% of top-funded have this)")
else:
    feedback.append("❌ Website REQUIRED (100% of top-funded have one)")

if ask_yes_no("16. Have you prepared a detailed budget breakdown?"):
    score += 5
    feedback.append("✓ Budget prepared")
else:
    feedback.append("❌ Prepare detailed budget (REQUIRED)")

max_score += 10

# Calculate final score
percentage = (score / max_score) * 100

print("\n" + "=" * 100)
print("YOUR READINESS SCORE")
print("=" * 100)
print(f"\nScore: {score}/{max_score} ({percentage:.1f}%)\n")

if percentage >= 80:
    grade = "🔥🔥🔥 EXCELLENT - READY TO APPLY"
    recommendation = "Your application is well-prepared. Apply with confidence!"
elif percentage >= 60:
    grade = "⭐⭐ GOOD - ALMOST READY"
    recommendation = "Address the items below, then apply."
elif percentage >= 40:
    grade = "⚠️ FAIR - NEEDS WORK"
    recommendation = "Significant preparation needed before applying."
else:
    grade = "❌ NOT READY"
    recommendation = "Focus on fundamentals before applying."

print(f"Grade: {grade}")
print(f"\nRecommendation: {recommendation}\n")

print("=" * 100)
print("DETAILED FEEDBACK")
print("=" * 100)
print()

for item in feedback:
    print(item)

# Success factors comparison
print("\n" + "=" * 100)
print("COMPARISON TO TOP-FUNDED PROJECTS")
print("=" * 100)
print("""
Top 25% funded projects (>$144k) typically have:

✓ 72% reached Mainnet status
✓ 68% received multiple rounds (2-4 applications)
✓ 68% are Soroban-based
✓ 75% are Stellar-only
✓ 70% have public Github
✓ 100% have website
✓ Detailed technical architecture
✓ Evidence of PMF validation
✓ Clear differentiation

Your profile should match these patterns for best results.
""")

# Funding estimate
print("=" * 100)
print("ESTIMATED FUNDING POTENTIAL")
print("=" * 100)

if percentage >= 80:
    print("""
Round 1: $75k - $100k (strong application)
Round 2: $100k - $125k (with progress)
Round 3: $150k - $200k (at mainnet)
Total Potential: $325k - $425k (over 12-18 months)
""")
elif percentage >= 60:
    print("""
Round 1: $50k - $75k (good application)
Round 2: $75k - $100k (with progress)
Total Potential: $125k - $175k (over 12-18 months)
""")
else:
    print("""
Round 1: $50k - $75k (if approved)
Recommendation: Strengthen application before applying
""")

print("=" * 100)
print("NEXT STEPS:")
if percentage >= 80:
    print("  1. Double-check all application materials")
    print("  2. Submit to dashboard.communityfund.stellar.org")
    print("  3. Engage with community in Discord")
elif percentage >= 60:
    print("  1. Address feedback items marked with ❌")
    print("  2. Strengthen items marked with ⚠️")
    print("  3. Re-run this scorer to confirm readiness")
    print("  4. Apply when score >80%")
else:
    print("  1. Focus on PMF validation (user interviews)")
    print("  2. Write technical architecture document")
    print("  3. Build prototype/MVP if possible")
    print("  4. Create website and Github repo")
    print("  5. Re-run scorer monthly to track progress")
print("=" * 100)
