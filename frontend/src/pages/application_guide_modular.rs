use yew::prelude::*;
use gloo_net::http::Request;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use crate::components::layout::Layout;
use web_sys::Event;

#[derive(Clone, PartialEq)]
enum GuideModule {
    OfficialCriteria,
    Benchmarking,
    EfficiencyCalculator,
    GeographicInsights,
    SuccessPatterns,
    FullHandbook,
}

#[function_component(ApplicationGuideModular)]
pub fn application_guide_modular() -> Html {
    let active_module = use_state(|| GuideModule::OfficialCriteria);
    let handbook_criteria = use_state(|| Option::<Value>::None);
    let benchmarking = use_state(|| Option::<Value>::None);
    let efficiency = use_state(|| Option::<Value>::None);
    let geographic = use_state(|| Option::<Value>::None);
    let statistical = use_state(|| Option::<Value>::None);
    let handbook_full = use_state(|| Option::<Value>::None);

    let selected_category = use_state(|| "DeFi".to_string());
    let selected_handbook_section = use_state(|| 0_usize);
    let loading = use_state(|| false);

    // Fetch handbook criteria on mount
    {
        let handbook_criteria = handbook_criteria.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            loading.set(true);
            spawn_local(async move {
                if let Ok(response) = Request::get("/api/handbook/criteria").send().await {
                    if let Ok(data) = response.json::<Value>().await {
                        if let Some(criteria_data) = data.get("data") {
                            handbook_criteria.set(Some(criteria_data.clone()));
                        }
                    }
                }
                loading.set(false);
            });
        });
    }

    // Fetch benchmarking data
    {
        let benchmarking = benchmarking.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(response) = Request::get("/api/handbook/benchmarking").send().await {
                    if let Ok(data) = response.json::<Value>().await {
                        if let Some(bench_data) = data.get("data") {
                            benchmarking.set(Some(bench_data.clone()));
                        }
                    }
                }
            });
        });
    }

    // Fetch efficiency data
    {
        let efficiency = efficiency.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(response) = Request::get("/api/handbook/efficiency").send().await {
                    if let Ok(data) = response.json::<Value>().await {
                        if let Some(eff_data) = data.get("data") {
                            efficiency.set(Some(eff_data.clone()));
                        }
                    }
                }
            });
        });
    }

    // Fetch geographic data
    {
        let geographic = geographic.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(response) = Request::get("/api/handbook/geographic").send().await {
                    if let Ok(data) = response.json::<Value>().await {
                        if let Some(geo_data) = data.get("data") {
                            geographic.set(Some(geo_data.clone()));
                        }
                    }
                }
            });
        });
    }

    // Fetch statistical data
    {
        let statistical = statistical.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(response) = Request::get("/api/handbook/statistical").send().await {
                    if let Ok(data) = response.json::<Value>().await {
                        if let Some(stat_data) = data.get("data") {
                            statistical.set(Some(stat_data.clone()));
                        }
                    }
                }
            });
        });
    }

    // Fetch full handbook data
    {
        let handbook_full = handbook_full.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(response) = Request::get("/api/handbook/full").send().await {
                    if let Ok(data) = response.json::<Value>().await {
                        if let Some(full_data) = data.get("data") {
                            handbook_full.set(Some(full_data.clone()));
                        }
                    }
                }
            });
        });
    }

    let switch_module = {
        let active_module = active_module.clone();
        Callback::from(move |module: GuideModule| {
            active_module.set(module);
        })
    };

    html! {
        <Layout>
            <div class="guide-modular-page">
                // Sidebar Navigation
                <aside class="guide-sidebar-nav">
                    <div class="sidebar-header">
                        <h2>{"SCF APPLICATION GUIDE"}</h2>
                        <p class="sidebar-subtitle">{"Modular Information Directory"}</p>
                    </div>

                    <nav class="module-nav">
                        <button
                            class={if *active_module == GuideModule::OfficialCriteria { "nav-module active" } else { "nav-module" }}
                            onclick={let switch = switch_module.clone(); Callback::from(move |_| switch.emit(GuideModule::OfficialCriteria))}
                        >
                            <span class="module-icon">{"📋"}</span>
                            <div class="module-info">
                                <span class="module-title">{"Official Criteria"}</span>
                                <span class="module-desc">{"Requirements & Disqualifiers"}</span>
                            </div>
                        </button>

                        <button
                            class={if *active_module == GuideModule::Benchmarking { "nav-module active" } else { "nav-module" }}
                            onclick={let switch = switch_module.clone(); Callback::from(move |_| switch.emit(GuideModule::Benchmarking))}
                        >
                            <span class="module-icon">{"📊"}</span>
                            <div class="module-info">
                                <span class="module-title">{"Benchmarking Tool"}</span>
                                <span class="module-desc">{"Compare with 443 Projects"}</span>
                            </div>
                        </button>

                        <button
                            class={if *active_module == GuideModule::EfficiencyCalculator { "nav-module active" } else { "nav-module" }}
                            onclick={let switch = switch_module.clone(); Callback::from(move |_| switch.emit(GuideModule::EfficiencyCalculator))}
                        >
                            <span class="module-icon">{"⚡"}</span>
                            <div class="module-info">
                                <span class="module-title">{"Efficiency Calculator"}</span>
                                <span class="module-desc">{"Optimize Funding Strategy"}</span>
                            </div>
                        </button>

                        <button
                            class={if *active_module == GuideModule::GeographicInsights { "nav-module active" } else { "nav-module" }}
                            onclick={let switch = switch_module.clone(); Callback::from(move |_| switch.emit(GuideModule::GeographicInsights))}
                        >
                            <span class="module-icon">{"🌍"}</span>
                            <div class="module-info">
                                <span class="module-title">{"Geographic Insights"}</span>
                                <span class="module-desc">{"Regional Market Analysis"}</span>
                            </div>
                        </button>

                        <button
                            class={if *active_module == GuideModule::SuccessPatterns { "nav-module active" } else { "nav-module" }}
                            onclick={let switch = switch_module.clone(); Callback::from(move |_| switch.emit(GuideModule::SuccessPatterns))}
                        >
                            <span class="module-icon">{"🎯"}</span>
                            <div class="module-info">
                                <span class="module-title">{"Success Patterns"}</span>
                                <span class="module-desc">{"What Actually Works"}</span>
                            </div>
                        </button>

                        <button
                            class={if *active_module == GuideModule::FullHandbook { "nav-module active" } else { "nav-module" }}
                            onclick={let switch = switch_module.clone(); Callback::from(move |_| switch.emit(GuideModule::FullHandbook))}
                        >
                            <span class="module-icon">{"📖"}</span>
                            <div class="module-info">
                                <span class="module-title">{"Full Handbook"}</span>
                                <span class="module-desc">{"Complete SCF Documentation"}</span>
                            </div>
                        </button>
                    </nav>
                </aside>

                // Main Content Area
                <main class="guide-main-content">
                    {
                        if *loading {
                            html! { <div class="loading-state"><div class="spinner"></div><p>{"LOADING MODULE..."}</p></div> }
                        } else {
                            match *active_module {
                                GuideModule::OfficialCriteria => render_official_criteria(&handbook_criteria),
                                GuideModule::Benchmarking => render_benchmarking(&benchmarking),
                                GuideModule::EfficiencyCalculator => render_efficiency(&efficiency),
                                GuideModule::GeographicInsights => render_geographic(&geographic),
                                GuideModule::SuccessPatterns => render_success_patterns(&statistical),
                                GuideModule::FullHandbook => {
                                    let set_section = {
                                        let selected = selected_handbook_section.clone();
                                        Callback::from(move |idx: usize| selected.set(idx))
                                    };
                                    render_full_handbook(&handbook_full, *selected_handbook_section, set_section)
                                },
                            }
                        }
                    }
                </main>
            </div>
        </Layout>
    }
}

