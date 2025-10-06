use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_json::Value;

#[derive(Clone, PartialEq)]
enum ActiveTab {
    CompetitiveLandscape,
    GapAnalysis,
    SuccessPatterns,
    FundingCalculator,
    SocialIntelligence,
    TeamInsights,
    GeographicTrends,
    TechStackAnalysis,
}

#[function_component(AnalyticsHub)]
pub fn analytics_hub() -> Html {
    let active_tab = use_state(|| ActiveTab::CompetitiveLandscape);

    let set_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: ActiveTab| {
            active_tab.set(tab);
        })
    };

    html! {
        <div class="intelligence_hub">
            <style>
                {include_str!("../../styles/analytics/analytics_hub.css")}
            </style>

            <div class="hub_header">
                <h1>{"INTELLIGENCE HUB"}</h1>
                <p class="hub_subtitle">{"DATA-DRIVEN INSIGHTS | 443 PROJECTS | $42.1M ANALYZED"}</p>
            </div>

            <div class="hub_tabs">
                <button
                    class={classes!("tab", (*active_tab == ActiveTab::CompetitiveLandscape).then_some("active"))}
                    onclick={set_tab.reform(|_| ActiveTab::CompetitiveLandscape)}
                >
                    {"COMPETITIVE LANDSCAPE"}
                </button>
                <button
                    class={classes!("tab", (*active_tab == ActiveTab::GapAnalysis).then_some("active"))}
                    onclick={set_tab.reform(|_| ActiveTab::GapAnalysis)}
                >
                    {"GAP ANALYSIS"}
                </button>
                <button
                    class={classes!("tab", (*active_tab == ActiveTab::SuccessPatterns).then_some("active"))}
                    onclick={set_tab.reform(|_| ActiveTab::SuccessPatterns)}
                >
                    {"SUCCESS PATTERNS"}
                </button>
                <button
                    class={classes!("tab", (*active_tab == ActiveTab::FundingCalculator).then_some("active"))}
                    onclick={set_tab.reform(|_| ActiveTab::FundingCalculator)}
                >
                    {"FUNDING CALCULATOR"}
                </button>
                <button
                    class={classes!("tab", (*active_tab == ActiveTab::SocialIntelligence).then_some("active"))}
                    onclick={set_tab.reform(|_| ActiveTab::SocialIntelligence)}
                >
                    {"SOCIAL INTELLIGENCE"}
                </button>
                <button
                    class={classes!("tab", (*active_tab == ActiveTab::TeamInsights).then_some("active"))}
                    onclick={set_tab.reform(|_| ActiveTab::TeamInsights)}
                >
                    {"TEAM INSIGHTS"}
                </button>
                <button
                    class={classes!("tab", (*active_tab == ActiveTab::GeographicTrends).then_some("active"))}
                    onclick={set_tab.reform(|_| ActiveTab::GeographicTrends)}
                >
                    {"GEOGRAPHIC TRENDS"}
                </button>
                <button
                    class={classes!("tab", (*active_tab == ActiveTab::TechStackAnalysis).then_some("active"))}
                    onclick={set_tab.reform(|_| ActiveTab::TechStackAnalysis)}
                >
                    {"TECH STACK"}
                </button>
            </div>

            <div class="hub_content">
                {match *active_tab {
                    ActiveTab::CompetitiveLandscape => html! { <CompetitiveLandscape /> },
                    ActiveTab::GapAnalysis => html! { <GapAnalysis /> },
                    ActiveTab::SuccessPatterns => html! { <SuccessPatterns /> },
                    ActiveTab::FundingCalculator => html! { <FundingCalculator /> },
                    ActiveTab::SocialIntelligence => html! { <SocialIntelligence /> },
                    ActiveTab::TeamInsights => html! { <TeamInsights /> },
                    ActiveTab::GeographicTrends => html! { <GeographicTrends /> },
                    ActiveTab::TechStackAnalysis => html! { <TechStackAnalysis /> },
                }}
            </div>
        </div>
    }
}

