# Generates README.md from the lib.rs docs
generate-readme:
    cargo readme -o README.md

# Checks if README.md needs to be updated
check-readme:
    mkdir -p target
    cargo readme -o target/README.md
    diff -q README.md target/README.md || ( \
        echo "README.md needs to be regenerated. Run 'just generate-readme'." \
        exit 1 \
    )

