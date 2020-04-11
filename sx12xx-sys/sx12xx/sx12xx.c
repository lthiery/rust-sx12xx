#include "board.h"
#include <string.h>
#include "sx12xx.h"

typedef struct Internal_t
{
    void (*dio_irq_handles[NUM_IRQ_HANDLES])();
    RadioEvents_t   radio_events;
    Sx126xxState_t cur_event;
    uint16_t rx_len;
    int16_t rssi;
    int8_t snr;
} Internal_t;

static Internal_t internal;

void OnTxDone(void);

void OnRxDone(uint8_t * payload, uint16_t size, int16_t rssi, int8_t snr);

void OnTxTimeout(void);

void OnRxTimeout(void);

void OnRxError(void);

void
sx126xx_init(Radio_t * radio, BoardBindings_t * bindings_input)
{
    // store pointer to internal context for callback definitions
    bindings = bindings_input;

    // configure sx12xx radio events with local functions
    internal.radio_events.TxDone    = OnTxDone;
    internal.radio_events.RxDone    = OnRxDone;
    internal.radio_events.TxTimeout = OnTxTimeout;
    internal.radio_events.RxTimeout = OnRxTimeout;
    internal.radio_events.RxError   = OnRxError;

    // this function calls TimerInits and radio->IoIrqInit, which are
    // implemented here
    radio->Init(&internal.radio_events);

    // sleep the radio and wait for a send or receive call
    radio->Sleep();
}

Sx126xxState_t
sx126xx_handle_event(Radio_t * handle, Sx126xxEvent_t event)
{
    internal.cur_event = Sx126xxState_Busy;

    switch (event)
    {
    case Sx12xxEvent_DIO0:
        (*(internal.dio_irq_handles[0]))();
        break;
    case Sx12xxEvent_DIO1:
        // SX126x library only has the one handle, so fire it even fore DIO1
        (*(internal.dio_irq_handles[0]))();
        break;
    case Sx12xxEvent_DIO2:
        (*internal.dio_irq_handles[2])();
        break;
    case Sx12xxEvent_DIO3:
        (*internal.dio_irq_handles[3])();
        break;
    case Sx12xxEvent_DIO4:
        (*internal.dio_irq_handles[4])();
        break;
    case Sx12xxEvent_DIO5:
        (*internal.dio_irq_handles[5])();
        break;
    case Sx12xxEvent_Timer1:
        // TODO: needs to dispatch the callback stashed from TimerInit
        break;
    case Sx12xxEvent_Timer2:
        // TODO: needs to dispatch the callback stashed from TimerInit
        break;
    case Sx12xxEvent_Timer3:
        // TODO: needs to dispatch the callback stashed from TimerInit
        break;
    default:
        break;
    }

    return internal.cur_event;
}

// each sx12xx board invokes this during initialization
void
IoIrqInit(IrqHandler * irq_handlers[NUM_IRQ_HANDLES])
{
    for (uint32_t i = 0; i < NUM_IRQ_HANDLES; i++)
    {
        internal.dio_irq_handles[i] = irq_handlers[i];
    }
}

void
OnTxDone(void)
{
    internal.cur_event = Sx126xxState_TxDone;
}

void
OnRxDone(uint8_t * payload, uint16_t size, int16_t rssi, int8_t snr)
{
    internal.cur_event = Sx126xxState_RxDone;
    internal.rx_len    = size;
    internal.rssi = rssi;
    internal.snr  = snr;
}

void
OnTxTimeout(void)
{
    internal.cur_event = Sx126xxState_TxTimeout;
}

void
OnRxTimeout(void)
{
    internal.cur_event = Sx126xxState_RxTimeout;
}

void
OnRxError(void)
{
    internal.cur_event = Sx126xxState_RxError;
}