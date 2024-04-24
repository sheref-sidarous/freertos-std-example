# CMake minimum version
cmake_minimum_required (VERSION 3.1)

# Project Infomation
project( Demo )
enable_language(ASM)
enable_language(C)

# Reset output path
set (EXECUTABLE_OUTPUT_PATH ${CMAKE_BINARY_DIR}/bin)
set (LIBRARY_OUTPUT_PATH ${CMAKE_BINARY_DIR}/lib)

# STDLIB
set (CMAKE_SHARED_LIBRARY_LINK_C_FLAGS)

# Set include path
include_directories (.)
include_directories (./CMSIS/Device/ST/STM32F30x/Include)
include_directories (./CMSIS/Device/Include)
include_directories (./STM32F30x_StdPeriph_Driver/inc)
include_directories (../FreeRTOS-Kernel/include)
include_directories (../FreeRTOS-Kernel/portable/GCC/ARM_CM4F/)

set(CROSS_TARGET_TRIPLET "arm-none-eabi-")

# CC AR LD AS
set(CMAKE_C_COMPILER "${CROSS_TARGET_TRIPLET}gcc")
set(CMAKE_ASM_COMPILER "${CROSS_TARGET_TRIPLET}gcc")

# CFLAGS
set (CMAKE_C_FLAGS "-g -Wextra -Wshadow -Wimplicit-function-declaration -Wredundant-decls -Wmissing-prototypes -Wstrict-prototypes -fno-common -ffunction-sections -fdata-sections -MD -Wall -Wundef -mthumb -mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16" CACHE INTERNAL "c compiler flags")
set (CMAKE_C_FLAGS "${CMAKE_C_FLAGS}  -D USE_STDPERIPH_DRIVER  -D STM32F30X ")

# CXXFLAGS
set (CMAKE_CXX_FLAGS "-Wextra -Wshadow -Wredundant-decls  -Weffc++ -fno-common -ffunction-sections -fdata-sections -MD -Wall -Wundef -mthumb -mcpu=cortex-m4 " CACHE INTERNAL "cxx compiler flags")

# ASMFLAGS
set (CMAKE_ASM_FLAGS "-g -mthumb -mcpu=cortex-m4 " CACHE INTERNAL "asm compiler flags")

# LDFLAGS
set (CMAKE_EXE_LINKER_FLAGS "-g -Wl,-Map=Demo.map -mthumb -mcpu=cortex-m4 -nostdlib" CACHE INTERNAL "executable linker flags")
set (CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -T ${CMAKE_SOURCE_DIR}/STM32FLASH.ld -L ")

add_library( drivers STATIC
    ./STM32F30x_StdPeriph_Driver/src/stm32f30x_gpio.c
    ./STM32F30x_StdPeriph_Driver/src/stm32f30x_misc.c
    ./STM32F30x_StdPeriph_Driver/src/stm32f30x_rcc.c
    ./STM32F30x_StdPeriph_Driver/src/stm32f30x_syscfg.c
    ./STM32F30x_StdPeriph_Driver/src/stm32f30x_exti.c
)

add_library(startup STATIC


    ./CMSIS/Device/ST/STM32F30x/Source/startup_stm32f30x.s



)

add_library (freertos STATIC
    ../FreeRTOS-Kernel/list.c
    ../FreeRTOS-Kernel/queue.c
    ../FreeRTOS-Kernel/tasks.c
    ../FreeRTOS-Kernel/timers.c
    ../FreeRTOS-Kernel/portable/MemMang/heap_2.c
    ../FreeRTOS-Kernel/portable/GCC/ARM_CM4F/port.c
)

add_library( main STATIC
    ./stm32f3_discovery.c
    ./system_stm32f30x.c
    ./main.c

)

# Generate the target
add_executable (${CMAKE_PROJECT_NAME}.elf
    #../../../Libraries/CMSIS/Device/ST/STM32F30x/Source/Templates/iar/startup_stm32f30x.s
    ./empty.c





)

add_dependencies(${CMAKE_PROJECT_NAME}.elf startup)

# Link the library to the target
target_link_libraries (${CMAKE_PROJECT_NAME}.elf startup main drivers freertos)

# Generate the binary file
add_custom_target (${CMAKE_PROJECT_NAME}.bin ALL arm-none-eabi-objcopy -Obinary "${EXECUTABLE_OUTPUT_PATH}/${CMAKE_PROJECT_NAME}.elf" "${EXECUTABLE_OUTPUT_PATH}/${CMAKE_PROJECT_NAME}.bin" DEPENDS ${EXECUTABLE_OUTPUT_PATH}/${CMAKE_PROJECT_NAME}.elf)

# Generate the hex file
add_custom_target (${CMAKE_PROJECT_NAME}.hex ALL arm-none-eabi-objcopy -Oihex "${EXECUTABLE_OUTPUT_PATH}/${CMAKE_PROJECT_NAME}.elf" "${EXECUTABLE_OUTPUT_PATH}/${CMAKE_PROJECT_NAME}.hex" DEPENDS ${EXECUTABLE_OUTPUT_PATH}/${CMAKE_PROJECT_NAME}.elf)

# Echo the size Infomation
add_custom_target (size ALL arm-none-eabi-size "${EXECUTABLE_OUTPUT_PATH}/${CMAKE_PROJECT_NAME}.elf" DEPENDS ${EXECUTABLE_OUTPUT_PATH}/${CMAKE_PROJECT_NAME}.elf)

# Make flash to the board by st-link
add_custom_target (flash COMMAND st-flash write ${EXECUTABLE_OUTPUT_PATH}/${CMAKE_PROJECT_NAME}.bin 0x8000000)

# Make clean-all
add_custom_target (clean-all COMMAND rm -rf ${CMAKE_BINARY_DIR}/*)
