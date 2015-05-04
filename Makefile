LINKARGS = "-mthumb -mcpu=cortex-m3 -Tefm32gg.ld --specs=nosys.specs -lgcc -lc -lnosys -lm"

OBJCOPY = arm-none-eabi-objcopy


OUT=circle-game

DEBUG_DIR=target/thumbv7m-none-eabi/debug

all: debug

debug: $(DEBUG_DIR)/$(OUT) $(DEBUG_DIR)/$(OUT).hex $(DEBUG_DIR)/$(OUT).bin $(DEBUG_DIR)/$(OUT).axf

$(DEBUG_DIR)/$(OUT): src/main.rs
	cargo linkargs $(LINKARGS) --target thumbv7m-none-eabi --verbose

%.hex: %
	$(OBJCOPY) -O ihex $< $@

%.bin: %
	$(OBJCOPY) -O binary $< $@

%.axf: %
	$(OBJCOPY) $< $@
