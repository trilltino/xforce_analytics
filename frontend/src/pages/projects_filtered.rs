use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use serde_json::Value;
use crate::components::layout::Layout;
use crate::components::project_card::ProjectCard;
use shared::{ProjectsResponse, ProjectFilter};

#[function_component(ProjectsFiltered)]
pub fn projects_filtered() -> Html {
    let projects_data = use_state(|| Option::<ProjectsResponse>::None);
    let loading = use_state(|| true);

    // Filter states
    let search_query = use_state(|| String::new());
    let selected_categories = use_state(|| Vec::<String>::new());
    let funding_tier = use_state(|| String::from("all"));
    let soroban_only = use_state(|| false);
    let stellar_only = use_state(|| false);
    let has_mainnet = use_state(|| false);
    let has_github = use_state(|| false);
    let selected_countries = use_state(|| Vec::<String>::new());
    let selected_programs = use_state(|| Vec::<String>::new());
    let min_rounds = use_state(|| 0u32);
    let sort_by = use_state(|| String::from("funding_desc"));
    let page = use_state(|| 1usize);
    let show_advanced = use_state(|| false);

    // Fetch projects with current filters
    {
        let projects_data = projects_data.clone();
        let loading = loading.clone();
        let search = (*search_query).clone();
        let cats = (*selected_categories).clone();
        let tier = (*funding_tier).clone();
        let soroban = *soroban_only;
        let stellar = *stellar_only;
        let mainnet = *has_mainnet;
        let github = *has_github;
        let countries = (*selected_countries).clone();
        let programs = (*selected_programs).clone();
        let rounds = *min_rounds;
        let sort = (*sort_by).clone();
        let current_page = *page;

        use_effect_with((search.clone(), cats.clone(), tier.clone(), soroban, stellar, mainnet, github, countries.clone(), programs.clone(), rounds, sort.clone(), current_page), move |_| {
            spawn_local(async move {
                loading.set(true);

                let mut filter = ProjectFilter::default();
                filter.page = Some(current_page);
                filter.per_page = Some(12);
                filter.sort_by = Some(sort.clone());

                if !search.is_empty() {
                    filter.search_query = Some(search);
                }
                if !cats.is_empty() {
                    filter.categories = Some(cats);
                }
                if tier != "all" {
                    filter.funding_tier = Some(tier);
                }
                filter.soroban_only = Some(soroban);
                filter.stellar_only = Some(stellar);
                filter.has_mainnet = Some(mainnet);
                filter.has_github = Some(github);
                if !countries.is_empty() {
                    filter.countries = Some(countries);
                }
                if !programs.is_empty() {
                    filter.programs = Some(programs);
                }
                if rounds > 0 {
                    filter.min_rounds = Some(rounds);
                }

                // Build query string
                let query_params = build_query_params(&filter);

                match Request::get(&format!("/api/projects?{}", query_params))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(data) = response.json::<Value>().await {
                            if let Some(response_data) = data.get("data") {
                                if let Ok(projects_resp) = serde_json::from_value::<ProjectsResponse>(response_data.clone()) {
                                    projects_data.set(Some(projects_resp));
                                }
                            }
                        }
                        loading.set(false);
                    }
                    Err(_) => {
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // Search input handler
    let on_search = {
        let search_query = search_query.clone();
        let page = page.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                search_query.set(input.value());
                page.set(1); // Reset to first page on search
            }
        })
    };

    // Category toggle
    let toggle_category = {
        let selected_categories = selected_categories.clone();
        let page = page.clone();
        move |category: String| {
            let mut cats = (*selected_categories).clone();
            if cats.contains(&category) {
                cats.retain(|c| c != &category);
            } else {
                cats.push(category);
            }
            selected_categories.set(cats);
            page.set(1);
        }
    };

    // Country toggle
    let toggle_country = {
        let selected_countries = selected_countries.clone();
        let page = page.clone();
        move |country: String| {
            let mut countries = (*selected_countries).clone();
            if countries.contains(&country) {
                countries.retain(|c| c != &country);
            } else {
                countries.push(country);
            }
            selected_countries.set(countries);
            page.set(1);
        }
    };

    // Program toggle
    let toggle_program = {
        let selected_programs = selected_programs.clone();
        let page = page.clone();
        move |program: String| {
            let mut progs = (*selected_programs).clone();
            if progs.contains(&program) {
                progs.retain(|p| p != &program);
            } else {
                progs.push(program);
            }
            selected_programs.set(progs);
            page.set(1);
        }
    };

    // Pagination
    let next_page = {
        let page = page.clone();
        let data = (*projects_data).clone();
        Callback::from(move |_| {
            if let Some(resp) = &data {
                if *page < resp.total_pages {
                    page.set(*page + 1);
                }
            }
        })
    };

    let prev_page = {
        let page = page.clone();
        Callback::from(move |_| {
            if *page > 1 {
                page.set(*page - 1);
            }
        })
    };

    html! {
        <Layout>
            <div class="projects_filtered_page">
                <div class="page_header">
                    <h1>{"ECOSYSTEM PROJECTS"}</h1>
                    <p class="subtitle">{"FILTER AND EXPLORE 443 SDF-FUNDED PROJECTS"}</p>
                </div>

                // Filter Panel
                <div class="filter_container">
                    // Search Bar
                    <div class="search_bar">
                        <input
                            type="text"
                            class="search_input"
                            placeholder="SEARCH BY PROJECT NAME, COMPANY, OR DESCRIPTION..."
                            value={(*search_query).clone()}
                            oninput={on_search}
                        />
                    </div>

                    // Quick Filters
                    <div class="quick_filters">
                        <div class="filter_group">
                            <span class="filter_label">{"FUNDING TIER:"}</span>
                            <select
                                class="filter_select"
                                value={(*funding_tier).clone()}
                                onchange={{
                                    let funding_tier = funding_tier.clone();
                                    let page = page.clone();
                                    Callback::from(move |e: Event| {
                                        if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                            funding_tier.set(select.value());
                                            page.set(1);
                                        }
                                    })
                                }}
                            >
                                <option value="all">{"ALL TIERS"}</option>
                                <option value="0-50k">{"$0 - $50K"}</option>
                                <option value="50k-100k">{"$50K - $100K"}</option>
                                <option value="100k-150k">{"$100K - $150K"}</option>
                                <option value="150k+">{"$150K+"}</option>
                            </select>
                        </div>

                        <div class="filter_group">
                            <span class="filter_label">{"SORT BY:"}</span>
                            <select
                                class="filter_select"
                                value={(*sort_by).clone()}
                                onchange={{
                                    let sort_by = sort_by.clone();
                                    Callback::from(move |e: Event| {
                                        if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                            sort_by.set(select.value());
                                        }
                                    })
                                }}
                            >
                                <option value="funding_desc">{"FUNDING: HIGH TO LOW"}</option>
                                <option value="funding_asc">{"FUNDING: LOW TO HIGH"}</option>
                                <option value="name_asc">{"NAME: A TO Z"}</option>
                                <option value="name_desc">{"NAME: Z TO A"}</option>
                            </select>
                        </div>

                        <button
                            class="advanced_toggle"
                            onclick={{
                                let show = show_advanced.clone();
                                Callback::from(move |_| show.set(!*show))
                            }}
                        >
                            {if *show_advanced { "HIDE ADVANCED FILTERS" } else { "SHOW ADVANCED FILTERS" }}
                        </button>
                    </div>

                    // Advanced Filters (collapsible)
                    if *show_advanced {
                        <div class="advanced_filters">
                            // Categories
                            <div class="filter_section">
                                <h3 class="section_title">{"CATEGORIES"}</h3>
                                <div class="checkbox_grid">
                                    {for vec![
                                        "Financial Protocols",
                                        "Infrastructure & Services",
                                        "Applications",
                                        "Developer Tooling",
                                        "Payments",
                                        "DeFi",
                                        "NFT & Collectibles",
                                        "Data & Analytics"
                                    ].iter().map(|cat| {
                                        let category = cat.to_string();
                                        let is_selected = (*selected_categories).contains(&category);
                                        let on_click = {
                                            let cat = category.clone();
                                            let toggle = toggle_category.clone();
                                            Callback::from(move |_| toggle(cat.clone()))
                                        };
                                        html! {
                                            <label class={classes!("filter_checkbox", is_selected.then(|| "active"))}>
                                                <input
                                                    type="checkbox"
                                                    checked={is_selected}
                                                    onchange={on_click}
                                                />
                                                <span>{category}</span>
                                            </label>
                                        }
                                    })}
                                </div>
                            </div>

                            // Technology
                            <div class="filter_section">
                                <h3 class="section_title">{"TECHNOLOGY"}</h3>
                                <div class="checkbox_grid">
                                    <label class={classes!("filter_checkbox", (*soroban_only).then(|| "active"))}>
                                        <input
                                            type="checkbox"
                                            checked={*soroban_only}
                                            onchange={{
                                                let soroban = soroban_only.clone();
                                                let page = page.clone();
                                                Callback::from(move |_| {
                                                    soroban.set(!*soroban);
                                                    page.set(1);
                                                })
                                            }}
                                        />
                                        <span>{"SOROBAN PROJECTS"}</span>
                                    </label>
                                    <label class={classes!("filter_checkbox", (*stellar_only).then(|| "active"))}>
                                        <input
                                            type="checkbox"
                                            checked={*stellar_only}
                                            onchange={{
                                                let stellar = stellar_only.clone();
                                                let page = page.clone();
                                                Callback::from(move |_| {
                                                    stellar.set(!*stellar);
                                                    page.set(1);
                                                })
                                            }}
                                        />
                                        <span>{"STELLAR-ONLY"}</span>
                                    </label>
                                    <label class={classes!("filter_checkbox", (*has_mainnet).then(|| "active"))}>
                                        <input
                                            type="checkbox"
                                            checked={*has_mainnet}
                                            onchange={{
                                                let mainnet = has_mainnet.clone();
                                                let page = page.clone();
                                                Callback::from(move |_| {
                                                    mainnet.set(!*mainnet);
                                                    page.set(1);
                                                })
                                            }}
                                        />
                                        <span>{"MAINNET LIVE"}</span>
                                    </label>
                                    <label class={classes!("filter_checkbox", (*has_github).then(|| "active"))}>
                                        <input
                                            type="checkbox"
                                            checked={*has_github}
                                            onchange={{
                                                let github = has_github.clone();
                                                let page = page.clone();
                                                Callback::from(move |_| {
                                                    github.set(!*github);
                                                    page.set(1);
                                                })
                                            }}
                                        />
                                        <span>{"OPEN SOURCE"}</span>
                                    </label>
                                </div>
                            </div>

                            // Geographic
                            <div class="filter_section">
                                <h3 class="section_title">{"GEOGRAPHIC"}</h3>
                                <div class="checkbox_grid">
                                    {for vec!["United States", "Portugal", "Germany", "United Kingdom", "Netherlands", "Spain", "France", "Singapore", "Chile", "Colombia"]
                                        .iter().map(|country| {
                                        let country_name = country.to_string();
                                        let is_selected = (*selected_countries).contains(&country_name);
                                        let on_click = {
                                            let c = country_name.clone();
                                            let toggle = toggle_country.clone();
                                            Callback::from(move |_| toggle(c.clone()))
                                        };
                                        html! {
                                            <label class={classes!("filter_checkbox", is_selected.then(|| "active"))}>
                                                <input
                                                    type="checkbox"
                                                    checked={is_selected}
                                                    onchange={on_click}
                                                />
                                                <span>{country_name}</span>
                                            </label>
                                        }
                                    })}
                                </div>
                            </div>

                            // Programs
                            <div class="filter_section">
                                <h3 class="section_title">{"PROGRAMS"}</h3>
                                <div class="checkbox_grid">
                                    {for vec!["Kickstart", "Build", "Growth Hack", "Audit Bank"]
                                        .iter().map(|prog| {
                                        let program = prog.to_string();
                                        let is_selected = (*selected_programs).contains(&program);
                                        let on_click = {
                                            let p = program.clone();
                                            let toggle = toggle_program.clone();
                                            Callback::from(move |_| toggle(p.clone()))
                                        };
                                        html! {
                                            <label class={classes!("filter_checkbox", is_selected.then(|| "active"))}>
                                                <input
                                                    type="checkbox"
                                                    checked={is_selected}
                                                    onchange={on_click}
                                                />
                                                <span>{program}</span>
                                            </label>
                                        }
                                    })}
                                </div>
                            </div>

                            // Rounds
                            <div class="filter_section">
                                <h3 class="section_title">{"MINIMUM FUNDING ROUNDS"}</h3>
                                <select
                                    class="filter_select_full"
                                    value={min_rounds.to_string()}
                                    onchange={{
                                        let min_rounds = min_rounds.clone();
                                        let page = page.clone();
                                        Callback::from(move |e: Event| {
                                            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                if let Ok(rounds) = select.value().parse::<u32>() {
                                                    min_rounds.set(rounds);
                                                    page.set(1);
                                                }
                                            }
                                        })
                                    }}
                                >
                                    <option value="0">{"ANY"}</option>
                                    <option value="1">{"1+ ROUNDS"}</option>
                                    <option value="2">{"2+ ROUNDS"}</option>
                                    <option value="3">{"3+ ROUNDS"}</option>
                                    <option value="4">{"4+ ROUNDS"}</option>
                                </select>
                            </div>
                        </div>
                    }

                    // Active Filters Summary
                    if (!(*selected_categories).is_empty()
                        || !(*selected_countries).is_empty()
                        || !(*selected_programs).is_empty()
                        || *soroban_only
                        || *stellar_only
                        || *has_mainnet
                        || *has_github
                        || *min_rounds > 0
                        || !(*search_query).is_empty()
                        || (*funding_tier).as_str() != "all")
                    {
                        <div class="active_filters">
                            <span class="active_label">{"ACTIVE FILTERS:"}</span>
                            {for (*selected_categories).iter().map(|cat| {
                                html! { <span class="filter_tag">{cat}</span> }
                            })}
                            {for (*selected_countries).iter().map(|country| {
                                html! { <span class="filter_tag">{country}</span> }
                            })}
                            {for (*selected_programs).iter().map(|prog| {
                                html! { <span class="filter_tag">{prog}</span> }
                            })}
                            {if *soroban_only { html! { <span class="filter_tag">{"SOROBAN"}</span> } } else { html! {} }}
                            {if *stellar_only { html! { <span class="filter_tag">{"STELLAR-ONLY"}</span> } } else { html! {} }}
                            {if *has_mainnet { html! { <span class="filter_tag">{"MAINNET"}</span> } } else { html! {} }}
                            {if *has_github { html! { <span class="filter_tag">{"OPEN SOURCE"}</span> } } else { html! {} }}
                            {if *min_rounds > 0 { html! { <span class="filter_tag">{format!("{}+ ROUNDS", *min_rounds)}</span> } } else { html! {} }}
                            <button class="clear_filters" onclick={{
                                let search = search_query.clone();
                                let cats = selected_categories.clone();
                                let tier = funding_tier.clone();
                                let soroban = soroban_only.clone();
                                let stellar = stellar_only.clone();
                                let mainnet = has_mainnet.clone();
                                let github = has_github.clone();
                                let countries = selected_countries.clone();
                                let programs = selected_programs.clone();
                                let rounds = min_rounds.clone();
                                let p = page.clone();
                                Callback::from(move |_| {
                                    search.set(String::new());
                                    cats.set(Vec::new());
                                    tier.set(String::from("all"));
                                    soroban.set(false);
                                    stellar.set(false);
                                    mainnet.set(false);
                                    github.set(false);
                                    countries.set(Vec::new());
                                    programs.set(Vec::new());
                                    rounds.set(0);
                                    p.set(1);
                                })
                            }}>{"CLEAR ALL"}</button>
                        </div>
                    }
                </div>

                // Results
                <div class="results_container">
                    if *loading {
                        <div class="loading_state">
                            <div class="spinner"></div>
                            <p>{"LOADING PROJECTS..."}</p>
                        </div>
                    } else if let Some(data) = (*projects_data).clone() {
                        <>
                            <div class="results_header">
                                <h2>{format!("{} PROJECTS FOUND", data.total)}</h2>
                                <span class="page_info">{format!("PAGE {} OF {}", data.page, data.total_pages)}</span>
                            </div>

                            <div class="projects_grid">
                                {for data.projects.iter().map(|project| {
                                    html! { <ProjectCard project={project.clone()} /> }
                                })}
                            </div>

                            <div class="pagination">
                                <button
                                    class="page_btn"
                                    disabled={data.page == 1}
                                    onclick={prev_page}
                                >
                                    {"PREVIOUS"}
                                </button>
                                <span class="page_indicator">
                                    {format!("PAGE {} OF {}", data.page, data.total_pages)}
                                </span>
                                <button
                                    class="page_btn"
                                    disabled={data.page >= data.total_pages}
                                    onclick={next_page}
                                >
                                    {"NEXT"}
                                </button>
                            </div>
                        </>
                    } else {
                        <div class="error_state">
                            <p>{"FAILED TO LOAD PROJECTS"}</p>
                        </div>
                    }
                </div>
            </div>
        </Layout>
    }
}

