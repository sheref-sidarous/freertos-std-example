/*
 * FreeRTOS V202212.00
 * Copyright (C) 2020 Amazon.com, Inc. or its affiliates.  All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * https://www.FreeRTOS.org
 * https://github.com/FreeRTOS
 *
 */

/* Kernel includes. */
#include "FreeRTOS.h"
#include "task.h"
#include "timers.h"
#include "semphr.h"

#include "stm32f30x.h"
#include "stm32f3_discovery.h"

/* Priorities for the demo application tasks. */
#define mainFLASH_TASK_PRIORITY				( tskIDLE_PRIORITY + 1UL )

/*
 * Set up the hardware ready to run this demo.
 */
static void prvSetupHardware( void );

/*-----------------------------------------------------------*/

static void vLEDFlashTask1(void* args) {

  (void)args; // turn off that warning

   while (1)
  {
    /* Toggle LED3 */
    STM_EVAL_LEDToggle(LED3);
    /* Toggle LED7 */
    //STM_EVAL_LEDToggle(LED7);
    
    /* Insert 100 ms delay */
    vTaskDelay(100);
    
    /* Toggle LED4 */
    //STM_EVAL_LEDToggle(LED4);
    /* Toggle LED5 */
    //STM_EVAL_LEDToggle(LED5);
    
    /* Insert 150 ms delay */
    vTaskDelay(150);
  }
}

static void vLEDFlashTask2(void* args) {

  (void)args; // turn off that warning

   while (1)
  {
    /* Toggle LED6 */
    STM_EVAL_LEDToggle(LED6);
    /* Toggle LED10 */
    //STM_EVAL_LEDToggle(LED10);
    
    /* Insert 100 ms delay */
    vTaskDelay(100);
    
    /* Toggle LED9 */
    //STM_EVAL_LEDToggle(LED9);
    /* Toggle LED8 */
    //STM_EVAL_LEDToggle(LED8);
    
    /* Insert 150 ms delay */
    vTaskDelay(200);
  }
}

int main(void)
{
  /* Configure the hardware ready to run the test. */
  prvSetupHardware();


  /* Spawn the tasks. */
  xTaskCreate( vLEDFlashTask1, "LEDx", configMINIMAL_STACK_SIZE, NULL, mainFLASH_TASK_PRIORITY, ( TaskHandle_t * ) NULL );
  xTaskCreate( vLEDFlashTask2, "LEDx", configMINIMAL_STACK_SIZE, NULL, mainFLASH_TASK_PRIORITY, ( TaskHandle_t * ) NULL );

  /* Start the scheduler. */
  vTaskStartScheduler();

  /* If all is well, the scheduler will now be running, and the following line
  will never be reached.  If the following line does execute, then there was
  insufficient FreeRTOS heap memory available for the idle and/or timer tasks
  to be created.  See the memory management section on the FreeRTOS web site
  for more details. */
  for( ;; );
}
/*-----------------------------------------------------------*/

