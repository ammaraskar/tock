# default board and architecture
TOCK_BOARD ?= storm
TOCK_ARCH ?= cortex-m4


# rules for making the kernel
.PHONY: all
all: $(TOCK_BOARD)

$(TOCK_BOARD): boards/$(TOCK_BOARD)/
	$(MAKE) -C $<

clean: boards/$(TOCK_BOARD)/
	$(MAKE) clean -C $<

doc: boards/$(TOCK_BOARD)/
	$(MAKE) doc -C $<

program: boards/$(TOCK_BOARD)/
	$(MAKE) program -C $<

flash: boards/$(TOCK_BOARD)/
	$(MAKE) flash -C $<


# rule for making userland example applications
examples/%: userland/examples/%
	$(MAKE) -C $< TOCK_ARCH=$(TOCK_ARCH)

