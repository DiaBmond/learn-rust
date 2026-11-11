#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub(crate) fn sort_by_key_demo() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{list:#?}");
    println!("Sorted in {num_sort_operations} operations");

    // Read the height field explicitly.
    for rectangle in &list {
        println!("Width: {}, Height: {}", rectangle.width, rectangle.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_by_key_orders_rectangles_by_width() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        let mut operations = 0;

        list.sort_by_key(|r| {
            operations += 1;
            r.width
        });

        assert_eq!([list[0].width, list[1].width, list[2].width], [3, 7, 10]);

        assert!(operations > 0);
    }
}
