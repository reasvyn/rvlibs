use rvtest::spec::describe;
use rvmath::Set;

#[test]
fn set_tests() {
    describe("Basic Operations")
        .it("new is empty", || {
            let s: Set<i32> = Set::new();
            assert!(s.is_empty());
            assert_eq!(s.len(), 0);
        })
        .it("insert and contains", || {
            let mut s: Set<i32> = Set::new();
            s.insert(1);
            s.insert(2);
            s.insert(3);
            assert_eq!(s.len(), 3);
            assert!(s.contains(&1));
            assert!(s.contains(&3));
            assert!(!s.contains(&4));
        })
        .it("duplicate insert does not increase length", || {
            let mut s: Set<i32> = Set::new();
            s.insert(1);
            s.insert(1);
            assert_eq!(s.len(), 1);
        })
        .it("remove", || {
            let mut s: Set<i32> = Set::new();
            s.insert(1);
            s.insert(2);
            s.remove(&1);
            assert_eq!(s.len(), 1);
            assert!(!s.contains(&1));
        })
        .it("remove non-existent is a no-op", || {
            let mut s: Set<i32> = Set::new();
            s.insert(1);
            s.remove(&999);
            assert_eq!(s.len(), 1);
        })
        .tag("basic")
        .run();

    describe("Iteration")
        .it("iter yields all elements", || {
            let mut s: Set<i32> = Set::new();
            s.insert(1);
            s.insert(2);
            s.insert(3);
            let mut collected: Vec<_> = s.iter().copied().collect();
            collected.sort();
            assert_eq!(collected, vec![1, 2, 3]);
        })
        .tag("iteration")
        .run();

    describe("Set Operations")
        .it("union", || {
            let mut a: Set<i32> = Set::new();
            a.insert(1); a.insert(2); a.insert(3);
            let mut b: Set<i32> = Set::new();
            b.insert(3); b.insert(4); b.insert(5);
            let u = a.union(&b);
            assert_eq!(u.len(), 5);
            assert!(u.contains(&1));
            assert!(u.contains(&5));
        })
        .it("union of disjoint sets", || {
            let mut a: Set<i32> = Set::new();
            a.insert(1); a.insert(2);
            let mut b: Set<i32> = Set::new();
            b.insert(3); b.insert(4);
            assert_eq!(a.union(&b).len(), 4);
        })
        .it("intersection", || {
            let mut a: Set<i32> = Set::new();
            a.insert(1); a.insert(2); a.insert(3);
            let mut b: Set<i32> = Set::new();
            b.insert(2); b.insert(3); b.insert(4);
            let i = a.intersection(&b);
            assert_eq!(i.len(), 2);
            assert!(i.contains(&2));
            assert!(i.contains(&3));
            assert!(!i.contains(&1));
        })
        .it("intersection of disjoint sets is empty", || {
            let mut a: Set<i32> = Set::new();
            a.insert(1); a.insert(2);
            let mut b: Set<i32> = Set::new();
            b.insert(3); b.insert(4);
            assert!(a.intersection(&b).is_empty());
        })
        .it("difference", || {
            let mut a: Set<i32> = Set::new();
            a.insert(1); a.insert(2); a.insert(3);
            let mut b: Set<i32> = Set::new();
            b.insert(2); b.insert(4);
            let d = a.difference(&b);
            assert_eq!(d.len(), 2);
            assert!(d.contains(&1));
            assert!(d.contains(&3));
            assert!(!d.contains(&2));
        })
        .it("difference results in empty when identical", || {
            let mut a: Set<i32> = Set::new();
            a.insert(1);
            let mut b: Set<i32> = Set::new();
            b.insert(1);
            assert!(a.difference(&b).is_empty());
        })
        .tag("set_ops")
        .run();

    describe("FromIterator")
        .it("collect from vec with duplicates", || {
            let s: Set<i32> = vec![1, 2, 3, 1, 2].into_iter().collect();
            assert_eq!(s.len(), 3);
            assert!(s.contains(&1));
            assert!(s.contains(&2));
            assert!(s.contains(&3));
        })
        .it("empty iterator gives empty set", || {
            let s: Set<i32> = Vec::<i32>::new().into_iter().collect();
            assert!(s.is_empty());
        })
        .tag("from_iter")
        .run();

    describe("f64 Set")
        .it("handles floating point values", || {
            let mut s: Set<f64> = Set::new();
            s.insert(1.0);
            s.insert(2.0);
            s.insert(1.0);
            assert_eq!(s.len(), 2);
            assert!(s.contains(&2.0));
        })
        .tag("f64")
        .run()
        .assert_all_pass();
}
