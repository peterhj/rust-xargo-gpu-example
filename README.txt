Build command with cross compiled core (need `cargo install xargo` first):

    RUST_TARGET_PATH=`pwd` xargo rustc --target nvptx64-nvidia-cuda --release -- --emit=asm
