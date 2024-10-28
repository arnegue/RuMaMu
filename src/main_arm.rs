use core::{
    cell::RefCell,
    fmt::Write,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use cortex_m::interrupt::Mutex;

use panic_halt as _;

use rumamu::seatalk::{seatalk::SeatalkMessage, seatalk_00::Sentence00, seatalk::MAX_SEATALK_LENGTH};
use rumamu::ship_data_traits::WaterDepth;

use stm32f4xx_hal::{
    pac::{self, interrupt, USART1},
    prelude::*,
    serial::{
        config::{Config, StopBits},
        CommonPins, Rx, Serial, Tx,
    },
};

use rtt_target::{rprint, rprintln, rtt_init_print};

// Stuff for Serial interrupts

static MESSAGE_BUFFER: Mutex<RefCell<Option<[u8; MAX_SEATALK_LENGTH]>>> = Mutex::new(RefCell::new(None)); // Shared buffer for messages
static BUFFER_INDEX: AtomicUsize = AtomicUsize::new(0); // Message length to be shared
static BUFFER_FILLED: AtomicBool = AtomicBool::new(false); // Notifier that message transmission is complete

pub fn real_arm_main() -> ! {
    rtt_init_print!();

    // General HAL-Stuff
    let device = pac::Peripherals::take().unwrap();
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let _ = device.SYSCFG.constrain();
    let mut delay = device.TIM2.delay_ms(&clocks);
    let gpioa = device.GPIOA.split();

    // LED
    let mut led = gpioa.pa5.into_push_pull_output();

    // Serial
    let usart1_tx_pin = gpioa.pa9.into_alternate();
    let usart1_rx_pin = gpioa.pa10.into_pull_down_input();

    let config = Config::default()
        .baudrate(4800.bps())
        .wordlength_9()
        .parity_none()
        .stopbits(StopBits::STOP1);

    let serial_instance = device
        .USART1
        .serial((usart1_tx_pin, usart1_rx_pin), config, &clocks)
        .unwrap()
        .with_u16_data(); // Make this Serial object use u16s instead of u8s

    let (mut tx, mut rx) = serial_instance.split();

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::USART1); // Enable UART-Receive-Interrupts
    }

    rx.listen(); // Interrupt to receive each byte on line

    let mut loop_counter = 0;
    loop {
        rprintln!("Hello, world! {}", loop_counter);
        led.toggle();
        delay.delay_ms(200u32);
        loop_counter += 1;

        if BUFFER_FILLED.load(Ordering::SeqCst) {
            cortex_m::interrupt::free(|cs| {
                if let Some(buffer) = MESSAGE_BUFFER.borrow(cs).borrow_mut().take() {
                    rprint!("New stuff received:");
                    let message_length = BUFFER_INDEX.load(Ordering::SeqCst);

                    let parse_result = Sentence00::parse_seatalk_data(buffer, message_length);
                    match parse_result {
                        Ok(sentence00) => rprint!(
                            "Depth: {} Transducer defect: {}\n",
                            sentence00.get_depth_cm(),
                            sentence00.transducer_defect
                        ),
                        Err(x) => rprintln!("Parsing failed {}", x as u8),
                    }

                    for val in &buffer[0..message_length] {
                        rprint!("{:02X} ", val);
                        //tx.write_char(*val as char);
                    }
                    rprint!("\n\n");
                }
            });

            BUFFER_FILLED.store(false, Ordering::SeqCst);
        }
    }
}

fn write_str<USART1: CommonPins>(tx: &mut Tx<pac::USART1>, my_string: &str) {
    let error_code = tx.write_str(my_string);
    match error_code {
        Ok(_) => rprintln!("Funktioniert!"),
        Err(x) => rprintln!("Funktioniert nicht {}", x),
    }
}

#[interrupt]
fn USART1() {
    static mut RECEIVED_FIRST_COMMAND_BIT: bool = false; // Indicator for startup to discard every byte until the first command-bit is received
    static mut BUFFER: [u8; MAX_SEATALK_LENGTH] = [0; MAX_SEATALK_LENGTH]; // Internal message buffer
    static mut INDEX: usize = 0; // Internal current message index

    let usart1_rb: &pac::usart1::RegisterBlock = unsafe { &*pac::USART1::ptr() };

    if usart1_rb.sr.read().rxne().bit_is_set() {
        let two_byte = usart1_rb.dr.read().dr().bits() as u16;

        // Command bit?
        if (two_byte >> 8) > 0 {
            cortex_m::interrupt::free(|cs| {
                MESSAGE_BUFFER.borrow(cs).replace(Some(*BUFFER));
                BUFFER_INDEX.store(*INDEX, Ordering::SeqCst);
                BUFFER_FILLED.store(true, Ordering::SeqCst);
            });

            *RECEIVED_FIRST_COMMAND_BIT = true;
            *INDEX = 0; // Reset the index for the next message
        }

        if *RECEIVED_FIRST_COMMAND_BIT && *INDEX < BUFFER.len() {
            BUFFER[*INDEX] = two_byte as u8; // Cast down that (we don't need the command bit information anymore. The command byte is always at Idx 0)
            *INDEX += 1;
        }
    }
}