// ===== MODULE RENDERERS =====

fn render_official_criteria(criteria: &Option<Value>) -> Html {
    if let Some(data) = criteria {
        let _keywords = data.get("global_keywords").and_then(|k| k.as_object());
        let requirements = data.get("eligibility_requirements").and_then(|r| r.as_array());
        let disqualifiers = data.get("disqualifiers").and_then(|d| d.as_array());
        let important_quotes = data.get("important_quotes").and_then(|q| q.as_array());

        html! {
            <div class="module-content">
                <header class="module-header">
                    <h1>{"OFFICIAL SCF CRITERIA"}</h1>
                    <p class="module-intro">{"Direct from the Stellar Community Fund Handbook - These are the official success criteria, requirements, and disqualifiers."}</p>
                </header>

                <div class="criteria-grid">
                    // Disqualifiers - CRITICAL
                    <section class="criteria-section critical">
                        <h2 class="section-title">{"AUTOMATIC DISQUALIFIERS"}</h2>
                        <p class="section-desc">{"Projects with ANY of these will be rejected:"}</p>
                        <ul class="criteria-list disqualifiers">
                            {
                                disqualifiers.unwrap_or(&vec![]).iter().map(|d| {
                                    html! { <li>{d.as_str().unwrap_or("")}</li> }
                                }).collect::<Html>()
                            }
                        </ul>
                    </section>

                    // Requirements - MUST HAVE
                    <section class="criteria-section required">
                        <h2 class="section-title">{format!("REQUIRED CRITERIA ({})", requirements.map(|r| r.len()).unwrap_or(0))}</h2>
                        <p class="section-desc">{"Your proposal MUST demonstrate:"}</p>
                        <ul class="criteria-list requirements">
                            {
                                requirements.unwrap_or(&vec![]).iter().take(20).map(|r| {
                                    html! { <li>{r.as_str().unwrap_or("")}</li> }
                                }).collect::<Html>()
                            }
                        </ul>
                    </section>

                    // Important Quotes - KEY INSIGHTS
                    <section class="criteria-section priorities">
                        <h2 class="section-title">{"IMPORTANT QUOTES FROM HANDBOOK"}</h2>
                        <p class="section-desc">{"Critical information from official documentation:"}</p>
                        <ul class="criteria-list priorities-list">
                            {
                                important_quotes.unwrap_or(&vec![]).iter().take(15).map(|q| {
                                    html! { <li>{q.as_str().unwrap_or("")}</li> }
                                }).collect::<Html>()
                            }
                        </ul>
                    </section>

                    // Top Keywords
                    <section class="criteria-section keywords">
                        <h2 class="section-title">{"TOP KEYWORDS"}</h2>
                        <p class="section-desc">{"Most mentioned terms in the SCF handbook:"}</p>
                        <div class="keyword-cloud">
                            {
                                if let Some(kw_obj) = _keywords {
                                    let mut keywords_vec: Vec<_> = kw_obj.iter().collect();
                                    keywords_vec.sort_by(|a, b| {
                                        let count_a = a.1.as_u64().unwrap_or(0);
                                        let count_b = b.1.as_u64().unwrap_or(0);
                                        count_b.cmp(&count_a)
                                    });

                                    keywords_vec.iter().take(30).map(|(keyword, count)| {
                                        let count_val = count.as_u64().unwrap_or(0);
                                        html! {
                                            <span class="keyword-tag" title={format!("{} mentions", count_val)}>
                                                {keyword}
                                                <span class="keyword-count">{count_val}</span>
                                            </span>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <p>{"No keywords data available"}</p> }
                                }
                            }
                        </div>
                    </section>
                </div>
            </div>
        }
    } else {
        html! { <div class="module-content"><p>{"Loading criteria..."}</p></div> }
    }
}

fn render_benchmarking(benchmarking: &Option<Value>) -> Html {
    if let Some(data) = benchmarking {
        let best_in_class = data.get("best_in_class").and_then(|b| b.as_object());
        let categories = data.get("categories").and_then(|c| c.as_object());

        html! {
            <div class="module-content">
                <header class="module-header">
                    <h1>{"BENCHMARKING TOOL"}</h1>
                    <p class="module-intro">{"Compare your project against 443 funded projects across categories."}</p>
                </header>

                <div class="criteria-grid">
                    // Best in Class Projects
                    <section class="criteria-section">
                        <h2 class="section-title">{"BEST IN CLASS - Top Funded Projects"}</h2>
                        <p class="section-desc">{"Highest funded project in each category"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(bic) = best_in_class {
                                    bic.iter().map(|(category, project)| {
                                        let title = project.get("title").and_then(|t| t.as_str()).unwrap_or("");
                                        let funding = project.get("funding").and_then(|f| f.as_f64()).unwrap_or(0.0);
                                        let status = project.get("status").and_then(|s| s.as_str()).unwrap_or("");
                                        html! {
                                            <li>
                                                <strong>{category}{": "}</strong>
                                                {title}{" - "}
                                                <span class="funding-amount">{format!("${:.0}", funding)}</span>
                                                {format!(" ({})", status)}
                                            </li>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>

                    // Category Statistics
                    <section class="criteria-section">
                        <h2 class="section-title">{"CATEGORY STATISTICS"}</h2>
                        <p class="section-desc">{"Average funding and project counts by category"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(cats) = categories {
                                    cats.iter().map(|(category, stats)| {
                                        let avg_funding = stats.get("avg_funding").and_then(|a| a.as_f64()).unwrap_or(0.0);
                                        let count = stats.get("count").and_then(|c| c.as_u64()).unwrap_or(0);
                                        html! {
                                            <li>
                                                <strong>{category}{": "}</strong>
                                                {format!("{} projects, avg ", count)}
                                                <span class="funding-amount">{format!("${:.0}", avg_funding)}</span>
                                            </li>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>
                </div>
            </div>
        }
    } else {
        html! { <div class="module-content"><p>{"Loading benchmarking data..."}</p></div> }
    }
}

fn render_efficiency(efficiency: &Option<Value>) -> Html {
    if let Some(data) = efficiency {
        let category_efficiency = data.get("category_efficiency").and_then(|c| c.as_object());
        let cost_per_milestone = data.get("cost_per_milestone").and_then(|c| c.as_object());
        let rounds_roi = data.get("rounds_roi").and_then(|r| r.as_object());

        html! {
            <div class="module-content">
                <header class="module-header">
                    <h1>{"FUNDING EFFICIENCY CALCULATOR"}</h1>
                    <p class="module-intro">{"Calculate optimal funding strategy based on real data from 443 projects."}</p>
                </header>

                <div class="criteria-grid">
                    // Category Efficiency
                    <section class="criteria-section">
                        <h2 class="section-title">{"CATEGORY EFFICIENCY SCORES"}</h2>
                        <p class="section-desc">{"Success rate and efficiency by category"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(ce) = category_efficiency {
                                    ce.iter().map(|(category, stats)| {
                                        let avg_funding = stats.get("avg_funding").and_then(|a| a.as_f64()).unwrap_or(0.0);
                                        let success_rate = stats.get("success_rate").and_then(|s| s.as_f64()).unwrap_or(0.0);
                                        let efficiency_score = stats.get("efficiency_score").and_then(|e| e.as_f64()).unwrap_or(0.0);
                                        html! {
                                            <li>
                                                <strong>{category}{": "}</strong>
                                                <span class="funding-amount">{format!("${:.0} avg", avg_funding)}</span>
                                                {format!(", {:.1}% success rate, {:.1} efficiency", success_rate, efficiency_score)}
                                            </li>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>

                    // Cost Per Milestone
                    <section class="criteria-section">
                        <h2 class="section-title">{"COST PER MILESTONE"}</h2>
                        <p class="section-desc">{"Average funding by project status"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(cpm) = cost_per_milestone {
                                    cpm.iter().map(|(status, stats)| {
                                        let avg_funding = stats.get("avg_funding").and_then(|a| a.as_f64()).unwrap_or(0.0);
                                        let avg_rounds = stats.get("avg_rounds").and_then(|r| r.as_f64()).unwrap_or(0.0);
                                        html! {
                                            <li>
                                                <strong>{status}{": "}</strong>
                                                <span class="funding-amount">{format!("${:.0}", avg_funding)}</span>
                                                {format!(" across {:.1} rounds avg", avg_rounds)}
                                            </li>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>

                    // ROI by Rounds
                    <section class="criteria-section">
                        <h2 class="section-title">{"ROI BY FUNDING ROUNDS"}</h2>
                        <p class="section-desc">{"Mainnet success rate by number of rounds"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(roi) = rounds_roi {
                                    let mut rounds: Vec<_> = roi.iter().collect();
                                    rounds.sort_by_key(|(round, _)| round.parse::<u32>().unwrap_or(0));
                                    rounds.iter().map(|(round, stats)| {
                                        let avg_funding = stats.get("avg_funding").and_then(|a| a.as_f64()).unwrap_or(0.0);
                                        let mainnet_rate = stats.get("mainnet_rate").and_then(|m| m.as_f64()).unwrap_or(0.0);
                                        html! {
                                            <li>
                                                <strong>{format!("{} Round{}: ", round, if *round == "1" { "" } else { "s" })}</strong>
                                                <span class="funding-amount">{format!("${:.0} avg", avg_funding)}</span>
                                                {format!(", {:.1}% mainnet rate", mainnet_rate)}
                                            </li>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>
                </div>
            </div>
        }
    } else {
        html! { <div class="module-content"><p>{"Loading efficiency data..."}</p></div> }
    }
}

fn render_geographic(geographic: &Option<Value>) -> Html {
    if let Some(data) = geographic {
        let top_countries = data.get("top_countries").and_then(|t| t.as_array());
        let regional_funding = data.get("regional_funding_density").and_then(|r| r.as_object());
        let underserved = data.get("underserved_regions").and_then(|u| u.as_array());

        html! {
            <div class="module-content">
                <header class="module-header">
                    <h1>{"GEOGRAPHIC MARKET INSIGHTS"}</h1>
                    <p class="module-intro">{"Regional funding patterns, top countries, and underserved markets."}</p>
                </header>

                <div class="criteria-grid">
                    // Top Countries
                    <section class="criteria-section">
                        <h2 class="section-title">{"TOP 20 COUNTRIES BY TOTAL FUNDING"}</h2>
                        <p class="section-desc">{"Countries with highest SCF funding received"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(countries) = top_countries {
                                    countries.iter().take(20).map(|country| {
                                        let name = country.get("country").and_then(|n| n.as_str()).unwrap_or("");
                                        let total_funding = country.get("total_funding").and_then(|t| t.as_f64()).unwrap_or(0.0);
                                        let project_count = country.get("project_count").and_then(|p| p.as_u64()).unwrap_or(0);
                                        let avg_funding = country.get("avg_funding").and_then(|a| a.as_f64()).unwrap_or(0.0);
                                        html! {
                                            <li>
                                                <strong>{name}{": "}</strong>
                                                <span class="funding-amount">{format!("${:.0}", total_funding)}</span>
                                                {format!(" ({} projects, ${:.0} avg)", project_count, avg_funding)}
                                            </li>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>

                    // Regional Funding Density
                    <section class="criteria-section">
                        <h2 class="section-title">{"REGIONAL FUNDING DENSITY"}</h2>
                        <p class="section-desc">{"Total funding and project counts by region"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(regions) = regional_funding {
                                    regions.iter().map(|(region, stats)| {
                                        let total_funding = stats.get("total_funding").and_then(|t| t.as_f64()).unwrap_or(0.0);
                                        let project_count = stats.get("project_count").and_then(|p| p.as_u64()).unwrap_or(0);
                                        let country_count = stats.get("country_count").and_then(|c| c.as_u64()).unwrap_or(0);
                                        html! {
                                            <li>
                                                <strong>{region}{": "}</strong>
                                                <span class="funding-amount">{format!("${:.0}", total_funding)}</span>
                                                {format!(" ({} projects across {} countries)", project_count, country_count)}
                                            </li>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>

                    // Underserved Markets
                    <section class="criteria-section">
                        <h2 class="section-title">{"UNDERSERVED MARKETS"}</h2>
                        <p class="section-desc">{"Regions with growth opportunities"}</p>
                        <div class="keyword-cloud">
                            {
                                if let Some(regions) = underserved {
                                    regions.iter().map(|region| {
                                        let region_str = region.as_str().unwrap_or("");
                                        html! {
                                            <span class="keyword-tag">{region_str}</span>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <p>{"No underserved regions identified"}</p> }
                                }
                            }
                        </div>
                    </section>
                </div>
            </div>
        }
    } else {
        html! { <div class="module-content"><p>{"Loading geographic data..."}</p></div> }
    }
}

fn render_success_patterns(statistical: &Option<Value>) -> Html {
    if let Some(data) = statistical {
        let correlation_analysis = data.get("correlation_analysis").and_then(|c| c.as_object());
        let outliers = data.get("outliers").and_then(|o| o.as_array());
        let distribution = data.get("distribution").and_then(|d| d.as_object());
        let significance_tests = data.get("significance_tests").and_then(|s| s.as_object());

        html! {
            <div class="module-content">
                <header class="module-header">
                    <h1>{"SUCCESS PATTERNS & CORRELATIONS"}</h1>
                    <p class="module-intro">{"Statistical analysis of what actually leads to funding success."}</p>
                </header>

                <div class="criteria-grid">
                    // Correlation Analysis
                    <section class="criteria-section">
                        <h2 class="section-title">{"CORRELATION ANALYSIS"}</h2>
                        <p class="section-desc">{"Factors correlated with funding amounts"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(corr) = correlation_analysis {
                                    corr.iter().map(|(factor, stats)| {
                                        let correlation = stats.get("correlation").and_then(|c| c.as_f64()).unwrap_or(0.0);
                                        let p_value = stats.get("p_value").and_then(|p| p.as_f64()).unwrap_or(0.0);
                                        let significant = p_value < 0.05;
                                        html! {
                                            <li>
                                                <strong>{factor.replace("_", " ")}{": "}</strong>
                                                {format!("{:.3} correlation", correlation)}
                                                {if significant { " (significant)" } else { " (not significant)" }}
                                            </li>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>

                    // Funding Distribution
                    <section class="criteria-section">
                        <h2 class="section-title">{"FUNDING DISTRIBUTION STATS"}</h2>
                        <p class="section-desc">{"Key statistical metrics"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(dist) = distribution {
                                    let mean = dist.get("mean").and_then(|m| m.as_f64()).unwrap_or(0.0);
                                    let median = dist.get("median").and_then(|m| m.as_f64()).unwrap_or(0.0);
                                    let std_dev = dist.get("std_dev").and_then(|s| s.as_f64()).unwrap_or(0.0);
                                    let percentiles = dist.get("percentiles").and_then(|p| p.as_object());

                                    vec![
                                        html! { <li><strong>{"Mean: "}</strong><span class="funding-amount">{format!("${:.0}", mean)}</span></li> },
                                        html! { <li><strong>{"Median: "}</strong><span class="funding-amount">{format!("${:.0}", median)}</span></li> },
                                        html! { <li><strong>{"Std Dev: "}</strong><span class="funding-amount">{format!("${:.0}", std_dev)}</span></li> },
                                        if let Some(p) = percentiles {
                                            let p25 = p.get("25").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                            let p75 = p.get("75").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                            let p95 = p.get("95").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                            html! {
                                                <>
                                                    <li><strong>{"25th percentile: "}</strong><span class="funding-amount">{format!("${:.0}", p25)}</span></li>
                                                    <li><strong>{"75th percentile: "}</strong><span class="funding-amount">{format!("${:.0}", p75)}</span></li>
                                                    <li><strong>{"95th percentile: "}</strong><span class="funding-amount">{format!("${:.0}", p95)}</span></li>
                                                </>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    ].into_iter().collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>

                    // Outliers (Top Funded)
                    <section class="criteria-section">
                        <h2 class="section-title">{"TOP FUNDED PROJECTS (OUTLIERS)"}</h2>
                        <p class="section-desc">{"Exceptionally well-funded projects"}</p>
                        <ul class="criteria-list">
                            {
                                if let Some(out) = outliers {
                                    out.iter().take(8).map(|project| {
                                        let title = project.get("title").and_then(|t| t.as_str()).unwrap_or("");
                                        let funding = project.get("funding").and_then(|f| f.as_f64()).unwrap_or(0.0);
                                        let category = project.get("category").and_then(|c| c.as_str()).unwrap_or("");
                                        html! {
                                            <li>
                                                <strong>{title}{": "}</strong>
                                                <span class="funding-amount">{format!("${:.0}", funding)}</span>
                                                {format!(" ({})", category)}
                                            </li>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! { <li>{"No data available"}</li> }
                                }
                            }
                        </ul>
                    </section>
                </div>
            </div>
        }
    } else {
        html! { <div class="module-content"><p>{"Loading success patterns data..."}</p></div> }
    }
}

fn render_full_handbook(handbook: &Option<Value>, selected_idx: usize, set_selected: Callback<usize>) -> Html {
    if let Some(data) = handbook {
        let metadata = data.get("metadata");
        let sections = data.get("sections").and_then(|s| s.as_array());

        if let Some(secs) = sections {
            let current_section = secs.get(selected_idx);

            // Build navigation items
            let nav_items: Vec<Html> = secs.iter().enumerate().map(|(idx, section)| {
                let section_name = section.get("section_name").and_then(|n| n.as_str()).unwrap_or("Untitled");
                let word_count = section.get("word_count").and_then(|w| w.as_u64()).unwrap_or(0);
                let is_selected = idx == selected_idx;

                let set_selected_clone = set_selected.clone();
                let onclick = Callback::from(move |_| {
                    set_selected_clone.emit(idx);
                });

                html! {
                    <button
                        class={if is_selected { "section-nav-item selected" } else { "section-nav-item" }}
                        onclick={onclick}
                        style={format!("
                            width: 100%;
                            text-align: left;
                            padding: 15px;
                            margin-bottom: 10px;
                            background: {};
                            border: {};
                            border-radius: 12px;
                            cursor: pointer;
                            transition: all 0.3s ease;
                            position: relative;
                        ",
                            if is_selected { "linear-gradient(135deg, rgba(0, 212, 255, 0.15), rgba(0, 150, 200, 0.1))" } else { "rgba(255, 255, 255, 0.03)" },
                            if is_selected { "2px solid rgba(0, 212, 255, 0.6)" } else { "1px solid rgba(255, 255, 255, 0.1)" }
                        )}
                    >
                        <div style="display: flex; align-items: flex-start; gap: 12px;">
                            <span style={format!("
                                font-weight: 700;
                                color: {};
                                font-size: 1.1em;
                                min-width: 28px;
                            ", if is_selected { "#00d4ff" } else { "rgba(255, 255, 255, 0.5)" })}>
                                {format!("{:02}", idx + 1)}
                            </span>
                            <div style="flex: 1;">
                                <div style={format!("
                                    font-weight: {};
                                    color: {};
                                    font-size: 0.95em;
                                    line-height: 1.4;
                                    margin-bottom: 6px;
                                ", if is_selected { "600" } else { "500" }, if is_selected { "#ffffff" } else { "rgba(255, 255, 255, 0.8)" })}>
                                    {section_name}
                                </div>
                                <div style="font-size: 0.75em; opacity: 0.6;">
                                    {format!("{} words • ~{} min", word_count, (word_count / 200).max(1))}
                                </div>
                            </div>
                        </div>
                    </button>
                }
            }).collect();

            html! {
                <div class="handbook-wrapper" style="display: flex; gap: 30px; height: calc(100vh - 200px); min-height: 600px;">
                    // Sidebar Navigation
                    <aside class="handbook-sidebar" style="
                        width: 320px;
                        flex-shrink: 0;
                        overflow-y: auto;
                        background: rgba(0, 0, 0, 0.2);
                        border-radius: 16px;
                        padding: 20px;
                        border: 1px solid rgba(0, 212, 255, 0.1);
                    ">
                        <div class="sidebar-header" style="margin-bottom: 25px; padding-bottom: 20px; border-bottom: 2px solid rgba(0, 212, 255, 0.2);">
                            <h2 style="font-size: 1.3em; color: #00d4ff; margin: 0 0 10px 0;">{"📖 Handbook Sections"}</h2>
                            {
                                if let Some(meta) = metadata {
                                    html! {
                                        <p style="font-size: 0.85em; opacity: 0.7; margin: 0;">
                                            {format!("{} sections • {}k words",
                                                meta.get("total_sections").and_then(|s| s.as_u64()).unwrap_or(0),
                                                meta.get("total_words").and_then(|w| w.as_u64()).unwrap_or(0) / 1000
                                            )}
                                        </p>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>

                        <nav class="section-nav">
                            { for nav_items }
                        </nav>
                    </aside>

                    // Main Content Area
                    <main class="handbook-content" style="
                        flex: 1;
                        overflow-y: auto;
                        background: rgba(0, 0, 0, 0.15);
                        border-radius: 16px;
                        padding: 40px;
                        border: 1px solid rgba(0, 212, 255, 0.1);
                    ">
                        {
                            if let Some(section) = current_section {
                                let section_name = section.get("section_name").and_then(|n| n.as_str()).unwrap_or("Untitled Section");
                                let content = section.get("content").and_then(|c| c.as_str()).unwrap_or("");
                                let word_count = section.get("word_count").and_then(|w| w.as_u64()).unwrap_or(0);

                                html! {
                                    <article class="section-detail">
                                        // Section Header
                                        <header style="margin-bottom: 35px; padding-bottom: 25px; border-bottom: 2px solid rgba(0, 212, 255, 0.2);">
                                            <div style="display: flex; align-items: center; gap: 15px; margin-bottom: 15px;">
                                                <span style="
                                                    display: inline-flex;
                                                    align-items: center;
                                                    justify-content: center;
                                                    width: 50px;
                                                    height: 50px;
                                                    background: linear-gradient(135deg, rgba(0, 212, 255, 0.2), rgba(0, 150, 200, 0.1));
                                                    border: 2px solid rgba(0, 212, 255, 0.4);
                                                    border-radius: 12px;
                                                    font-weight: 700;
                                                    font-size: 1.3em;
                                                    color: #00d4ff;
                                                ">
                                                    {format!("{:02}", selected_idx + 1)}
                                                </span>
                                                <h1 style="
                                                    font-size: 2em;
                                                    color: #ffffff;
                                                    margin: 0;
                                                    font-weight: 700;
                                                    line-height: 1.2;
                                                ">
                                                    {section_name}
                                                </h1>
                                            </div>
                                            <div style="display: flex; gap: 20px; font-size: 0.9em; opacity: 0.7;">
                                                <span>{"📖 "}{word_count}{" words"}</span>
                                                <span>{"⏱️ ~"}{(word_count / 200).max(1)}{" min read"}</span>
                                            </div>
                                        </header>

                                        // Section Content with Better Formatting
                                        <div class="section-text" style="
                                            line-height: 1.9;
                                            font-size: 1.05em;
                                            color: rgba(255, 255, 255, 0.9);
                                        ">
                                            {format_section_content(content)}
                                        </div>

                                        // Navigation Footer
                                        <footer style="
                                            margin-top: 50px;
                                            padding-top: 30px;
                                            border-top: 2px solid rgba(0, 212, 255, 0.2);
                                            display: flex;
                                            justify-content: space-between;
                                            align-items: center;
                                        ">
                                            {
                                                if selected_idx > 0 {
                                                    let prev_idx = selected_idx - 1;
                                                    let set_selected_prev = set_selected.clone();
                                                    let onclick = Callback::from(move |_| {
                                                        set_selected_prev.emit(prev_idx);
                                                    });
                                                    html! {
                                                        <button onclick={onclick} style="
                                                            padding: 12px 24px;
                                                            background: rgba(0, 212, 255, 0.1);
                                                            border: 1px solid rgba(0, 212, 255, 0.3);
                                                            border-radius: 8px;
                                                            color: #00d4ff;
                                                            cursor: pointer;
                                                            font-weight: 600;
                                                            transition: all 0.3s ease;
                                                        ">
                                                            {"← Previous Section"}
                                                        </button>
                                                    }
                                                } else {
                                                    html! { <div></div> }
                                                }
                                            }
                                            {
                                                if selected_idx < secs.len() - 1 {
                                                    let next_idx = selected_idx + 1;
                                                    let set_selected_next = set_selected.clone();
                                                    let onclick = Callback::from(move |_| {
                                                        set_selected_next.emit(next_idx);
                                                    });
                                                    html! {
                                                        <button onclick={onclick} style="
                                                            padding: 12px 24px;
                                                            background: linear-gradient(135deg, rgba(0, 212, 255, 0.2), rgba(0, 150, 200, 0.15));
                                                            border: 1px solid rgba(0, 212, 255, 0.4);
                                                            border-radius: 8px;
                                                            color: #00d4ff;
                                                            cursor: pointer;
                                                            font-weight: 600;
                                                            transition: all 0.3s ease;
                                                        ">
                                                            {"Next Section →"}
                                                        </button>
                                                    }
                                                } else {
                                                    html! {
                                                        <a href="https://communityfund.stellar.org" target="_blank" rel="noopener noreferrer" style="
                                                            padding: 12px 24px;
                                                            background: linear-gradient(135deg, #00d4ff, #0099cc);
                                                            border: none;
                                                            border-radius: 8px;
                                                            color: #000;
                                                            font-weight: 700;
                                                            text-decoration: none;
                                                            display: inline-block;
                                                        ">
                                                            {"Apply Now 🚀"}
                                                        </a>
                                                    }
                                                }
                                            }
                                        </footer>
                                    </article>
                                }
                            } else {
                                html! { <p>{"Section not found"}</p> }
                            }
                        }
                    </main>
                </div>
            }
        } else {
            html! { <div class="module-content"><p>{"No handbook sections available"}</p></div> }
        }
    } else {
        html! {
            <div style="display: flex; align-items: center; justify-content: center; height: 400px; flex-direction: column; gap: 20px;">
                <div style="
                    width: 60px;
                    height: 60px;
                    border: 4px solid rgba(0, 212, 255, 0.3);
                    border-top-color: #00d4ff;
                    border-radius: 50%;
                    animation: spin 1s linear infinite;
                "></div>
                <p style="font-size: 1.1em; opacity: 0.8;">{"Loading Stellar Community Fund Handbook..."}</p>
            </div>
        }
    }
}

// Helper function to format section content with better structure
fn format_section_content(content: &str) -> Html {
    let mut elements = Vec::new();
    let paragraphs: Vec<&str> = content.split("\n\n").collect();

    for para in paragraphs {
        let para = para.trim();
        if para.is_empty() {
            continue;
        }

        // Check if it's a heading (ALL CAPS line)
        if para.len() < 100 && para == para.to_uppercase() && !para.contains('.') && !para.starts_with("1.") {
            elements.push(html! {
                <h2 style="
                    font-size: 1.6em;
                    color: #00d4ff;
                    font-weight: 700;
                    margin: 35px 0 20px 0;
                    padding-bottom: 12px;
                    border-bottom: 2px solid rgba(0, 212, 255, 0.2);
                ">{para}</h2>
            });
        }
        // Check if it's a list (starts with bullet or number)
        else if para.starts_with("•") || para.starts_with("-") || para.starts_with("○") || para.lines().any(|l| l.trim().starts_with("•") || l.trim().starts_with("-")) {
            let items: Vec<&str> = para.lines().filter(|l| !l.trim().is_empty()).collect();
            elements.push(html! {
                <ul style="
                    margin: 20px 0;
                    padding-left: 30px;
                    list-style: none;
                ">
                    {
                        items.iter().map(|item| {
                            let clean = item.trim().trim_start_matches(&['•', '-', '○'][..]).trim();
                            html! {
                                <li style="
                                    margin: 12px 0;
                                    padding-left: 25px;
                                    position: relative;
                                    line-height: 1.7;
                                " class="before:content-['▸'] before:absolute before:left-0 before:color-[#00d4ff]">
                                    <span style="display: inline-block; margin-right: 12px; color: #00d4ff; font-weight: 700;">{"▸"}</span>
                                    {clean}
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            });
        }
        // Check if it's a numbered section (starts with number.)
        else if para.starts_with(|c: char| c.is_numeric()) && para.contains('.') && para.len() < 200 {
            elements.push(html! {
                <h3 style="
                    font-size: 1.3em;
                    color: #ffffff;
                    font-weight: 600;
                    margin: 30px 0 15px 0;
                ">{para}</h3>
            });
        }
        // Regular paragraph
        else {
            elements.push(html! {
                <p style="
                    margin: 16px 0;
                    line-height: 1.85;
                    text-align: justify;
                ">{para}</p>
            });
        }
    }

    html! {
        <div class="formatted-content">
            { for elements }
        </div>
    }
}
