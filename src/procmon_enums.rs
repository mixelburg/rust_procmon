pub enum Action {
    Include,
    Exclude,
}

impl Action {
    pub(crate) fn value(&self) -> &str {
        match *self {
            Action::Include => "Include",
            Action::Exclude => "Exclude",
        }
    }
}

pub enum Relation {
    Is,
    IsNot,
    LessThan,
    MoreThan,
    BeginsWith,
    EndsWith,
    Contains,
    Excludes,
}

impl Relation {
    pub(crate) fn value(&self) -> &str {
        match *self {
            Relation::Is => "is",
            Relation::IsNot => "is not",
            Relation::LessThan => "less than",
            Relation::MoreThan => "more than",
            Relation::BeginsWith => "begins with",
            Relation::EndsWith => "ends with",
            Relation::Contains => "contains",
            Relation::Excludes => "excludes",
        }
    }
}


pub enum Column {
    ProcessName,
    ProcessId,
    ProcessParentId,
}

impl Column {
    pub(crate) fn value(&self) -> &str {
        match *self {
            Column::ProcessName => "Process Name",
            Column::ProcessId => "PID",
            Column::ProcessParentId => "Parent PID",
        }
    }
}

