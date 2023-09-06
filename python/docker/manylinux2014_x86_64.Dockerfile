FROM quay.io/pypa/manylinux2014_x86_64
RUN yum install -y clang-devel openssl-devel blas-devel lapack-devel rsync
RUN git clone --depth=1 https://github.com/llvm/llvm-project.git /llvm-project \
 && cd /llvm-project \
 && mkdir build \
 && cd build
 && cmake -DLLVM_ENABLE_PROJECTS=clang -DCMAKE_BUILD_TYPE=Release -G "Unix Makefiles" ../llvm \
 && make \
 && cd / \
 && rsync -av /llvm-project/build/lib/ /lib/ \
 && rsync -av /llvm-project/build/bin/ /bin/ \
 && curl https://sh.rustup.rs -o /rustup.sh \
 && sh /rustup.sh -y \
 && rm -rf /llvm-project /rustup.sh
