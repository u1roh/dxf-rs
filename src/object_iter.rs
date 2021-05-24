use crate::code_pair_put_back::CodePairPutBack;
use crate::objects::Object;

pub(crate) struct ObjectIter<'a> {
    pub iter: &'a mut CodePairPutBack<'a>,
}

impl<'a> Iterator for ObjectIter<'a> {
    type Item = Object;

    fn next(&mut self) -> Option<Object> {
        let mut iter = self.iter;
        match Object::read(iter) {
            Ok(Some(o)) => Some(o),
            Ok(None) | Err(_) => None,
        }
    }
}
