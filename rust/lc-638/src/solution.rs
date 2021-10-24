use std::cell::Cell;

thread_local! {static MIN_PRICE: Cell<i32> = Cell::new(std::i32::MAX);}

pub struct Solution {}

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        MIN_PRICE.with(|cell| {
            cell.set(std::i32::MAX);
        });

        if needs.iter().sum::<i32>() == 0 {
            return 0;
        }

        let mut temp = Vec::with_capacity(special.len());
        for bundle in special.iter() {
            let mut price_in_bulk = 0;
            let n = bundle.len();
            for (i, &amnt) in bundle.iter().enumerate() {
                if i < n - 1 {
                    price_in_bulk += amnt * price[i];
                } else {
                    if price_in_bulk >= amnt {
                        temp.push(bundle.clone());
                    }
                }
            }
        }
        let special = temp;

        Self::backtrace_rec(&price, &special, needs, 0);

        MIN_PRICE.with(|cell| cell.take())
    }

    fn backtrace_rec(price: &[i32], special: &[Vec<i32>], needs: Vec<i32>, sum: i32) {
        MIN_PRICE.with(|cell| {
            let min_price = cell.get();
            let mut candidate = sum;
            let mut should_apply = true;
            for (i, &need) in needs.iter().enumerate() {
                candidate += price[i] * need;
                if candidate >= min_price {
                    should_apply = false;
                    break;
                }
            }

            if should_apply {
                cell.set(candidate);
            }
        });

        'bundleLoop: for bundle in special.iter() {
            let mut new_needs = needs.clone();
            let mut new_sum = sum;
            let mut is_all_zeroes = true;
            for (i, amnt) in bundle.iter().enumerate() {
                if i < bundle.len() - 1 {
                    let new_need_i = new_needs[i] - amnt;
                    if new_need_i < 0 {
                        continue 'bundleLoop;
                    }

                    if is_all_zeroes && new_need_i > 0 {
                        is_all_zeroes = false;
                    }
                    new_needs[i] = new_need_i;
                }
            }
            new_sum += bundle.last().unwrap();

            if is_all_zeroes {
                MIN_PRICE.with(|cell| {
                    cell.set(cell.take().min(new_sum));
                });
                continue;
            }

            Self::backtrace_rec(price, special, new_needs, new_sum);
        }
    }
}
