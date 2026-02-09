use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

/// Define the primary matcher trait for collections
pub trait CollectionMatchers<T> {
    fn to_be_empty(self) -> Self;
    fn to_have_length(self, expected: usize) -> Self;
    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self;
    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self;
    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self;
}

/// Helper trait for types that can be examined as collections
trait AsCollection {
    type Item;

    fn is_empty(&self) -> bool;
    fn length(&self) -> usize;
    fn contains_item<U>(&self, item: &U) -> bool
    where
        U: PartialEq<Self::Item>;
    fn contains_all_items<U>(&self, items: &[U]) -> bool
    where
        U: PartialEq<Self::Item>;
    fn equals_items<U>(&self, other: &[U]) -> bool
    where
        U: PartialEq<Self::Item>;
}

// Implement AsCollection for slice references
impl<T: PartialEq> AsCollection for &[T] {
    type Item = T;

    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn contains_item<U>(&self, item: &U) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        self.iter().any(|x| item == x)
    }

    fn contains_all_items<U>(&self, items: &[U]) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        items.iter().all(|item| self.contains_item(item))
    }

    fn equals_items<U>(&self, other: &[U]) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        if self.len() != other.len() {
            return false;
        }

        self.iter().zip(other.iter()).all(|(a, b)| b == a)
    }
}

// Implement AsCollection for Vec references
impl<T: PartialEq> AsCollection for &Vec<T> {
    type Item = T;

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn contains_item<U>(&self, item: &U) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        self.iter().any(|x| item == x)
    }

    fn contains_all_items<U>(&self, items: &[U]) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        items.iter().all(|item| self.contains_item(item))
    }

    fn equals_items<U>(&self, other: &[U]) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        if self.len() != other.len() {
            return false;
        }

        self.iter().zip(other.iter()).all(|(a, b)| b == a)
    }
}

// Implement AsCollection for owned Vecs
impl<T: PartialEq> AsCollection for Vec<T> {
    type Item = T;

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn contains_item<U>(&self, item: &U) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        self.iter().any(|x| item == x)
    }

    fn contains_all_items<U>(&self, items: &[U]) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        items.iter().all(|item| self.contains_item(item))
    }

    fn equals_items<U>(&self, other: &[U]) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        if self.len() != other.len() {
            return false;
        }

        self.iter().zip(other.iter()).all(|(a, b)| b == a)
    }
}

// Implement AsCollection for array references
impl<T: PartialEq, const N: usize> AsCollection for &[T; N] {
    type Item = T;

    fn is_empty(&self) -> bool {
        N == 0
    }

    fn length(&self) -> usize {
        N
    }

    fn contains_item<U>(&self, item: &U) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        self.iter().any(|x| item == x)
    }

    fn contains_all_items<U>(&self, items: &[U]) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        items.iter().all(|item| self.contains_item(item))
    }

    fn equals_items<U>(&self, other: &[U]) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        if N != other.len() {
            return false;
        }

        self.iter().zip(other.iter()).all(|(a, b)| b == a)
    }
}

// Implementation of CollectionMatchers that works with any type implementing AsCollection
impl<T, V> CollectionMatchers<T> for Assertion<V>
where
    T: Debug + Clone + PartialEq,
    V: AsCollection<Item = T> + Debug + Clone,
{
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let sentence = AssertionSentence::new("be", "empty")
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.length();
        let result = actual_length == expected;
        let sentence = AssertionSentence::new("have", format!("length {}", expected))
            .with_actual(format!("{}", actual_length));

        return self.add_step(sentence, result);
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.contains_item(&expected);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = self.value.contains_all_items(expected);
        let sentence = AssertionSentence::new("contain", format!("all of {:?}", expected))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = self.value.equals_items(expected);

        // Different message if lengths don't match
        if self.value.length() != expected.len() {
            let sentence = AssertionSentence::new("equal", format!("collection {:?} (different lengths)", expected))
                .with_actual(format!("{:?}", self.value));
            return self.add_step(sentence, result);
        }

        let sentence = AssertionSentence::new("equal", format!("collection {:?}", expected))
            .with_actual(format!("{:?}", self.value));
        return self.add_step(sentence, result);
    }
}

/// Extension trait for adding helper methods to collections
pub trait CollectionExtensions<T> {
    fn first(&self) -> Option<&T>;
    fn last(&self) -> Option<&T>;
}

impl<T> CollectionExtensions<T> for Vec<T> {
    fn first(&self) -> Option<&T> {
        return <[T]>::first(self);
    }

    fn last(&self) -> Option<&T> {
        return <[T]>::last(self);
    }
}

impl<T> CollectionExtensions<T> for &[T] {
    fn first(&self) -> Option<&T> {
        return <[T]>::first(self);
    }

    fn last(&self) -> Option<&T> {
        return <[T]>::last(self);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_collection_length() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();

        expect!(slice).to_have_length(5);
        expect!(&collection).to_have_length(5);
    }

    #[test]
    #[should_panic(expected = "have length 6")]
    fn test_wrong_length_fails() {
        // This should fail
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_have_length(6);
    }

    #[test]
    #[should_panic(expected = "not have length 5")]
    fn test_right_length_not_fails() {
        // This should fail
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).not().to_have_length(5);
    }

    #[test]
    fn test_collection_contains() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();

        expect!(slice).to_contain(3);
        expect!(&collection).to_contain(3);
    }

    #[test]
    #[should_panic(expected = "contain 6")]
    fn test_missing_value_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_contain(6);
    }

    #[test]
    #[should_panic(expected = "not contain 3")]
    fn test_present_value_not_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).not().to_contain(3);
    }

    #[test]
    fn test_collection_contains_all() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();

        expect!(slice).to_contain_all_of(&[1, 3, 5]);
        expect!(&collection).to_contain_all_of(&[1, 3, 5]);
    }

    #[test]
    #[should_panic(expected = "contain all of")]
    fn test_missing_values_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_contain_all_of(&[1, 6, 7]);
    }

    #[test]
    #[should_panic(expected = "not contain all of")]
    fn test_present_values_not_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).not().to_contain_all_of(&[1, 3, 5]);
    }

    #[test]
    fn test_collection_equality() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();

        expect!(slice).to_equal_collection(&[1, 2, 3, 4, 5]);
        expect!(&collection).to_equal_collection(&[1, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic(expected = "equal collection")]
    fn test_different_collection_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_equal_collection(&[5, 4, 3, 2, 1]);
    }

    #[test]
    #[should_panic(expected = "different lengths")]
    fn test_shorter_collection_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_equal_collection(&[1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "not equal collection")]
    fn test_same_collection_not_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).not().to_equal_collection(&[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_empty_collection() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let empty: Vec<i32> = vec![];
        let slice = empty.as_slice();

        expect!(slice).to_be_empty();
        expect!(&empty).to_be_empty();
    }

    #[test]
    #[should_panic(expected = "be empty")]
    fn test_non_empty_to_be_empty_fails() {
        let collection = vec![1, 2, 3];
        let slice = collection.as_slice();
        expect!(slice).to_be_empty();
    }

    #[test]
    #[should_panic(expected = "not be empty")]
    fn test_empty_not_to_be_empty_fails() {
        let empty: Vec<i32> = vec![];
        let slice = empty.as_slice();
        expect!(slice).not().to_be_empty();
    }
}