fn build_query_params(filter: &ProjectFilter) -> String {
    let mut params = Vec::new();

    if let Some(ref query) = filter.search_query {
        params.push(format!("search_query={}", query.replace(" ", "%20")));
    }
    if let Some(ref cats) = filter.categories {
        for cat in cats {
            params.push(format!("categories={}", cat.replace(" ", "%20").replace("&", "%26")));
        }
    }
    if let Some(ref tier) = filter.funding_tier {
        params.push(format!("funding_tier={}", tier));
    }
    if let Some(soroban) = filter.soroban_only {
        params.push(format!("soroban_only={}", soroban));
    }
    if let Some(stellar) = filter.stellar_only {
        params.push(format!("stellar_only={}", stellar));
    }
    if let Some(mainnet) = filter.has_mainnet {
        params.push(format!("has_mainnet={}", mainnet));
    }
    if let Some(github) = filter.has_github {
        params.push(format!("has_github={}", github));
    }
    if let Some(ref countries) = filter.countries {
        for country in countries {
            params.push(format!("countries={}", country.replace(" ", "%20")));
        }
    }
    if let Some(ref programs) = filter.programs {
        for program in programs {
            params.push(format!("programs={}", program.replace(" ", "%20")));
        }
    }
    if let Some(rounds) = filter.min_rounds {
        params.push(format!("min_rounds={}", rounds));
    }
    if let Some(ref sort) = filter.sort_by {
        params.push(format!("sort_by={}", sort));
    }
    if let Some(page) = filter.page {
        params.push(format!("page={}", page));
    }
    if let Some(per_page) = filter.per_page {
        params.push(format!("per_page={}", per_page));
    }

    params.join("&")
}
