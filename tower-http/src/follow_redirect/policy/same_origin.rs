use super::{eq_origin, Action, Attempt, Policy};
use std::fmt;

/// A redirection [`Policy`] that stops cross-origin redirections.
#[derive(Clone, Copy, Default)]
pub struct SameOrigin {
    _priv: (),
}

impl SameOrigin {
    /// Create a new [`SameOrigin`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Debug for SameOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SameOrigin").finish()
    }
}

impl<B> Policy<B> for SameOrigin {
    fn redirect(&mut self, attempt: &Attempt<'_>) -> Action {
        if eq_origin(attempt.previous(), attempt.location()) {
            Action::follow()
        } else {
            Action::stop()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::{Request, Uri};

    #[test]
    fn works() {
        let mut policy = SameOrigin::default();

        let initial = Uri::from_static("http://example.com/old");
        let same_origin = Uri::from_static("http://example.com/new");
        let cross_origin = Uri::from_static("https://example.com/new");

        let mut request = Request::builder().uri(initial).body(()).unwrap();
        policy.on_request(&mut request);

        let attempt = Attempt {
            status: Default::default(),
            location: &same_origin,
            previous: request.uri(),
        };
        assert!(Policy::<()>::redirect(&mut policy, &attempt).follows());

        let mut request = Request::builder().uri(same_origin).body(()).unwrap();
        policy.on_request(&mut request);

        let attempt = Attempt {
            status: Default::default(),
            location: &cross_origin,
            previous: request.uri(),
        };
        assert!(Policy::<()>::redirect(&mut policy, &attempt).stops());
    }
}
