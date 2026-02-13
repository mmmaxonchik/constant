FROM ubuntu:18.04 AS sysfilter_build
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-lc"]
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      build-essential gdb libc6-dbg libreadline-dev \
      git openssh-client ca-certificates lsb-release && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
RUN mkdir -p /root/.ssh && chmod 700 /root/.ssh && \
    ssh-keyscan -t rsa,ecdsa,ed25519 gitlab.com github.com >> /root/.ssh/known_hosts
RUN --mount=type=ssh \
    git clone --depth=1 git@gitlab.com:Egalito/sysfilter.git /workspace/sysfilter && \
    cd /workspace/sysfilter && \
    git fetch --depth=1 origin 1469319ba6ea7cab87638c1f879541e78d72d470 && \
    git checkout 1469319ba6ea7cab87638c1f879541e78d72d470 && \
    git submodule sync --recursive && \
    git submodule update --init --recursive
WORKDIR /workspace/sysfilter/
RUN cd extraction && make -j"$(nproc)"
# extraction/app/build_x86_64/sysfilter_extract
# libegalito.so => /workspace/sysfilter/extraction/egalito/src/libegalito.so (0x00007ff9736b2000)
# libcapstone.so.5 => /workspace/sysfilter/extraction/egalito/dep/capstone/install/lib/libcapstone.so.5 (0x00007ff9727cb000)
# libdistorm3.so => /workspace/sysfilter/extraction/egalito/dep/distorm3/make/linux/libdistorm3.so (0x00007ff9725ba000)
FROM ubuntu:18.04 AS syspart_build
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-lc"]
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      make g++ build-essential \
      libreadline-dev gdb lsb-release unzip \
      libc6-dbg libstdc++6-7-dbg \
      libunwind-dev python3 \
      git openssh-client ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
RUN mkdir -p /root/.ssh && chmod 700 /root/.ssh && \
    ssh-keyscan -t rsa,ecdsa,ed25519 github.com >> /root/.ssh/known_hosts
RUN --mount=type=ssh \
    git clone --depth=1 git@github.com:vidyalakshmir/SysPartCode.git /workspace/SysPartCode && \
    cd /workspace/SysPartCode && \
    git fetch --depth=1 origin 6ad3ce8f01d7cab4c036235a72c8c78a7a28a0f7 && \
    git checkout 6ad3ce8f01d7cab4c036235a72c8c78a7a28a0f7 && \
    git submodule sync --recursive && \
    git submodule update --init --recursive
WORKDIR /workspace/SysPartCode
RUN ./build.sh
# analysis/app/build_x86_64/syspart
# libegalito.so => /workspace/SysPartCode/analysis/tools/egalito/src/build_x86_64/libegalito.so (0x00007fa63a2b8000)
# libcapstone.so.4 => /workspace/SysPartCode/analysis/tools/egalito/dep/capstone/install/lib/libcapstone.so.4 (0x00007fa639511000)
# libdistorm3.so => /workspace/SysPartCode/analysis/tools/egalito/dep/distorm3/make/linux/libdistorm3.so (0x00007fa639300000)
FROM ubuntu:18.04 AS chestnut_clone
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-lc"]
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    git openssh-client ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
RUN mkdir -p /root/.ssh && chmod 700 /root/.ssh && \
    ssh-keyscan -t rsa,ecdsa,ed25519 gitlab.com github.com >> /root/.ssh/known_hosts
RUN --mount=type=ssh \
    git clone --depth=1 git@github.com:isec-tugraz/Chestnut.git /workspace/chestnut && \
    cd /workspace/chestnut && \
    git fetch --depth=1 origin 839c39d4ee91c993c92fd312ca70b7d0d7bd1b83 && \
    git checkout 839c39d4ee91c993c92fd312ca70b7d0d7bd1b83
WORKDIR /workspace/chestnut/
COPY test/src/chestnut/main.py /workspace/chestnut/Binalyzer/main.py
# Binalyzer
FROM ubuntu:18.04 AS confine_clone
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-lc"]
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    git openssh-client ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
RUN mkdir -p /root/.ssh && chmod 700 /root/.ssh && \
    ssh-keyscan -t rsa,ecdsa,ed25519 gitlab.com github.com >> /root/.ssh/known_hosts
RUN --mount=type=ssh \
    git clone --depth=1 git@github.com:shamedgh/confine.git /workspace/confine && \
    cd /workspace/confine && \
    git fetch --depth=1 origin a75dad3ad336e6258a42cd1ce4af061eea68ed76 && \
    git checkout a75dad3ad336e6258a42cd1ce4af061eea68ed76
WORKDIR /workspace/confine/
COPY test/src/confine/main.py /workspace/confine/main.py
# .
FROM ubuntu:18.04 AS tests_build
SHELL ["/bin/bash", "-lc"]
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
      git \
      openssh-client \
      make \
      cmake \
      strace \
      build-essential \
      gcc \
      g++ \
      clang \
      lld \
      llvm \
      musl-tools \
      golang-go \
      gawk; \
    rm -rf /var/lib/apt/lists/*
ENV RUST_VERSION=1.93.1
RUN set -eux; \
    curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain "${RUST_VERSION}"; \
    /root/.cargo/bin/rustup toolchain install "${RUST_VERSION}"; \
    /root/.cargo/bin/rustup default "${RUST_VERSION}"; \
    /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /workspace
COPY test/build-tests.sh /workspace/test/build-tests.sh
COPY test/src /workspace/test/src
RUN set -euxo pipefail; \
    mapfile -t tests < <(find test/src/c test/src/cpp test/src/go test/src/rust -maxdepth 1 -type f \
      \( -name 't*.c' -o -name 't*.cpp' -o -name 't*.go' -o -name 't*.rs' \) \
      | sed -E 's@.*/(t[^./]+)\.[^.]+$@\1@' | sort -u); \
    for t in "${tests[@]}"; do \
      echo "=== Building test matrix for ${t} ==="; \
      bash test/build-tests.sh "$t" || { \
        echo "=== build-tests failed for ${t}; printing build logs ==="; \
        find /workspace/test/build -name '*.build.log' -maxdepth 6 -type f -print -exec tail -n +1 {} \; || true; \
        exit 1; \
      }; \
    done
