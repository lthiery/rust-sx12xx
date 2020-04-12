#ifndef __SX126xx_H__
#define __SX126xx_H__

#ifdef __cplusplus
extern "C"
{
#endif

#include "board.h"
#include "radio.h"
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

    typedef enum Sx126xxEvent
    {
        Sx12xxEvent_DIO0,   // TxDone or Rx
        Sx12xxEvent_DIO1,   // unimplemented
        Sx12xxEvent_DIO2,   // unimplemented
        Sx12xxEvent_DIO3,   // unimplemented
        Sx12xxEvent_DIO4,   // unimplemented
        Sx12xxEvent_DIO5,   // unimplemented
        Sx12xxEvent_Timer1, // unimplemented
        Sx12xxEvent_Timer2, // unimplemented
        Sx12xxEvent_Timer3 = 0xFFFFFFFF // force 32-bit value,
    } Sx12xxEvent_t;

    typedef enum Sx126xxState
    {
        Sx12xxState_Busy,
        Sx12xxState_TxDone,
        Sx12xxState_RxDone,
        Sx12xxState_TxTimeout,
        Sx12xxState_RxTimeout,
        Sx12xxState_RxError = 0xFFFFFFFF // force 32-bit value,
    } Sx12xxState_t;

    typedef struct Sx12xx
    {
        void (*dio_irq_handles[NUM_IRQ_HANDLES])();
        RadioEvents_t   radio_events;
        Sx12xxState_t state;
        uint16_t rx_len;
        int16_t rssi;
        int8_t snr;
        BoardBindings_t bindings;
    } Sx12xx_t;

    /*!
     * \brief  Run time initialization of library
     *
     */
    void sx12xx_init(Sx12xx_t *, Radio_t *, BoardBindings_t);

    /*!
     * \brief To be used by client in a low-priorty loop, feeding events into the library
     *
     */
    Sx12xxState_t sx12xx_handle_event(Sx12xx_t *, Sx12xxEvent_t);

#ifdef __cplusplus
}
#endif

#endif // __SX126xx_H__
