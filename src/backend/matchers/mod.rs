pub mod boolean;
pub mod collection;
pub mod equality;
pub mod float;
pub mod hashmap;
pub mod numeric;
pub mod option;
pub mod result;
pub mod string;

// Instead of glob imports, we explicitly export the trait names
// to avoid conflicts and ambiguities
pub use boolean::BooleanMatchers;
pub use collection::{CollectionExtensions, CollectionMatchers};
pub use equality::EqualityMatchers;
pub use float::FloatMatchers;
pub use hashmap::HashMapMatchers;
pub use numeric::NumericMatchers;
pub use option::OptionMatchers;
pub use result::ResultMatchers;
pub use string::StringMatchers;
