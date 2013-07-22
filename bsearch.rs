/**
 * Binary search that returns the closest position of the value to find; returns Match if the position was found, or NoMatch if the position was not found.
 */
#[inline(always)]
fn binary_search<K, T>(vector: &[T], value: &K, range: (uint, uint), comparer: &fn(&K, &T) -> std::cmp::Ordering) -> SearchResult {
    let mut l = match range { (x, _) => x as int };
    let mut u = match range { (_, y) => y as int };
    
    assert!(u as uint <= vector.len() && !(u < l));
    
    let mut i = 0;
    loop {
	if u < l {
	    return NoMatch(i);
	}
	
	i = std::uint::div_floor((l + u) as uint, 2);
	
	match comparer(value, &vector[i]) {
	    Equal => return Match(i),
	    Less => u = i as int - 1,
	    Greater => l = i as int + 1
	}
    }
}

enum SearchResult {
    Match(uint),
    NoMatch(uint)
}

struct KeyValuePair<K, V> {
    key: K,
    value: V
}

fn compare_keys<K: Ord + Eq, V>(key: &K, key_value: &Option<~KeyValuePair<K, V>>) -> std::cmp::Ordering {
    let key_value_key: &K = match *key_value { 
	Some(ref key_value) => &key_value.key,
	None => fail!()
    };
    
    if *key > *key_value_key {
	return Greater;
    }
    else if *key < *key_value_key {
	return Less;
    }
    else {
	return Equal;
    }
}

fn compare_uint(x: &uint, y: &uint) -> std::cmp::Ordering {
    if *x > *y {
	return Greater;
    } 
    else if *x < *y {
	return Less;
    }
    else {
	return Equal;
    }
}

#[test]
fn test_exact_middle_match() {
    let vector: [uint, ..5] = [1,4,10,50,70];
    
    let valueToFind = ~10;
    
    let result = binary_search(vector, valueToFind, (0, vector.len() - 1), compare_uint);
    
    match result {
	Match(index) => assert!(vector[index] == *valueToFind),
	NoMatch(_) => fail!()
    }
}

#[test]
fn test_exact_match_with_range() {
    let vector: [uint, ..12] = [1,4,6,7,8,10,11,14,34,45,50,70];
    
    let valueToFind = ~10;
    
    let result = binary_search(vector, valueToFind, (3, vector.len() - 4), compare_uint);
    
    match result {
	Match(index) => assert!(vector[index] == *valueToFind),
	NoMatch(_) => fail!()
    }
}

#[test]
fn test_exact_one_item_match() {
    let vector: [uint, ..1] = [10];
    
    let valueToFind = ~10;
    
    let result = binary_search(vector, valueToFind, (0, vector.len() - 1), compare_uint);
    
    match result {
	Match(index) => assert!(vector[index] == *valueToFind),
	NoMatch(_) => fail!()
    }
}

#[test]
fn test_exact_two_item_match() {
    let vector: [uint, ..2] = [1, 10];
    
    let valueToFind = ~10;
    
    let result = binary_search(vector, valueToFind, (0, vector.len() - 1), compare_uint);
    
    match result {
	Match(index) => assert!(vector[index] == *valueToFind),
	NoMatch(_) => fail!()
    }
}

#[test]
fn test_exact_first_item_match() {
    let vector: [uint, ..4] = [10, 15, 17, 19];
    
    let valueToFind = ~10;
    
    let result = binary_search(vector, valueToFind, (0, vector.len() - 1), compare_uint);
    
    match result {
	Match(index) => assert!(vector[index] == *valueToFind),
	NoMatch(_) => fail!()
    }
}

#[test]
fn test_exact_last_item_match() {
    let vector: [uint, ..4] = [1, 2, 3, 10];
    
    let valueToFind = ~10;
    
    let result = binary_search(vector, valueToFind, (0, vector.len() - 1), compare_uint);
    
    match result {
	Match(index) => assert!(vector[index] == *valueToFind),
	NoMatch(_) => fail!()
    }
}

#[test]
fn test_middle_item_no_match() {
    let vector: [uint, ..5] = [1,4,10,50,70];
    
    let valueToFind = ~9;
    
    let result = binary_search(vector, valueToFind, (0, vector.len() - 1), compare_uint);
    
    match result {
	Match(_) => fail!(),
	NoMatch(index) => assert!(vector[index] == 4 || vector[index] == 10)
    }
}

#[test]
fn test_no_match_with_range() {
    let vector: [uint, ..12] = [1,4,6,7,8,10,11,14,34,45,50,70];
    
    let valueToFind = ~9;
    
    let result = binary_search(vector, valueToFind, (3, vector.len() - 4), compare_uint);
    
    match result {
	Match(_) => fail!(),
	NoMatch(index) => assert!(vector[index] == 10 || vector[index] == 8)
    }
}

#[test]
fn test_first_item_no_match() {
    let vector: [uint, ..5] = [1,4,10,50,70];
    
    let valueToFind = ~0;
    
    let result = binary_search(vector, valueToFind, (0, vector.len() - 1), compare_uint);
    
    match result {
	Match(_) => fail!(),
	NoMatch(index) => assert!(vector[index] == 1)
    }
}

#[test]
fn test_last_item_no_match() {
    let vector: [uint, ..5] = [1,4,10,50,70];
    
    let valueToFind = ~71;
    
    let result = binary_search(vector, valueToFind, (0, vector.len() - 1), compare_uint);
    
    match result {
	Match(_) => fail!(),
	NoMatch(index) => assert!(vector[index] == 70)
    }
}

#[test]
fn test_one_item_no_match() {
    let vector: [uint, ..1] = [1];
    
    let valueToFind = ~9;
    
    let result = binary_search(vector, valueToFind, (0, vector.len() - 1), compare_uint);
    
    match result {
	Match(_) => fail!(),
	NoMatch(index) => assert!(vector[index] == 1)
    }
}