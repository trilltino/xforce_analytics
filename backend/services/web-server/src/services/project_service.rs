use shared::{Project, ProjectFilter, ProjectSearchRequest, ProjectsResponse};
use lib_web::AppError;

pub struct ProjectService;

impl ProjectService {
    /// List projects with filtering and pagination
    pub async fn list_projects(
        projects: &[Project],
        filter: ProjectFilter,
    ) -> Result<ProjectsResponse, AppError> {
        let mut filtered: Vec<&Project> = projects.iter().collect();

        // Apply filters
        if let Some(category) = &filter.category {
            filtered.retain(|p| {
                p.category
                    .as_ref()
                    .map(|c| c.to_lowercase().contains(&category.to_lowercase()))
                    .unwrap_or(false)
            });
        }

        if let Some(project_type) = &filter.project_type {
            filtered.retain(|p| {
                p.project_type
                    .as_ref()
                    .map(|pt| pt.to_lowercase().contains(&project_type.to_lowercase()))
                    .unwrap_or(false)
            });
        }

        if let Some(true) = filter.soroban_only {
            filtered.retain(|p| p.uses_soroban());
        }

        if let Some(min) = filter.min_funding {
            filtered.retain(|p| {
                p.get_funding_amount_numeric()
                    .map(|amount| amount >= min)
                    .unwrap_or(false)
            });
        }

        if let Some(max) = filter.max_funding {
            filtered.retain(|p| {
                p.get_funding_amount_numeric()
                    .map(|amount| amount <= max)
                    .unwrap_or(false)
            });
        }

        if let Some(query) = &filter.search_query {
            let query_lower = query.to_lowercase();
            filtered.retain(|p| {
                p.title.to_lowercase().contains(&query_lower)
                    || p.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            });
        }

        let total = filtered.len();
        let page = filter.page.unwrap_or(1);
        let per_page = filter.per_page.unwrap_or(20);
        let total_pages = (total + per_page - 1) / per_page;

        // Paginate
        let start = (page - 1) * per_page;
        let end = (start + per_page).min(total);
        let paginated: Vec<Project> = filtered[start..end].iter().map(|&p| p.clone()).collect();

        Ok(ProjectsResponse {
            projects: paginated,
            total,
            page,
            per_page,
            total_pages,
        })
    }

