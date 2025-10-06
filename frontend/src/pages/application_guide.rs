use yew::prelude::*;
use crate::components::layout::Layout;

#[function_component(ApplicationGuide)]
pub fn application_guide() -> Html {
    let active_section = use_state(|| String::from("quick-start"));

    let scroll_to_section = {
        let active_section = active_section.clone();
        Callback::from(move |section_id: String| {
            active_section.set(section_id.clone());
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(&section_id) {
                        element.scroll_into_view();
                    }
                }
            }
        })
    };

    html! {
        <Layout>
            <style>
                {include_str!("../styles/application_guide.css")}
            </style>

            <div class="guide_page">
                // Sidebar Navigation
                <aside class="guide_sidebar">
                    <div class="sidebar_content">
                        <h2 class="sidebar_title">{"Application Guide"}</h2>

                        <nav class="sidebar_nav">
                            <div class="nav_group">
                                <h3>{"Quick Access"}</h3>
                                <button
                                    class={classes!("nav_item", (*active_section == "quick-start").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("quick-start".to_string())}
                                >
                                    <i class="fas fa-bolt"></i>
                                    {" Quick Start"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "opportunities").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("opportunities".to_string())}
                                >
                                    <i class="fas fa-chart-line"></i>
                                    {" Opportunities"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "calculator").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("calculator".to_string())}
                                >
                                    <i class="fas fa-calculator"></i>
                                    {" Funding Calculator"}
                                </button>
                            </div>

                            <div class="nav_group">
                                <h3>{"Core Programs"}</h3>
                                <button
                                    class={classes!("nav_item", (*active_section == "kickstart").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("kickstart".to_string())}
                                >
                                    <i class="fas fa-rocket"></i>
                                    {" Kickstart"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "scf-build").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("scf-build".to_string())}
                                >
                                    <i class="fas fa-hammer"></i>
                                    {" Build"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "audit-bank").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("audit-bank".to_string())}
                                >
                                    <i class="fas fa-shield-alt"></i>
                                    {" Audit Bank"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "liquidity").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("liquidity".to_string())}
                                >
                                    <i class="fas fa-tint"></i>
                                    {" Liquidity"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "public-goods").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("public-goods".to_string())}
                                >
                                    <i class="fas fa-globe"></i>
                                    {" Public Goods"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "growth-hack").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("growth-hack".to_string())}
                                >
                                    <i class="fas fa-chart-line"></i>
                                    {" Growth Hack"}
                                </button>
                            </div>

                            <div class="nav_group">
                                <h3>{"Category Strategies"}</h3>
                                <button
                                    class={classes!("nav_item", (*active_section == "applications").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("applications".to_string())}
                                >
                                    {"Applications"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "financial").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("financial".to_string())}
                                >
                                    {"Financial Protocols"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "infrastructure").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("infrastructure".to_string())}
                                >
                                    {"Infrastructure"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "developer").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("developer".to_string())}
                                >
                                    {"Developer Tools"}
                                </button>
                            </div>

                            <div class="nav_group">
                                <h3>{"Resources"}</h3>
                                <button
                                    class={classes!("nav_item", (*active_section == "case-studies").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("case-studies".to_string())}
                                >
                                    {"Case Studies"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "checklists").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("checklists".to_string())}
                                >
                                    {"Checklists"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "templates").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("templates".to_string())}
                                >
                                    {"Templates"}
                                </button>
                                <button
                                    class={classes!("nav_item", (*active_section == "success-formula").then(|| "active"))}
                                    onclick={let cb = scroll_to_section.clone(); move |_| cb.emit("success-formula".to_string())}
                                >
                                    {"Success Formula"}
                                </button>
                            </div>
                        </nav>
                    </div>
                </aside>

                // Main Content
                <div class="guide_content">
                    <div class="guide_header">
                        <h1>{"ULTIMATE SDF FUNDING APPLICATION GUIDE"}</h1>
                        <p class="guide_subtitle">{"Complete Strategy Guide for All Categories and Programs"}</p>
                        <div class="guide_stats">
                            <div class="stat">
                                <div class="stat_value">{"443"}</div>
                                <div class="stat_label">{"Projects Analyzed"}</div>
                            </div>
                            <div class="stat">
                                <div class="stat_value">{"$42.1M"}</div>
                                <div class="stat_label">{"Total Funding"}</div>
                            </div>
                            <div class="stat">
                                <div class="stat_value">{"$95k"}</div>
                                <div class="stat_label">{"Average Award"}</div>
                            </div>
                            <div class="stat">
                                <div class="stat_value">{"$144k"}</div>
                                <div class="stat_label">{"Top 25% Threshold"}</div>
                            </div>
                            <div class="stat">
                                <div class="stat_value">{"5.0x"}</div>
                                <div class="stat_label">{"4-Round Multiplier"}</div>
                            </div>
                        </div>
                    </div>

                    // Quick Start Section
                    <section id="quick-start" class="guide_section">
                        <h2>{" QUICK START: 3-Minute Action Plan"}</h2>

                        <div class="info_box info_box_primary">
                            <h3>{"Step 1: Choose Your Category (2 minutes)"}</h3>

                            <h4>{"HIGHEST OPPORTUNITY (Data-Validated):"}</h4>
                            <ul>
                                <li><strong>{"Anchor"}</strong>{" - Only 4 projects, $331k average"}
                                    <br/><span class="text_muted">{"Build: Soroban-native anchor for specific region | Expected: $150k-$350k"}</span>
                                </li>
                            </ul>

                            <h4>{"ZERO-COMPETITION BLUE OCEAN OPPORTUNITIES:"}</h4>
                            <ul>
                                <li><strong>{"Credit Scoring Infrastructure"}</strong>{" (0 projects)"}
                                    <br/><span class="text_muted">{"Potential: $150K Build + enables DeFi lending ecosystem"}</span>
                                </li>
                                <li><strong>{"Subscription Payment Platform"}</strong>{" (0 projects)"}
                                    <br/><span class="text_muted">{"Potential: $120K Build + $220K Growth Hack = $340K"}</span>
                                </li>
                                <li><strong>{"Escrow & Insurance Protocol"}</strong>{" (0 projects)"}
                                    <br/><span class="text_muted">{"Potential: $150K Build + $100K Liquidity = $250K"}</span>
                                </li>
                                <li><strong>{"Gas Optimization Tools"}</strong>{" (0 projects)"}
                                    <br/><span class="text_muted">{"Potential: $100K Build + every developer needs this"}</span>
                                </li>
                                <li><strong>{"Soroban-native Anchor"}</strong>{" (0 Soroban anchors exist!)"}
                                    <br/><span class="text_muted">{"Potential: $330K average (highest funding category!)"}</span>
                                </li>
                            </ul>

                            <h4>{" AVOID (Saturated):"}</h4>
                            <ul>
                                <li>{"Generic Wallets (35 projects)"}</li>
                                <li>{"Generic Payments (44 projects)"}</li>
                                <li>{"Generic Dev Tools (57 projects)"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Step 2: Plan Your Timeline"}</h3>
                            <ul>
                                <li><strong>{"Q2 (April-June):"}</strong>{" 30.2% of all funding - BEST"}</li>
                                <li><strong>{"Q4 (October-December):"}</strong>{" 28.3% of all funding - SECOND BEST"}</li>
                                <li><strong>{"Multi-round spacing:"}</strong>{" 4-5 months between applications"}</li>
                            </ul>
                        </div>
                    </section>

                    // Opportunities Section
                    <section id="opportunities" class="guide_section">
                        <h2>{"DATA-DRIVEN OPPORTUNITY ANALYSIS"}</h2>

                        <div class="table_wrapper">
                            <h3>{"TOP 10 VALIDATED OPPORTUNITIES"}</h3>
                            <table class="data_table">
                                <thead>
                                    <tr>
                                        <th>{"Rank"}</th>
                                        <th>{"Category"}</th>
                                        <th>{"Projects"}</th>
                                        <th>{"Avg Funding"}</th>
                                        <th>{"Opportunity"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td>{"1"}</td>
                                        <td><strong>{"Anchor"}</strong></td>
                                        <td>{"4"}</td>
                                        <td><strong>{"$331,764"}</strong></td>
                                        <td><span class="badge badge-success">{"VERY LOW"}</span></td>
                                    </tr>
                                    <tr>
                                        <td>{"2"}</td>
                                        <td><strong>{"NFTs"}</strong></td>
                                        <td>{"2"}</td>
                                        <td>{"$263,417"}</td>
                                        <td><span class="badge badge-success">{"VERY LOW"}</span></td>
                                    </tr>
                                    <tr>
                                        <td>{"3"}</td>
                                        <td><strong>{"CDP/Basis Trading"}</strong></td>
                                        <td>{"3"}</td>
                                        <td>{"$214,517"}</td>
                                        <td><span class="badge badge-success">{"VERY LOW"}</span></td>
                                    </tr>
                                    <tr>
                                        <td>{"4"}</td>
                                        <td><strong>{"Yield Aggregator"}</strong></td>
                                        <td>{"1"}</td>
                                        <td>{"$200,000"}</td>
                                        <td><span class="badge badge-success">{"ZERO"}</span></td>
                                    </tr>
                                    <tr>
                                        <td>{"5"}</td>
                                        <td><strong>{"Formal Verification"}</strong></td>
                                        <td>{"1"}</td>
                                        <td>{"$150,000"}</td>
                                        <td><span class="badge badge-success">{"ZERO"}</span></td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </section>

                    // Calculator Section
                    <section id="calculator" class="guide_section">
                        <h2>{"FUNDING MULTIPLIER CALCULATOR"}</h2>

                        <div class="info_box">
                            <h3>{"Multiple Rounds Multiplier Effect (Real Data)"}</h3>
                            <ul>
                                <li><strong>{"1 Round:"}</strong>{" $68,634 average (1.0x baseline)"}</li>
                                <li><strong>{"2 Rounds:"}</strong>{" $125,248 average (1.8x multiplier)"}</li>
                                <li><strong>{"3 Rounds:"}</strong>{" $232,526 average (3.4x multiplier)"}</li>
                                <li><strong>{"4 Rounds:"}</strong>{" $339,956 average (5.0x multiplier)"}</li>
                            </ul>
                            <p class="text_muted">{"Key Insight: 68.5% of top-funded projects received 2+ rounds"}</p>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Geographic Sweet Spots (Top-Funded Averages)"}</h3>
                            <ul>
                                <li><strong>{"Portugal:"}</strong>{" $297,197 average"}</li>
                                <li><strong>{"Germany:"}</strong>{" $284,650 average"}</li>
                                <li><strong>{"Netherlands:"}</strong>{" $263,387 average"}</li>
                                <li><strong>{"Chile:"}</strong>{" $248,375 average"}</li>
                                <li><strong>{"Colombia:"}</strong>{" $219,495 average"}</li>
                            </ul>
                            <p class="text_muted">{"European and South American projects show highest success rates"}</p>
                        </div>

                        <div class="code_block">
{r#"Example Path to $470K Total Funding:

Round 1 (Build Award):    $150,000
├─ Mainnet launch:   +30% premium
├─ Stellar-only:     +30% premium
└─ Open source:      Success factor

Round 2 (Build Award):    $150,000
Round 3 (Audit Bank):     FREE (covered by SDF)
└─ Correlation:      +28% avg total funding

Round 4 (Liquidity):      $100,000
└─ For DeFi protocols with completed audit

Round 5 (Growth Hack):    $20K + up to $200K performance
└─ User acquisition campaign

TOTAL POTENTIAL: $470K+ over 12-18 months"#}
                        </div>
                    </section>

                    // Category Strategies Section
                    <section id="applications" class="guide_section">
                        <h2>{"APPLICATIONS STRATEGY"}</h2>

                        <div class="info_box">
                            <h3>{"Category Stats"}</h3>
                            <ul>
                                <li><strong>{"Total Projects:"}</strong>{" 178 (40.2% of ecosystem)"}</li>
                                <li><strong>{"Average Funding:"}</strong>{" $93,899"}</li>
                                <li><strong>{"Median Funding:"}</strong>{" $75,000"}</li>
                                <li><strong>{"Top Funded:"}</strong>{" Beans App ($490,160), Litemint ($376,834), Alfred ($335,000)"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"What SCF Wants (From Official Handbook)"}</h3>
                            <ul>
                                <li>{" Targeted new-to-crypto audiences with simplified UX"}</li>
                                <li>{" Apps hiding blockchain complexity for non-native users"}</li>
                                <li>{" Multichain wallets integrating Stellar DeFi (Blend, Aquarius, Soroswap)"}</li>
                                <li>{" Projects targeting APAC for cross-border payments"}</li>
                                <li>{" Apps enhancing transparency in International Aid delivery"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Underserved Niches (High Opportunity)"}</h3>
                            <ul>
                                <li><strong>{"Subscription Payments:"}</strong>{" 0 projects, $120K-$340K potential"}</li>
                                <li><strong>{"Enterprise Expense Management:"}</strong>{" 0 projects"}</li>
                                <li><strong>{"APAC Cross-Border:"}</strong>{" Explicitly prioritized in handbook"}</li>
                                <li><strong>{"International Aid:"}</strong>{" Only 3 projects"}</li>
                            </ul>
                        </div>
                    </section>

                    <section id="financial" class="guide_section">
                        <h2>{"FINANCIAL PROTOCOLS STRATEGY"}</h2>

                        <div class="info_box info_box_gradient">
                            <h3>{"HIGHEST FUNDING CATEGORY"}</h3>
                            <ul>
                                <li><strong>{"Average Funding:"}</strong>{" $130,893 (HIGHEST)"}</li>
                                <li><strong>{"Median Funding:"}</strong>{" $125,000 (HIGHEST)"}</li>
                                <li><strong>{"Total Distributed:"}</strong>{" $8.9M to 69 projects"}</li>
                                <li><strong>{"Top 3:"}</strong>{" Anclap ($572k), Phoenix ($395k), Soroswap ($347k)"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"Critical Gaps (Handbook Priorities)"}</h3>
                            <ul>
                                <li><strong>{"Yield Aggregators:"}</strong>{" Only 1 project (Defindex on Testnet)"}
                                    <br/><span class="text_muted">{" Handbook priority - compose with Blend, Aquarius, Soroswap"}</span>
                                </li>
                                <li><strong>{"Synthetic Assets:"}</strong>{" Only 2 projects"}
                                    <br/><span class="text_muted">{"Currencies, commodities, digital assets - explicitly mentioned"}</span>
                                </li>
                                <li><strong>{"Options & Derivatives:"}</strong>{" Only 2 projects"}
                                    <br/><span class="text_muted">{"For protocols/developers, not retail - infrastructure priority"}</span>
                                </li>
                                <li><strong>{"Credit Scoring:"}</strong>{" 0 projects"}
                                    <br/><span class="text_muted">{"Enables DeFi lending, trustless markets"}</span>
                                </li>
                                <li><strong>{"Escrow & Insurance:"}</strong>{" 0 projects"}
                                    <br/><span class="text_muted">{"Decentralized infrastructure for efficient markets"}</span>
                                </li>
                            </ul>
                        </div>

                        <div class="table_wrapper">
                            <h3>{"Project Type Success Rates"}</h3>
                            <table class="data_table">
                                <thead>
                                    <tr>
                                        <th>{"Type"}</th>
                                        <th>{"Projects"}</th>
                                        <th>{"Top-Funded Avg"}</th>
                                        <th>{"Opportunity"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td><strong>{"Anchor"}</strong></td>
                                        <td>{"4"}</td>
                                        <td><strong>{"$332k"}</strong></td>
                                        <td><span class="badge badge-success">{"VERY LOW"}</span></td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"DEX"}</strong></td>
                                        <td>{"18"}</td>
                                        <td>{"$230k"}</td>
                                        <td><span class="badge badge-warning">{"MEDIUM"}</span></td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Lending"}</strong></td>
                                        <td>{"13"}</td>
                                        <td>{"$197k"}</td>
                                        <td><span class="badge badge-warning">{"MEDIUM"}</span></td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Yield Aggregator"}</strong></td>
                                        <td>{"1"}</td>
                                        <td>{"$198k"}</td>
                                        <td><span class="badge badge-success">{"VERY LOW"}</span></td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </section>

                    <section id="infrastructure" class="guide_section">
                        <h2>{"INFRASTRUCTURE & SERVICES STRATEGY"}</h2>

                        <div class="info_box">
                            <h3>{"Category Stats"}</h3>
                            <ul>
                                <li><strong>{"Average Funding:"}</strong>{" $101,757"}</li>
                                <li><strong>{"Median Funding:"}</strong>{" $100,000"}</li>
                                <li><strong>{"Top 3:"}</strong>{" Reflector ($445k), Stellarchain.io ($241k), Chaincerts ($178k)"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"High-Priority Needs (From Handbook)"}</h3>
                            <ul>
                                <li><strong>{"Soroban-native Anchors:"}</strong>{" 0 projects (all 4 anchors are Classic)"}
                                    <br/><span class="text_muted">{"Local currency connectivity, regulatory compliance required"}</span>
                                </li>
                                <li><strong>{"Proof of Reserves:"}</strong>{" Minimal infrastructure"}
                                    <br/><span class="text_muted">{"Transparent custodial models for bridges, stablecoins"}</span>
                                </li>
                                <li><strong>{"Advanced Monitoring:"}</strong>{" Only 1-2 projects"}
                                    <br/><span class="text_muted">{"Alerting, observability for production systems"}</span>
                                </li>
                                <li><strong>{"Oracles:"}</strong>{" Only 6 projects, avg $212k"}
                                    <br/><span class="text_muted">{"Critical for DeFi expansion"}</span>
                                </li>
                            </ul>
                        </div>
                    </section>

                    <section id="developer" class="guide_section">
                        <h2>{"DEVELOPER TOOLING STRATEGY"}</h2>

                        <div class="info_box">
                            <h3>{"Category Stats"}</h3>
                            <ul>
                                <li><strong>{"Total Projects:"}</strong>{" 102 (23.0% of ecosystem)"}</li>
                                <li><strong>{"Average Funding:"}</strong>{" $79,752"}</li>
                                <li><strong>{"Top 3:"}</strong>{" AnChain.AI ($435k), Scout ($240k), Stellar PHP SDK ($225k)"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Critical Gaps ( Priorities)"}</h3>
                            <ul>
                                <li><strong>{"Gas Optimization & Profiling:"}</strong>{" 0 projects"}
                                    <br/><span class="text_muted">{"Every contract needs this - developer experience priority"}</span>
                                </li>
                                <li><strong>{"Reusable Contract Building Blocks:"}</strong>{" Only 1 (Otter Contracts)"}
                                    <br/><span class="text_muted">{" Handbook priority - DeFi primitives, templates"}</span>
                                </li>
                                <li><strong>{"Formal Verification:"}</strong>{" Only 1 project"}
                                    <br/><span class="text_muted">{"Mathematical proof systems for security"}</span>
                                </li>
                                <li><strong>{"Advanced Debugging/Profiling:"}</strong>{" 1-2 projects"}
                                    <br/><span class="text_muted">{"Production-grade tooling gap"}</span>
                                </li>
                            </ul>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"Success Pattern: Security Tools"}</h3>
                            <ul>
                                <li>{" Only 8 projects but $273k average when top-funded"}</li>
                                <li>{" Growing demand with Soroban smart contract adoption"}</li>
                                <li>{" Examples: AnChain.AI ($435k), Scout ($240k)"}</li>
                            </ul>
                        </div>
                    </section>

                    // Kickstart Program Section
                    <section id="kickstart" class="guide_section">
                        <h2>{"SCF KICKSTART PROGRAM"}</h2>

                        <div class="info_box info_box_gradient">
                            <h3>{"5-Day Intensive Bootcamp → Up to $15,000 Award"}</h3>
                            <p>{"For first-time applicants: Get hands-on mentorship before applying to Build Award. Developed by BIGGER (90+ person blockchain dev company) and SDF."}</p>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"Program Structure"}</h3>
                            <ul>
                                <li><strong>{"Award:"}</strong>{" Up to $15,000 in XLM for MVP development"}</li>
                                <li><strong>{"Duration:"}</strong>{" 5 days intensive + 1-1.5 months build time"}</li>
                                <li><strong>{"Support:"}</strong>{" Dedicated Slack channel with Business, UX/UI, and Engineering experts"}</li>
                                <li><strong>{"Outcome:"}</strong>{" Direct path to SCF Build Award application"}</li>
                            </ul>
                        </div>

                        <div class="table_wrapper">
                            <h3>{"5-Day Bootcamp Schedule"}</h3>
                            <table class="data_table">
                                <thead>
                                    <tr>
                                        <th>{"Day"}</th>
                                        <th>{"Focus"}</th>
                                        <th>{"Deliverable"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td><strong>{"Day 1"}</strong></td>
                                        <td>{"Define MVP"}</td>
                                        <td>{"Problem validation & Minimum Viable Product definition"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Day 2"}</strong></td>
                                        <td>{"UI/UX Design"}</td>
                                        <td>{"User interface mockups and experience flows"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Day 3"}</strong></td>
                                        <td>{"Technical Architecture"}</td>
                                        <td>{"Draft technical architecture with Stellar integration"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Day 4"}</strong></td>
                                        <td>{"Go-to-Market"}</td>
                                        <td>{"Comprehensive GTM strategy for launch and adoption"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Day 5"}</strong></td>
                                        <td>{"Pitch & Feedback"}</td>
                                        <td>{"Present to industry experts panel, gain feedback"}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Who Should Apply to Kickstart?"}</h3>
                            <ul>
                                <li>{" First-time applicants to SCF"}</li>
                                <li>{" Teams with an idea but need help refining it"}</li>
                                <li>{" Projects wanting expert validation before major Build Award application"}</li>
                                <li>{" Teams needing structured guidance on Stellar integration"}</li>
                            </ul>
                        </div>

                        <div class="info_box">
                            <h3>{"Requirements"}</h3>
                            <ul>
                                <li>{" At least 2 team members available for all 5 bootcamp days"}</li>
                                <li>{" Complete KYC (Know Your Customer) check before bootcamp"}</li>
                                <li>{" Active participation in workshops and activities"}</li>
                                <li>{" Present during investor demo day if selected as finalist"}</li>
                            </ul>
                        </div>
                    </section>

                    // SCF Build Section
                    <section id="scf-build" class="guide_section">
                        <h2>{"SCF BUILD PROGRAM"}</h2>

                        <div class="info_box info_box_gradient">
                            <h3>{"The Main Funding Program (99.8% of all awards)"}</h3>
                            <p>{"Up to $150,000 in XLM distributed in 3 tranches based on milestone completion"}</p>
                        </div>

                        <div class="table_wrapper">
                            <h3>{"Payment Structure (Critical: Understand This!)"}</h3>
                            <table class="data_table">
                                <thead>
                                    <tr>
                                        <th>{"Tranche"}</th>
                                        <th>{"Payment"}</th>
                                        <th>{"Milestone"}</th>
                                        <th>{"What You Get"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td><strong>{"Tranche 1"}</strong></td>
                                        <td>{"⅓ budget"}</td>
                                        <td>{"Initial approval"}</td>
                                        <td>{"Build MVP (weeks 1-12)"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Tranche 2"}</strong></td>
                                        <td><strong>{"UNPAID"}</strong></td>
                                        <td>{"Testnet launch"}</td>
                                        <td>{"Access to Stellar LaunchKit (RPC credits, Audit Bank eligibility)"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Tranche 3"}</strong></td>
                                        <td>{"⅓ budget"}</td>
                                        <td>{"Mainnet launch"}</td>
                                        <td>{"Final payment + growth opportunities"}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"Core Eligibility Criteria (Checklist)"}</h3>
                            <ul>
                                <li>{" Demonstrated product-market fit (significant traction OR validated need by experienced team)"}</li>
                                <li>{" High-quality, technically detailed submission"}</li>
                                <li>{" Clear Stellar/Soroban integration explanation (use case + technical details)"}</li>
                                <li>{" Ready to start building IMMEDIATELY (dedicated devs with relevant experience)"}</li>
                                <li>{" Stellar significantly improves core features (explain WHY Stellar)"}</li>
                                <li>{" Smart contracts must be open-sourced (if using Soroban)"}</li>
                                <li>{" Budget covers ONLY development costs (no marketing, no audit - Audit Bank covers that)"}</li>
                                <li>{" Clear tranches: MVP → Testnet (unpaid) → Mainnet with feasible deliverables"}</li>
                                <li>{" Unique and competitive project with clear value to Stellar ecosystem"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Budget Guidelines (What's Allowed)"}</h3>
                            <ul>
                                <li><strong>{" Direct Development Costs:"}</strong>{" Engineering salaries, smart contract development"}</li>
                                <li><strong>{" Infrastructure:"}</strong>{" RPC services, cloud hosting, testing environments"}</li>
                                <li><strong>{" Design & UX:"}</strong>{" UI/UX design, prototyping"}</li>
                                <li><strong>{" Operations:"}</strong>{" Legal/compliance (KYC, entity setup), project management tools"}</li>
                            </ul>
                            <h4>{" NOT Allowed in Budget:"}</h4>
                            <ul>
                                <li>{" Security audits (covered by Audit Bank)"}</li>
                                <li>{" Marketing & user acquisition (use Growth Hack program instead)"}</li>
                                <li>{" Costs unrelated to Stellar development"}</li>
                            </ul>
                        </div>

                        <div class="info_box">
                            <h3>{"Timeline Expectations"}</h3>
                            <ul>
                                <li>{"• Ideal scope: 6 months maximum"}</li>
                                <li>{"• Technical architecture should exist BEFORE submission (not part of deliverables)"}</li>
                                <li>{"• Testnet tranche is UNPAID but unlocks critical resources"}</li>
                                <li>{"• Spacing between tranches: typically 4-8 weeks"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"Resubmission Rules"}</h3>
                            <ul>
                                <li>{"• Can resubmit if significantly improved based on feedback"}</li>
                                <li>{"• After 3 unsuccessful submissions: 3-round timeout"}</li>
                                <li>{"• Each additional failure after that: another 3-round timeout"}</li>
                                <li>{"• Previous winners can resubmit NEW projects (must show significant progress on prior work)"}</li>
                            </ul>
                        </div>

                        <div class="code_block">
{r#"Pre-Screen Evaluation Criteria (How Reviewers Judge):

 Eligibility Requirements
  └─ Meets all participant and category-specific requirements

 Product-Market Fit
  └─ Significant user traction OR validated need identified by
     experienced Stellar team

 Submission Quality
  └─ High quality, clear language, rich technical details

 Stellar Integration
  └─ Clear use case + technical architecture + WHY Stellar

 Team Readiness
  └─ Can start building IMMEDIATELY with experienced devs

 Project Innovation
  └─ Using Stellar to significantly improve core features

 Smart Contract Plan
  └─ If using Soroban: open-source plan clearly explained

 Budget Justification
  └─ Reasonable for scope, stage, and Stellar ecosystem impact
  └─ Covers ONLY development (not marketing or audits)

 Milestones
  └─ Clear: MVP → Testnet (unpaid) → Mainnet
  └─ Feasible deliverables with sufficient detail

 Uniqueness & Value
  └─ Competitive analysis showing clear ecosystem value"#}
                        </div>
                    </section>

                    // Add remaining sections with same pattern...
                    <section id="audit-bank" class="guide_section">
                        <h2>{"AUDIT BANK PROGRAM"}</h2>

                        <div class="info_box info_box_gradient">
                            <h3>{"KEY INSIGHT: +28% FUNDING CORRELATION"}</h3>
                            <p>{"Projects that participate in Audit Bank receive 28% more funding on average ($241,799 vs $188,899)"}</p>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"Program Structure"}</h3>
                            <ul>
                                <li><strong>{"Coverage:"}</strong>{" Up to 100% of audit cost"}</li>
                                <li><strong>{"Eligibility:"}</strong>{" SCF-funded projects building Soroban smart contracts"}</li>
                                <li><strong>{"Timing:"}</strong>{" After testnet launch, before/during mainnet"}</li>
                                <li><strong>{"Co-payment:"}</strong>{" 5% upfront (refunded if you fix critical/high/medium issues in 20 days)"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Approved Audit Firms (10 firms)"}</h3>
                            <ul>
                                <li><strong>{"Certora"}</strong>{" - Formal verification with mathematical reasoning"}</li>
                                <li><strong>{"Code4rena"}</strong>{" - Competitive audits with 100+ researchers"}</li>
                                <li><strong>{"ChainSecurity"}</strong>{" - Leading firm for complex Web3 infrastructures"}</li>
                                <li><strong>{"Halborn"}</strong>{" - World-class assessments for Web3 and Fortune 500"}</li>
                                <li><strong>{"Oak Security"}</strong>{" - 600+ audits, zero exploits, 'blinded' process"}</li>
                                <li><strong>{"OtterSec"}</strong>{" - Secured $36B+ TVL across 120+ protocols"}</li>
                                <li><strong>{"Runtime Verification"}</strong>{" - Formal methods for blockchain safety"}</li>
                                <li><strong>{"Spearbit + Cantina"}</strong>{" - World-class researcher network"}</li>
                                <li><strong>{"Veridise"}</strong>{" - Rigorous audits with advanced tooling"}</li>
                                <li><strong>{"Zellic"}</strong>{" - Leading blockchain & cryptography specialists"}</li>
                            </ul>
                        </div>

                        <div class="table_wrapper">
                            <h3>{"Co-Payment System by Stage"}</h3>
                            <table class="data_table">
                                <thead>
                                    <tr>
                                        <th>{"Audit Stage"}</th>
                                        <th>{"Traction Threshold"}</th>
                                        <th>{"Co-Payment"}</th>
                                        <th>{"Notes"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td><strong>{"Initial Audit"}</strong></td>
                                        <td>{"None (priority categories)"}</td>
                                        <td>{"5% (refundable*)"}</td>
                                        <td>{"*Refunded if critical/high/medium fixed in 20 days"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Growth Audit"}</strong></td>
                                        <td>{">$10M TVL"}</td>
                                        <td>{"0%"}</td>
                                        <td>{"May include formal verification"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Scale Audit"}</strong></td>
                                        <td>{">$100M TVL"}</td>
                                        <td>{"0%"}</td>
                                        <td>{"High-value projects nearing maturity"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Pre-Traction Follow-Up"}</strong></td>
                                        <td>{"N/A"}</td>
                                        <td>{"20% (1st), 50% (2nd)"}</td>
                                        <td>{"Before achieving Growth/Scale traction"}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>

                        <div class="info_box">
                            <h3>{"Priority Categories (Automatic Eligibility)"}</h3>
                            <ul>
                                <li><strong>{"Financial Protocols"}</strong>{" - Managing on-chain value (prime targets for attacks)"}</li>
                                <li><strong>{"Widely Used Applications"}</strong>{" - Using smart contracts, $1M+ TVL or 100K+ users expected"}</li>
                                <li><strong>{"Infrastructure Contracts"}</strong>{" - Oracles, vaults, account abstraction (widely integrated)"}</li>
                                <li><strong>{"Yield-Bearing Tokens"}</strong>{" - Representing real-world value through smart contracts"}</li>
                            </ul>
                        </div>

                        <div class="code_block">
{r#"Audit Bank Application Timeline:

Phase 1: Intake & Eligibility (immediate)
└─ Submit audit request form (provided at testnet stage)

Phase 2: Readiness Review (< 1 week)
├─ Eligibility assessment
├─ Threat modeling review
├─ Documentation completeness check
└─ Codebase maturity assessment

Phase 3: Audit Scheduling (1 week)
├─ Multiple quotes from pre-approved firms
└─ Scheduling based on scope, impact, readiness

Phase 4: Pre-Audit Preparation (2-3 weeks)
├─ Automated testing with security tools
├─ Internal code reviews
└─ Initial vulnerability remediation

Phase 5: Audit Execution (1-6 weeks)
├─ 5% co-payment to SDF (prior to audit start)
├─ Comprehensive security audit
└─ Manual reviews + automated tooling

Phase 6: Post-Audit Resolution (1-4 weeks)
├─ Fix critical/high/medium issues
└─ Fix within 20 days → 5% refunded

Phase 7: Verification (2-3 weeks)
├─ Audit firm verifies fixes
└─ Additional audits for high-value projects

Phase 8: Public Disclosure
└─ Final report published by audit firm"#}
                        </div>
                    </section>

                    <section id="liquidity" class="guide_section">
                        <h2>{"💧 LIQUIDITY AWARD Program"}</h2>

                        <div class="info_box info_box_primary">
                            <h3>{"Program Overview"}</h3>
                            <p>{"Financial protocols with completed audit and live mainnet implementation can receive up to $100,000 in XLM to bootstrap initial liquidity."}</p>
                        </div>

                        <div class="table_wrapper">
                            <h3>{"Award Structure"}</h3>
                            <table class="data_table">
                                <thead>
                                    <tr>
                                        <th>{"Award Type"}</th>
                                        <th>{"Amount"}</th>
                                        <th>{"Requirements"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td><strong>{"Base Liquidity Award"}</strong></td>
                                        <td>{"$50,000 in XLM"}</td>
                                        <td>{"Completed audit + Mainnet launch"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Supplemental Award"}</strong></td>
                                        <td>{"Additional $50,000"}</td>
                                        <td>{"7-day consecutive TVL >$250K in recognized assets"}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Eligibility Requirements"}</h3>
                            <ul>
                                <li><strong>{"Business:"}</strong>{" Fully functioning protocol on Stellar mainnet with viable business model"}</li>
                                <li><strong>{"Technical:"}</strong>{" Valid security audit from SDF-recognized firm, all vulnerabilities resolved"}</li>
                                <li><strong>{"Legal:"}</strong>{" Passed SDF's KYC, Risk Assessment, Due Diligence checks"}</li>
                                <li><strong>{"Supplemental:"}</strong>{" Attracted liquidity showing 7-day TVL >$250K"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"Reviewer Evaluation Criteria"}</h3>
                            <ul>
                                <li><strong>{"Functionality:"}</strong>{" Does the protocol demonstrate working functionality?"}</li>
                                <li><strong>{"Roadmap:"}</strong>{" Detailed plan with clear milestones and metrics?"}</li>
                                <li><strong>{"Team Experience:"}</strong>{" Experienced in DeFi? Successful projects?"}</li>
                                <li><strong>{"Business Experience:"}</strong>{" Relevant startup/business background?"}</li>
                                <li><strong>{"Community Engagement:"}</strong>{" Strong community engagement strategy?"}</li>
                            </ul>
                        </div>
                    </section>

                    <section id="public-goods" class="guide_section">
                        <h2>{"🌐 PUBLIC GOODS AWARD Program"}</h2>

                        <div class="info_box info_box_primary">
                            <h3>{"Program Structure"}</h3>
                            <ul>
                                <li><strong>{"Award:"}</strong>{" Up to $50,000 in XLM per quarter"}</li>
                                <li><strong>{"Payment:"}</strong>{" 50% upfront, 50% after deliverables review"}</li>
                                <li><strong>{"Frequency:"}</strong>{" Quarterly (every 3 months)"}</li>
                                <li><strong>{"Governance:"}</strong>{" Neural Quorum Governance (NQG) voting by SCF Pilots"}</li>
                            </ul>
                        </div>

                        <div class="table_wrapper">
                            <h3>{"Eligible Categories"}</h3>
                            <table class="data_table">
                                <thead>
                                    <tr>
                                        <th>{"Category"}</th>
                                        <th>{"Purpose"}</th>
                                        <th>{"Examples"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td><strong>{"SDKs"}</strong></td>
                                        <td>{"Maintain SDKs in popular languages"}</td>
                                        <td>{"Python Stellar SDK"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Data Support"}</strong></td>
                                        <td>{"Dashboards, indexers, analytics APIs"}</td>
                                        <td>{"Dune Dashboards"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Wallet Support"}</strong></td>
                                        <td>{"Shared tooling and starter kits"}</td>
                                        <td>{"Stellar Wallets Kit"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Developer Experience"}</strong></td>
                                        <td>{"Productivity tools for developers"}</td>
                                        <td>{"Solang Compiler"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Ecosystem Visibility"}</strong></td>
                                        <td>{"Surface projects, activity, funding"}</td>
                                        <td>{"Stellarlight.xyz"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Infrastructure Monitoring"}</strong></td>
                                        <td>{"Validators, node status, uptime"}</td>
                                        <td>{"Stellarbeat"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Governance Tools"}</strong></td>
                                        <td>{"Voting interfaces and analytics"}</td>
                                        <td>{"Soroban Governor"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Security & Auditing"}</strong></td>
                                        <td>{"Vulnerability identification tools"}</td>
                                        <td>{"Fuzzing Framework"}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Key Eligibility Requirements"}</h3>
                            <ul>
                                <li>{" NO existing/planned revenue model making it self-sustaining"}</li>
                                <li>{" NO currently active SDF grants or outstanding SCF Build tranches"}</li>
                                <li>{" Must demonstrate usage/validation by ecosystem teams"}</li>
                                <li>{" Submitter must be SCF Pilot or higher (proven governance participation)"}</li>
                                <li>{" Must be direct maintainer/steward of the public good"}</li>
                                <li>{" SMART deliverables (Specific, Measurable, Achievable, Relevant, Time-bound)"}</li>
                            </ul>
                        </div>
                    </section>

                    <section id="growth-hack" class="guide_section">
                        <h2>{"GROWTH HACK PROGRAM"}</h2>

                        <div class="info_box info_box_gradient">
                            <h3>{"Competition-Style GTM/PMF Program"}</h3>
                            <p>{"8-week program funding 10-15 mainnet projects with $20K each to run acquisition campaigns. Top performers share up to $200K in performance awards."}</p>
                        </div>

                        <div class="table_wrapper">
                            <h3>{"Award Structure"}</h3>
                            <table class="data_table">
                                <thead>
                                    <tr>
                                        <th>{"Award Type"}</th>
                                        <th>{"Amount"}</th>
                                        <th>{"Purpose"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td><strong>{"Upfront Campaign"}</strong></td>
                                        <td>{"$20,000"}</td>
                                        <td>{"All 10-15 projects get this to run 8-week campaign"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Performance Award (Top 5)"}</strong></td>
                                        <td>{"$50K - $200K total"}</td>
                                        <td>{"Split among top 5 based on WAA & txn volume growth"}</td>
                                    </tr>
                                    <tr>
                                        <td><strong>{"Total Potential"}</strong></td>
                                        <td>{"$70K - $220K"}</td>
                                        <td>{"Per project (if you're top performer)"}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>

                        <div class="info_box info_box_primary">
                            <h3>{"Performance Award Levels (Dynamic)"}</h3>
                            <ul>
                                <li><strong>{"Level 1:"}</strong>{" <1.5X baseline WAA → $50,000 total pool"}</li>
                                <li><strong>{"Level 2:"}</strong>{" 1.5X - 3X baseline → $100,000 total pool"}</li>
                                <li><strong>{"Level 3:"}</strong>{" >3X baseline → $200,000 total pool"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Winner Selection Metrics"}</h3>
                            <p><strong>{"Applications:"}</strong></p>
                            <ul>
                                <li>{"Weekly Active Accounts (WAA) Increase"}</li>
                                <li>{"Weekly Transaction Volume Increase"}</li>
                            </ul>
                            <p><strong>{"Financial Protocols:"}</strong></p>
                            <ul>
                                <li>{"Weekly Total Value Locked (TVL) Increase"}</li>
                                <li>{"Weekly Transaction Volume Increase"}</li>
                            </ul>
                        </div>

                        <div class="code_block">
{r#"Growth Hack Timeline (8 weeks + prep):

Weeks 1-2: Resource Mapping
├─ SDF workshops on agencies, contractors, GTM
└─ Decide on external support by Week 2 end

Weeks 3-4: Strategy & Measurement
├─ Draft campaign plan (budget, channels, incentives)
├─ Build Dune dashboard for on-chain metrics
├─ Set up off-chain dashboard
└─ Weekly metric updates during competition

Weeks 5-6: Pre-launch Testing
├─ Run closed beta groups
├─ Collect and apply feedback
└─ Submit final campaign plan to SDF

Weeks 7-10: Campaign (4 weeks)
├─ Execute user acquisition campaign
├─ Track WAA and transaction volume
├─ Weekly 15-min check-ins with SDF
└─ Document activities and conversion rates

Week 11: Evaluation
├─ SDF reviews results
├─ Announces performance award pool size
├─ Names top 5 winners
└─ First 50% of performance award distributed

Weeks 12-15: Retention (4 weeks)
├─ Sustain growth with funding
├─ Maintain metrics dashboards
├─ Bi-weekly reporting to SDF
└─ If metrics stay within 30% of campaign levels:
    Final 50% of performance award paid"#}
                        </div>

                        <div class="info_box">
                            <h3>{"Allowed Uses of Funds"}</h3>
                            <ul>
                                <li>{" Onboarding incentives (sign-up bonuses, fee discounts)"}</li>
                                <li>{" Digital advertising (Twitter, Discord, Telegram, LinkedIn)"}</li>
                                <li>{" Community engagement (contests, AMAs, referral programs)"}</li>
                                <li>{" Influencer partnerships"}</li>
                                <li>{" Content marketing (blogs, tutorials, videos)"}</li>
                                <li>{" Quest platforms (task completion rewards)"}</li>
                                <li>{" Airdrops (for significant contributors)"}</li>
                                <li>{" Analytics & optimization tools"}</li>
                                <li>{" Contractors and agencies"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Restricted Uses (NEVER ALLOWED)"}</h3>
                            <ul>
                                <li>{" Bot traffic or fake engagement"}</li>
                                <li>{" Wash trading or manipulative transactions"}</li>
                                <li>{" Misleading advertising"}</li>
                                <li>{" Personal expenses or unrelated activities"}</li>
                                <li>{" Excluding US users from DeFi apps"}</li>
                                <li>{" Using XLM to provide additional yields"}</li>
                            </ul>
                        </div>
                    </section>

                    <section id="case-studies" class="guide_section">
                        <h2>{"📚 CASE STUDIES: Top-Funded Success Stories"}</h2>

                        <div class="case_study">
                            <h3>{"Case Study 1: Anclap - $572,162 (HIGHEST FUNDED PROJECT)"}</h3>
                            <div class="case_meta">
                                <span><strong>{"Type:"}</strong>{" Anchor (Financial Protocols)"}</span>
                                <span><strong>{"Country:"}</strong>{" Argentina"}</span>
                                <span><strong>{"Rounds:"}</strong>{" 3 (SCF #7, #17, #26)"}</span>
                                <span><strong>{"Timeline:"}</strong>{" Q1 '21 - Q2 '24"}</span>
                                <span><strong>{"Status:"}</strong>{" Mainnet"}</span>
                            </div>
                            <h4>{"What They Did Right:"}</h4>
                            <ul>
                                <li>{" Chose highest-value category (Anchors avg $332k)"}</li>
                                <li>{" Web3-to-financial-systems bridge (critical infrastructure)"}</li>
                                <li>{" FIAT-Crypto interoperability (real-world need)"}</li>
                                <li>{" LatAm focus (underserved, high-growth market)"}</li>
                                <li>{" Partially open-source (70% of top-funded have Github)"}</li>
                                <li>{" 3 rounds showing consistent progress"}</li>
                            </ul>
                        </div>

                        <div class="case_study">
                            <h3>{"Case Study 2: Phoenix - $394,500 (DeFi Protocol)"}</h3>
                            <div class="case_meta">
                                <span><strong>{"Type:"}</strong>{" DEX (Financial Protocols)"}</span>
                                <span><strong>{"Country:"}</strong>{" Germany"}</span>
                                <span><strong>{"Rounds:"}</strong>{" 4 (SCF #16, #20, #25 + Liquidity Award)"}</span>
                                <span><strong>{"Programs:"}</strong>{" Build + Audit Bank"}</span>
                                <span><strong>{"Status:"}</strong>{" Mainnet"}</span>
                            </div>
                            <h4>{"Success Factors:"}</h4>
                            <ul>
                                <li>{" Soroban-native (67.6% of top-funded)"}</li>
                                <li>{" Fully open-source (success correlation)"}</li>
                                <li>{" Full DeFi protocol suite (AMM + more)"}</li>
                                <li>{" Audit Bank participant (+28% funding correlation)"}</li>
                                <li>{" Geographic sweet spot (Germany $285k avg)"}</li>
                                <li>{" 4 rounds = 5.0x multiplier effect"}</li>
                            </ul>
                        </div>

                        <div class="case_study">
                            <h3>{"Case Study 3: Reflector - $444,840 (Infrastructure)"}</h3>
                            <div class="case_meta">
                                <span><strong>{"Type:"}</strong>{" Oracle (Infrastructure)"}</span>
                                <span><strong>{"Country:"}</strong>{" Portugal"}</span>
                                <span><strong>{"Rounds:"}</strong>{" 4 (SCF #15, #20, #26, #29)"}</span>
                                <span><strong>{"Programs:"}</strong>{" Build + Audit Bank"}</span>
                            </div>
                            <h4>{"What Made It Work:"}</h4>
                            <ul>
                                <li>{" Critical infrastructure (oracles avg $212k)"}</li>
                                <li>{" Soroban-native from the start"}</li>
                                <li>{" Fully open-source"}</li>
                                <li>{" Real traction (protects $14M+ TVL)"}</li>
                                <li>{" THE oracle for Stellar (market leader)"}</li>
                                <li>{" Portugal base ($297k avg for top-funded)"}</li>
                                <li>{" Audit Bank participation"}</li>
                            </ul>
                        </div>

                        <div class="case_study">
                            <h3>{"Case Study 4: AnChain.AI - $435,000 (Security Tools)"}</h3>
                            <div class="case_meta">
                                <span><strong>{"Type:"}</strong>{" Security Tool (Developer Tooling)"}</span>
                                <span><strong>{"Country:"}</strong>{" United States"}</span>
                                <span><strong>{"Rounds:"}</strong>{" 4 (SCF #11, #15, #20, #23)"}</span>
                            </div>
                            <h4>{"Key Success Drivers:"}</h4>
                            <ul>
                                <li>{" Security tools highly valued ($273k avg)"}</li>
                                <li>{" Open-source catalog of audited contracts"}</li>
                                <li>{" Soroban-focused (growing demand)"}</li>
                                <li>{" Critical infrastructure for ecosystem security"}</li>
                                <li>{" 4 rounds showing sustained value delivery"}</li>
                            </ul>
                        </div>

                        <div class="case_study">
                            <h3>{"Case Study 5: Beans App - $490,160 (Consumer Application)"}</h3>
                            <div class="case_meta">
                                <span><strong>{"Type:"}</strong>{" Non-Custodial Wallet (Applications)"}</span>
                                <span><strong>{"Country:"}</strong>{" Netherlands"}</span>
                                <span><strong>{"Rounds:"}</strong>{" 4 (SCF #10, #15, #21, #29)"}</span>
                                <span><strong>{"Timeline:"}</strong>{" Q2 '22 - Q3 '24"}</span>
                            </div>
                            <h4>{"Application Category Winner Strategy:"}</h4>
                            <ul>
                                <li>{" Non-custodial payments for general public"}</li>
                                <li>{" Abstracts blockchain complexity (UX focus)"}</li>
                                <li>{" Partially open-source"}</li>
                                <li>{" Netherlands base ($263k avg top-funded)"}</li>
                                <li>{" 4 rounds = 5.0x multiplier ($68k → $340k avg)"}</li>
                                <li>{" Clear target market and use case validation"}</li>
                            </ul>
                        </div>
                    </section>

                    <section id="checklists" class="guide_section">
                        <h2>{" APPLICATION CHECKLISTS"}</h2>

                        <div class="info_box info_box_primary">
                            <h3>{"Pre-Application Checklist (Do This BEFORE You Apply)"}</h3>
                            <ul>
                                <li>{" Chosen category with <15 competitors OR high avg funding (>$150k)"}</li>
                                <li>{" Validated use case through: 10+ user interviews OR existing traction OR team expertise"}</li>
                                <li>{" Technical architecture document completed (required for Build Award)"}</li>
                                <li>{" Website live (100% of top-funded have one)"}</li>
                                <li>{" Github repository created (even if private/partial - 70% of top-funded have public repos)"}</li>
                                <li>{" Team identified with relevant experience (no anonymous teams)"}</li>
                                <li>{" Stellar integration plan clearly defined (not just 'we'll use Stellar')"}</li>
                                <li>{" Competitive analysis completed (know your differentiation)"}</li>
                                <li>{" KYC documentation ready (US regulations require this)"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Strong Application Must Include:"}</h3>
                            <ul>
                                <li>{" Clear problem statement and validated market need"}</li>
                                <li>{" Technical architecture showing Stellar/Soroban integration"}</li>
                                <li>{" Team background with relevant crypto/fintech experience"}</li>
                                <li>{" Detailed milestones with measurable, verifiable outcomes"}</li>
                                <li>{" Website and strong online presence"}</li>
                                <li>{" Github repository (even if partial/in-progress)"}</li>
                                <li>{" Clear differentiation from existing projects"}</li>
                                <li>{" Realistic budget breakdown and 6-month timeline"}</li>
                                <li>{" Long-term sustainability plan (how will you fund yourself post-grant?)"}</li>
                                <li>{" Evidence of execution ability (previous work, early traction, etc.)"}</li>
                            </ul>
                        </div>

                        <div class="info_box">
                            <h3>{"Red Flags to AVOID:"}</h3>
                            <ul>
                                <li>{" Vague technical descriptions ('We'll use blockchain')"}</li>
                                <li>{" Unrealistic timelines or budgets"}</li>
                                <li>{" No clear Stellar-specific value proposition"}</li>
                                <li>{" Missing team information or anonymous founders"}</li>
                                <li>{" Copy-paste content from other chains"}</li>
                                <li>{" No Github/website presence"}</li>
                                <li>{" Missing clear, measurable milestones"}</li>
                                <li>{" Generic 'me-too' projects without differentiation"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_gradient">
                            <h3>{"Audit Bank Readiness Checklist"}</h3>
                            <p><strong>{"Required Before Applying:"}</strong></p>
                            <ul>
                                <li>{" Code repository well-organized and documented"}</li>
                                <li>{" Integration tests completed and passing"}</li>
                                <li>{" STRIDE threat model completed with data flow diagram"}</li>
                                <li>{" Data flow diagram shows trust boundaries and entities"}</li>
                                <li>{" Deployed on Soroban testnet for validation"}</li>
                                <li>{" Nearly mainnet-ready (audit needed within 4-6 weeks)"}</li>
                            </ul>
                            <p><strong>{"Bonus (Improves Your Application):"}</strong></p>
                            <ul>
                                <li>{" Tooling scan report from ecosystem scanning tools"}</li>
                                <li>{" Remediation plan for identified vulnerabilities"}</li>
                            </ul>
                        </div>
                    </section>

                    <section id="templates" class="guide_section">
                        <h2>{"📋 APPLICATION TEMPLATES"}</h2>

                        <div class="code_block">
{r#"TEMPLATE: Elevator Pitch (30 seconds)

"We're building [PROJECT NAME], a [CATEGORY] that [CORE VALUE PROP].

Unlike [MAIN COMPETITOR], we [KEY DIFFERENTIATION].

We solve [SPECIFIC PROBLEM] for [TARGET AUDIENCE] by [SOLUTION].

Our team has [RELEVANT EXPERIENCE]. We've already [TRACTION/VALIDATION].

We're seeking $[AMOUNT] to [SPECIFIC MILESTONE] over [TIMEFRAME]."

Example (Yield Aggregator):
"We're building StellarYield, a yield aggregator that automatically
compounds returns across Blend, Aquarius, and Soroswap.

Unlike Defindex (the only competitor on Testnet), we use AI-optimized
strategies and support cross-protocol composability.

We solve the problem of manual yield farming for DeFi users by
automatically rebalancing across 10+ protocols based on APY.

Our team built a $50M TVL yield optimizer on Ethereum. We have 200
beta users generating $500K in test transactions.

We're seeking $150K to launch on Mainnet with full audit over 6 months.""#}
                        </div>

                        <div class="code_block">
{r#"TEMPLATE: Technical Architecture (Required for Build Award)

1. System Overview
   - High-level architecture diagram
   - Core components and their interactions
   - Technology stack

2. Stellar Integration
   - Which Stellar features you're using (Soroban contracts? Classic ops?)
   - Why Stellar is essential (not just 'nice to have')
   - Technical specifics of integration

3. Smart Contract Architecture (if applicable)
   - Contract structure and relationships
   - Key functions and access controls
   - State management approach
   - Funds flow diagram

4. Infrastructure
   - RPC providers (public? private?)
   - Data indexing approach
   - Frontend architecture
   - Wallet integration strategy

5. Security Considerations
   - Threat model overview
   - Initial security measures
   - Audit preparation plan

Example Statement:
"Our protocol uses 3 Soroban smart contracts: VaultManager (handles
deposits), StrategyExecutor (rebalancing logic), and FeeCollector
(revenue). The VaultManager interfaces with Blend's lending pools
and Soroswap's AMM using the token interface standard. All funds flow
through time-locked withdrawals with emergency pause functionality.""#}
                        </div>

                        <div class="code_block">
{r#"TEMPLATE: Milestone Structure (Build Award)

Tranche 1: MVP Development (⅓ budget)
├─ Deliverable 1.1: Core smart contracts deployed on testnet
│   Success Criteria: All 3 contracts deployed, unit tests passing
│   Timeline: Week 1-4
│
├─ Deliverable 1.2: Basic frontend with wallet connection
│   Success Criteria: Users can connect Freighter, view balances
│   Timeline: Week 5-8
│
└─ Deliverable 1.3: Integration with 1 protocol (e.g., Blend)
    Success Criteria: Users can deposit/withdraw from Blend via our UI
    Timeline: Week 9-12

Tranche 2: Testnet Launch (Unpaid - unlocks LaunchKit)
├─ Deliverable 2.1: Full testnet deployment
│   Success Criteria: All features working on testnet
│   Timeline: Week 13-16
│
├─ Deliverable 2.2: Documentation and testing
│   Success Criteria: User docs, 100 beta testers recruited
│   Timeline: Week 17-20
│
└─ Deliverable 2.3: Audit preparation
    Success Criteria: Security scan complete, STRIDE model done
    Timeline: Week 21-24

Tranche 3: Mainnet Launch (⅓ budget)
├─ Deliverable 3.1: Audit remediation
│   Success Criteria: All critical/high/medium issues fixed
│   Timeline: Post-audit, 2-4 weeks
│
├─ Deliverable 3.2: Mainnet deployment
│   Success Criteria: Contracts deployed, >$100K TVL first week
│   Timeline: Week 25-26
│
└─ Deliverable 3.3: Marketing and growth
    Success Criteria: 500+ users, integration with 2 more protocols
    Timeline: Week 27-30"#}
                        </div>

                        <div class="code_block">
{r#"TEMPLATE: Budget Breakdown (Build Award)

WRONG (Too vague):
"$150,000 for development costs over 6 months"

RIGHT (Detailed and justified):

Total Budget: $150,000 (over 6 months)

Engineering (70% = $105,000):
├─ Lead Smart Contract Developer: $60,000
│   (Senior Rust dev, 6 months @ $10K/month)
├─ Frontend Developer: $30,000
│   (React/Yew specialist, 6 months @ $5K/month)
└─ Backend/Infrastructure: $15,000
    (Part-time, API and indexing)

Design & UX (10% = $15,000):
├─ UI/UX Design: $8,000
└─ Brand & Marketing Materials: $7,000

Infrastructure & Tools (15% = $22,500):
├─ RPC/Node Services: $6,000
├─ Testing & Staging Environments: $4,500
├─ Development Tools & Software: $3,000
├─ Cloud Hosting (testnet/staging): $6,000
└─ Security Scanning Tools: $3,000

Operations & Admin (5% = $7,500):
├─ Legal/Compliance (KYC, entity setup): $3,000
├─ Project Management Tools: $1,500
└─ Contingency Buffer: $3,000

Note: Audit costs NOT included (covered by Audit Bank)
Note: Marketing budget NOT included (focus on product development)"#}
                        </div>
                    </section>

                    <section id="success-formula" class="guide_section">
                        <h2>{"SUCCESS FORMULA: PROVEN PATH TO $470K+"}</h2>

                        <div class="info_box info_box_gradient">
                            <h3>{"The 5-Step Proven Formula"}</h3>
                            <ol>
                                <li><strong>{"Choose Wisely"}</strong>
                                    <br/>{"→ Pick high-value ($150k+), low-competition (<10 projects) category"}
                                    <br/>{"→ Examples: Anchors ($332k avg), Credit Scoring (0 projects), Yield Aggregators (1 project)"}
                                </li>
                                <li><strong>{"Validate Ruthlessly"}</strong>
                                    <br/>{"→ Minimum 10+ user interviews OR existing traction OR proven team expertise"}
                                    <br/>{"→ 100% of top-funded have validated use cases"}
                                </li>
                                <li><strong>{"Apply Strong"}</strong>
                                    <br/>{"→ Complete technical architecture (required)"}
                                    <br/>{"→ Website + Github (70% of top-funded have repos)"}
                                    <br/>{"→ Detailed milestones with measurable outcomes"}
                                </li>
                                <li><strong>{"Execute Flawlessly"}</strong>
                                    <br/>{"→ Hit every milestone on time"}
                                    <br/>{"→ Get to Mainnet (72% of top-funded launched)"}
                                    <br/>{"→ Plan for 2-4 rounds (68.5% of top-funded got multiple rounds)"}
                                </li>
                                <li><strong>{"Stack Programs"}</strong>
                                    <br/>{"→ Round 1-2: Build Awards ($150K each)"}
                                    <br/>{"→ Round 3: Audit Bank (FREE, +28% correlation)"}
                                    <br/>{"→ Round 4: Liquidity Award ($100K for DeFi)"}
                                    <br/>{"→ Round 5: Growth Hack ($20K + up to $200K)"}
                                </li>
                            </ol>
                        </div>

                        <div class="key_metrics">
                            <h3>{"Critical Success Factors (From 443 Projects Analysis)"}</h3>
                            <ul>
                                <li><strong>{"Launch Status:"}</strong>{" 72% of top-funded reached Mainnet (+30% funding premium)"}</li>
                                <li><strong>{"Multiple Rounds:"}</strong>{" 68.5% of top-funded received 2+ rounds (1.8x - 5.0x multiplier)"}</li>
                                <li><strong>{"Technology:"}</strong>{" 67.6% of top-funded are Soroban-based"}</li>
                                <li><strong>{"Strategy:"}</strong>{" 74.8% of top-funded are Stellar-only (+30% funding premium)"}</li>
                                <li><strong>{"Open Source:"}</strong>{" 70.3% of top-funded have public Github repos"}</li>
                                <li><strong>{"Security:"}</strong>{" Audit Bank participation = +28% average total funding"}</li>
                                <li><strong>{"Website:"}</strong>{" 100% of top-funded have websites"}</li>
                            </ul>
                        </div>

                        <div class="info_box info_box_success">
                            <h3>{"Geographic Success Patterns"}</h3>
                            <p>{"Location matters for top-funded projects:"}</p>
                            <ul>
                                <li><strong>{"Europe:"}</strong>{" Portugal ($297K), Germany ($285K), Netherlands ($263K)"}</li>
                                <li><strong>{"Latin America:"}</strong>{" Chile ($248K), Colombia ($219K) - Strong performance"}</li>
                                <li><strong>{"United States:"}</strong>{" $181K average - Solid but not highest"}</li>
                                <li><strong>{"Emerging Markets:"}</strong>{" Well-represented, no geographical disadvantage"}</li>
                            </ul>
                        </div>

                        <div class="code_block">
{r#"EXAMPLE: Path to $470K Over 18 Months

Month 1-6: Build Award Round 1
├─ Apply with strong application
├─ $150K awarded (⅓, ⅓, ⅓ tranches)
├─ Launch MVP → Testnet → Mainnet
└─ Mainnet bonus: +30% future funding potential

Month 7-12: Build Award Round 2
├─ Show clear progress from Round 1
├─ $150K awarded for scaling/features
├─ Apply to Audit Bank (testnet ready)
└─ Audit Bank: FREE audit + 28% correlation

Month 13: Liquidity Award (DeFi projects)
├─ Mainnet + Audit complete
├─ $50K base liquidity award
└─ Hit $250K TVL → $50K supplemental

Month 14-15: Growth Hack Program
├─ $20K upfront for user acquisition
├─ Run 4-week campaign + 4-week retention
├─ Top 5 performance → share $200K pool
└─ Potential: +$40K - $80K performance award

TOTAL POTENTIAL: $470K - $510K
├─ Build Awards: $300K
├─ Audit Bank: FREE (value: $50K-100K)
├─ Liquidity: $100K
└─ Growth Hack: $70K - $110K"#}
                        </div>
                    </section>

                    <section class="guide_section">
                        <h2>{"📞 Resources & Contact"}</h2>
                        <div class="resource_links">
                            <a href="https://dashboard.communityfund.stellar.org" target="_blank" class="resource_link">
                                <i class="fas fa-external-link-alt"></i>
                                {" Application Dashboard"}
                            </a>
                            <a href="https://discord.gg/ShHGRudAGv" target="_blank" class="resource_link">
                                <i class="fab fa-discord"></i>
                                {" Join Discord"}
                            </a>
                            <a href="https://stellarcommunityfund.gitbook.io/scf-handbook" target="_blank" class="resource_link">
                                <i class="fas fa-book"></i>
                                {" Handbook"}
                            </a>
                        </div>
                    </section>
                </div>
            </div>
        </Layout>
    }
}
