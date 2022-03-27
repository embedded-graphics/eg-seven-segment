# Generates README.md from the lib.rs docs
generate-readme: _build-readme
    cp target/README.md README.md

# Checks if README.md needs to be updated
check-readme: _build-readme
    diff -q README.md target/README.md || ( \
        echo "README.md needs to be regenerated. Run 'just generate-readme'." \
        exit 1 \
    )

_build-readme:
    mkdir -p target
    cargo readme | \
    sed "/README-LINKS/d" \
    > target/README.md

base64-images:
    echo -n "[img1]: data:image/png;base64," > assets/styles.png_base64
    base64 -w0 assets/styles.png >> assets/styles.png_base64
