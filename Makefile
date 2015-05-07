LINKARGS = "-mthumb -mcpu=cortex-m3 -Tefm32gg.ld --specs=nosys.specs -lgcc -lc -lnosys -lm"

OBJCOPY = arm-none-eabi-objcopy

OUT=circle-game

DEBUG_DIR=target/thumbv7m-none-eabi/debug
RELEASE_DIR=target/thumbv7m-none-eabi/release
DEBUG_OUT=$(DEBUG_DIR)/$(OUT)


.PHONY: all example clean debug-build release-build

all: debug

debug: debug-build $(DEBUG_OUT).hex $(DEBUG_OUT).bin $(DEBUG_OUT).axf
release: release-build $(RELEASE_DIR)/$(OUT).hex $(RELEASE_DIR)/$(OUT).bin $(RELEASE_DIR)/$(OUT).axf

debug-build:
	cargo linkargs $(LINKARGS) --target thumbv7m-none-eabi --verbose

release-build:
	cargo linkargs $(LINKARGS) --target thumbv7m-none-eabi --verbose --release

%.hex: %
	$(OBJCOPY) -O ihex $< $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

%.axf: %
	$(OBJCOPY) $< $@

clean:
	cargo clean
