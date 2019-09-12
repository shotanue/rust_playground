pub fn bubble_sort(source: &mut Vec<i32>) {
    let length = source.len();

    for _ in 0..length {
        for j in 1..(length - 1) {
            if source[j - 1] > source[j] {
                source.swap(j - 1, j);
            };
        }
    }
}

pub fn selection_sort(source: &mut Vec<i32>) {
    let length = source.len();
    let mut min;
    for i in 0..length {
        min = i;
        for j in (i + 1)..length {
            if source[j] < source[min] {
                min = j;
            };
        }

        source.swap(min, i);
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm;

    #[test]
    fn it_tests_bubble_sort() {
        let mut source = vec![1, 4, 2, 3, 7];
        algorithm::selection_sort(&mut source);
        assert_eq!(source, vec![1, 2, 3, 4, 7]);
    }

    #[test]
    fn it_tests_selection_sort() {
        let mut source = vec![1, 5, 2, 3, 7];
        algorithm::selection_sort(&mut source);
        assert_eq!(source, vec![1, 2, 3, 5, 7]);
    }

    #[test]
    fn it_tests_rpn() {
        let ex: Vec<&str> = "1 2 + 3 4 - *".split_whitespace().collect();
        let mut stack: Vec<i32> = Vec::with_capacity(100);
        let is_operand = |x: &str| -> bool{
            match x.parse::<i32>() {
                Ok(_) => { true }
                Err(_) => { false }
            }
        };

        let is_operator: fn(&str) -> bool = |x| match x {
            "+" | "-" | "*" | "/" => true,
            _ => false
        };

        for s in ex.iter() {
            if is_operand(s) {
                stack.push(s.parse::<i32>().unwrap());
            } else if is_operator(s) {
                let operator = s;
                let operand1 = stack.pop().unwrap();
                let operand2 = stack.pop().unwrap();
                match operator.as_ref() {
                    "+" => stack.push(operand1 + operand2),
                    "-" => stack.push(operand2 - operand1),
                    "*" => stack.push(operand1 * operand2),
                    "/" => stack.push(operand1 / operand2),
                    _ => panic!("unexpected operator")
                }
            } else {
                panic!(format!("arg {}", s))
            }
        }
        assert_eq!(stack.pop().unwrap(), -3);
    }
}
