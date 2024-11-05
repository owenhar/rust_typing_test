# Maintainer: Owen Harris <eowenh0410@gmail.com>

pkgname="typing_test"
pkgver="1.0.2"
pkgrel=1
pkgdesc='Simple typing test for terminal'
url='https://github.com/owenhar/rust_typing_test'
license=('custom')
makedepends=('cargo')
depends=()
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
source=()
# b2sums=()

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    # install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
    install -Dm0755 "$srcdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm0644 "$srcdir/../words.txt" "$pkgdir/etc/words.txt"

    # for custom license, e.g. MIT
    # install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}