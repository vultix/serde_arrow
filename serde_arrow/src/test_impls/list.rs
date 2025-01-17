use super::macros::test_example;

test_example!(
    test_name = large_list_u32,
    field = GenericField::new("root", GenericDataType::LargeList, false)
        .with_child(GenericField::new("element", GenericDataType::U32, false)),
    ty = Vec<u32>,
    values = [vec![0, 1, 2], vec![3, 4], vec![]],
    nulls = [false, false, false],
);

test_example!(
    test_name = large_list_nullable_u64,
    field = GenericField::new("root", GenericDataType::LargeList, false)
        .with_child(GenericField::new("element", GenericDataType::U64, true)),
    ty = Vec<Option<u64>>,
    values = [vec![Some(0), None, Some(2)], vec![Some(3)], vec![None], vec![]],
    nulls = [false, false, false, false],
);

test_example!(
    test_name = nullable_large_list_u32,
    field = GenericField::new("root", GenericDataType::LargeList, true)
        .with_child(GenericField::new("element", GenericDataType::U32, false)),
    ty = Option<Vec<u32>>,
    values = [Some(vec![0, 1, 2]), None, Some(vec![3, 4]), Some(vec![])],
    nulls = [false, true, false, false],
);

test_example!(
    test_name = list_u32,
    field = GenericField::new("root", GenericDataType::LargeList, false)
        .with_child(GenericField::new("element", GenericDataType::U32, false)),
    overwrite_field = GenericField::new("root", GenericDataType::List, false)
        .with_child(GenericField::new("element", GenericDataType::U32, false)),
    ty = Vec<u32>,
    values = [vec![0, 1, 2], vec![3, 4], vec![]],
    nulls = [false, false, false],
);

test_example!(
    test_name = large_list_large_list_u32,
    field = GenericField::new("root", GenericDataType::LargeList, false)
        .with_child(GenericField::new("element", GenericDataType::LargeList, false)
            .with_child(GenericField::new("element", GenericDataType::U32, false))),
    ty = Vec<Vec<u32>>,
    values = [vec![vec![0, 1, 2], vec![3, 4]], vec![vec![5, 6], vec![]], vec![]],
    nulls = [false, false, false],
);
