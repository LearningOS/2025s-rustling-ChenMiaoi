/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn partition<T>(array: &mut [T]) -> usize 
where
    T: Ord,
{
    let len = array.len();
    let pivot_index = len / 2;
    array.swap(pivot_index, len - 1);
    
    let mut i = 0;
    for j in 0..len - 1 {
        if array[j] <= array[len - 1] {
            array.swap(i, j);
            i += 1;
        }
    }
    
    array.swap(i, len - 1);
    i
}

fn quick_sort<T>(array: &mut [T])
where 
    T: Ord
{
    if array.len() <= 1 {
        return;
    }

    let pivot = partition(array);
    let (left, right) = array.split_at_mut(pivot);
    quick_sort(left);
    quick_sort(right);
}

fn sort<T>(array: &mut [T])
where 
    T: Ord
{
	//TODO
    quick_sort(array);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
