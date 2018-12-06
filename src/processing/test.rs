use super::{
    double_overlapping_chunks_iterator::*, image::*, overlapping_chunks_iterator::*,
    variable_chunks_iterator::*,
};

mod double_overlapping_chunks_iterator {
    use super::*;

    #[test]
    fn test_not_enough_chunks() {
        let mut v = vec![1u8, 2];
        let mut it = DoubleOverlappingChunksIterator::from_slice(&mut v, 2);

        match it.next() {
            Some(_) => {
                panic!("Should not generate elements!");
            }
            None => {}
        };
    }

    #[test]
    fn test_minimal_chunks() {
        let mut v = vec![1u8, 2, 3];
        let mut it = DoubleOverlappingChunksIterator::from_slice(&mut v, 2);

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[]);
                assert_eq!(left, &[1u8, 2]);
                assert_eq!(middle, &[3u8]);
                assert_eq!(right, &[]);
                assert_eq!(next_right, &[]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        assert!(it.next().is_none());
    }

    #[test]
    fn test_no_neighbour_chunks() {
        let mut v = vec![1u8, 2, 3];
        let mut it = DoubleOverlappingChunksIterator::from_slice(&mut v, 1);

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[]);
                assert_eq!(left, &[1u8]);
                assert_eq!(middle, &[2u8]);
                assert_eq!(right, &[3u8]);
                assert_eq!(next_right, &[]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        assert!(it.next().is_none());
    }

    #[test]
    fn test_two_iterations() {
        let mut v = vec![1u8, 2, 3, 4, 5];
        let mut it = DoubleOverlappingChunksIterator::from_slice(&mut v, 1);

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[]);
                assert_eq!(left, &[1u8]);
                assert_eq!(middle, &[2u8]);
                assert_eq!(right, &[3u8]);
                assert_eq!(next_right, &[5u8]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[1u8]);
                assert_eq!(left, &[3u8]);
                assert_eq!(middle, &[4u8]);
                assert_eq!(right, &[5u8]);
                assert_eq!(next_right, &[]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        assert!(it.next().is_none());
    }

    #[test]
    fn test_three_iterations() {
        let mut v = vec![1u8, 2, 3, 4, 5, 6];
        let mut it = DoubleOverlappingChunksIterator::from_slice(&mut v, 1);

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[]);
                assert_eq!(left, &[1u8]);
                assert_eq!(middle, &[2u8]);
                assert_eq!(right, &[3u8]);
                assert_eq!(next_right, &[5u8]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[1u8]);
                assert_eq!(left, &[3u8]);
                assert_eq!(middle, &[4u8]);
                assert_eq!(right, &[5u8]);
                assert_eq!(next_right, &[]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[3u8]);
                assert_eq!(left, &[5u8]);
                assert_eq!(middle, &[6u8]);
                assert_eq!(right, &[]);
                assert_eq!(next_right, &[]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        assert!(it.next().is_none());
    }

    #[test]
    fn test_independent_mutability() {
        let mut v = vec![1u8, 2, 3, 4, 5, 6, 7];
        {
            let mut it = DoubleOverlappingChunksIterator::from_slice(&mut v, 1);
            let DoubleOverlappingChunks { middle: m1, .. } = it.next().unwrap();
            let DoubleOverlappingChunks { middle: m2, .. } = it.next().unwrap();
            let DoubleOverlappingChunks { middle: m3, .. } = it.next().unwrap();

            m1[0] = 0;
            m2[0] = 1;
            m3[0] = 2;
        }
        assert_eq!(v, &[1u8, 0, 3, 1, 5, 2, 7])
    }

    #[test]
    fn test_pointer_equality() {
        let mut v = vec![1u8, 2, 3, 4, 5, 6, 7];
        let mut it = DoubleOverlappingChunksIterator::from_slice(&mut v, 1);

        let DoubleOverlappingChunks {
            left: l1,
            right: r1,
            next_right: nr1,
            ..
        } = it.next().unwrap();
        let DoubleOverlappingChunks {
            prev_left: pl2,
            left: l2,
            right: r2,
            next_right: nr2,
            ..
        } = it.next().unwrap();
        let DoubleOverlappingChunks {
            prev_left: pl3,
            left: l3,
            right: r3,
            ..
        } = it.next().unwrap();

        assert_eq!(l1.as_ptr(), pl2.as_ptr());
        assert_eq!(r1.as_ptr(), l2.as_ptr());
        assert_eq!(nr1.as_ptr(), r2.as_ptr());

        assert_eq!(l2.as_ptr(), pl3.as_ptr());
        assert_eq!(r2.as_ptr(), l3.as_ptr());
        assert_eq!(nr2.as_ptr(), r3.as_ptr());
    }

