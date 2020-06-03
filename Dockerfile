FROM fedora:30
RUN dnf install -y git
RUN git clone https://github.com/scylladb/seastar.git
RUN cd seastar && \
        ./install-dependencies.sh
RUN dnf install -y clang
RUN cd seastar && \
        ./configure.py \
            --mode=release \
            --compiler=clang++ \
            --cflags="-Wno-error" \
            --c-compiler=clang \
            --prefix=/usr/local && \
        ninja -C build/release install
ENV PKG_CONFIG_PATH /usr/local/lib64/pkgconfig
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN source ~/.cargo/env && rustup install beta
RUN dnf install -y lldb
