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


    /*!
     * \brief  Run time initialization of library
     *
     * \param [IN] radio driver used
     * \param [IN] board bindings
     */
    void sx12xx_init(Radio_t *, BoardBindings_t *);

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
        Sx12xxEvent_Timer3  // unimplemented
    } Sx126xxEvent_t;

    typedef enum Sx126xxState
    {
        Sx126xxState_Busy,
        Sx126xxState_TxDone,
        Sx126xxState_RxDone,
        Sx126xxState_TxTimeout,
        Sx126xxState_RxTimeout,
        Sx126xxState_RxError,
    } Sx126xxState_t;

    /*!
     * \brief To be used by client in a low-priorty loop, feeding events into the library
     *
     * \param [IN] Radio_t
     * \param [IN] Sx126xxEvent   A system generated event
     *
     * \retval Sx126xxState_t should be handled by a client
     */
    Sx126xxState_t sx126xx_handle_event(Radio_t *, Sx126xxEvent_t);

#ifdef __cplusplus
}
#endif

#endif // __SX126xx_H__