// COMPETITIVE LANDSCAPE EXPLORER
#[function_component(CompetitiveLandscape)]
fn competitive_landscape() -> Html {
    let projects = use_state(|| Vec::<Value>::new());
    let category_filter = use_state(|| "All".to_string());
    let min_funding = use_state(|| 0.0);
    let integration_filter = use_state(|| "All".to_string());
    let soroban_only = use_state(|| false);

    {
        let projects = projects.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(response) = gloo_net::http::Request::get("/api/enriched/projects-with-regions")
                    .send()
                    .await
                {
                    if let Ok(data) = response.json::<Value>().await {
                        if let Some(project_list) = data.get("projects").and_then(|p| p.as_array()) {
                            projects.set(project_list.clone());
                        }
                    }
                }
            });
            || ()
        });
    }

    let filtered_projects: Vec<Value> = projects.iter()
        .filter(|p| {
            let category_match = *category_filter == "All" ||
                p.get("category").and_then(|c| c.as_str()) == Some(&category_filter);

            let funding_match = p.get("total_awarded").and_then(|f| f.as_f64()).unwrap_or(0.0) >= *min_funding;

            let integration_match = *integration_filter == "All" ||
                p.get("integration_status").and_then(|i| i.as_str()) == Some(&integration_filter);

            let soroban_match = !*soroban_only ||
                p.get("soroban").and_then(|s| s.as_bool()).unwrap_or(false);

            category_match && funding_match && integration_match && soroban_match
        })
        .cloned()
        .collect();

    html! {
        <div class="feature_container">
            <h2>{"COMPETITIVE LANDSCAPE EXPLORER"}</h2>
            <p class="feature_desc">{"FILTER AND ANALYZE ALL 443 FUNDED PROJECTS"}</p>

            <div class="filter_panel">
                <div class="filter_group">
                    <label>{"CATEGORY"}</label>
                    <select
                        class="filter_select"
                        onchange={let filter = category_filter.clone(); Callback::from(move |e: Event| {
                            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
                            if let Some(select) = target {
                                filter.set(select.value());
                            }
                        })}
                    >
                        <option value="All">{"ALL CATEGORIES"}</option>
                        <option value="Infrastructure & Services">{"INFRASTRUCTURE & SERVICES"}</option>
                        <option value="Applications">{"APPLICATIONS"}</option>
                        <option value="Financial Protocols">{"FINANCIAL PROTOCOLS"}</option>
                        <option value="Developer Tooling">{"DEVELOPER TOOLING"}</option>
                    </select>
                </div>

                <div class="filter_group">
                    <label>{"MIN FUNDING"}</label>
                    <input
                        type="number"
                        class="filter_input"
                        placeholder="0"
                        step="10000"
                        oninput={let filter = min_funding.clone(); Callback::from(move |e: InputEvent| {
                            let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
                            if let Some(input) = target {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    filter.set(value);
                                }
                            }
                        })}
                    />
                </div>

                <div class="filter_group">
                    <label>{"INTEGRATION STATUS"}</label>
                    <select
                        class="filter_select"
                        onchange={let filter = integration_filter.clone(); Callback::from(move |e: Event| {
                            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
                            if let Some(select) = target {
                                filter.set(select.value());
                            }
                        })}
                    >
                        <option value="All">{"ALL STATUS"}</option>
                        <option value="Mainnet">{"MAINNET ONLY"}</option>
                        <option value="Testnet">{"TESTNET"}</option>
                        <option value="In Development">{"IN DEVELOPMENT"}</option>
                    </select>
                </div>

                <div class="filter_group">
                    <label class="checkbox_label">
                        <input
                            type="checkbox"
                            checked={*soroban_only}
                            onchange={let filter = soroban_only.clone(); Callback::from(move |e: Event| {
                                let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                if let Some(checkbox) = target {
                                    filter.set(checkbox.checked());
                                }
                            })}
                        />
                        {" SOROBAN ONLY"}
                    </label>
                </div>
            </div>

            <div class="results_summary">
                <span class="result_count">{filtered_projects.len()}{" PROJECTS FOUND"}</span>
                <span class="total_funding">
                    {"TOTAL: $"}
                    {format!("{:.1}M", filtered_projects.iter()
                        .map(|p| p.get("total_awarded").and_then(|f| f.as_f64()).unwrap_or(0.0))
                        .sum::<f64>() / 1_000_000.0)}
                </span>
            </div>

            <div class="projects_table">
                <table>
                    <thead>
                        <tr>
                            <th>{"PROJECT"}</th>
                            <th>{"CATEGORY"}</th>
                            <th>{"TYPE"}</th>
                            <th>{"COUNTRY"}</th>
                            <th>{"FUNDING"}</th>
                            <th>{"STATUS"}</th>
                            <th>{"SOROBAN"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for filtered_projects.iter().take(50).map(|project| {
                            let title = project.get("title").and_then(|t| t.as_str()).unwrap_or("N/A");
                            let category = project.get("category").and_then(|c| c.as_str()).unwrap_or("N/A");
                            let proj_type = project.get("type").and_then(|t| t.as_str()).unwrap_or("N/A");
                            let country = project.get("country").and_then(|c| c.as_str()).unwrap_or("N/A");
                            let funding = project.get("total_awarded").and_then(|f| f.as_f64()).unwrap_or(0.0);
                            let status = project.get("integration_status").and_then(|s| s.as_str()).unwrap_or("N/A");
                            let soroban = project.get("soroban").and_then(|s| s.as_bool()).unwrap_or(false);

                            html! {
                                <tr>
                                    <td><strong>{title}</strong></td>
                                    <td>{category}</td>
                                    <td>{proj_type}</td>
                                    <td>{country}</td>
                                    <td class="funding_cell">{format!("${:.0}K", funding / 1000.0)}</td>
                                    <td><span class="status_badge">{status}</span></td>
                                    <td>{if soroban { "YES" } else { "NO" }}</td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

// GAP ANALYSIS
#[function_component(GapAnalysis)]
fn gap_analysis() -> Html {
    html! {
        <div class="feature_container">
            <h2>{"GAP ANALYSIS"}</h2>
            <p class="feature_desc">{"IDENTIFY UNDERSERVED CATEGORIES WITH HIGH FUNDING POTENTIAL"}</p>

            <div class="gap_grid">
                <div class="gap_card critical">
                    <h3>{"CRITICAL GAPS (0 PROJECTS)"}</h3>
                    <ul class="gap_list">
                        <li>
                            <div class="gap_title">{"CREDIT SCORING INFRASTRUCTURE"}</div>
                            <div class="gap_potential">{"POTENTIAL: $150K+"}</div>
                            <div class="gap_desc">{"Enable DeFi lending with on-chain credit assessment"}</div>
                        </li>
                        <li>
                            <div class="gap_title">{"SUBSCRIPTION PAYMENT PLATFORM"}</div>
                            <div class="gap_potential">{"POTENTIAL: $340K (BUILD + GROWTH HACK)"}</div>
                            <div class="gap_desc">{"Recurring payments for Web3 services"}</div>
                        </li>
                        <li>
                            <div class="gap_title">{"ESCROW & INSURANCE PROTOCOL"}</div>
                            <div class="gap_potential">{"POTENTIAL: $250K (BUILD + LIQUIDITY)"}</div>
                            <div class="gap_desc">{"Trustless escrow and insurance infrastructure"}</div>
                        </li>
                        <li>
                            <div class="gap_title">{"GAS OPTIMIZATION TOOLS"}</div>
                            <div class="gap_potential">{"POTENTIAL: $100K+"}</div>
                            <div class="gap_desc">{"Contract optimization and profiling"}</div>
                        </li>
                        <li>
                            <div class="gap_title">{"SOROBAN-NATIVE ANCHOR"}</div>
                            <div class="gap_potential">{"POTENTIAL: $332K AVG"}</div>
                            <div class="gap_desc">{"All 4 anchors are Classic - huge opportunity"}</div>
                        </li>
                    </ul>
                </div>

                <div class="gap_card moderate">
                    <h3>{"LOW COMPETITION (1-2 PROJECTS)"}</h3>
                    <ul class="gap_list">
                        <li>
                            <div class="gap_title">{"YIELD AGGREGATORS"}</div>
                            <div class="gap_potential">{"1 PROJECT (TESTNET) | AVG: $198K"}</div>
                            <div class="gap_desc">{"Auto-compounding across Blend, Aquarius, Soroswap"}</div>
                        </li>
                        <li>
                            <div class="gap_title">{"SYNTHETIC ASSETS"}</div>
                            <div class="gap_potential">{"2 PROJECTS | HIGH DEMAND"}</div>
                            <div class="gap_desc">{"Currencies, commodities, digital asset representation"}</div>
                        </li>
                        <li>
                            <div class="gap_title">{"OPTIONS & DERIVATIVES"}</div>
                            <div class="gap_potential">{"2 PROJECTS | INFRASTRUCTURE PRIORITY"}</div>
                            <div class="gap_desc">{"For protocols/developers, not retail"}</div>
                        </li>
                        <li>
                            <div class="gap_title">{"FORMAL VERIFICATION"}</div>
                            <div class="gap_potential">{"1 PROJECT | SECURITY CRITICAL"}</div>
                            <div class="gap_desc">{"Mathematical proof systems for contract safety"}</div>
                        </li>
                    </ul>
                </div>

                <div class="gap_card emerging">
                    <h3>{"EMERGING OPPORTUNITIES (3-5 PROJECTS)"}</h3>
                    <ul class="gap_list">
                        <li>
                            <div class="gap_title">{"INTERNATIONAL AID TRANSPARENCY"}</div>
                            <div class="gap_potential">{"3 PROJECTS | HANDBOOK PRIORITY"}</div>
                            <div class="gap_desc">{"Track aid delivery with blockchain transparency"}</div>
                        </li>
                        <li>
                            <div class="gap_title">{"APAC CROSS-BORDER PAYMENTS"}</div>
                            <div class="gap_potential">{"EXPLICITLY PRIORITIZED"}</div>
                            <div class="gap_desc">{"Target Asian markets for remittances"}</div>
                        </li>
                        <li>
                            <div class="gap_title">{"SECURITY TOOLS"}</div>
                            <div class="gap_potential">{"8 PROJECTS | $273K AVG WHEN TOP-FUNDED"}</div>
                            <div class="gap_desc">{"Auditing, fuzzing, vulnerability detection"}</div>
                        </li>
                    </ul>
                </div>
            </div>

            <div class="strategy_panel">
                <h3>{"STRATEGIC RECOMMENDATIONS"}</h3>
                <div class="strategy_grid">
                    <div class="strategy_card">
                        <h4>{"BLUE OCEAN STRATEGY"}</h4>
                        <p>{"Target 0-project categories for zero competition. First-mover advantage with handbook validation."}</p>
                        <div class="strategy_metric">{"RISK: LOW | REWARD: VERY HIGH"}</div>
                    </div>
                    <div class="strategy_card">
                        <h4>{"DIFFERENTIATION PLAY"}</h4>
                        <p>{"Enter 1-2 project categories with superior tech or UX. Analyze existing solutions' weaknesses."}</p>
                        <div class="strategy_metric">{"RISK: MEDIUM | REWARD: HIGH"}</div>
                    </div>
                    <div class="strategy_card">
                        <h4>{"HANDBOOK ALIGNMENT"}</h4>
                        <p>{"Focus on explicitly mentioned priorities: APAC, Aid, Yield Aggregators, Soroban Anchors."}</p>
                        <div class="strategy_metric">{"RISK: LOW | REWARD: HIGH"}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// SUCCESS PATTERNS
#[function_component(SuccessPatterns)]
fn success_patterns() -> Html {
    html! {
        <div class="feature_container">
            <h2>{"SUCCESS PATTERNS ANALYZER"}</h2>
            <p class="feature_desc">{"WHAT SEPARATES TOP-FUNDED FROM AVERAGE PROJECTS"}</p>

            <div class="patterns_grid">
                <div class="pattern_card">
                    <h3>{"MULTIPLE ROUNDS EFFECT"}</h3>
                    <div class="pattern_data">
                        <div class="data_row">
                            <span class="data_label">{"1 ROUND:"}</span>
                            <span class="data_value">{"$68,634 AVG"}</span>
                            <span class="data_mult">{"1.0X BASELINE"}</span>
                        </div>
                        <div class="data_row highlight">
                            <span class="data_label">{"2 ROUNDS:"}</span>
                            <span class="data_value">{"$125,248 AVG"}</span>
                            <span class="data_mult">{"1.8X MULTIPLIER"}</span>
                        </div>
                        <div class="data_row highlight">
                            <span class="data_label">{"3 ROUNDS:"}</span>
                            <span class="data_value">{"$232,526 AVG"}</span>
                            <span class="data_mult">{"3.4X MULTIPLIER"}</span>
                        </div>
                        <div class="data_row critical">
                            <span class="data_label">{"4 ROUNDS:"}</span>
                            <span class="data_value">{"$339,956 AVG"}</span>
                            <span class="data_mult">{"5.0X MULTIPLIER"}</span>
                        </div>
                    </div>
                    <p class="pattern_insight">{"68.5% of top-funded projects received 2+ rounds"}</p>
                </div>

                <div class="pattern_card">
                    <h3>{"GEOGRAPHIC SWEET SPOTS"}</h3>
                    <div class="pattern_data">
                        <div class="data_row">
                            <span class="data_label">{"PORTUGAL:"}</span>
                            <span class="data_value">{"$297K AVG"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"GERMANY:"}</span>
                            <span class="data_value">{"$285K AVG"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"NETHERLANDS:"}</span>
                            <span class="data_value">{"$263K AVG"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"CHILE:"}</span>
                            <span class="data_value">{"$248K AVG"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"COLOMBIA:"}</span>
                            <span class="data_value">{"$219K AVG"}</span>
                        </div>
                    </div>
                    <p class="pattern_insight">{"Europe & LatAm show strongest performance"}</p>
                </div>

                <div class="pattern_card">
                    <h3>{"PROJECT TYPE PERFORMANCE"}</h3>
                    <div class="pattern_data">
                        <div class="data_row critical">
                            <span class="data_label">{"ANCHOR:"}</span>
                            <span class="data_value">{"$332K AVG"}</span>
                            <span class="data_mult">{"4 PROJECTS ONLY"}</span>
                        </div>
                        <div class="data_row highlight">
                            <span class="data_label">{"SECURITY:"}</span>
                            <span class="data_value">{"$273K AVG"}</span>
                            <span class="data_mult">{"8 PROJECTS"}</span>
                        </div>
                        <div class="data_row highlight">
                            <span class="data_label">{"DEX:"}</span>
                            <span class="data_value">{"$230K AVG"}</span>
                            <span class="data_mult">{"18 PROJECTS"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"ORACLE:"}</span>
                            <span class="data_value">{"$212K AVG"}</span>
                            <span class="data_mult">{"6 PROJECTS"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"YIELD AGGREGATOR:"}</span>
                            <span class="data_value">{"$198K AVG"}</span>
                            <span class="data_mult">{"1 PROJECT"}</span>
                        </div>
                    </div>
                    <p class="pattern_insight">{"Infrastructure projects dominate top funding"}</p>
                </div>

                <div class="pattern_card">
                    <h3>{"CATEGORY SUCCESS RATES"}</h3>
                    <div class="pattern_data">
                        <div class="data_row critical">
                            <span class="data_label">{"FINANCIAL PROTOCOLS:"}</span>
                            <span class="data_value">{"$130,893 AVG"}</span>
                            <span class="data_mult">{"HIGHEST"}</span>
                        </div>
                        <div class="data_row highlight">
                            <span class="data_label">{"INFRASTRUCTURE:"}</span>
                            <span class="data_value">{"$101,757 AVG"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"APPLICATIONS:"}</span>
                            <span class="data_value">{"$93,899 AVG"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"DEV TOOLING:"}</span>
                            <span class="data_value">{"$79,752 AVG"}</span>
                        </div>
                    </div>
                    <p class="pattern_insight">{"Financial Protocols lead by 28% over average"}</p>
                </div>

                <div class="pattern_card">
                    <h3>{"TECHNICAL FACTORS"}</h3>
                    <div class="pattern_data">
                        <div class="data_row highlight">
                            <span class="data_label">{"MAINNET STATUS:"}</span>
                            <span class="data_value">{"+30% PREMIUM"}</span>
                        </div>
                        <div class="data_row highlight">
                            <span class="data_label">{"SOROBAN-NATIVE:"}</span>
                            <span class="data_value">{"67.6% OF TOP-FUNDED"}</span>
                        </div>
                        <div class="data_row highlight">
                            <span class="data_label">{"STELLAR-ONLY:"}</span>
                            <span class="data_value">{"74.8% OF TOP-FUNDED"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"OPEN SOURCE:"}</span>
                            <span class="data_value">{"70.3% HAVE GITHUB"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"AUDIT BANK:"}</span>
                            <span class="data_value">{"+28% CORRELATION"}</span>
                        </div>
                    </div>
                    <p class="pattern_insight">{"Soroban + Stellar-only = winning combination"}</p>
                </div>

                <div class="pattern_card">
                    <h3>{"TOP 25% THRESHOLD"}</h3>
                    <div class="pattern_data">
                        <div class="data_row critical">
                            <span class="data_label">{"THRESHOLD:"}</span>
                            <span class="data_value">{"$144,000"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"MEDIAN:"}</span>
                            <span class="data_value">{"$75,000"}</span>
                        </div>
                        <div class="data_row">
                            <span class="data_label">{"AVERAGE:"}</span>
                            <span class="data_value">{"$95,067"}</span>
                        </div>
                    </div>
                    <p class="pattern_insight">{"Target $144K+ to enter top quartile"}</p>
                </div>
            </div>

            <div class="winning_formula">
                <h3>{"THE WINNING FORMULA"}</h3>
                <div class="formula_steps">
                    <div class="formula_step">
                        <div class="step_number">{"1"}</div>
                        <div class="step_content">
                            <h4>{"CHOOSE HIGH-VALUE, LOW-COMPETITION"}</h4>
                            <p>{"Anchor ($332K avg, 4 projects) or Credit Scoring (0 projects)"}</p>
                        </div>
                    </div>
                    <div class="formula_step">
                        <div class="step_number">{"2"}</div>
                        <div class="step_content">
                            <h4>{"SOROBAN-NATIVE + STELLAR-ONLY"}</h4>
                            <p>{"68% of top-funded are Soroban, 75% are Stellar-only"}</p>
                        </div>
                    </div>
                    <div class="formula_step">
                        <div class="step_number">{"3"}</div>
                        <div class="step_content">
                            <h4>{"LAUNCH ON MAINNET"}</h4>
                            <p>{"72% of top-funded reached Mainnet (+30% funding premium)"}</p>
                        </div>
                    </div>
                    <div class="formula_step">
                        <div class="step_number">{"4"}</div>
                        <div class="step_content">
                            <h4>{"PLAN FOR MULTIPLE ROUNDS"}</h4>
                            <p>{"4 rounds = 5.0x multiplier ($68K → $340K average)"}</p>
                        </div>
                    </div>
                    <div class="formula_step">
                        <div class="step_number">{"5"}</div>
                        <div class="step_content">
                            <h4>{"USE AUDIT BANK"}</h4>
                            <p>{"Free audit + 28% average funding correlation"}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// FUNDING CALCULATOR
#[function_component(FundingCalculator)]
fn funding_calculator() -> Html {
    let category = use_state(|| "Financial Protocols".to_string());
    let stage = use_state(|| "mainnet".to_string());
    let soroban = use_state(|| true);
    let stellar_only = use_state(|| true);
    let rounds = use_state(|| 2);
    let audit_bank = use_state(|| true);
    let location = use_state(|| "Europe".to_string());

    // Real data-driven calculation
    let base_by_category = match (*category).as_str() {
        "Financial Protocols" => 130893.0,
        "Infrastructure & Services" => 101757.0,
        "Applications" => 93899.0,
        "Developer Tooling" => 79752.0,
        _ => 95067.0,
    };

    let stage_mult = match (*stage).as_str() {
        "mainnet" => 1.3,
        "testnet" => 1.0,
        "development" => 0.8,
        _ => 0.6,
    };

    let round_mult = match *rounds {
        1 => 1.0,
        2 => 1.8,
        3 => 3.4,
        4 => 5.0,
        _ => 5.0,
    };

    let geo_bonus = match (*location).as_str() {
        "Portugal" => 1.15,
        "Germany" => 1.12,
        "Netherlands" => 1.10,
        "Europe" => 1.08,
        "LatAm" => 1.05,
        _ => 1.0,
    };

    let soroban_bonus = if *soroban { 1.1 } else { 1.0 };
    let stellar_bonus = if *stellar_only { 1.1 } else { 1.0 };
    let audit_bonus = if *audit_bank { 1.28 } else { 1.0 };

    let estimated_total = base_by_category * stage_mult * round_mult * geo_bonus * soroban_bonus * stellar_bonus * audit_bonus;

    html! {
        <div class="feature_container">
            <h2>{"FUNDING CALCULATOR"}</h2>
            <p class="feature_desc">{"CALCULATE EXPECTED FUNDING BASED ON 443 REAL PROJECTS"}</p>

            <div class="calculator_layout">
                <div class="calculator_inputs">
                    <div class="input_group">
                        <label>{"CATEGORY"}</label>
                        <select
                            class="calc_select"
                            value={(*category).clone()}
                            onchange={let cat = category.clone(); Callback::from(move |e: Event| {
                                if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                    cat.set(select.value());
                                }
                            })}
                        >
                            <option value="Financial Protocols">{"FINANCIAL PROTOCOLS ($130K AVG)"}</option>
                            <option value="Infrastructure & Services">{"INFRASTRUCTURE & SERVICES ($101K AVG)"}</option>
                            <option value="Applications">{"APPLICATIONS ($93K AVG)"}</option>
                            <option value="Developer Tooling">{"DEVELOPER TOOLING ($79K AVG)"}</option>
                        </select>
                    </div>

                    <div class="input_group">
                        <label>{"PROJECT STAGE"}</label>
                        <select
                            class="calc_select"
                            value={(*stage).clone()}
                            onchange={let st = stage.clone(); Callback::from(move |e: Event| {
                                if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                    st.set(select.value());
                                }
                            })}
                        >
                            <option value="mainnet">{"MAINNET (+30% PREMIUM)"}</option>
                            <option value="testnet">{"TESTNET (BASELINE)"}</option>
                            <option value="development">{"IN DEVELOPMENT (-20%)"}</option>
                            <option value="idea">{"IDEA STAGE (-40%)"}</option>
                        </select>
                    </div>

                    <div class="input_group">
                        <label>{"NUMBER OF ROUNDS"}</label>
                        <select
                            class="calc_select"
                            value={rounds.to_string()}
                            onchange={let rnd = rounds.clone(); Callback::from(move |e: Event| {
                                if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                    if let Ok(val) = select.value().parse::<i32>() {
                                        rnd.set(val);
                                    }
                                }
                            })}
                        >
                            <option value="1">{"1 ROUND (1.0X BASELINE)"}</option>
                            <option value="2">{"2 ROUNDS (1.8X MULTIPLIER)"}</option>
                            <option value="3">{"3 ROUNDS (3.4X MULTIPLIER)"}</option>
                            <option value="4">{"4 ROUNDS (5.0X MULTIPLIER)"}</option>
                        </select>
                    </div>

                    <div class="input_group">
                        <label>{"GEOGRAPHIC LOCATION"}</label>
                        <select
                            class="calc_select"
                            value={(*location).clone()}
                            onchange={let loc = location.clone(); Callback::from(move |e: Event| {
                                if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                    loc.set(select.value());
                                }
                            })}
                        >
                            <option value="Portugal">{"PORTUGAL (+15% AVG)"}</option>
                            <option value="Germany">{"GERMANY (+12% AVG)"}</option>
                            <option value="Netherlands">{"NETHERLANDS (+10% AVG)"}</option>
                            <option value="Europe">{"OTHER EUROPE (+8%)"}</option>
                            <option value="LatAm">{"LATIN AMERICA (+5%)"}</option>
                            <option value="Other">{"OTHER REGIONS (BASELINE)"}</option>
                        </select>
                    </div>

                    <div class="checkbox_group">
                        <label class="checkbox_label">
                            <input
                                type="checkbox"
                                checked={*soroban}
                                onchange={let sor = soroban.clone(); Callback::from(move |e: Event| {
                                    if let Some(cb) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                        sor.set(cb.checked());
                                    }
                                })}
                            />
                            {" SOROBAN-NATIVE (+10%)"}
                        </label>
                    </div>

                    <div class="checkbox_group">
                        <label class="checkbox_label">
                            <input
                                type="checkbox"
                                checked={*stellar_only}
                                onchange={let st = stellar_only.clone(); Callback::from(move |e: Event| {
                                    if let Some(cb) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                        st.set(cb.checked());
                                    }
                                })}
                            />
                            {" STELLAR-ONLY (+10%)"}
                        </label>
                    </div>

                    <div class="checkbox_group">
                        <label class="checkbox_label">
                            <input
                                type="checkbox"
                                checked={*audit_bank}
                                onchange={let aud = audit_bank.clone(); Callback::from(move |e: Event| {
                                    if let Some(cb) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                        aud.set(cb.checked());
                                    }
                                })}
                            />
                            {" AUDIT BANK PARTICIPANT (+28%)"}
                        </label>
                    </div>
                </div>

                <div class="calculator_results">
                    <div class="result_card_main">
                        <div class="result_label">{"ESTIMATED TOTAL FUNDING"}</div>
                        <div class="result_value_large">{format!("${:.0}K", estimated_total / 1000.0)}</div>
                        <div class="result_breakdown">
                            <div class="breakdown_item">
                                <span>{"BASE (CATEGORY AVG):"}</span>
                                <span>{format!("${:.0}K", base_by_category / 1000.0)}</span>
                            </div>
                            <div class="breakdown_item">
                                <span>{"STAGE MULTIPLIER:"}</span>
                                <span>{format!("{:.1}x", stage_mult)}</span>
                            </div>
                            <div class="breakdown_item">
                                <span>{"ROUNDS MULTIPLIER:"}</span>
                                <span>{format!("{:.1}x", round_mult)}</span>
                            </div>
                            <div class="breakdown_item">
                                <span>{"GEOGRAPHIC BONUS:"}</span>
                                <span>{format!("{:.1}x", geo_bonus)}</span>
                            </div>
                            {if *soroban {
                                html! {<div class="breakdown_item"><span>{"SOROBAN BONUS:"}</span><span>{"1.1x"}</span></div>}
                            } else { html!{} }}
                            {if *stellar_only {
                                html! {<div class="breakdown_item"><span>{"STELLAR-ONLY BONUS:"}</span><span>{"1.1x"}</span></div>}
                            } else { html!{} }}
                            {if *audit_bank {
                                html! {<div class="breakdown_item"><span>{"AUDIT BANK CORRELATION:"}</span><span>{"1.28x"}</span></div>}
                            } else { html!{} }}
                        </div>
                    </div>

                    <div class="recommendations">
                        <h4>{"OPTIMIZATION RECOMMENDATIONS"}</h4>
                        {if estimated_total < 144000.0 {
                            html! {
                                <div class="rec_item warning">
                                    {"BELOW TOP 25% THRESHOLD ($144K). Consider higher-value category or more rounds."}
                                </div>
                            }
                        } else {
                            html! {
                                <div class="rec_item success">
                                    {"ABOVE TOP 25% THRESHOLD. Strong funding potential."}
                                </div>
                            }
                        }}
                        {if !*soroban {
                            html! {
                                <div class="rec_item">
                                    {"Consider Soroban implementation: 67.6% of top-funded projects use it."}
                                </div>
                            }
                        } else { html!{} }}
                        {if !*stellar_only {
                            html! {
                                <div class="rec_item">
                                    {"Consider Stellar-only approach: 74.8% of top-funded are not multichain."}
                                </div>
                            }
                        } else { html!{} }}
                        {if *rounds < 3 {
                            html! {
                                <div class="rec_item">
                                    {"Plan for 3-4 rounds: 68.5% of top-funded received multiple rounds."}
                                </div>
                            }
                        } else { html!{} }}
                    </div>
                </div>
            </div>
        </div>
    }
}

// SOCIAL INTELLIGENCE - Using actual social_accounts_detailed.json
#[function_component(SocialIntelligence)]
fn social_intelligence() -> Html {
    let social_data = use_state(|| Vec::<Value>::new());

    {
        let social_data = social_data.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(response) = gloo_net::http::Request::get("/api/enriched/social-accounts")
                    .send()
                    .await
                {
                    if let Ok(data) = response.json::<Value>().await {
                        if let Some(accounts) = data.get("social_accounts").and_then(|a| a.as_array()) {
                            social_data.set(accounts.clone());
                        }
                    }
                }
            });
            || ()
        });
    }

    let projects_with_twitter = social_data.iter().filter(|p| p.get("twitter").is_some()).count();
    let projects_with_discord = social_data.iter().filter(|p| p.get("discord").is_some()).count();
    let projects_with_github = social_data.iter().filter(|p| p.get("github").is_some()).count();
    let projects_with_linkedin = social_data.iter().filter(|p| p.get("linkedin").is_some()).count();

    html! {
        <div class="feature_container">
            <h2>{"SOCIAL INTELLIGENCE"}</h2>
            <p class="feature_desc">{"SOCIAL MEDIA PRESENCE ANALYSIS ACROSS 443 PROJECTS"}</p>

            <div class="social_stats_grid">
                <div class="social_stat">
                    <div class="stat_number">{projects_with_twitter}</div>
                    <div class="stat_label">{"TWITTER ACCOUNTS"}</div>
                    <div class="stat_percent">{format!("{:.1}%", (projects_with_twitter as f64 / social_data.len() as f64) * 100.0)}</div>
                </div>
                <div class="social_stat">
                    <div class="stat_number">{projects_with_discord}</div>
                    <div class="stat_label">{"DISCORD SERVERS"}</div>
                    <div class="stat_percent">{format!("{:.1}%", (projects_with_discord as f64 / social_data.len() as f64) * 100.0)}</div>
                </div>
                <div class="social_stat">
                    <div class="stat_number">{projects_with_github}</div>
                    <div class="stat_label">{"GITHUB REPOS"}</div>
                    <div class="stat_percent">{format!("{:.1}%", (projects_with_github as f64 / social_data.len() as f64) * 100.0)}</div>
                </div>
                <div class="social_stat">
                    <div class="stat_number">{projects_with_linkedin}</div>
                    <div class="stat_label">{"LINKEDIN PAGES"}</div>
                    <div class="stat_percent">{format!("{:.1}%", (projects_with_linkedin as f64 / social_data.len() as f64) * 100.0)}</div>
                </div>
            </div>

            <div class="social_insights">
                <h3>{"KEY INSIGHTS"}</h3>
                <ul class="insights_list">
                    <li>{"70.3% of top-funded projects have public GitHub repositories"}</li>
                    <li>{"100% of top-funded projects have active websites"}</li>
                    <li>{"Strong social presence correlates with higher community engagement"}</li>
                    <li>{"Discord servers indicate active community building"}</li>
                </ul>
            </div>

            <div class="projects_table">
                <h3>{"PROJECT SOCIAL PROFILES (SAMPLE)"}</h3>
                <table>
                    <thead>
                        <tr>
                            <th>{"PROJECT"}</th>
                            <th>{"TWITTER"}</th>
                            <th>{"DISCORD"}</th>
                            <th>{"GITHUB"}</th>
                            <th>{"LINKEDIN"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for social_data.iter().take(30).map(|project| {
                            let title = project.get("title").and_then(|t| t.as_str()).unwrap_or("N/A");
                            let has_twitter = project.get("twitter").is_some();
                            let has_discord = project.get("discord").is_some();
                            let has_github = project.get("github").is_some();
                            let has_linkedin = project.get("linkedin").is_some();

                            html! {
                                <tr>
                                    <td><strong>{title}</strong></td>
                                    <td>{if has_twitter { "YES" } else { "NO" }}</td>
                                    <td>{if has_discord { "YES" } else { "NO" }}</td>
                                    <td>{if has_github { "YES" } else { "NO" }}</td>
                                    <td>{if has_linkedin { "YES" } else { "NO" }}</td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

// TEAM INSIGHTS - Placeholder for team_profiles.json
#[function_component(TeamInsights)]
fn team_insights() -> Html {
    html! {
        <div class="feature_container">
            <h2>{"TEAM INSIGHTS"}</h2>
            <p class="feature_desc">{"ANALYZE TEAM COMPOSITION AND EXPERIENCE PATTERNS"}</p>
            <p>{"Coming soon - team profile analysis from enriched data"}</p>
        </div>
    }
}

// GEOGRAPHIC TRENDS - Using projects_with_regions data
#[function_component(GeographicTrends)]
fn geographic_trends() -> Html {
    html! {
        <div class="feature_container">
            <h2>{"GEOGRAPHIC TRENDS"}</h2>
            <p class="feature_desc">{"FUNDING PATTERNS BY COUNTRY AND REGION"}</p>

            <div class="geo_stats">
                <h3>{"TOP COUNTRIES BY AVERAGE FUNDING (TOP-FUNDED PROJECTS)"}</h3>
                <div class="geo_grid">
                    <div class="geo_card">
                        <div class="geo_flag">{"PT"}</div>
                        <div class="geo_country">{"PORTUGAL"}</div>
                        <div class="geo_amount">{"$297,197"}</div>
                        <div class="geo_desc">{"EUROPE & CENTRAL ASIA"}</div>
                    </div>
                    <div class="geo_card">
                        <div class="geo_flag">{"DE"}</div>
                        <div class="geo_country">{"GERMANY"}</div>
                        <div class="geo_amount">{"$284,650"}</div>
                        <div class="geo_desc">{"EUROPE & CENTRAL ASIA"}</div>
                    </div>
                    <div class="geo_card">
                        <div class="geo_flag">{"NL"}</div>
                        <div class="geo_country">{"NETHERLANDS"}</div>
                        <div class="geo_amount">{"$263,387"}</div>
                        <div class="geo_desc">{"EUROPE & CENTRAL ASIA"}</div>
                    </div>
                    <div class="geo_card">
                        <div class="geo_flag">{"CL"}</div>
                        <div class="geo_country">{"CHILE"}</div>
                        <div class="geo_amount">{"$248,375"}</div>
                        <div class="geo_desc">{"LATIN AMERICA"}</div>
                    </div>
                    <div class="geo_card">
                        <div class="geo_flag">{"CO"}</div>
                        <div class="geo_country">{"COLOMBIA"}</div>
                        <div class="geo_amount">{"$219,495"}</div>
                        <div class="geo_desc">{"LATIN AMERICA"}</div>
                    </div>
                    <div class="geo_card">
                        <div class="geo_flag">{"US"}</div>
                        <div class="geo_country">{"UNITED STATES"}</div>
                        <div class="geo_amount">{"$181,000"}</div>
                        <div class="geo_desc">{"NORTH AMERICA"}</div>
                    </div>
                </div>
            </div>

            <div class="regional_insights">
                <h3>{"REGIONAL INSIGHTS"}</h3>
                <div class="insight_grid">
                    <div class="insight_card">
                        <h4>{"EUROPE & CENTRAL ASIA"}</h4>
                        <p>{"Strongest performance overall. Portugal, Germany, Netherlands lead top-funded averages. Strong fintech and DeFi ecosystems."}</p>
                    </div>
                    <div class="insight_card">
                        <h4>{"LATIN AMERICA"}</h4>
                        <p>{"High growth region. Chile and Colombia show exceptional results. Focus on financial inclusion and cross-border payments."}</p>
                    </div>
                    <div class="insight_card">
                        <h4>{"NORTH AMERICA"}</h4>
                        <p>{"Solid but not highest. US projects average $181K. Opportunity exists but competition is higher."}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

// TECH STACK ANALYSIS
#[function_component(TechStackAnalysis)]
fn tech_stack_analysis() -> Html {
    html! {
        <div class="feature_container">
            <h2>{"TECH STACK ANALYSIS"}</h2>
            <p class="feature_desc">{"TECHNOLOGY CHOICES AND FUNDING CORRELATION"}</p>

            <div class="tech_stats">
                <div class="tech_card">
                    <h3>{"SOROBAN VS CLASSIC"}</h3>
                    <div class="tech_comparison">
                        <div class="tech_item highlight">
                            <div class="tech_label">{"SOROBAN-NATIVE:"}</div>
                            <div class="tech_value">{"67.6% OF TOP-FUNDED"}</div>
                        </div>
                        <div class="tech_item">
                            <div class="tech_label">{"CLASSIC:"}</div>
                            <div class="tech_value">{"32.4% OF TOP-FUNDED"}</div>
                        </div>
                    </div>
                    <p class="tech_insight">{"Soroban dominance in recent high-value projects"}</p>
                </div>

                <div class="tech_card">
                    <h3>{"MULTICHAIN VS STELLAR-ONLY"}</h3>
                    <div class="tech_comparison">
                        <div class="tech_item highlight">
                            <div class="tech_label">{"STELLAR-ONLY:"}</div>
                            <div class="tech_value">{"74.8% OF TOP-FUNDED"}</div>
                        </div>
                        <div class="tech_item">
                            <div class="tech_label">{"MULTICHAIN:"}</div>
                            <div class="tech_value">{"25.2% OF TOP-FUNDED"}</div>
                        </div>
                    </div>
                    <p class="tech_insight">{"Focus pays off - Stellar-only projects get +30% premium"}</p>
                </div>

                <div class="tech_card">
                    <h3>{"OPEN SOURCE STATUS"}</h3>
                    <div class="tech_comparison">
                        <div class="tech_item highlight">
                            <div class="tech_label">{"OPEN SOURCE:"}</div>
                            <div class="tech_value">{"70.3% HAVE GITHUB"}</div>
                        </div>
                        <div class="tech_item">
                            <div class="tech_label">{"PROPRIETARY:"}</div>
                            <div class="tech_value">{"29.7%"}</div>
                        </div>
                    </div>
                    <p class="tech_insight">{"Open source strongly correlates with top funding"}</p>
                </div>

                <div class="tech_card">
                    <h3>{"INTEGRATION STATUS"}</h3>
                    <div class="tech_comparison">
                        <div class="tech_item highlight">
                            <div class="tech_label">{"MAINNET:"}</div>
                            <div class="tech_value">{"72% OF TOP-FUNDED"}</div>
                        </div>
                        <div class="tech_item">
                            <div class="tech_label">{"TESTNET:"}</div>
                            <div class="tech_value">{"20%"}</div>
                        </div>
                        <div class="tech_item">
                            <div class="tech_label">{"DEVELOPMENT:"}</div>
                            <div class="tech_value">{"8%"}</div>
                        </div>
                    </div>
                    <p class="tech_insight">{"Mainnet launch provides significant funding advantage"}</p>
                </div>
            </div>

            <div class="tech_recommendations">
                <h3>{"TECHNOLOGY RECOMMENDATIONS"}</h3>
                <div class="rec_grid">
                    <div class="rec_item success">
                        <h4>{"OPTIMAL STACK"}</h4>
                        <ul>
                            <li>{"Soroban smart contracts"}</li>
                            <li>{"Stellar-only (not multichain)"}</li>
                            <li>{"Open-source on GitHub"}</li>
                            <li>{"Target Mainnet launch"}</li>
                            <li>{"Audit Bank participation"}</li>
                        </ul>
                    </div>
                    <div class="rec_item">
                        <h4>{"RISK FACTORS"}</h4>
                        <ul>
                            <li>{"Multichain approach (-30% average)"}</li>
                            <li>{"No GitHub presence"}</li>
                            <li>{"Staying on Testnet too long"}</li>
                            <li>{"No audit (miss 28% correlation)"}</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
