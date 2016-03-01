//! Module that describes QF_BV (closed quatifier-free formulas built over
//! FixedSizeBitVector) logic.
//!
//! Note that the functions and structs that are defined.

use smt::theories::{bitvec, core};
use smt::smt::{Logic, SMTNode};
use std::fmt;

define_sorts_for_logic!(QF_BV_Sorts,
                  BV -> bitvec::Sorts,
                  Core -> core::Sorts
                 );

define_fns_for_logic!(QF_BV_Fn,
                      BVOps -> bitvec::OpCodes,
                      CoreOps -> core::OpCodes
                     );

define_logic!(QF_BV,
              QF_BV_Fn,
              QF_BV_Sorts,
              map {
                  QF_BV_Sorts::BV(_) => bitvec::OpCodes::FreeVar
              }
             );
