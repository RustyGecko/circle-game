LINKARGS = "-mthumb -mcpu=cortex-m3 -Tefm32gg.ld --specs=nosys.specs -lgcc -lc -lnosys -lm"

OBJCOPY = arm-none-eabi-objcopy

OUT=circle-game

DEBUG_DIR=target/thumbv7m-none-eabi/debug
DEBUG_OUT=$(DEBUG_DIR)/$(OUT)

.PHONY: all example clean

all: debug

debug: $(DEBUG_OUT).elf $(DEBUG_OUT).hex $(DEBUG_OUT).bin $(DEBUG_OUT).axf

%.elf:
	cargo linkargs $(LINKARGS) --target thumbv7m-none-eabi --verbose

%.hex: %
	$(OBJCOPY) -O ihex $< $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

%.axf: %
	$(OBJCOPY) $< $@

clean:
	cargo clean
