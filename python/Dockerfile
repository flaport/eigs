#FROM quay.io/pypa/manylinux2014_x86_64
#RUN git clone --depth=1 https://github.com/llvm/llvm-project.git \
# && cd llvm-project \
# && mkdir build \
# && cd build && cmake -DLLVM_ENABLE_PROJECTS=clang -DCMAKE_BUILD_TYPE=Release -G "Unix Makefiles" ../llvm \
# && make
FROM flaport/ci:init
RUN yum install -y clang-devel openssl-devel blas-devel lapack-devel rsync
RUN rsync -av /llvm-project/build/lib/ /lib/ \
 && rsync -av /llvm-project/build/bin/ /bin/ \
 && curl https://sh.rustup.rs -o /tmp/rustup.sh \
 && sh /tmp/rustup.sh -y
