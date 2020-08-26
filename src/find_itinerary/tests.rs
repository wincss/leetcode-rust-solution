use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_itinerary(vec![
            to_string_vec(&["MUC", "LHR"]),
            to_string_vec(&["JFK", "MUC"]),
            to_string_vec(&["SFO", "SJC"]),
            to_string_vec(&["LHR", "SFO"])
        ]),
        vec!["JFK", "MUC", "LHR", "SFO", "SJC"]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_itinerary(vec![
            to_string_vec(&["JFK", "SFO"]),
            to_string_vec(&["JFK", "ATL"]),
            to_string_vec(&["SFO", "ATL"]),
            to_string_vec(&["ATL", "JFK"]),
            to_string_vec(&["ATL", "SFO"])
        ]),
        vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
    );
}