    #[test]
    fn test_bigger_step() {
        let mut v = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut it = DoubleOverlappingChunksIterator::from_slice(&mut v, 3);

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[]);
                assert_eq!(left, &[1u8, 2, 3]);
                assert_eq!(middle, &[4u8, 5, 6]);
                assert_eq!(right, &[7u8, 8, 9]);
                assert_eq!(next_right, &[13u8, 14, 15]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[1u8, 2, 3]);
                assert_eq!(left, &[7u8, 8, 9]);
                assert_eq!(middle, &[10u8, 11, 12]);
                assert_eq!(right, &[13u8, 14, 15]);
                assert_eq!(next_right, &[]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        match it.next() {
            Some(DoubleOverlappingChunks {
                prev_left,
                left,
                middle,
                right,
                next_right,
            }) => {
                assert_eq!(prev_left, &[7u8, 8, 9]);
                assert_eq!(left, &[13u8, 14, 15]);
                assert_eq!(middle, &[16u8]);
                assert_eq!(right, &[]);
                assert_eq!(next_right, &[]);
            }
            None => {
                panic!("Should generate elements!");
            }
        };

        assert!(it.next().is_none());
    }
}

mod image {
    use super::*;

    #[test]
    fn test_multiple_channels() {
        let mut v = vec![1u8, 2, 3, 2u8, 3, 4, 3u8, 4, 5, 4u8, 5, 6];
        let mut i = Image::from_slice(&mut v, (3, 2), 2).into_chunks_mut(4, 1);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.0, 0);
        assert_eq!(chunk.x_range.1, 3);
        assert_eq!(chunk.y_range.0, 0);
        assert_eq!(chunk.y_range.1, 2);
        assert_eq!(chunk[(0, 0)], 1u8);
        assert_eq!(chunk[(0, 1)], 2u8);
        assert_eq!(chunk[(0, 2)], 3u8);
        assert_eq!(chunk[(1, 0)], 2u8);
        assert_eq!(chunk[(1, 1)], 3u8);
        assert_eq!(chunk[(1, 2)], 4u8);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.0, 0);
        assert_eq!(chunk.x_range.1, 3);
        assert_eq!(chunk.y_range.0, 0);
        assert_eq!(chunk.y_range.1, 2);
        assert_eq!(chunk[(0, 0)], 3u8);
        assert_eq!(chunk[(0, 1)], 4u8);
        assert_eq!(chunk[(0, 2)], 5u8);
        assert_eq!(chunk[(1, 0)], 4u8);
        assert_eq!(chunk[(1, 1)], 5u8);
        assert_eq!(chunk[(1, 2)], 6u8);

        assert!(i.next().is_none(), "Should not generate elements!");
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_channel_should_panic() {
        let mut v = vec![1u8, 2, 3, 2u8, 3, 4, 3u8, 4, 5, 4u8, 5, 6];
        let mut i = Image::from_slice(&mut v, (3, 2), 2).into_chunks_mut(4, 1);

        let chunk = i.next().unwrap();
        let _ = chunk[(2, 0)];
    }

    #[test]
    fn test_chunks_division() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];
        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(2, 2);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.1 - chunk.x_range.0, 2);
        assert_eq!(chunk.y_range.1 - chunk.y_range.0, 2);
        assert_eq!(chunk[(0, 0)], 1u8);
        assert_eq!(chunk[(0, 1)], 2u8);
        assert_eq!(chunk[(1, 0)], 2u8);
        assert_eq!(chunk[(1, 1)], 3u8);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.1 - chunk.x_range.0, 2);
        assert_eq!(chunk.y_range.1 - chunk.y_range.0, 2);
        assert_eq!(chunk.x_range.0, 2);
        assert_eq!(chunk.x_range.1, 4);
        assert_eq!(chunk.y_range.0, 0);
        assert_eq!(chunk.y_range.1, 2);
        assert_eq!(chunk[(0, 2)], 3u8);
        assert_eq!(chunk[(0, 3)], 4u8);
        assert_eq!(chunk[(1, 2)], 4u8);
        assert_eq!(chunk[(1, 3)], 5u8);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.1 - chunk.x_range.0, 1);
        assert_eq!(chunk.y_range.1 - chunk.y_range.0, 2);
        assert_eq!(chunk[(0, 4)], 5u8);
        assert_eq!(chunk[(1, 4)], 6u8);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.0, 0);
        assert_eq!(chunk.x_range.1, 2);
        assert_eq!(chunk.y_range.0, 2);
        assert_eq!(chunk.y_range.1, 4);
        assert_eq!(chunk.x_range.1 - chunk.x_range.0, 2);
        assert_eq!(chunk.y_range.1 - chunk.y_range.0, 2);
        assert_eq!(chunk[(2, 0)], 3u8);
        assert_eq!(chunk[(2, 1)], 4u8);
        assert_eq!(chunk[(3, 0)], 4u8);
        assert_eq!(chunk[(3, 1)], 5u8);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.1 - chunk.x_range.0, 2);
        assert_eq!(chunk.y_range.1 - chunk.y_range.0, 2);
        assert_eq!(chunk[(2, 2)], 5u8);
        assert_eq!(chunk[(2, 3)], 6u8);
        assert_eq!(chunk[(3, 2)], 6u8);
        assert_eq!(chunk[(3, 3)], 7u8);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.1 - chunk.x_range.0, 1);
        assert_eq!(chunk.y_range.1 - chunk.y_range.0, 2);
        assert_eq!(chunk[(2, 4)], 7u8);
        assert_eq!(chunk[(3, 4)], 8u8);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.1 - chunk.x_range.0, 2);
        assert_eq!(chunk.y_range.1 - chunk.y_range.0, 1);
        assert_eq!(chunk[(4, 0)], 5u8);
        assert_eq!(chunk[(4, 1)], 6u8);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.1 - chunk.x_range.0, 2);
        assert_eq!(chunk.y_range.1 - chunk.y_range.0, 1);
        assert_eq!(chunk[(4, 2)], 7u8);
        assert_eq!(chunk[(4, 3)], 8u8);

        let chunk = i.next().unwrap();
        assert_eq!(chunk.x_range.1 - chunk.x_range.0, 1);
        assert_eq!(chunk.y_range.1 - chunk.y_range.0, 1);
        assert_eq!(chunk[(4, 4)], 9u8);

        assert!(i.next().is_none(), "Should not generate elements!");
    }

    #[test]
    fn test_owned_chunk_all_can_be_read() {
        let mut v = vec![1u8, 2, 3, 2u8, 3, 4, 3u8, 4, 5];
        let mut i = Image::from_slice(&mut v, (3, 3), 1).into_chunks_mut(3, 1);
        let chunk = i.next().unwrap();
        assert_eq!(chunk[(0, 0)], 1u8);
        assert_eq!(chunk[(0, 1)], 2u8);
        assert_eq!(chunk[(0, 2)], 3u8);
        assert_eq!(chunk[(1, 0)], 2u8);
        assert_eq!(chunk[(1, 1)], 3u8);
        assert_eq!(chunk[(1, 2)], 4u8);
        assert_eq!(chunk[(2, 0)], 3u8);
        assert_eq!(chunk[(2, 1)], 4u8);
        assert_eq!(chunk[(2, 2)], 5u8);
    }

    #[test]
    fn test_owned_chunk_write_to_mutable() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];
        {
            let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(6, 2);
            let mut chunk = i.next().unwrap();
            chunk[(0, 0)] = 0;
            chunk[(0, 2)] = 0;
            chunk[(2, 0)] = 0;
            chunk[(2, 2)] = 0;
            chunk[(2, 4)] = 0;
            chunk[(4, 2)] = 0
        }

        let expected = vec![
            0u8, 2, 0, 4, 5, 2u8, 3, 4, 5, 6, 0u8, 4, 0, 6, 0, 4u8, 5, 6, 7, 8, 5u8, 6, 0, 8, 9,
        ];

        assert_eq!(v, expected);
    }

    #[test]
    #[should_panic]
    fn test_owned_chunk_write_to_immutable1() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];

        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(6, 2);
        let mut chunk = i.next().unwrap();
        chunk[(0, 1)] = 0
    }

    #[test]
    #[should_panic]
    fn test_owned_chunk_write_to_immutable2() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];

        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(6, 2);
        let mut chunk = i.next().unwrap();
        chunk[(0, 3)] = 0;
    }

    #[test]
    #[should_panic]
    fn test_owned_chunk_write_to_immutable3() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];

        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(6, 2);
        let mut chunk = i.next().unwrap();
        chunk[(0, 4)] = 0;
    }

    #[test]
    #[should_panic]
    fn test_owned_chunk_write_to_immutable4() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];

        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(6, 2);
        let mut chunk = i.next().unwrap();
        chunk[(1, 2)] = 0;
    }

    #[test]
    #[should_panic]
    fn test_owned_chunk_write_to_immutable5() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];

        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(6, 2);
        let mut chunk = i.next().unwrap();
        chunk[(2, 1)] = 0;
    }

    #[test]
    #[should_panic]
    fn test_owned_chunk_write_to_immutable6() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];

        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(6, 2);
        let mut chunk = i.next().unwrap();
        chunk[(2, 3)] = 0;
    }

    #[test]
    #[should_panic]
    fn test_owned_chunk_write_to_immutable7() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];

        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(6, 2);
        let mut chunk = i.next().unwrap();
        chunk[(3, 3)] = 0;
    }

    #[test]
    #[should_panic]
    fn test_read_neighbour_mutable_should_panic1() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];

        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(2, 2);
        let mut chunk = i.next().unwrap();
        chunk[(0, 2)] = 0;
    }

    #[test]
    #[should_panic]
    fn test_read_neighbour_mutable_should_panic2() {
        let mut v = vec![
            1u8, 2, 3, 4, 5, 2u8, 3, 4, 5, 6, 3u8, 4, 5, 6, 7, 4u8, 5, 6, 7, 8, 5u8, 6, 7, 8, 9,
        ];

        let mut i = Image::from_slice(&mut v, (5, 5), 1).into_chunks_mut(2, 2);
        // skip 1 chunk
        i.next().unwrap();
        let mut chunk = i.next().unwrap();
        chunk[(0, 0)] = 0;
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_can_be_used_with_rayon() {
        use rayon::prelude::*;

        let mut v = vec![1u8, 2, 3, 4, 2u8, 3, 4, 5, 3u8, 4, 5, 6, 4u8, 5, 6, 7];
        Image::from_slice(&mut v, (4, 4), 1)
            .into_chunks_mut(2, 2)
            .par_bridge()
            .for_each(|mut chunk| {
                let (x0, _x1) = chunk.x_range;
                let (y0, _y1) = chunk.y_range;

                chunk[(y0, x0)] = 42;
            });

        let expected = vec![42u8, 2, 42, 4, 2u8, 3, 4, 5, 42u8, 4, 42, 6, 4u8, 5, 6, 7];

        assert_eq!(v, expected);
    }

    // If this function can be compiled - bad sings may happen.
    #[cfg(feature = "test_build_fails")]
    fn test_playground() {
        use std::thread;

        let mut v = vec![1u8, 2, 3, 4, 2u8, 3, 4, 5, 3u8, 4, 5, 6, 4u8, 5, 6, 7];

        let mut i = Image::from_slice(&mut v, (4, 4), 1).into_chunks_mut(2, 2);

        thread::spawn(|| i.next().unwrap());
    }
}

