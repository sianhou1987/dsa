# dsa
Learning DSA

https://dsa.cs.tsinghua.edu.cn/~deng/ds/dsacpp/


### Perf
https://www.ibm.com/developerworks/cn/linux/l-cn-perf1/index.html

wsl安装perf
**WARNING: most features of perf will not work because WSL doesnt support hardware counters**

**See the issue https://github.com/microsoft/WSL/issues/4678**
```shell
apt install flex bison
git clone https://github.com/microsoft/WSL2-Linux-Kernel --depth 1
cd WSL2-Linux-Kernel/tools/perf
make -j8
sudo cp perf /usr/local/bin
```
