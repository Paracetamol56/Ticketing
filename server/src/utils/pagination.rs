use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    page: Option<i64>,
    limit: Option<i64>,
}

impl PaginationQuery {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(page) = self.page {
            if page < 1 {
                return Err("Page must be greater than 0".to_string());
            }
        }
        if let Some(limit) = self.limit {
            if limit < 1 {
                return Err("Limit must be greater than 0".to_string());
            }
            if limit > 100 {
                return Err("Limit must not exceed 100".to_string());
            }
        }
        Ok(())
    }

    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).max(1) as u32
    }
    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(10).clamp(1, 100) as u32
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total_items: u32,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, page: u32, limit: u32, total_items: u32) -> Self {
        let total_pages = ((total_items as f64) / (limit as f64)).ceil() as u32;
        Self {
            items,
            page,
            limit,
            total_items,
            total_pages,
        }
    }
}
