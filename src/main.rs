fn median(a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }

    let mut a_clone = a.clone();

    a_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let middle_index = a_clone.len() / 2;

    if middle_index * 2 == a_clone.len() {
        Some((a_clone[middle_index - 1] + a_clone[middle_index]) / 2.0)
    } else {
        Some(a_clone[middle_index])
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
