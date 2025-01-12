use super::Matcher;

#[derive(Clone)]
enum CombineUnaryMatcherKind {
    Not,
}

#[derive(Clone)]
pub struct UnaryCombineMatcher<T> {
    matcher: Box<dyn Matcher<T>>,
    kind: CombineUnaryMatcherKind,
}

impl<T: Clone> UnaryCombineMatcher<T> {
    pub fn not(matcher: Box<dyn Matcher<T>>) -> Self {
        UnaryCombineMatcher {
            matcher,
            kind: CombineUnaryMatcherKind::Not,
        }
    }
}

impl<T: Clone> Matcher<T> for UnaryCombineMatcher<T> {
    fn is_match(&self, node: &T) -> bool {
        match &self.kind {
            CombineUnaryMatcherKind::Not => !self.matcher.is_match(node),
        }
    }
}
