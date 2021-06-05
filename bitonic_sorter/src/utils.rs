use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

use super::SortOrder;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
  let rng = Pcg64Mcg::from_seed([0; 16]);
  rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_as<T: Ord>(x: &[T], order: &SortOrder) -> bool {
  x.windows(2).all(|pair| -> bool {
    match order {
      SortOrder::Ascending => pair[0] <= pair[1],
      SortOrder::Descending => pair[0] >= pair[1],
    }
  })
}

mod tests {
  use crate::utils::{new_u32_vec, is_sorted_as};
  use crate::third::{sort};
  use super::SortOrder::*;

  #[test]
  fn sort_u32_large() {
    {
      let mut x = new_u32_vec(65536);
      assert_eq!(sort(&mut x, &Ascending), Ok(()));
      assert!(is_sorted_as(&mut x, &Ascending));
    }
    {
      let mut x = new_u32_vec(65536);
      assert_eq!(sort(&mut x, &Descending), Ok(()));
      assert!(is_sorted_as(&mut x, &Descending));
    }
  }
}