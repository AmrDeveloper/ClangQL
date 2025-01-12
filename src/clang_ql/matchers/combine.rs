use super::Matcher;

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Clone)]
enum CombineMatcherKind {
    OneOf,
    AllOf,
    NoneOf,
}

#[derive(Clone)]
pub struct CombineMatcher<T> {
    matchers: Vec<Box<dyn Matcher<T>>>,
    kind: CombineMatcherKind,
}

impl<T: Clone> CombineMatcher<T> {
    pub fn create_one_of(matchers: Vec<Box<dyn Matcher<T>>>) -> Self {
        CombineMatcher {
            matchers,
            kind: CombineMatcherKind::OneOf,
        }
    }

    pub fn create_all_of(matchers: Vec<Box<dyn Matcher<T>>>) -> Self {
        CombineMatcher {
            matchers,
            kind: CombineMatcherKind::AllOf,
        }
    }

    pub fn create_none_of(matchers: Vec<Box<dyn Matcher<T>>>) -> Self {
        CombineMatcher {
            matchers,
            kind: CombineMatcherKind::AllOf,
        }
    }
}

impl<T: Clone> Matcher<T> for CombineMatcher<T> {
    fn is_match(&self, node: &T) -> bool {
        let mut matches_count = 0;
        let matcher_kind = &self.kind;
        for matcher in self.matchers.iter() {
            let is_matches = matcher.is_match(node);

            // If kind is `oneOf` and one if matches, return true
            if is_matches && CombineMatcherKind::OneOf.eq(matcher_kind) {
                return true;
            }

            // If kind is `allOf` and one is not matches, return false
            if !is_matches && CombineMatcherKind::AllOf.eq(matcher_kind) {
                return false;
            }

            // If kind is `noneOf` and one is matches, return false
            if is_matches && CombineMatcherKind::NoneOf.eq(matcher_kind) {
                return false;
            }

            if is_matches {
                matches_count += 1;
            }
        }

        match self.kind {
            CombineMatcherKind::OneOf => matches_count > 1,
            CombineMatcherKind::AllOf => matches_count == self.matchers.len(),
            CombineMatcherKind::NoneOf => matches_count == 0,
        }
    }
}

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
