#[derive(Clone, clap::ValueEnum)]
pub enum SortBy {
    #[clap(name = "created")]
    CreatedAt,
    #[clap(name = "updated")]
    UpdatedAt,
}

#[derive(Clone, clap::ValueEnum, strum::EnumIter, PartialEq, Eq, Hash)]
pub enum IssueState {
    Started,
    Unstarted,
    Backlog,
    Completed,
    Canceled,
}
