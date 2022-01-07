use std::fmt::Display;

use gw2api_model::{BulkEndpoint, FixedEndpoint};

pub trait HttpFixedEndpoint: FixedEndpoint {
    fn get() -> EndpointResult<Self> {
        todo!()
    }
}

impl<T> HttpFixedEndpoint for T where T: FixedEndpoint {}

pub trait HttpBulkEndpoint: BulkEndpoint {
    /// request a single item
    fn get(_id: Self::IdType) -> EndpointResult<Self> {
        todo!()
    }

    /// request all available ids
    fn ids() -> EndpointResult<Vec<Self::IdType>> {
        todo!()
    }

    /// request multiple ids
    fn many(_ids: &[Self::IdType]) -> EndpointResult<Vec<Self>> {
        todo!()
    }

    /// requests a page of items and returns the maximum number of pages
    fn page(_page: usize, _page_size: u8, _buf: &mut [Self]) -> EndpointResult<usize> {
        todo!()
    }

    /// requests all items using the most efficient method available
    fn all() -> EndpointResult<Vec<Self>> {
        if Self::ALL {
            Self::get_all_by_ids_all()
        } else if Self::PAGING {
            Self::get_all_by_paging()
        } else {
            Self::get_all_by_requesting_ids()
        }
    }

    /// Intended for internal use
    /// Gets all items by querying ids=all
    /// use [`all`] to use the most efficient way to request all items
    fn get_all_by_ids_all() -> EndpointResult<Vec<Self>> {
        if !Self::ALL {
            return Err(Box::new(UnsupportedEndpointQuery));
        }
        todo!()
    }

    /// Intended for internal use
    /// Gets all items by querying all pages
    /// use [`all`] to use the most efficient way to request all items
    fn get_all_by_paging() -> EndpointResult<Vec<Self>> {
        if !Self::PAGING {
            return Err(Box::new(UnsupportedEndpointQuery));
        }

        let mut result = Vec::new();
        let max_pages = Self::page(1, 200, &mut result)?;
        result.reserve_exact((max_pages - 1) * 200);

        for page in (1..=max_pages).into_iter() {
            Self::page(page, 200, &mut result)?;
        }

        Ok(result)
    }

    /// Intended for internal use
    /// Gets all items by querying all ids
    /// use [`all`] to use the most efficient way to request all items
    fn get_all_by_requesting_ids() -> EndpointResult<Vec<Self>> {
        let ids = Self::ids()?;
        Self::many(&ids)
    }
}

impl<T> HttpBulkEndpoint for T where T: BulkEndpoint {}

struct UnsupportedEndpointQuery;

impl Display for UnsupportedEndpointQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "This endpoint does not support this operation")
    }
}

impl std::fmt::Debug for UnsupportedEndpointQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Display::fmt(&self, f)
    }
}

impl std::error::Error for UnsupportedEndpointQuery {}

type EndpointResult<T> = Result<T, Box<dyn std::error::Error>>;
