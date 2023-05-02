#![allow(clippy::enum_glob_use)]

use super::*;
use crate::HexOrientation;
use std::f32::consts::PI;
use std::f32::EPSILON;

mod hex_directions {
    use super::Direction::*;
    use super::*;

    #[test]
    fn rotate_ccw_cw() {
        for direction in Direction::ALL_DIRECTIONS {
            assert_eq!(direction, direction.rotate_cw(6));
            assert_eq!(direction, direction.rotate_cw(12));
            assert_eq!(direction, direction.rotate_cw(1).rotate_ccw(1));
            assert_eq!(direction, direction.rotate_ccw(1).rotate_cw(1));
            assert_eq!(direction.counter_clockwise(), direction.rotate_ccw(1));
            assert_eq!(
                direction.counter_clockwise().counter_clockwise(),
                direction.rotate_ccw(2)
            );
            assert_eq!(direction.clockwise(), direction.rotate_cw(1));
            assert_eq!(direction.clockwise().clockwise(), direction.rotate_cw(2));
        }
    }

    #[test]
    fn rotations_reverse_each_other() {
        for direction in Direction::ALL_DIRECTIONS {
            assert_eq!(direction, direction.counter_clockwise().clockwise());
            assert_eq!(direction, direction.clockwise().counter_clockwise());
        }
    }

    #[test]
    fn six_rotations_comes_home() {
        for direction in Direction::ALL_DIRECTIONS {
            let mut clockwise_dir = direction;
            let mut counter_clockwise_dir = direction;

            for _ in 0..6 {
                clockwise_dir = clockwise_dir.counter_clockwise();
                counter_clockwise_dir = counter_clockwise_dir.clockwise();
            }

            assert_eq!(direction, clockwise_dir);
            assert_eq!(direction, counter_clockwise_dir);
        }
    }

    #[test]
    fn flat_angles_degrees() {
        let expected = [
            (TopRight, 30.0),
            (Top, 90.0),
            (TopLeft, 150.0),
            (BottomLeft, 210.0),
            (Bottom, 270.0),
            (BottomRight, 330.0),
        ];
        for (dir, angle) in expected {
            assert!(dir.angle_flat_degrees() - angle <= EPSILON);
        }
    }

    #[test]
    fn flat_angles_rad() {
        let expected = [
            (TopRight, PI / 6.0),
            (Top, PI / 2.0),
            (TopLeft, 5.0 * PI / 6.0),
            (BottomLeft, 7.0 * PI / 6.0),
            (Bottom, 3.0 * PI / 2.0),
            (BottomRight, 11.0 * PI / 6.0),
        ];
        let orientation = HexOrientation::flat();
        for (dir, angle) in expected {
            assert!(dir.angle_flat() - angle <= EPSILON);
            assert!(dir.angle(&orientation) - angle <= EPSILON);
        }
    }

    #[test]
    fn pointy_angles_degrees() {
        let expected = [
            (TopRight, 0.0),
            (Top, 60.0),
            (TopLeft, 120.0),
            (BottomLeft, 180.0),
            (Bottom, 240.0),
            (BottomRight, 300.0),
        ];
        for (dir, angle) in expected {
            assert!(dir.angle_pointy_degrees() - angle <= EPSILON);
        }
    }

    #[test]
    fn pointy_angles_rad() {
        let expected = [
            (TopRight, 0.0),
            (Top, PI / 3.0),
            (TopLeft, 2.0 * PI / 3.0),
            (BottomLeft, PI),
            (Bottom, 4.0 * PI / 3.0),
            (BottomRight, 5.0 * PI / 3.0),
        ];
        let orientation = HexOrientation::pointy();
        for (dir, angle) in expected {
            assert!(dir.angle_pointy() - angle <= EPSILON);
            assert!(dir.angle(&orientation) - angle <= EPSILON);
        }
    }

    #[test]
    fn diagonal_neighbors() {
        for dir in Direction::ALL_DIRECTIONS {
            assert_eq!(dir.diagonal_ccw().direction_cw(), dir);
            assert_eq!(dir.diagonal_cw().direction_ccw(), dir);
        }
    }
}

mod diagonal_direction {
    use super::*;

    #[test]
    fn dir_neighbors() {
        for dir in DiagonalDirection::ALL_DIRECTIONS {
            assert_eq!(dir.direction_ccw().diagonal_cw(), dir);
            assert_eq!(dir.direction_cw().diagonal_ccw(), dir);
        }
    }

    #[test]
    fn flat_angles_rad() {
        for dir in DiagonalDirection::ALL_DIRECTIONS {
            let expected = dir.direction_ccw().angle_flat() - PI / 6.0;
            assert!(dir.angle_flat() - expected <= EPSILON);
        }
    }

    #[test]
    fn pointy_angles_rad() {
        for dir in DiagonalDirection::ALL_DIRECTIONS {
            let expected = dir.direction_ccw().angle_pointy() - PI / 6.0;
            assert!(dir.angle_pointy() - expected <= EPSILON);
        }
    }

    #[test]
    fn flat_angles_degrees() {
        for dir in DiagonalDirection::ALL_DIRECTIONS {
            let expected = dir.direction_ccw().angle_flat_degrees() - 30.0;
            assert!(dir.angle_flat_degrees() - expected <= EPSILON);
        }
    }

    #[test]
    fn pointy_angles_degrees() {
        for dir in DiagonalDirection::ALL_DIRECTIONS {
            let expected = dir.direction_ccw().angle_pointy_degrees() - 30.0;
            assert!(dir.angle_pointy_degrees() - expected <= EPSILON);
        }
    }
}