mod overlapping_chunks_iterator {
    use super::*;

    #[test]
    fn test_not_enough_chunks() {
        let mut v = vec![1u8, 2];
        let mut iter = OverlappingChunksIterator::from_slice(&mut v, 2);

        assert!(iter.next().is_none());
    }

    #[test]
    fn test_minimal_chunks() {
        let mut v = vec![1u8, 2, 3, 4];
        let mut iter = OverlappingChunksIterator::from_slice(&mut v, 2);

        match iter.next() {
            Some((l, m, r)) => {
                assert_eq!(*l, [1u8, 2]);
                assert_eq!(*m, [3u8, 4]);
                assert_eq!(*r, []);
            }
            None => {
                panic!("Iterator should produce element");
            }
        }
    }

    #[test]
    fn test_incomplete_last_slice() {
        let mut v = vec![1u8, 2, 3, 4, 5];
        let mut iter = OverlappingChunksIterator::from_slice(&mut v, 2);

        match iter.next() {
            Some((l, m, r)) => {
                assert_eq!(*l, [1u8, 2]);
                assert_eq!(*m, [3u8, 4]);
                assert_eq!(*r, [5u8]);
            }
            None => {
                panic!("Iterator should produce element");
            }
        }
    }

    #[test]
    fn test_two_iterations() {
        let mut v = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let mut iter = OverlappingChunksIterator::from_slice(&mut v, 2);

        match iter.next() {
            Some((l, m, r)) => {
                assert_eq!(*l, [1u8, 2]);
                assert_eq!(*m, [3u8, 4]);
                assert_eq!(*r, [5u8, 6]);
            }
            None => {
                panic!("Iterator should produce element");
            }
        }
        match iter.next() {
            Some((l, m, r)) => {
                assert_eq!(*l, [5u8, 6]);
                assert_eq!(*m, [7u8, 8]);
                assert_eq!(*r, []);
            }
            None => {
                panic!("Iterator should produce element");
            }
        }
        match iter.next() {
            Some(_) => {
                panic!("Iterator should not produce element");
            }
            None => {}
        }
    }

