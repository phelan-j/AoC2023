use super::*;

#[test]
fn test_calculate_combos() {
        assert_eq!(get_possible_arrangements("# 1"),1);
        assert_eq!(get_possible_arrangements("## 2"),1);
        assert_eq!(get_possible_arrangements("#? 2"),1);
        assert_eq!(get_possible_arrangements("?# 2"),1);
        assert_eq!(get_possible_arrangements("?? 2"),1);
        assert_eq!(get_possible_arrangements(".? 2"),0);
        assert_eq!(get_possible_arrangements("?. 2"),0);
        assert_eq!(get_possible_arrangements(".. 2"),0);
        assert_eq!(get_possible_arrangements("### 1"),0);
        assert_eq!(get_possible_arrangements("### 2"),0);
        assert_eq!(get_possible_arrangements("### 3"),1);
        assert_eq!(get_possible_arrangements("### 1,1"),0);
        assert_eq!(get_possible_arrangements("#?# 1,1"),1);
        assert_eq!(get_possible_arrangements("??# 1,1"),1);
        assert_eq!(get_possible_arrangements("#?? 1,1"),1);
        assert_eq!(get_possible_arrangements("??? 1,1"),1);
        assert_eq!(get_possible_arrangements("?.? 1,1"),1);
        assert_eq!(get_possible_arrangements("??. 1,1"),0);
        assert_eq!(get_possible_arrangements(".?? 1,1"),0);
        assert_eq!(get_possible_arrangements("..? 1,1"),0);
        assert_eq!(get_possible_arrangements("... 1,1"),0);
        assert_eq!(get_possible_arrangements("?.. 1,1"),0);
        assert_eq!(get_possible_arrangements(".?. 1,1"),0);
        assert_eq!(get_possible_arrangements("#?# 1,1"),1);
        assert_eq!(get_possible_arrangements("### 1,1"),0);
        assert_eq!(get_possible_arrangements("#.# 1,1"),1);
        assert_eq!(get_possible_arrangements("#.# 1"),0);
        assert_eq!(get_possible_arrangements(".## 1"),0);
        assert_eq!(get_possible_arrangements("##. 1"),0);
        assert_eq!(get_possible_arrangements("### 1"),0);
        assert_eq!(get_possible_arrangements("#.. 1"),1);
        assert_eq!(get_possible_arrangements(".#. 1"),1);
        assert_eq!(get_possible_arrangements("..# 1"),1);
        assert_eq!(get_possible_arrangements("#?? 1"),1);
        assert_eq!(get_possible_arrangements(".?? 1"),2);
        assert_eq!(get_possible_arrangements("??. 1"),2);
        assert_eq!(get_possible_arrangements("??? 1"),3);
}
