use crate::collections::TypeSet;

const SAMPLE_STR: &str = "A sample string";

#[test]
fn test_contains() {
    let mut set = TypeSet::new();
    let element = String::from(SAMPLE_STR);
    set.insert(element);
    assert!(set.contains::<String>())
}

#[test]
fn test_get() {
    let mut set = TypeSet::new();
    let element = String::from(SAMPLE_STR);
    set.insert(element.clone());
    assert_eq!(set.get::<String>(), Some(&element))
}

#[test]
fn test_get_mut() {
    let mut set = TypeSet::new();
    let mut element = String::from(SAMPLE_STR);
    set.insert(element.clone());
    assert_eq!(set.get_mut::<String>(), Some(&mut element))
}

#[test]
fn test_remove_equality() {
    let mut set = TypeSet::new();
    let element = String::from(SAMPLE_STR);
    set.insert(element.clone());
    assert_eq!(set.remove::<String>(), Some(element))
}

#[test]
fn test_remove_removes() {
    let mut set = TypeSet::new();
    let element = String::from(SAMPLE_STR);
    set.insert(element.clone());
    set.remove::<String>();
    assert!(!set.contains::<String>())
}

#[test]
fn test_clear() {
    let mut set = TypeSet::new();
    let element = String::from(SAMPLE_STR);
    set.insert(element.clone());
    set.clear();
    assert!(!set.contains::<String>())
}

#[test]
fn test_insert_dynamic() {
    let mut set = TypeSet::new();
    let element = Box::new(String::from(SAMPLE_STR));
    set.insert_dynamic(element);
    assert!(set.contains::<String>())
}