    #[test]
    fn test_independent_mutability() {
        let mut v = vec![1u8, 2, 3, 4];
        {
            let mut it = OverlappingChunksIterator::from_slice(&mut v, 1);
            let (_, m1, _) = it.next().unwrap();
            let (_, m2, _) = it.next().unwrap();

            m1[0] = 0;
            m2[0] = 1;
        }
        assert_eq!(v, &[1u8, 0, 3, 1])
    }

    #[test]
    fn test_pointer_equality() {
        let mut v = vec![1u8, 2, 3, 4, 5, 6, 7];
        let mut it = OverlappingChunksIterator::from_slice(&mut v, 1);

        let (_l, _, r1) = it.next().unwrap();
        let (l2, _, r2) = it.next().unwrap();
        let (l3, _, _r) = it.next().unwrap();

        assert_eq!(r1.as_ptr(), l2.as_ptr());
        assert_eq!(r2.as_ptr(), l3.as_ptr());
    }
}

mod variable_chunks_iterator {
    use super::*;

    #[test]
    fn test_chunk_generation() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9_u8];
        let chunk_sizes = vec![1, 3, 5];

        let mut it = VariableChunksIterator::new(&data, &chunk_sizes);

        assert_eq!(it.next().unwrap(), &[1]);
        assert_eq!(it.next().unwrap(), &[2, 3, 4]);
        assert_eq!(it.next().unwrap(), &[5, 6, 7, 8, 9]);
        assert!(it.next().is_none(), "Should not generate elements!");
    }

    #[test]
    fn test_last_chunk_truncated() {
        let data = vec![1, 2, 3];
        let chunk_sizes = vec![1, 10];

        let mut it = VariableChunksIterator::new(&data, &chunk_sizes);

        assert_eq!(it.next().unwrap(), &[1]);
        assert_eq!(it.next().unwrap(), &[2, 3]);
        assert!(it.next().is_none(), "Should not generate elements!");
    }

    #[test]
    fn test_not_enough_chunks() {
        let data = vec![42];
        let chunk_sizes = vec![1, 10, 55];

        let mut it = VariableChunksIterator::new(&data, &chunk_sizes);

        assert_eq!(it.next().unwrap(), &[42]);
        assert!(it.next().is_none(), "Should not generate elements!");
    }

    #[test]
    fn test_should_not_generate_remainder() {
        let data = vec![1, 2, 3, 4];
        let chunk_sizes = vec![1, 2];

        let mut it = VariableChunksIterator::new(&data, &chunk_sizes);

        assert_eq!(it.next().unwrap(), &[1]);
        assert_eq!(it.next().unwrap(), &[2, 3]);
        assert!(it.next().is_none(), "Should not generate elements!");
    }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_work_with_rayon() {
        use rayon::prelude::*;

        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9_u8];
        let chunk_sizes = vec![2, 3, 3, 42];

        let result = VariableChunksIterator::new(&data, &chunk_sizes)
            .par_bridge()
            .map(|slice| slice.last().unwrap())
            .sum::<u8>();

        // [1, <2>], [3, 4, <5>], [6, 7, <8>], [<9>] => 2 + 5 + 8 + 9 = 24
        assert_eq!(result, 24)
    }
}
