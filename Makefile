EXEC = onion_omega
HOST = root@Omega-FC8B.local
LOCAL = target/mipsel-unknown-linux-musl/release/$(EXEC)
REMOTE = $(HOST):/root/

TARGET = mipsel-unknown-linux-musl
CARGO = cargo
VPATH = src

.PHONY: clean run

$(LOCAL): main.rs
	@$(CARGO) build --release --target=$(TARGET)

clean:
	@$(CARGO) clean
	rm push

push: $(LOCAL)
	rsync -avPh $(LOCAL) $(REMOTE)
	touch push

run: push
	ssh $(HOST) /root/$(EXEC)
