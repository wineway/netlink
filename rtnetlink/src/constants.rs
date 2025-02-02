// SPDX-License-Identifier: MIT

pub const RTMGRP_LINK: u32 = 1;
pub const RTMGRP_NOTIFY: u32 = 2;
pub const RTMGRP_NEIGH: u32 = 4;
pub const RTMGRP_TC: u32 = 8;
pub const RTMGRP_IPV4_IFADDR: u32 = 16;
pub const RTMGRP_IPV4_MROUTE: u32 = 32;
pub const RTMGRP_IPV4_ROUTE: u32 = 64;
pub const RTMGRP_IPV4_RULE: u32 = 128;
pub const RTMGRP_IPV6_IFADDR: u32 = 256;
pub const RTMGRP_IPV6_MROUTE: u32 = 512;
pub const RTMGRP_IPV6_ROUTE: u32 = 1024;
pub const RTMGRP_IPV6_IFINFO: u32 = 2048;
pub const RTMGRP_DECNET_IFADDR: u32 = 4096;
pub const RTMGRP_DECNET_ROUTE: u32 = 16_384;
pub const RTMGRP_IPV6_PREFIX: u32 = 131_072;
pub const TIME_UNITS_PER_SEC: u32 = 1000000;
pub const ATM_CELL_SIZE: u32 = 53; /* ATM cell size incl. header */
pub const ATM_CELL_PAYLOAD: u32 = 48; /* ATM payload size */
pub const TC_LINKLAYER_MASK: u32 = 0x0F;

pub const SC_CLK_TCK: i32 = 2;
