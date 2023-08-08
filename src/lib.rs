pub mod paginate_database {

    use std::cmp::min;

    use sqlx::Error;
    pub trait DataPages {
        fn count_length(&self) -> usize;
        fn query_pages(
            &self,
            start: usize,
            limit: usize,
        ) -> Result<std::string::String, serde_json::Error>;
    }

    #[derive(Debug)]
    pub struct Pages<M: DataPages> {
        offset: usize,
        length: usize,
        limit: usize,
        model: M,
    }

    impl<M: DataPages> Pages<M> {
        pub fn new(length: usize, limit: usize, model: M) -> Pages<M> {
            Pages {
                offset: 0,
                length,
                limit,
                model,
            }
        }
    }

    #[derive(Debug)]
    pub struct Page {
        page: Option<std::string::String>,
        start: usize,
        end: usize,
        limit: usize,
    }

    impl<M: DataPages> Iterator for Pages<M> {
        type Item = Page;
        fn next(&mut self) -> Option<Self::Item> {
            let result = self.model.query_pages(self.offset, self.limit);
            let page: Page = Page {
                page: match result {
                    Ok(s) => Some(s),
                    _ => None,
                },
                start: self.offset * self.limit,
                end: min(
                    (self.offset + 1) * self.limit - 1,
                    self.model.count_length(),
                ),
                limit: self.limit,
            };
            self.offset += 1;
            if page.is_empty() {
                None
            } else {
                Some(page)
            }
        }
    }

    impl Page {
        pub fn is_empty(&self) -> bool {
            match self.page {
                Some(_) => true,
                None => false,
            }
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     #[test]
    //     fn it_works() {}
    // }
}
