use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::count_of_atoms(s!("H2O")), s!("H2O"));
}

#[test]
fn case_2() {
    assert_eq!(Solution::count_of_atoms(s!("Mg(OH)2")), s!("H2MgO2"));
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::count_of_atoms(s!("K4(ON(SO3)2)2")),
        s!("K4N2O14S4")
    );
}

#[test]
fn case_4() {
    assert_eq!(Solution::count_of_atoms(s!("Be32")), s!("Be32"));
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::count_of_atoms(s!("H11He49NO35B7N46Li20")),
        s!("B7H11He49Li20N47O35")
    );
}
