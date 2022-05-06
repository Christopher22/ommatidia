use std::borrow::Cow;

#[derive(Debug)]
pub struct Errors<E: std::error::Error> {
    errors: Vec<(E, Option<String>)>,
}

impl<E: std::error::Error> Default for Errors<E> {
    fn default() -> Self {
        Self {
            errors: Default::default(),
        }
    }
}

impl<E: std::error::Error> Errors<E> {
    pub fn any_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn report<T: Into<E>>(&mut self, error: T) {
        self.errors.push((error.into(), None))
    }

    pub fn with_id<'a, 'b, T: Into<Cow<'b, str>>>(&'a mut self, name: T) -> ErrorWithId<'a, 'b, E> {
        ErrorWithId {
            errors: self,
            name: name.into(),
        }
    }
}

pub struct ErrorWithId<'a, 'b, T: std::error::Error> {
    errors: &'a mut Errors<T>,
    name: Cow<'b, str>,
}

impl<'a, 'b, E: std::error::Error> ErrorWithId<'a, 'b, E> {
    pub fn report<T: Into<E>>(self, error: T) {
        self.errors
            .errors
            .push((error.into(), Some(self.name.into_owned())))
    }
}
