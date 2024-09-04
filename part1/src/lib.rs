/// Returns the sqrt of `n`
///
/// Using built-in square root functions is not allowed.
pub fn sqrt(n: u32) -> u32 {
    println!("Input: {}", n);
    for i in 0..(n + 1) {
        if i * i == n {
            println!("Answer: {}", i);
            return i;
        }
        if i * i > n {
            return i - 1;
        }
    }
    return 0;
}

/// Consumes a sorted list of integers and a query integer. Returns the index of the query integer
///
/// Note that a 3-valued comparison between numbers `a` and `b` can be done easily:
/// ```rust,ignore
// / match a.cmp(&b) {
// /    std::cmp::Ordering::Less => { ... },
// /    std::cmp::Ordering::Greater => { ... },
// /    std::cmp::Ordering::Equal => { ... },
// / }
/// ```
pub fn binary_search(arr: &[i32], query: i32) -> Option<u32> {
    if arr.len() == 0 {
        return None;
    }
    let mid = arr.len() / 2;
    let val = arr[mid];
    return match val.cmp(&query) {
        std::cmp::Ordering::Greater => { 
            binary_search(&arr[..mid], query)
        },
        std::cmp::Ordering::Less => {
            let ans = binary_search(&arr[mid + 1..], query);
            match ans {
                Some(f) => Some(f + 1 + (mid as u32)),
                None => None,
            }
        },
        std::cmp::Ordering::Equal => {
            Some(mid as u32)
        },
    }
    }

/// Consumes a list of numbers representing daily rainfall. The list may contain -999 signifying
/// the end of data of interest. Returns the average of non-negative values up to the first
/// occurrence of -999 (if it occurs). There may be negative numbers other -999 in the list.
/// Returns None if the average is incomputable.
///
/// example: rainfall([6, 8, -1, 1, -999, 4, 5, 6]) -> Some(5.0)
pub fn rainfall(values: &[i32]) -> Option<f64> {
    let mut sum = 0;
    let mut occurences = 0;

    for val in values {
        if *val == -999 {
            break;
        }
        if *val >= 0 {
            sum = sum + *val;
            occurences = occurences + 1;
        }
    }
    
    if sum == 0 {
        return None
    } else {
        return Some((sum as f64) / (occurences as f64))
    }
}
