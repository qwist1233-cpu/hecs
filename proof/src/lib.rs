//! Issue #481 reproducer: the code from the issue report, verbatim.
use hecs::Query;

pub struct Transform;
pub struct DamageBody;
pub struct DamageSource;

#[derive(Query)]
pub struct DamageEntity<'a> {
    transform: &'a Transform,
    body: &'a mut DamageBody,
    source: Option<&'a mut DamageSource>,
}

#[derive(Query)]
pub enum EitherEntity<'a> {
    Body {
        transform: &'a Transform,
        body: &'a mut DamageBody,
    },
    Source {
        source: &'a mut DamageSource,
    },
}
