pub(crate) use structs::Paginate;

mod postgres_implement;
mod structs;
pub(crate) mod traits;

const DEFAULT_PAGE_SIZE: u8 = 30;
