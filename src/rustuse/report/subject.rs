#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReportSubject {
    Facade,
    Fleet,
    // Crate,
}

impl ReportSubject {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Facade => "facade",
            Self::Fleet => "fleet",
            // Self::Crate => "crate",
        }
    }
}
