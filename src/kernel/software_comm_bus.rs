//! # Software Communication Module
//!
//! This module instantiates a global instance of MessageTable and defines Kernel Routines
//! and primitives which handle task communication.

use crate::utils::arch::is_privileged;
use crate::KernelError;

use crate::kernel::task_management::{get_curr_tid, release};
use crate::priv_execute;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::interrupt;

use crate::system::software_comm_bus::*;

use crate::system::types::MessageId;