    /// Get a single project by title
    pub async fn get_project(
        projects: &[Project],
        id: &str,
    ) -> Result<Project, AppError> {
        projects
            .iter()
            .find(|p| p.title == id)
            .cloned()
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))
    }

    /// Advanced filter projects
    pub async fn filter_projects(
        projects: &[Project],
        filter: ProjectFilter,
    ) -> Result<ProjectsResponse, AppError> {
        let mut filtered: Vec<&Project> = projects
            .iter()
            .filter(|p| Self::matches_filter(p, &filter))
            .collect();

        // Apply sorting
        if let Some(sort_by) = &filter.sort_by {
            match sort_by.as_str() {
                "funding_desc" => filtered.sort_by(|a, b| {
                    b.get_funding_amount_numeric().unwrap_or(0.0)
                        .partial_cmp(&a.get_funding_amount_numeric().unwrap_or(0.0))
                        .unwrap()
                }),
                "funding_asc" => filtered.sort_by(|a, b| {
                    a.get_funding_amount_numeric().unwrap_or(0.0)
                        .partial_cmp(&b.get_funding_amount_numeric().unwrap_or(0.0))
                        .unwrap()
                }),
                "name_asc" => filtered.sort_by(|a, b| a.title.cmp(&b.title)),
                "name_desc" => filtered.sort_by(|a, b| b.title.cmp(&a.title)),
                _ => {} // Default ordering
            }
        }

        // Pagination
        let page = filter.page.unwrap_or(1);
        let per_page = filter.per_page.unwrap_or(20);
        let total = filtered.len();
        let total_pages = (total as f64 / per_page as f64).ceil() as usize;

        let start = (page - 1) * per_page;
        let end = (start + per_page).min(total);

        let paginated_projects: Vec<Project> = filtered[start..end]
            .iter()
            .map(|&p| p.clone())
            .collect();

        Ok(ProjectsResponse {
            projects: paginated_projects,
            total,
            page,
            per_page,
            total_pages,
        })
    }

    fn matches_filter(project: &Project, filter: &ProjectFilter) -> bool {
        // Text search
        if let Some(query) = &filter.search_query {
            let query_lower = query.to_lowercase();
            let matches = project.title.to_lowercase().contains(&query_lower)
                || project.description.as_ref().map(|d| d.to_lowercase().contains(&query_lower)).unwrap_or(false)
                || project.company.as_ref().map(|c| c.to_lowercase().contains(&query_lower)).unwrap_or(false);
            if !matches {
                return false;
            }
        }

        // Category filter (single or multiple)
        if let Some(categories) = &filter.categories {
            if !categories.is_empty() {
                let matches = project.category.as_ref().map(|c| {
                    categories.iter().any(|cat| c.to_lowercase().contains(&cat.to_lowercase()))
                }).unwrap_or(false);
                if !matches {
                    return false;
                }
            }
        } else if let Some(category) = &filter.category {
            let matches = project.category.as_ref().map(|c| c.to_lowercase().contains(&category.to_lowercase())).unwrap_or(false);
            if !matches {
                return false;
            }
        }

        // Project type filter
        if let Some(ptype) = &filter.project_type {
            let matches = project.project_type.as_ref().map(|pt| pt.to_lowercase() == ptype.to_lowercase()).unwrap_or(false);
            if !matches {
                return false;
            }
        }

        // Funding range
        if let Some(min) = filter.min_funding {
            if project.get_funding_amount_numeric().unwrap_or(0.0) < min {
                return false;
            }
        }
        if let Some(max) = filter.max_funding {
            if project.get_funding_amount_numeric().unwrap_or(0.0) > max {
                return false;
            }
        }

        // Funding tier
        if let Some(tier) = &filter.funding_tier {
            let funding = project.get_funding_amount_numeric().unwrap_or(0.0);
            let matches = match tier.as_str() {
                "0-50k" => funding < 50000.0,
                "50k-100k" => funding >= 50000.0 && funding < 100000.0,
                "100k-150k" => funding >= 100000.0 && funding < 150000.0,
                "150k+" => funding >= 150000.0,
                _ => true,
            };
            if !matches {
                return false;
            }
        }

        // Technology filters
        if let Some(soroban) = filter.soroban_only {
            if soroban && !project.uses_soroban() {
                return false;
            }
        }
        if let Some(stellar) = filter.stellar_only {
            if stellar && project.other_chains.is_some() {
                return false;
            }
        }
        if let Some(has_gh) = filter.has_github {
            if has_gh && project.github.is_none() {
                return false;
            }
        }
        if let Some(has_mn) = filter.has_mainnet {
            if has_mn {
                let is_mainnet = project.integration_status.as_ref().map(|s| s.to_lowercase().contains("mainnet")).unwrap_or(false);
                if !is_mainnet {
                    return false;
                }
            }
        }
        if let Some(is_os) = filter.is_open_source {
            if is_os {
                let has_os = project.open_source.as_ref().map(|o| o.to_lowercase() == "yes" || o == "true").unwrap_or(false);
                if !has_os {
                    return false;
                }
            }
        }

        // Geographic filters
        if let Some(countries) = &filter.countries {
            if !countries.is_empty() {
                let matches = project.country.as_ref().map(|c| {
                    countries.iter().any(|country| c.to_lowercase().contains(&country.to_lowercase()))
                }).unwrap_or(false);
                if !matches {
                    return false;
                }
            }
        } else if let Some(country) = &filter.country {
            let matches = project.country.as_ref().map(|c| c.to_lowercase().contains(&country.to_lowercase())).unwrap_or(false);
            if !matches {
                return false;
            }
        }

        // Programs filter
        if let Some(programs) = &filter.programs {
            if !programs.is_empty() {
                let matches = project.programs.as_ref().map(|p| {
                    programs.iter().any(|prog| p.to_lowercase().contains(&prog.to_lowercase()))
                }).unwrap_or(false);
                if !matches {
                    return false;
                }
            }
        }

        // Rounds filter
        if let Some(min_rounds) = filter.min_rounds {
            let rounds_count = project.rounds.as_ref()
                .and_then(|r| r.split(',').count().try_into().ok())
                .unwrap_or(0u32);
            if rounds_count < min_rounds {
                return false;
            }
        }

        // Status filter
        if let Some(status) = &filter.status {
            let matches = project.status.as_ref().map(|s| s.to_lowercase().contains(&status.to_lowercase())).unwrap_or(false);
            if !matches {
                return false;
            }
        }

        // Social presence filters
        if let Some(has_web) = filter.has_website {
            if has_web && project.website.is_none() {
                return false;
            }
        }
        if let Some(has_trac) = filter.has_traction {
            if has_trac && project.traction.is_none() {
                return false;
            }
        }

        // Quarter/Year filters
        if let Some(quarters) = &filter.quarters {
            if !quarters.is_empty() {
                let matches = project.quarters.as_ref().map(|q| {
                    quarters.iter().any(|quarter| q.contains(quarter))
                }).unwrap_or(false);
                if !matches {
                    return false;
                }
            }
        }

        true
    }

    /// Search projects (legacy)
    pub async fn search_projects(
        projects: &[Project],
        req: ProjectSearchRequest,
    ) -> Result<Vec<Project>, AppError> {
        let query_lower = req.query.to_lowercase();
        let limit = req.limit.unwrap_or(10);

        let results: Vec<&Project> = projects
            .iter()
            .filter(|p| {
                let matches_query = p.title.to_lowercase().contains(&query_lower)
                    || p.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query_lower))
                        .unwrap_or(false);

                let matches_category = req
                    .category
                    .as_ref()
                    .map(|cat| {
                        p.category
                            .as_ref()
                            .map(|c| c.to_lowercase().contains(&cat.to_lowercase()))
                            .unwrap_or(false)
                    })
                    .unwrap_or(true);

                matches_query && matches_category
            })
            .take(limit)
            .collect();

        Ok(results.into_iter().cloned().collect())
    }
}