static void prvSetupHardware( void )
{
  /*!< At this stage the microcontroller clock setting is already configured, 
       this is done through SystemInit() function which is called from startup
       file (startup_stm32f30x.s) before to branch to application main.
       To reconfigure the default setting of SystemInit() function, refer to
       system_stm32f30x.c file
     */     
  
  /* Ensure all priority bits are assigned as preemption priority bits. */
  NVIC_PriorityGroupConfig( NVIC_PriorityGroup_4 );

  

  /* Configure the button input.  This configures the interrupt to use the
  lowest interrupt priority, so it is ok to use the ISR safe FreeRTOS API
  from the button interrupt handler. */
  STM_EVAL_PBInit( BUTTON_USER, BUTTON_MODE_EXTI );

    /* Initialize Leds mounted on STM32F3-discovery */
  STM_EVAL_LEDInit(LED3);
  STM_EVAL_LEDInit(LED4);
  STM_EVAL_LEDInit(LED5);
  STM_EVAL_LEDInit(LED6);
  STM_EVAL_LEDInit(LED7);
  STM_EVAL_LEDInit(LED8);
  STM_EVAL_LEDInit(LED9);
  STM_EVAL_LEDInit(LED10);
  
  /* Turn On LED3 */
  STM_EVAL_LEDOn(LED3);
  /* Turn On LED7 */
  STM_EVAL_LEDOn(LED7);
  /* Turn On LED6 */
  STM_EVAL_LEDOn(LED6);
  /* Turn On LED10 */
  STM_EVAL_LEDOn(LED10);
  
  /* Setup SysTick Timer for 1 msec interrupts.
     ------------------------------------------
    1. The SysTick_Config() function is a CMSIS function which configure:
       - The SysTick Reload register with value passed as function parameter.
       - Configure the SysTick IRQ priority to the lowest value (0x0F).
       - Reset the SysTick Counter register.
       - Configure the SysTick Counter clock source to be Core Clock Source (HCLK).
       - Enable the SysTick Interrupt.
       - Start the SysTick Counter.
    
    2. You can change the SysTick Clock source to be HCLK_Div8 by calling the
       SysTick_CLKSourceConfig(SysTick_CLKSource_HCLK_Div8) just after the
       SysTick_Config() function call. The SysTick_CLKSourceConfig() is defined
       inside the stm32f30x_misc.c file.

    3. You can change the SysTick IRQ priority by calling the
       NVIC_SetPriority(SysTick_IRQn,...) just after the SysTick_Config() function 
       call. The NVIC_SetPriority() is defined inside the core_cm0.h file.

    4. To adjust the SysTick time base, use the following formula:
                            
         Reload Value = SysTick Counter Clock (Hz) x  Desired Time base (s)
    
       - Reload Value is the parameter to be passed for SysTick_Config() function
       - Reload Value should not exceed 0xFFFFFF
   */
  if (SysTick_Config(SystemCoreClock / 1000))
  { 
    /* Capture error */ 
    while (1);
  }

}
/*-----------------------------------------------------------*/

void vApplicationTickHook( void )
{

}
/*-----------------------------------------------------------*/


void vApplicationMallocFailedHook( void )
{
  /* vApplicationMallocFailedHook() will only be called if
  configUSE_MALLOC_FAILED_HOOK is set to 1 in FreeRTOSConfig.h.  It is a hook
  function that will get called if a call to pvPortMalloc() fails.
  pvPortMalloc() is called internally by the kernel whenever a task, queue,
  timer or semaphore is created.  It is also called by various parts of the
  demo application.  If heap_1.c or heap_2.c are used, then the size of the
  heap available to pvPortMalloc() is defined by configTOTAL_HEAP_SIZE in
  FreeRTOSConfig.h, and the xPortGetFreeHeapSize() API function can be used
  to query the size of free heap space that remains (although it does not
  provide information on how the remaining heap might be fragmented). */
  taskDISABLE_INTERRUPTS();
  for( ;; );
}
/*-----------------------------------------------------------*/

void vApplicationIdleHook( void )
{
  /* vApplicationIdleHook() will only be called if configUSE_IDLE_HOOK is set
  to 1 in FreeRTOSConfig.h.  It will be called on each iteration of the idle
  task.  It is essential that code added to this hook function never attempts
  to block in any way (for example, call xQueueReceive() with a block time
  specified, or call vTaskDelay()).  If the application makes use of the
  vTaskDelete() API function (as this demo application does) then it is also
  important that vApplicationIdleHook() is permitted to return to its calling
  function, because it is the responsibility of the idle task to clean up
  memory allocated by the kernel to any task that has since been deleted. */
}
/*-----------------------------------------------------------*/

void vApplicationStackOverflowHook( TaskHandle_t pxTask, char *pcTaskName )
{
  ( void ) pcTaskName;
  ( void ) pxTask;

  /* Run time stack overflow checking is performed if
  configCHECK_FOR_STACK_OVERFLOW is defined to 1 or 2.  This hook
  function is called if a stack overflow is detected. */
  taskDISABLE_INTERRUPTS();
  for( ;; );
}
/*-----------------------------------------------------------*/
