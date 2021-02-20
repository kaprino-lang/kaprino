use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

pub mod exprs;
pub mod factors;
pub mod functions;
pub mod statements;
