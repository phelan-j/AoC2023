use super::*;

fn assert_maps_equal(a: &Vec<(u64,u64,u64)>, b: &Vec<(u64,u64,u64)>) {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        assert_eq!(a[i],b[i]);
    }
}

#[test]
fn test_f_lt_g() {
    // [0-10) -> [10-20)
    let f = vec![(0,10,10)];
    // [30-40) -> [40-50)
    let g = vec![(30,40,10)];
    // [0-10) -> [10-20)
    // [30-40) -> [40-50)
    let h_expected = vec![(0,10,10),(30,40,10)];
    let mut h = compose(&f,&g);
    h.sort_by(|(x,_,_),(y,_,_)| x.cmp(y));
    assert_maps_equal(&h, &h_expected);
}

#[test]
fn test_f_in_g() {
    // [0-10) -> [30-40)
    let f = vec![(0,30,10)];
    // [20-60) -> [100-140)
    let g = vec![(20,100,40)];
    // [0-10) -> [110-120)
    // [20-30) -> [100-110)
    // [40-60) -> [120-140)
    let h_expected = vec![(0,110,10),(20,100,10),(40,120,20)];
    let mut h = compose(&f,&g);
    h.sort_by(|(x,_,_),(y,_,_)| x.cmp(y));
    assert_maps_equal(&h, &h_expected);
}

#[test]
fn test_f_x_g() {
    // [0-20) -> [30-50)
    let f = vec![(0,30,20)];
    // [40-60) -> [100-120)
    let g = vec![(40,100,20)];
    // [0-10) -> [30-40)
    // [10-20) -> [100-110)
    // [50-60) -> [110-120) 
    let h_expected = vec![(0,30,10),(10,100,10),(50,110,10)];
    let mut h = compose(&f,&g);
    h.sort_by(|(x,_,_),(y,_,_)| x.cmp(y));
    assert_maps_equal(&h, &h_expected);
}

#[test]
fn test_g_lt_f() {
    // [0-10) -> [30-40)
    let f = vec![(0,30,10)];
    // [10-20) -> [40-50)
    let g = vec![(10,40,10)];
    // [0-10) -> [30-40)
    // [10-20) -> [40-50)
    let h_expected = vec![(0,30,10),(10,40,10)];
    let mut h = compose(&f,&g);
    h.sort_by(|(x,_,_),(y,_,_)| x.cmp(y));
    assert_maps_equal(&h, &h_expected);
}

#[test]
fn test_g_in_f() {
    // [0-40) -> [20-60)
    let f = vec![(0,20,40)];
    // [30-40) -> [100-110)
    let g = vec![(30,100,20)];
    // [0-10) -> [20-30)
    // [10-30) -> [100-120)
    // [30-40) -> [50-60)
    let h_expected = vec![(0,20,10),(10,100,20),(30,50,10)];
    let mut h = compose(&f,&g);
    h.sort_by(|(x,_,_),(y,_,_)| x.cmp(y));
    assert_maps_equal(&h, &h_expected);
}

#[test]
fn test_g_x_f() {
    // [0-20) -> [40-60)
    let f = vec![(0,40,20)];
    // [30-50) -> [100-120)
    let g = vec![(30,100,20)];
    // [0-10) -> [110-120)
    // [10-20) -> [50-60)
    // [30-40) -> [100-110) 
    let h_expected = vec![(0,110,10),(10,50,10),(30,100,10)];
    let mut h = compose(&f,&g);
    h.sort_by(|(x,_,_),(y,_,_)| x.cmp(y));
    assert_maps_equal(&h, &h_expected);
}

#[test]
fn test_big() {
    let input = vec![
        vec![(98,50,2),(50,52,48)],
        vec![(15,0,37),(52,37,2),(0,39,15)],
        vec![(53,49,8),(11,0,42),(0,42,7),(7,57,4)],
        vec![(18,88,7),(25,18,70)],
        vec![(77,45,23),(45,81,19),(64,68,13)],
        vec![(69,0,1),(0,1,69)],
        vec![(56,60,37),(93,56,4)]
    ];
    let expected = vec![
        (0,22,14),
        (14,43,1),
        (15,36,7),
        (22,90,4),
        (26,1,18),
        (44,61,6),
        (50,20,2),
        (52,44,2),
        (54,85,5),
        (59,94,3),
        (62,56,4),
        (66,97,3),
        (69,73,1),
        (70,0,1),
        (71,74,11),
        (82,46,10),
        (92,60,1),
        (93,68,5),
        (98,67,1),
        (99,19,1)
    ];
    let mut map_full : Vec<(u64,u64,u64)> = Vec::new();
    for map_curr in input {
        map_full = compose(&map_full, &map_curr);
    }
    map_full.sort_by(|(x,_,_),(y,_,_)| x.cmp(y));
    assert_maps_equal(&map_full, &expected);
}
