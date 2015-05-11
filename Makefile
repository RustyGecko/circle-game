LINKARGS = "-mthumb -mcpu=cortex-m3 -Tefm32gg.ld --specs=nosys.specs -lgcc -lc -lnosys -lm"

OBJCOPY = arm-none-eabi-objcopy

OUT=circle-game

DEBUG_DIR=target/thumbv7m-none-eabi/debug
DEBUG_OUT=$(DEBUG_DIR)/$(OUT)
RELEASE_DIR=target/thumbv7m-none-eabi/release
RELEASE_OUT=$(RELEASE_DIR)/$(OUT)


.PHONY: all example clean debug-build release-build

all: debug

debug: debug-build $(DEBUG_OUT).hex $(DEBUG_OUT).bin $(DEBUG_OUT).axf
release: release-build $(RELEASE_OUT).hex $(RELEASE_OUT).bin $(RELEASE_OUT).axf

debug-build:
	cargo rustc --target thumbv7m-none-eabi --verbose -- -C link-args=$(LINKARGS)

release-build:
	cargo rustc --target thumbv7m-none-eabi --verbose --release -- -C link-args=$(LINKARGS)

%.hex: %
	$(OBJCOPY) -O ihex $< $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

%.axf: %
	$(OBJCOPY) $< $@

clean:
	cargo clean
