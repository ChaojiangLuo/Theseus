#![no_std]
#![feature(alloc)]

#![allow(dead_code)]

extern crate alloc;
extern crate volatile;
extern crate owning_ref;
extern crate memory;

use alloc::boxed::Box;
use owning_ref::{BoxRef, BoxRefMut};
use volatile::{Volatile, ReadOnly, WriteOnly};
use memory::{Frame,PageTable, ActivePageTable, PhysicalAddress, VirtualAddress, EntryFlags,
             MappedPages, allocate_pages,allocate_frame,FRAME_ALLOCATOR};
// ------------------------------------------------------------------------------------------------
// USB Request Type

pub static RT_TRANSFER_MASK:u8 =                0x80;
pub static RT_DEV_TO_HOST:u8 =                  0x80;
pub static RT_HOST_TO_DEV:u8 =                  0x00;

pub static  RT_TYPE_MASK:u8 =                   0x60;
pub static  RT_STANDARD:u8 =                    0x00;
pub static  RT_CLASS:u8 =                       0x20;
pub static  RT_VENDOR:u8 =                      0x40;

pub static  RT_RECIPIENT_MASK:u8 =              0x1f;
pub static  RT_DEV:u8 =                         0x00;
pub static  RT_INTF:u8 =                        0x01;
pub static  RT_ENDP:u8 =                        0x02;
pub static  RT_OTHER:u8 =                       0x03;

// ------------------------------------------------------------------------------------------------
// USB Device Requests

pub static REQ_GET_STATUS:u8 =                  0x00;
pub static REQ_CLEAR_FEATURE:u8 =               0x01;
pub static REQ_SET_FEATURE:u8 =                 0x03;
pub static REQ_SET_ADDR:u8 =                    0x05;
pub static REQ_GET_DESC:u8 =                    0x06;
pub static REQ_SET_DESC:u8 =                    0x07;
pub static REQ_GET_CONF:u8 =                    0x08;
pub static REQ_SET_CONF:u8 =                    0x09;
pub static REQ_GET_INTF:u8 =                    0x0a;
pub static REQ_SET_INTF:u8 =                    0x0b;
pub static REQ_SYNC_FRAME:u8 =                  0x0c;

// ------------------------------------------------------------------------------------------------
// USB Hub Class Requests

pub static REQ_CLEAR_TT_BUFFER:u8 =              0x08;
pub static REQ_RESET_TT:u8 =                     0x09;
pub static REQ_GET_TT_STATE:u8 =                 0x0a;
pub static REQ_STOP_TT:u8 =                      0x0b;

// ------------------------------------------------------------------------------------------------
// USB HID Interface Requests

pub static REQ_GET_REPORT:u8 =                   0x01;
pub static REQ_GET_IDLE:u8 =                     0x02;
pub static REQ_GET_PROTOCOL:u8 =                 0x03;
pub static REQ_SET_REPORT:u8 =                   0x09;
pub static REQ_SET_IDLE:u8 =                     0x0a;
pub static REQ_SET_PROTOCOL:u8 =                 0x0b;

// ------------------------------------------------------------------------------------------------
// USB Standard Feature Selectors

pub static F_DEVICE_REMOTE_WAKEUP:u8 =          1;   // Device
pub static F_ENDPOINT_HALT:u8 =                 2;  // Endpoint
pub static F_TEST_MODE:u8 =                     3;   // Device

// ------------------------------------------------------------------------------------------------
// USB Hub Feature Seletcors

pub static F_C_HUB_LOCAL_POWER:u8 =              0;   // Hub
pub static F_C_HUB_OVER_CURRENT:u8 =             1;   // Hub
pub static F_PORT_CONNECTION:u8 =                0;   // Port
pub static F_PORT_ENABLE:u8 =                    1;   // Port
pub static F_PORT_SUSPEND:u8 =                   2;   // Port
pub static F_PORT_OVER_CURRENT:u8 =              3;   // Port
pub static F_PORT_RESET:u8 =                     4;   // Port
pub static F_PORT_POWER:u8 =                     8;   // Port
pub static F_PORT_LOW_SPEED:u8 =                 9;   // Port
pub static F_C_PORT_CONNECTION:u8 =              16;  // Port
pub static F_C_PORT_ENABLE:u8 =                  17;  // Port
pub static F_C_PORT_SUSPEND:u8 =                 18;  // Port
pub static F_C_PORT_OVER_CURRENT:u8 =            19;  // Port
pub static F_C_PORT_RESET:u8 =                   20;  // Port
pub static F_PORT_TEST:u8 =                      21;  // Port
pub static F_PORT_INDICATOR:u8 =                 22;  // Port

// ------------------------------------------------------------------------------------------------
// USB Device Request

#[repr(C,packed)]
pub struct UsbDevReq
{
    pub dev_req_type: Volatile<u8>,
    pub request:          Volatile<u8>,
    pub value:        Volatile<u16>,
    pub index:        Volatile<u16>,
    pub len:          Volatile<u16>,
}

impl UsbDevReq{

    pub fn init( &mut self, dev_req_type: u8,request: u8, value: u16,
                index: u16, len: u16) {

        self.dev_req_type.write(dev_req_type);
        self.request.write(request);
        self.value.write(value);
        self.index.write(index);
        self.len.write(len);


    }
}

/// Box the the frame pointer
pub fn box_dev_req(active_table: &mut ActivePageTable,page: MappedPages)
                       -> Result<BoxRefMut<MappedPages, UsbDevReq>, &'static str> {
    let dev_req: BoxRefMut<MappedPages, UsbDevReq> = BoxRefMut::new(Box::new(page))
        .try_map_mut(|mp| mp.as_type_mut::<UsbDevReq>(0))?;

    Ok(dev_req)
}