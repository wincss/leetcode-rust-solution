use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::can_eat(
            vec![7, 4, 5, 3, 8],
            vec![vec![0, 2, 2], vec![4, 2, 4], vec![2, 13, 1000000000]]
        ),
        vec![true, false, true]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::can_eat(
            vec![5, 2, 6, 4, 1],
            vec![
                vec![3, 1, 2],
                vec![4, 10, 3],
                vec![3, 10, 100],
                vec![4, 100, 30],
                vec![1, 3, 1]
            ]
        ),
        vec![false, true, true, false, false]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::can_eat(
            vec![
                5215, 14414, 67303, 93431, 44959, 34974, 22935, 64205, 28863, 3436, 45640, 34940,
                38519, 5705, 14594, 30510, 4418, 87954, 8423, 65872, 79062, 83736, 47851, 64523,
                15639, 19173, 88996, 97578, 1106, 17767, 63298, 8620, 67281, 76666, 50386, 97303,
                26476, 95239, 21967, 31606, 3943, 33752, 29634, 35981, 42216, 88584, 2774, 3839,
                81067, 59193, 225, 8289, 9295, 9268, 4762, 2276, 7641, 3542, 3415, 1372, 5538, 878,
                5051, 7631, 1394, 5372, 2384, 2050, 6766, 3616, 7181, 7605, 3718, 8498, 7065, 1369,
                1967, 2781, 7598, 6562, 7150, 8132, 1276, 6656, 1868, 8584, 9442, 8762, 6210, 6963,
                4068, 1605, 2780, 556, 6825, 4961, 4041, 4923, 8660, 4114
            ],
            vec![
                vec![46, 4191056, 444472063],
                vec![75, 865431, 146060662],
                vec![91, 244597, 840227137],
                vec![89, 2601754, 901415801],
                vec![69, 1777314, 444098682],
                vec![78, 2957259, 231019870],
                vec![19, 4350225, 516815116],
                vec![42, 4081198, 594990005],
                vec![59, 3176552, 508520222],
                vec![77, 4577766, 38900694],
                vec![92, 320256, 1362],
                vec![44, 3992014, 7209],
                vec![55, 1950613, 1370],
                vec![97, 734069, 3066],
                vec![39, 1188632, 661],
                vec![58, 4526426, 6202],
                vec![51, 3083812, 1767],
                vec![46, 2563654, 9680],
                vec![21, 4012578, 7014],
                vec![66, 2185952, 7039],
                vec![67, 3712445, 1239],
                vec![0, 1840130, 185],
                vec![35, 605159, 7105],
                vec![94, 2269908, 416],
                vec![68, 4117247, 2076],
                vec![0, 4540381, 2412],
                vec![20, 579583, 8917],
                vec![62, 4407388, 7127],
                vec![17, 4468545, 6287],
                vec![50, 3462654, 1410],
                vec![7, 1883037, 77],
                vec![4, 4089924, 5849],
                vec![5, 4340465, 3843],
                vec![68, 596099, 5796],
                vec![29, 542371, 5952],
                vec![91, 441898, 2227],
                vec![35, 912775, 6110],
                vec![12, 267236, 3248],
                vec![27, 990261, 771],
                vec![76, 320119, 5220],
                vec![23, 738123, 2504],
                vec![66, 439801, 4436],
                vec![18, 372357, 1654],
                vec![51, 846227, 5325],
                vec![80, 502088, 3751],
                vec![49, 117408, 102],
                vec![75, 837527, 8747],
                vec![46, 984134, 7924],
                vec![42, 463312, 7558],
                vec![50, 214995, 1043],
                vec![94, 981465, 6758],
                vec![79, 892988, 1063],
                vec![17, 985872, 2314],
                vec![71, 870151, 2004],
                vec![63, 793308, 7608],
                vec![49, 873121, 2846],
                vec![32, 453564, 3739],
                vec![42, 890492, 6026],
                vec![19, 278107, 2649],
                vec![64, 792101, 2208],
                vec![98, 577463, 526],
                vec![41, 572006, 748],
                vec![99, 478120, 895],
                vec![52, 224338, 423],
                vec![83, 532978, 600],
                vec![67, 92281, 486],
                vec![28, 829955, 925],
                vec![22, 171381, 749],
                vec![82, 986821, 603],
                vec![57, 294692, 194],
                vec![9, 730892, 973],
                vec![69, 241093, 931],
                vec![70, 646855, 27],
                vec![45, 233480, 669],
                vec![60, 369922, 965],
                vec![27, 935011, 659],
                vec![96, 667580, 837],
                vec![7, 919344, 188],
                vec![99, 584762, 131],
                vec![5, 93173, 898],
                vec![16, 736395, 184],
                vec![57, 893061, 196],
                vec![28, 352640, 924],
                vec![87, 980414, 80],
                vec![88, 432895, 129],
                vec![23, 461032, 85],
                vec![73, 645991, 268],
                vec![5, 241036, 458],
                vec![9, 422324, 785],
                vec![28, 124913, 224],
                vec![51, 815633, 765],
                vec![59, 894120, 559],
                vec![70, 459876, 192],
                vec![80, 423125, 584],
                vec![85, 824496, 142],
                vec![18, 578975, 104],
                vec![56, 477816, 303],
                vec![6, 702127, 400],
                vec![43, 35371, 850],
                vec![3, 226423, 10]
            ]
        ),
        vec![
            false, true, true, false, true, false, false, false, false, false, true, false, true,
            true, true, false, false, false, false, true, false, false, true, true, false, false,
            true, false, false, false, false, false, false, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, false,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, false, true, true, true, true, true, true, false, true, true, false,
            true, true, true, true, true, true, true, false, true, true, true, true, true, true,
            true, true, false, true, false
        ]
    );
}
