pub type Result<T> = std::result::Result<T, JetError>;

pub enum JetError {
    OutsideProject,
    IncompleteSiteConfig,
    InvalidBaseURL,
    FailedToCreateArticleFile,
    FailedToCreateHomepageFile,
    TemplateNotFound,
}
